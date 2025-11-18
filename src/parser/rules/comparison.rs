use std::fmt;

use crate::{ast::{ASTNode, ASTWrapper}, logger::Log, parser::{ExprParser, ParseRule, rules::term::TermRule}, token::TokenType};

pub struct ComparisonRule {}

impl fmt::Display for ComparisonRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Comparison")
    }
}

//term ((">" | ">=" | "<" | "<=") term)*
impl ParseRule<Box<dyn ASTNode>> for ComparisonRule {
    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        parser.log_debug(&format!("Entering comparison parser. Current token {:?}", parser.cur()));

        let mut term = parser.apply_rule(TermRule {});
        parser.log_parse_result(&term, "term expression");
        let mut expr = term?;

        let matches = [TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual];
        
        while let Some(operator) = parser.try_match(&matches) {
            term = parser.apply_rule(TermRule {});
            parser.log_parse_result(&term, "term expression");
            
            expr = Box::new(ASTWrapper::new_binary(expr, term?, operator.token_type));
        }

        Some(expr)
    }
}
