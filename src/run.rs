use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

use crate::compiler::{Compiler, CompilerResult};
use crate::error::DiagnosticType;
use crate::expr::{ExprParser, ParseResult};
use crate::logger::{LogSeverity, Logger};
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

fn run(code: String) {
    let logger = Logger::new("run");

    logger.log_brief_info("Running code");
    logger.log_detailed_info(format!("Source code: {}", code).as_str());

    logger.log_brief_info("Lexing code");
    let tokens: Vec<Token> = lexer::parse(&code);
    logger.log_brief_info("Done lexing");
    
    let parser = ExprParser::new(&tokens);

    logger.log_brief_info("Parsing code");
    let ParseResult { items, diagnostics: parse_diagnostics } = parser.parse();
    logger.log_brief_info("Done parsing");

    let mut symbol_table = SymbolTable::new();
    let resolve_errors = symbol_table.resolve(&items);
    
    let mut type_checker = TypeChecker::new(&symbol_table);
    let type_errors = type_checker.check_types(&items);
    
    let mut all_errors = Vec::new();

    all_errors.extend(parse_diagnostics);
    all_errors.extend(resolve_errors);
    all_errors.extend(type_errors);

    if !all_errors.is_empty() {
        for error in &all_errors {
            let log_severity = match error.diagnostic_type {
                DiagnosticType::Error => LogSeverity::Error,
                DiagnosticType::Warning => LogSeverity::Warning
            };

            logger.log_brief(log_severity, &format!("{}", error));
        }
    } else {
        logger.log_brief_info("No errors found");
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