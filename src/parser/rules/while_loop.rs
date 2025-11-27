use std::fmt;

use crate::{ast::{ASTWrapper, loop_expr::LoopExpr}, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, diagnostic::ErrMsg, rules::{block::BlockRule, expr::ExprRule}}, token::{PositionRange, Positioned, TokenType}};

pub struct WhileLoopRule {}

impl fmt::Display for WhileLoopRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WhileLoop")
    }
}

impl ParseRule<ASTWrapper<LoopExpr>> for WhileLoopRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::While).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<LoopExpr>> {
        let while_token = parser.try_consume(TokenType::While)?;
    
        let condition = parser.apply_rule(ExprRule {}, "while condition", Some(ErrMsg::ExpectedExpression))?;
        let body = parser.apply_rule(BlockRule {}, "while body", Some(ErrMsg::ExpectedBlock))?;
    
        let position = PositionRange::concat(&while_token.position, body.get_position());
    
        Some(ASTWrapper::new_while(condition, body, position))
    }
}