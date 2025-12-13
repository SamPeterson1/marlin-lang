use std::fmt;

use crate::ast::loop_expr::LoopExpr;
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::block::BlockRule;
use crate::lexer::token::TokenType;

pub struct LoopRule {}

impl fmt::Display for LoopRule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Loop")
    }
}

impl ParseRule<LoopExpr> for LoopRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::Loop).is_some()
    }
    
    fn parse(&self, parser: &mut ExprParser) -> Option<LoopExpr> {
        parser.begin_range();
        parser.try_consume(TokenType::Loop)?;
    
        let body = parser.apply_rule(BlockRule {}, "loop body", Some(ErrMsg::ExpectedBlock))?;
            
        Some(LoopExpr::new_loop(body, parser.end_range()))
    }
}