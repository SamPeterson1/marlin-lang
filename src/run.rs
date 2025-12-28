use std::collections::HashMap;
use std::{env, thread};
use std::fs::File;
use std::io::Read;
use std::path::Path;

use inkwell::context::Context;

use crate::ast::{AcceptsASTVisitor, Program};
use crate::codegen::CodeGen;
use crate::diagnostic::{self, Diagnostic};
use crate::lexer::Lexer;
use crate::logger::{DYN_CONSOLE_LOGGER, Log, LogTarget};
use crate::parser::ExprParser;
use crate::lexer::token::Token;
use crate::resolver::{SymbolTable, TypeResolver, VarResolver};
use crate::type_checker::TypeChecker;

fn read_file(file: &str, contents: &mut String) {
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

fn parse_file(file: &str, diagnostics: &mut Vec<Diagnostic>) -> Option<Program> {
    let mut contents = String::new();
    read_file(file, &mut contents);

    let lexer = Lexer::new(&DYN_CONSOLE_LOGGER, &contents, diagnostics);
    let tokens: Vec<Token> = lexer.parse();

    let parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, diagnostics);
    let program = parser.parse();

    Some(program)
}

impl Runner {
    pub fn new() -> Self {
        Runner {
            log_targets: std::slice::from_ref(&DYN_CONSOLE_LOGGER),
            diagnostics: HashMap::new(),
        }
    }

    pub fn run_files(&mut self, files: &[String]) {

        let thread_handles = files.to_vec().into_iter().map(|file| {
            thread::spawn(|| {
                let mut diagnostics = Vec::new();
                let program = parse_file(&file, &mut diagnostics);

                (file, diagnostics)
            })
        });

        thread_handles.for_each(|handle| {
            let (file, diagnostics) = handle.join().unwrap();

            self.diagnostics.insert(file.clone(), diagnostics);
        });

        for (file, diagnostics) in &self.diagnostics {
            for diagnostic in diagnostics {
                self.log_error(self.log_targets, &format!("In file {}: {}", file, diagnostic));
            }
        }
    }
}

pub struct Runner { 
    diagnostics: HashMap<String, Vec<Diagnostic>>,
    log_targets: &'static [&'static dyn LogTarget],
}


impl Log for Runner {
    fn get_source(&self) -> String {
        "Runner".to_string()
    }
}
