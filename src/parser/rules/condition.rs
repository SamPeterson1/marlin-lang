use std::fmt;

use crate::ast::{ASTNode, binary_expr::{BinaryExpr, BinaryOperator}};
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::boolean_factor::BooleanFactorRule;
use crate::lexer::token::TokenType;

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
            let binary_operator: BinaryOperator = operator.value.try_into().unwrap();

            let boolean_factor = parser.apply_rule(BooleanFactorRule {}, "boolean factor", None);    
            expr = Box::new(BinaryExpr::new(expr, boolean_factor?, binary_operator));
        }
    
        Some(expr)
    }
}

