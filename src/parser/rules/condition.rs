use std::fmt;

use crate::{ast::{ASTNode, ASTWrapper}, logger::Log, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, rules::boolean_factor::BooleanFactorRule}, token::TokenType};

pub struct ConditionRule {}

impl fmt::Display for ConditionRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Condition")
    }
}

impl ParseRule<Box<dyn ASTNode>> for ConditionRule {
    fn check_match(&self, _cursor: ParserCursor) -> bool {
        true
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {    
        let mut expr = parser.apply_rule(BooleanFactorRule {}, "boolean factor", None)?;
    
        while let Some(operator) = parser.try_consume(TokenType::Or) {
            let boolean_factor = parser.apply_rule(BooleanFactorRule {}, "boolean factor", None);    
            expr = Box::new(ASTWrapper::new_binary(expr, boolean_factor?, operator.token_type));
        }
    
        Some(expr)
    }
}

