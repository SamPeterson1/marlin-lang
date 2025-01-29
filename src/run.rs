use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

use crate::expr::{ExprParser, ParseResult};
use crate::interpreter::Interpreter;
use crate::resolver::Resolver;
use crate::type_checker::TypeChecker;
use crate::{lexer, log};
use crate::token::Token;


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
    run(contents);
}

pub fn run_prompt() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    loop {
        print!("> ");

        stdout.flush().expect("Error flushing stdout");

        let mut line = String::new();

        match stdin.read_line(&mut line) {
            Ok(_) => run(line),
            Err(err) => panic!("Error reading user input: {}", err)
        };
    }
}

fn run(code: String) {
    println!("{}", code);

    let tokens: Vec<Token> = lexer::parse(&code);

    
    let parser = ExprParser::new(&tokens);


    let ParseResult { exprs, diagnostics: parse_diagnostics } = parser.parse();
        
    println!("Parsed: {:?}", exprs);

    let mut resolver = Resolver::new();
    let resolve_errors = resolver.resolve(&exprs);
    
    let mut type_checker = TypeChecker::new(&resolver);
    let type_errors = type_checker.check_types(&exprs);
    

    let mut all_errors = Vec::new();

    all_errors.extend(parse_diagnostics);
    all_errors.extend(resolve_errors);
    all_errors.extend(type_errors);

    if !all_errors.is_empty() {
        for error in &all_errors {
            log::log(error);
        }
    }

    let mut interpreter = Interpreter::new(resolver);

    println!("\nRunning...");
    interpreter.interpret(&exprs);
}