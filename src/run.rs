use std::collections::HashSet;
use std::sync::{Arc, Mutex, MutexGuard};
use std::{env, path, thread};
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::ast::Scope;
use crate::diagnostic::Diagnostic;
use crate::lexer::Lexer;
use crate::logger::{CONSOLE_LOGGER, FileLogger, Log, LogTarget};
use crate::parser::ExprParser;
use crate::resolver::{GlobalSymbolTable, SymbolTable, VarResolver};

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

    pub fn run_files(self, files: Vec<String>) {
        let thread_handles = files.into_iter().map(|file| {
            thread::spawn(|| {
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

        thread_handles.for_each(|handle| {
            match handle.join().unwrap() {
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
        });

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

        let mut thread_handles = Vec::new();
        
        let resolver_finished = Arc::new((Mutex::new(0), std::sync::Condvar::new()));
        let n_scopes = flattened_scopes.len();

        for scope in flattened_scopes {
            let global_table_ref = Arc::clone(&global_table_arc);
            let resolver_finished_ref = Arc::clone(&resolver_finished);

            thread_handles.push(thread::spawn(move || {
                let mut symbol_table = global_table_ref.get_symbol_table(&scope).unwrap();

                let path = scope.path.segments.iter().map(|s| s.data.clone()).collect::<Vec<_>>();
                let log_target = FileLogger::new(&Path::new("modules").join(Path::new(&path.join("_"))).with_extension("log"));

                
                let mut diagnostics = Vec::new();

                let mut var_resolver = VarResolver::new(&log_target, &global_table_ref, &mut symbol_table, &mut diagnostics);
                var_resolver.resolve_vars(&scope);

                let (lock, cvar) = &*resolver_finished_ref;
                let mut finished = lock.lock().unwrap();
                *finished = *finished + 1;

                cvar.notify_all();

                let _unused = cvar.wait_while(finished, |finished| *finished < n_scopes);
                drop(_unused);

                var_resolver.finish_resolving();

                for diagnostic in diagnostics {
                    self.log_error(&CONSOLE_LOGGER, &format!("In scope {}: {}", scope.path.to_string(), diagnostic));
                }
            }));
        }

        for handle in thread_handles {
            let _ = handle.join();
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
