use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

use crate::logger::{LogLevel, LogSource, Logger};
use crate::parser::{ExprParser, ParseResult};
use crate::vm::VM;
use crate::lexer;
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

struct Runner { }

impl LogSource for Runner {
    fn get_source(&self) -> String {
        "Runner".to_string()
    }
}

fn run(code: String) {
    let runner = Runner {};

    Logger::log_info(&runner, "Running code");
    Logger::log_debug(&runner, format!("Source code: {}", code).as_str());

    Logger::log_info(&runner, "Lexing code");
    let tokens: Vec<Token> = lexer::parse(&code);
    Logger::log_info(&runner, "Done lexing");
    
    let parser = ExprParser::new(&tokens);

    Logger::log_info(&runner, "Parsing code");
    let ParseResult { program, diagnostics: parse_diagnostics } = parser.parse();
    Logger::log_info(&runner, "Done parsing");
}