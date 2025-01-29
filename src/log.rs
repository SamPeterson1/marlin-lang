use crate::error::Diagnostic;

pub fn log(diagnostic: &Diagnostic) {
    println!("{}", diagnostic);
}