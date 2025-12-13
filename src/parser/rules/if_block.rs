use std::fmt;

use crate::ast::if_expr::IfExpr;
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::{block::BlockRule, expr::ExprRule};
use crate::lexer::token::TokenType;

pub struct IfBlockRule {}

impl fmt::Display for IfBlockRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IfBlock")
    }
}

impl ParseRule<IfExpr> for IfBlockRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::If).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<IfExpr> {
        parser.begin_range();
        parser.try_consume(TokenType::If)?;
        
        let condition = parser.apply_rule(ExprRule {}, "if condition", Some(ErrMsg::ExpectedExpression))?;
    
        let success = parser.apply_rule(ExprRule {}, "if success", Some(ErrMsg::ExpectedBlock))?;
        
        let mut fail = None;

        if let Some(_) = parser.try_consume(TokenType::Else) {
            fail = parser.apply_rule_boxed(IfBlockRule {}, "else if block", None);

            if fail.is_none() {
                fail = parser.apply_rule_boxed(BlockRule {}, "else block", None);
            }
        }
        
        Some(IfExpr::new(condition, success, fail, parser.end_range()))
    }
}
