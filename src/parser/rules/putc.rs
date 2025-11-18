use std::fmt;

use crate::{ast::{ASTWrapper, put_char_expr::PutCharExpr}, parser::{ExprParser, ParseRule, rules::expr::ExprRule}, token::{Position, PositionRange}};

pub struct PutcRule {}

impl fmt::Display for PutcRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Putc")
    }
}

impl ParseRule<ASTWrapper<PutCharExpr>> for PutcRule {
    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<PutCharExpr>> {
        parser.advance();
    
        let expr = parser.apply_rule(ExprRule {});
    
        parser.log_parse_result(&expr, "putc expression");
    
        Some(ASTWrapper::new_put_char(expr?,PositionRange::new(Position::new(0, 0))))
    }
}