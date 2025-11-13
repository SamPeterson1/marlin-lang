use crate::{expr::{Expr, unary_expr::UnaryExpr}, logger::Log, parser::{ExprParser, ParseRule, rules::expr::ExprRule}, token::TokenType};
use std::fmt;

pub struct StatementRule {}

impl fmt::Display for StatementRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Statement")
    }
}

impl ParseRule<Box<dyn Expr>> for StatementRule {
    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn Expr>> {
        parser.log_debug(&format!("Entering statement parser. Current token {:?}", parser.cur()));
    
        let expr = parser.apply_rule(ExprRule {});
        
        parser.log_parse_result(&expr, "expression");
    
        if let Some(semicolon_token) = parser.try_consume(TokenType::Semicolon) {
            parser.log_debug(&format!("Parsed semicolon token"));
            Some(Box::new(UnaryExpr::new(expr?, semicolon_token)))
        } else {
            parser.log_debug(&format!("No semicolon found"));
            expr
        }
    }
}


