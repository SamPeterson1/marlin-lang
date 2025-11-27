use std::fmt;

use crate::{ast::{ASTWrapper, if_expr::IfExpr}, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, diagnostic::ErrMsg, rules::{block::BlockRule, expr::ExprRule}}, token::{PositionRange, TokenType}};

pub struct IfBlockRule {}

impl fmt::Display for IfBlockRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IfBlock")
    }
}

impl ParseRule<ASTWrapper<IfExpr>> for IfBlockRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::If).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<IfExpr>> {
        let if_token = parser.try_consume(TokenType::If)?;
        
        let condition = parser.apply_rule(ExprRule {}, "if condition", Some(ErrMsg::ExpectedExpression))?;
    
        let success = parser.apply_rule(ExprRule {}, "if success", Some(ErrMsg::ExpectedBlock))?;
        
        let mut fail = None;

        if let Some(_) = parser.try_consume(TokenType::Else) {
            fail = parser.apply_rule_boxed(IfBlockRule {}, "else if block", None);

            if fail.is_none() {
                fail = parser.apply_rule_boxed(BlockRule {}, "else block", None);
            }
        }
    
        let position = PositionRange::concat(&if_token.position, &parser.prev().position);
    
        Some(ASTWrapper::new_if(condition, success, fail, position))
    }
}
