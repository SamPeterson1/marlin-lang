use std::fmt;

use crate::{expr::{get_char_expr::GetCharExpr, put_char_expr::PutCharExpr}, logger::Log, parser::{ExprParser, ParseRule}, token::{Position, PositionRange}};

pub struct GetcRule {}

impl fmt::Display for GetcRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Getc")
    }
}

impl ParseRule<GetCharExpr> for GetcRule {
    fn parse(&self, parser: &mut ExprParser) -> Option<GetCharExpr> {
        parser.log_debug(&format!("Entering getc parser. Current token {:?}", parser.cur()));
        parser.advance();
    
        Some(GetCharExpr::new(PositionRange::new(Position::new(0, 0))))
    }
}