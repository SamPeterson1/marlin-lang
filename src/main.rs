use dotenv::dotenv;
use std::env;
use std::process;

use crate::run::Runner;

mod ast;
//mod codegen;
mod diagnostic;
mod lexer;
mod logger;
mod parser;
mod resolver;
mod run;
//mod type_checker;

fn main() {
    dotenv().unwrap();

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    
    let len = args.len();

    if len < 1 {
        println!("Usage: untitled [script]");
        process::exit(1);
    } else {
        Runner::new().run_files(args);
    }
}