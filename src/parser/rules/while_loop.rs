use std::fmt;

use crate::{expr::{get_char_expr::GetCharExpr, loop_expr::LoopExpr, put_char_expr::PutCharExpr}, logger::Log, parser::{ExprParser, ParseRule, rules::{block::BlockRule, inline_expr::InlineExprRule}}, token::{Position, PositionRange}};

pub struct WhileLoopRule {}

impl fmt::Display for WhileLoopRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WhileLoop")
    }
}

impl ParseRule<LoopExpr> for WhileLoopRule {
    fn parse(&self, parser: &mut ExprParser) -> Option<LoopExpr> {
        parser.log_info(&format!("Entering while parser. Current token {:?}", parser.cur()));
        let while_token = parser.advance();
    
        let condition = parser.apply_rule(InlineExprRule {});
        let body = parser.apply_rule_boxed(BlockRule {});
    
        parser.log_parse_result(&condition, "while condition");
        parser.log_parse_result(&body, "while body");
    
        let position = PositionRange::concat(&while_token.position, &parser.prev().position);
    
        Some(LoopExpr::new_while(condition?, body?, position))
    }
}