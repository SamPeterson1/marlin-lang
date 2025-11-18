use crate::{expr::{ASTWrapper, loop_expr::LoopExpr}, logger::Log, parser::{ExprParser, ParseRule, rules::block::BlockRule}, token::{PositionRange, TokenType}};
use std::fmt;

pub struct LoopRule {}

impl fmt::Display for LoopRule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Loop")
    }
}

//loop: LOOP [block]
impl ParseRule<ASTWrapper<LoopExpr>> for LoopRule {
    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<LoopExpr>> {
        parser.log_debug(&format!("Entering loop parser. Current token {:?}", parser.cur()));
        let loop_token = parser.advance();
    
        let body = parser.apply_rule_boxed(BlockRule {});
    
        parser.log_parse_result(&body, "loop body");
    
        let position = PositionRange::concat(&loop_token.position, &parser.prev().position);
    
        Some(ASTWrapper::new_loop(body?, position))
    }
}