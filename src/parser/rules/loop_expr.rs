use crate::{ast::{ASTWrapper, loop_expr::LoopExpr}, logger::Log, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, diagnostic::ErrMsg, rules::block::BlockRule}, token::{PositionRange, Positioned, TokenType}};
use std::fmt;

pub struct LoopRule {}

impl fmt::Display for LoopRule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Loop")
    }
}

impl ParseRule<ASTWrapper<LoopExpr>> for LoopRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::Loop).is_some()
    }
    
    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<LoopExpr>> {
        let loop_token = parser.try_consume(TokenType::Loop)?;
    
        let body = parser.apply_rule(BlockRule {}, "loop body", Some(ErrMsg::ExpectedBlock))?;
        
        let position = PositionRange::concat(&loop_token.position, body.get_position());
    
        Some(ASTWrapper::new_loop(body, position))
    }
}