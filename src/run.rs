use std::collections::HashMap;
use std::{env, thread};
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::diagnostic::Diagnostic;
use crate::lexer::Lexer;
use crate::logger::{CONSOLE_LOGGER, FileLogger, Log};
use crate::parser::ExprParser;
use crate::lexer::token::Token;
use crate::resolver::{SymbolTable, VarResolver};

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
            diagnostics: HashMap::new(),
        }
    }

    pub fn run_files(mut self, files: Vec<String>) {
        let thread_handles = files.into_iter().map(|file| {
            thread::spawn(|| {
                println!("Processing file: {}", file);

                let path = Path::new(&file);
                let mut file_runner = FileRunner::new(path);

                file_runner.run();
                let diagnostics = file_runner.into_diagnostics();

                (file, diagnostics)
            })
        });

        thread_handles.for_each(|handle| {
            let (file, diagnostics) = handle.join().unwrap();

            self.diagnostics.insert(file.clone(), diagnostics);
        });

        for (file, diagnostics) in &self.diagnostics {
            for diagnostic in diagnostics {
                self.log_error(&CONSOLE_LOGGER, &format!("In file {}: {}", file, diagnostic));
            }
        }
    }
}

pub struct Runner { 
    diagnostics: HashMap<String, Vec<Diagnostic>>,
}

impl Log for Runner {
    fn get_source(&self) -> String {
        "Runner".to_string()
    }
}

pub struct FileRunner<'ctx> {
    file_path: &'ctx Path,
    diagnostics: Vec<Diagnostic>,
    file_logger: FileLogger,
}

impl<'ctx> FileRunner<'ctx> {
    pub fn new(file_path: &'ctx Path) -> Self {
        FileRunner {
            file_path,
            diagnostics: Vec::new(),
            file_logger: FileLogger::new(file_path),
        }
    }

    pub fn run(&mut self) {
        let mut contents = String::new();
        read_file(self.file_path, &mut contents);
    
        let lexer = Lexer::new(&self.file_logger, &contents, &mut self.diagnostics);
        let tokens: Vec<Token> = lexer.parse();
    
        let parser = ExprParser::new(&self.file_logger, tokens, &mut self.diagnostics);
        let program = parser.parse();
    
        let mut symbol_table = SymbolTable::new();
    
        let var_resolver = VarResolver::new(&self.file_logger, &mut symbol_table, &mut self.diagnostics);
        var_resolver.resolve_vars(&program);

        self.log_info(&CONSOLE_LOGGER, &format!("Finished processing file {}", self.file_path.display()));
    }

    pub fn into_diagnostics(self) -> Vec<Diagnostic> {
        self.diagnostics
    }
}

impl Log for FileRunner<'_> {
    fn get_source(&self) -> String {
        format!("FileRunner({})", self.file_path.display())
    }
}