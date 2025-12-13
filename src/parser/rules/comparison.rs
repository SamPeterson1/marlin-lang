use std::fmt;

use crate::ast::{ASTNode, binary_expr::{BinaryExpr, BinaryOperator}};
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::term::TermRule;
use crate::lexer::token::TokenType;

pub struct ComparisonRule {}

impl fmt::Display for ComparisonRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Comparison")
    }
}

impl ParseRule<Box<dyn ASTNode>> for ComparisonRule {
    fn check_match(&self, _cursor: ParserCursor) -> bool {
        true
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {

        let mut expr = parser.apply_rule(TermRule {}, "term expression", None)?;
        let matches = [TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual];
        
        while let Some(operator) = parser.try_consume_match(&matches) {
            let binary_operator: BinaryOperator = operator.value.try_into().unwrap();

            let term = parser.apply_rule(TermRule {}, "term expression", None)?;            
            expr = Box::new(BinaryExpr::new(expr, term, binary_operator));
        }

        Some(expr)
    }
}
