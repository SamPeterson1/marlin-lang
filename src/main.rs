use std::env;
use std::process;

mod run;
mod error;
mod log;
mod lexer;
mod token;
mod expr;
mod instruction;
mod compiler;
mod opcodes;
mod vm;

mod environment;

mod operator;
mod resolver;
mod type_checker;

fn main() {
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
}