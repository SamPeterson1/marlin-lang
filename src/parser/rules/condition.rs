use std::fmt;

use crate::{expr::{ASTNode, ASTWrapper, binary_expr::BinaryExpr}, logger::Log, parser::{ExprParser, ParseRule, rules::boolean_factor::BooleanFactorRule}, token::{Position, PositionRange, TokenType}};

pub struct ConditionRule {}

impl fmt::Display for ConditionRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Condition")
    }
}

//condition: [boolean_factor] (OR [boolean_factor])*
impl ParseRule<Box<dyn ASTNode>> for ConditionRule {
    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        parser.log_debug(&format!("Entering condition parser. Current token {:?}", parser.cur()));
    
        let mut boolean_factor = parser.apply_rule(BooleanFactorRule {});
        parser.log_parse_result(&boolean_factor, "boolean factor");
        let mut expr = boolean_factor?;
    
        while let Some(operator) = parser.try_consume(TokenType::Or) {
            boolean_factor = parser.apply_rule(BooleanFactorRule {});
            parser.log_parse_result(&boolean_factor, "boolean factor");
    
            expr = Box::new(ASTWrapper::new_binary(expr, boolean_factor?, operator.token_type));
        }
    
        Some(expr)
    }
}

