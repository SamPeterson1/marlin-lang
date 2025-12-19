use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use inkwell::context::Context;

use crate::ast::AcceptsASTVisitor;
use crate::codegen::CodeGen;
use crate::diagnostic::{self, Diagnostic};
use crate::lexer::Lexer;
use crate::logger::Log;
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

pub fn run_file(file: &str) {
    let mut contents = String::new();
    
    read_file(file, &mut contents);

    let runner = Runner::new(contents);
    runner.run();
}

struct Runner { 
    diagnostics: Vec<Diagnostic>,
    code: String
}

impl Log for Runner {
    fn get_source(&self) -> String {
        "Runner".to_string()
    }
}

impl Runner {
    fn new(code: String) -> Runner {
        Runner {
            diagnostics: Vec::new(),
            code
        }
    }

    fn run(mut self) {
        self.log_info("Running code");
        self.log_debug(&format!("Source code: {}", self.code));

        self.log_info("Lexing code");
        let lexer = Lexer::new(&self.code, &mut self.diagnostics);
        let tokens: Vec<Token> = lexer.parse();
        self.log_info("Done lexing");
        
        self.log_info("Parsing code");
        let parser = ExprParser::new(tokens, &mut self.diagnostics);
        let mut program = parser.parse();
        self.log_info("Done parsing");

        let mut symbol_table = SymbolTable::new();
        let type_resolver = TypeResolver::new(&mut symbol_table, &mut self.diagnostics);
        type_resolver.resolve(&program);

        let var_resolver = VarResolver::new(&mut symbol_table, &mut self.diagnostics);
        var_resolver.resolve_vars(&program);

        let mut type_checker = TypeChecker::new(&mut self.diagnostics, &mut symbol_table);
        program.accept_visitor_mut(&mut type_checker);
        
        if self.diagnostics.len() > 0 {
            for diagnostic in &self.diagnostics {
                self.log_error(&format!("{}", diagnostic));
            }
            self.log_error("Aborting due to previous errors");
            return;
        }

        let context = Context::create();
        let mut codegen = CodeGen::new(&context, &symbol_table);
        program.accept_visitor(&mut codegen);


        self.log_info("Compiling to executable");
        match codegen.compile_with_clang("a.out") {
            Ok(_) => self.log_info("Compilation successful"),
            Err(e) => {
                self.log_error(&format!("Compilation failed: {}", e));
                return;
            }
        }
    } 
}