use std::fmt;

use crate::{ast::{ASTNode, ASTWrapper, call_expr::CallExpr}, logger::Log, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, diagnostic::{self, ErrMsg}, rules::{arguments::ArgumentsRule, expr::ExprRule, primary::PrimaryRule}}, token::{Position, PositionRange, TokenType}};

pub struct CallRule {}

impl fmt::Display for CallRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Call")
    }
}

impl ParseRule<ASTWrapper<CallExpr>> for CallRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::Identifier).is_some() && cursor.try_consume(TokenType::AtSign).is_some()
    }
    
    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<CallExpr>> {
        let function_identifier = parser.try_consume(TokenType::Identifier)?;

        let arguments = parser.apply_rule(ArgumentsRule {}, "call arguments", None);

        let applied_to = parser.apply_rule(ExprRule {}, "expression applied to", Some(ErrMsg::ExpectedExpression))?;

        Some(ASTWrapper::new_call(&function_identifier, arguments, applied_to))
    }
}
