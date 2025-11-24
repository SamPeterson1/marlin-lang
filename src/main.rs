use std::env;
use std::process;
use dotenv::dotenv;
use logger::Logger;

mod run;
mod logger;
mod lexer;
mod token;
mod parser;
mod ast;
mod instruction;
mod opcodes;
mod vm;
mod types;

mod operator;

fn main() {
    dotenv().unwrap();

    Logger::open();

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    
    let len = args.len();

    if len > 1 {
        println!("Usage: untitled [script]");
        process::exit(1);
    } else if len == 1 {
        run::run_file(&args[0]);
    } else {
        run::run_prompt();
    }

    Logger::close();
}