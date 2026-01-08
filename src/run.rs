use std::collections::HashSet;
use std::sync::Arc;
use std::env;
use std::io;
use std::path::Path;

use crate::ast::Scope;
use crate::compiler::Compiler;
use crate::diagnostic::Diagnostic;
use crate::logger::{CONSOLE_LOGGER, FileLogger, Log};
use crate::parser::ExprParser;
use crate::lexer::Lexer;

static LOG_SOURCE: &str = "Runner";

enum ParseError {
    IoError(io::Error),
    Diagnostics(Vec<Diagnostic>),
}

fn log_diagnostics(scope: &Scope, diagnostics: &[Diagnostic]) -> bool {
    for diagnostic in diagnostics {
        LOG_SOURCE.log_error(&CONSOLE_LOGGER, format!("In scope {}: {}", scope.path.to_string(), diagnostic));
    }

    !diagnostics.is_empty()
}

async fn compile_scope(scope: Scope, compiler: Arc<Compiler<'_>>) {
    let path = scope.path.segments.iter().map(|s| s.clone()).collect::<Vec<_>>();
    let log_target = FileLogger::new(&Path::new("modules").join(Path::new(&path.join("_"))).with_extension("log"));
 
}

async fn read_file(file: impl AsRef<Path>) -> io::Result<String> {
    let working_dir = env::current_dir()?;
    let path = working_dir.join(Path::new(file.as_ref()));

    Ok(tokio::fs::read_to_string(path).await?)
}

async fn parse_file(file_path: impl AsRef<Path>) -> Result<Vec<Scope>, ParseError> {
    let contents = match read_file(&file_path).await {
        Ok(c) => c,
        Err(e) => {
            LOG_SOURCE.log_error(&CONSOLE_LOGGER, format!("Error reading file {}: {}", file_path.as_ref().display(), e));
            return Err(ParseError::IoError(e));
        }
    };

    let file_logger = FileLogger::new(&Path::new("parser").join(&file_path).with_extension("log"));

    let mut diagnostics = Vec::new();
    let lexer = Lexer::new(&file_logger, &contents, &mut diagnostics);
    let tokens = lexer.parse();

    if !diagnostics.is_empty() {
        return Err(ParseError::Diagnostics(diagnostics));
    }

    let parser = ExprParser::new(&file_logger, tokens, &mut diagnostics);
    let scopes = parser.parse();

    if !diagnostics.is_empty() {
        return Err(ParseError::Diagnostics(diagnostics));
    }

    Ok(scopes)
}  

pub async fn get_scopes(files: &[String]) -> Option<Vec<Scope>> {
    let mut thread_handles = Vec::new();

    for file in files {
        let future = parse_file(file.to_string());
        let thread_handle = tokio::spawn(future);
        thread_handles.push((file, thread_handle));
    }

    let mut scopes = Vec::new();
    let mut success = true;

    for (file, thread_handle) in thread_handles {
        match thread_handle.await.unwrap() {
            Ok(file_scopes) => {
                scopes.extend(file_scopes);
            },
            Err(parse_error) => {
                success = false;

                match parse_error {
                    ParseError::Diagnostics(diagnostics) => {
                        for diagnostic in diagnostics {
                            LOG_SOURCE.log_error(&CONSOLE_LOGGER, format!("In file {}: {}", file, diagnostic));
                        }
                    },
                    ParseError::IoError(err) => {
                        LOG_SOURCE.log_error(&CONSOLE_LOGGER, format!("I/O error in file {}: {}", file, err));
                    }
                }
            }
        }
    }

    if !success {
        LOG_SOURCE.log_error(&CONSOLE_LOGGER, "Aborting due to previous errors.");
        return None;
    }

    let mut flattened_scopes = Vec::new();
    let mut path_set = HashSet::new();

    for scope in scopes.into_iter().map(|scope| scope.flatten()).flatten() {
        flattened_scopes.push(scope);
    }

    for scope in &flattened_scopes {
        if !path_set.insert(&*scope.path) {
            LOG_SOURCE.log_error(&CONSOLE_LOGGER, format!("Duplicate scope path detected: {}", scope.path.to_string()));
        }
    }

    Some(flattened_scopes)
}

pub async fn run_files(files: Vec<String>) {
    let flattened_scopes = match get_scopes(&files).await {
        Some(scopes) => scopes,
        None => return,
    };
    
    let run_context = Arc::new(Compiler::new());

    let mut thread_handles = Vec::new();

    for scope in flattened_scopes.into_iter() {
        let future = compile_scope(scope, run_context.clone());
        let thread_handle = tokio::spawn(future); 
        thread_handles.push(thread_handle);
    }

    for handle in thread_handles {
        let _ = handle.await;
    }
}