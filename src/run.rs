use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use inkwell::context::Context;
use tokio::sync::Barrier;

use crate::ast::{ASTVisitor, AcceptsASTVisitor};
use crate::codegen::CodeGen;
use crate::logger::{CONSOLE_LOGGER, FileLogger, Log};
use crate::parser::ExprParser;
use crate::resolver::{GlobalSymbolTable, TypeResolver, VarResolver};
use crate::type_checker::TypeChecker;
use crate::lexer::Lexer;

fn read_file(file: &Path, contents: &mut String) {
    let working_dir = env::current_dir().expect("Error reading working directory");
    
    let mut path = Path::new(file);
    let path_buf = working_dir.join(path);

    path = path_buf.as_path();

    let display = path.display();

    let mut file = match File::open(path) {
        Err(err) => panic!("Couldn't open {}: {}", display, err),
        Ok(file) => file
    };

    if let Err(err) = file.read_to_string(contents) {
        panic!("Couldn't read {}: {}", display, err);
    }
}

impl Runner {
    pub fn new() -> Self {
        Runner {
        }
    }

    pub async fn run_files(self, files: Vec<String>) {
        let thread_handles = files.into_iter().map(|file| {
            tokio::spawn(async {
                let mut contents = String::new();
                let file_path = Path::new(&file);

                read_file(file_path, &mut contents);

                let file_logger = FileLogger::new(&Path::new("parser").join(file_path).with_extension("log"));

                let mut diagnostics = Vec::new();
                let lexer = Lexer::new(&file_logger, &contents, &mut diagnostics);
                let tokens = lexer.parse();

                if !diagnostics.is_empty() {
                    return Err((file, diagnostics));
                }

                let parser = ExprParser::new(&file_logger, tokens, &mut diagnostics);
                let scopes = parser.parse();

                if !diagnostics.is_empty() {
                    return Err((file, diagnostics));
                }

                Ok(scopes)
            })
        });

        let mut scopes = Vec::new();

        for handle in thread_handles {
            match handle.await.unwrap() {
                Ok(file_scopes) => {
                    scopes.extend(file_scopes);
                },
                Err((file, diagnostics)) => {
                    for diagnostic in diagnostics {
                        self.log_error(&CONSOLE_LOGGER, &format!("In file {}: {}", file, diagnostic));
                    }

                    self.log_error(&CONSOLE_LOGGER, "Aborting due to previous errors.");
                }
            }
        }

        let mut flattened_scopes = Vec::new();
        let mut path_set = HashSet::new();

        scopes.into_iter().map(|scope| scope.flatten()).flatten().for_each(|scope| {
            flattened_scopes.push(scope);
            let scope = flattened_scopes.last().unwrap();

            if path_set.contains(&scope.path) {
                self.log_error(&CONSOLE_LOGGER, &format!("Duplicate scope path detected: {}", scope.path.to_string()));
            } else {
                path_set.insert(scope.path.clone());
            }
        });

        let mut global_table = GlobalSymbolTable::new();

        for scope in &flattened_scopes {
            global_table.add_scope(scope);
        }

        let global_table_arc = Arc::new(global_table);
        let barrier = Arc::new(Barrier::new(flattened_scopes.len()));
        let barrier_2 = Arc::new(Barrier::new(flattened_scopes.len()));

        let mut thread_handles = Vec::new();

        for scope in flattened_scopes {
            let global_table_ref = Arc::clone(&global_table_arc);
            let barrier_ref = Arc::clone(&barrier);
            let barrier_2_ref = Arc::clone(&barrier_2);

            thread_handles.push(tokio::spawn(async move {
                let mut symbol_table = global_table_ref.get_symbol_table(&scope).unwrap();

                let path = scope.path.segments.iter().map(|s| s.data.clone()).collect::<Vec<_>>();
                let log_target = FileLogger::new(&Path::new("modules").join(Path::new(&path.join("_"))).with_extension("log"));

                
                let mut diagnostics = Vec::new();

                let mut var_resolver = VarResolver::new(&log_target, &global_table_ref, symbol_table, &mut diagnostics);
                var_resolver.resolve_vars(&scope);

                // Wait for all tasks to finish resolve_vars before any can proceed to finish_resolving
                barrier_ref.wait().await;

                var_resolver.finish_resolving();

                println!("Finished variable resolution for scope {}", scope.path.to_string());

                for diagnostic in &diagnostics {
                    println!("In scope {}: {}", scope.path.to_string(), diagnostic);
                }

                let mut diagnostics = Vec::new();
                let type_resolver = TypeResolver::new(&log_target, &global_table_ref, &symbol_table, &mut diagnostics);

                println!("Starting type resolution for scope {}", scope.path.to_string());

                type_resolver.resolve(&scope);

                for diagnostic in &diagnostics {
                    println!("In scope {}: {}", scope.path.to_string(), diagnostic);
                }

                println!("Finished type resolution for scope {}", scope.path.to_string());

                // Wait for all tasks to finish type resolution before any can proceed to type checking
                barrier_2_ref.wait().await;

                let mut diagnostics = Vec::new();
                let mut type_checker = TypeChecker::new(&log_target, &mut diagnostics, &global_table_ref, &symbol_table);
                println!("Starting type checking for scope {}", scope.path.to_string());
                type_checker.visit_scope(&scope);

                drop(type_checker);

                for diagnostic in &diagnostics {
                    println!("In scope {}: {}", scope.path.to_string(), diagnostic);
                }

                let context = Context::create();
                let mut codegen = CodeGen::new(&log_target, &context, &global_table_ref, &symbol_table);
                scope.accept_visitor(&mut codegen);

                println!("Finished code generation for scope {}", scope.path.to_string());
                codegen.output_ll(&format!("{}.ll", scope.path.to_string().replace("::", "_")));
            }));
        }

        for handle in thread_handles {
            let _ = handle.await;
        }
    }
}

#[derive(Clone, Copy)]
pub struct Runner { 
}

impl Log for Runner {
    fn get_source(&self) -> String {
        "Runner".to_string()
    }
}
