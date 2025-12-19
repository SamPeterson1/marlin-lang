use std::fmt;

use crate::ast::{ASTNode, BinaryExpr, BinaryOperator};
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::bitwise_factor::BitwiseFactorRule;
use crate::lexer::token::TokenType;

pub struct BitwiseTermRule {}

impl fmt::Display for BitwiseTermRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BitwiseTerm")
    }
}

impl ParseRule<Box<dyn ASTNode>> for BitwiseTermRule {
    fn check_match(&self, _cursor: ParserCursor) -> bool {
        true
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        let mut expr = parser.apply_rule(BitwiseFactorRule {}, "bitwise factor expression", None)?;
        
        while let Some(operator) = parser.try_consume(TokenType::Bar) {
            let binary_operator: BinaryOperator = operator.value.try_into().unwrap();

            let factor = parser.apply_rule(BitwiseFactorRule {}, "bitwise factor expression", None)?;            
            expr = Box::new(BinaryExpr::new(expr, factor, binary_operator));
        }

        Some(expr)
    }
}
