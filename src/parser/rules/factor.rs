use std::fmt;

use crate::{expr::{ASTNode, ASTWrapper, binary_expr::BinaryExpr, declaration_expr::DeclarationExpr}, logger::Log, parser::{ExprParser, ParseRule, rules::{assignment::AssignmentRule, boolean_factor::BooleanFactorRule, comparison::ComparisonRule, expr::ExprRule, unary::UnaryRule}}, token::{Position, PositionRange, TokenType}};

pub struct FactorRule {}

impl fmt::Display for FactorRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Factor")
    }
}

//unary (("/" | "*")) unary)*
impl ParseRule<Box<dyn ASTNode>> for FactorRule {
    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        parser.log_debug(&format!("Entering factor parser. Current token {:?}", parser.cur()));

        let mut unary = parser.apply_rule(UnaryRule {});
        parser.log_parse_result(&unary, "unary expression");
        let mut expr = unary?;

        while let Some(operator) = parser.try_match(&[TokenType::Slash, TokenType::Star]) {
            unary = parser.apply_rule(UnaryRule {});
            parser.log_parse_result(&unary, "unary expression");
            expr = Box::new(ASTWrapper::new_binary(expr, unary?, operator.token_type));
        }

        Some(expr)
    }
}