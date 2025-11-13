use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

use crate::compiler::{Compiler, CompilerResult};
use crate::error::DiagnosticType;
use crate::logger::{LogLevel, LogSource, Logger};
use crate::parser::{ExprParser, ParseResult};
use crate::resolver::SymbolTable;
use crate::type_checker::TypeChecker;
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
    let ParseResult { items, diagnostics: parse_diagnostics } = parser.parse();
    Logger::log_info(&runner, "Done parsing");

    let mut symbol_table = SymbolTable::new();

    let mut all_errors = Vec::new();
    all_errors.extend(parse_diagnostics);

    if all_errors.len() == 0 {
        Logger::log_info(&runner, "Resolving symbols");
        let resolve_errors = symbol_table.resolve(&items);
        all_errors.extend(resolve_errors);
        Logger::log_info(&runner, "Done resolving symbols");
    } else {
        Logger::log_info(&runner, "Skipping symbol resolution due to previous errors");
    }
    
    if all_errors.len() == 0 {
        Logger::log_info(&runner, "Checking types");
        let mut type_checker = TypeChecker::new(&symbol_table);
        let type_errors = type_checker.check_types(&items);
        all_errors.extend(type_errors);
        Logger::log_info(&runner, "Done checking types");
    } else {
        Logger::log_info(&runner, "Skipping type checking due to previous errors");
    }

    if !all_errors.is_empty() {
        for error in &all_errors {
            let log_severity = match error.diagnostic_type {
                DiagnosticType::Error => LogLevel::Error,
                DiagnosticType::Warning => LogLevel::Warning
            };

            Logger::log(&runner, log_severity, &format!("{}", error));
        }
        Logger::log_info(&runner, "Errors found. Exiting");
        
        return;
    } else {
        Logger::log_info(&runner, "No errors found");
    }

    let mut compiler = Compiler::new(&symbol_table);
    let CompilerResult {instructions, constant_pool} = compiler.compile(&items);

    let mut vm = VM::new();

    vm.load_memory(0, &instructions, instructions.len());
    vm.load_memory(6000, &constant_pool, constant_pool.len());

    println!("Instructions: {:?}", instructions);
    println!("Number of instructions: {}", instructions.len());
    
    println!("Constant pool: {:?}", constant_pool);
    println!("Number of constants: {}", constant_pool.len());

    vm.run();

    println!("VM finished");
    println!("Result in R0: {}", vm.registers[0].as_u64());
}