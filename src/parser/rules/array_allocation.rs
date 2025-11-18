use std::fmt;

use crate::{expr::{static_array_expr::StaticArrayExpr}, parser::{ExprParser, ParseRule, diagnostic}, token::{Position, PositionRange, TokenType}};

pub struct ArrayAllocationRule {}

impl fmt::Display for ArrayAllocationRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ArrayAlloc")
    }
}

impl ParseRule<StaticArrayExpr> for ArrayAllocationRule {
    fn parse(&self, parser: &mut ExprParser) -> Option<StaticArrayExpr> {
        let alloc_token = parser.advance();
        let array_type = parser.try_type();
    
        parser.log_parse_result(&array_type, "array type");
    
        parser.consume_or_diagnostic(TokenType::LeftSquare, diagnostic::err_expected_token(PositionRange::new(Position::new(0, 0)), TokenType::LeftSquare));
        
        let array_size = parser.consume_or_diagnostic(TokenType::IntLiteral, diagnostic::err_expected_token(PositionRange::new(Position::new(0, 0)), TokenType::IntLiteral))
            .map(|x| x.get_int() as usize);
    
        parser.log_parse_result(&array_size, "array size");
    
        parser.consume_or_diagnostic(TokenType::RightSquare, diagnostic::err_expected_token(PositionRange::new(Position::new(0, 0)), TokenType::RightSquare));
    
        let position = PositionRange::concat(&alloc_token.position, &parser.prev().position);
    
        Some(StaticArrayExpr::new(array_size?, array_type?, position))
    }
}