#![feature(mapped_lock_guards)]

use dotenv::dotenv;
use std::env;
use std::process;

mod ast;
//mod codegen;
mod compiler;
mod diagnostic;
mod lexer;
mod logger;
mod parser;
//mod resolver;
mod run;
//mod type_checker;

#[tokio::main]
async fn main() {
    dotenv().unwrap();

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    
    let len = args.len();

    if len < 1 {
        println!("Usage: untitled [script]");
        process::exit(1);
    } else {
        run::run_files(args).await;
    }
}