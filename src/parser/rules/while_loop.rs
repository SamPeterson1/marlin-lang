use std::fmt;

use crate::ast::loop_expr::LoopExpr;
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::{block::BlockRule, expr::ExprRule};
use crate::lexer::token::TokenType;

pub struct WhileLoopRule {}

impl fmt::Display for WhileLoopRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WhileLoop")
    }
}

impl ParseRule<LoopExpr> for WhileLoopRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::While).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<LoopExpr> {
        parser.begin_range();
        parser.try_consume(TokenType::While)?;
    
        let condition = parser.apply_rule(ExprRule {}, "while condition", Some(ErrMsg::ExpectedExpression))?;
        let body = parser.apply_rule(BlockRule {}, "while body", Some(ErrMsg::ExpectedBlock))?;
    
        Some(LoopExpr::new_while(condition, body, parser.end_range()))
    }
}