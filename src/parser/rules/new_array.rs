use std::{array, fmt};

use crate::{ast::{ASTWrapper, new_array_expr::NewArrayExpr}, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, diagnostic::{self, ErrMsg}, rules::{expr::ExprRule, parsed_type::ParsedTypeRule}}, token::{Position, PositionRange, TokenType}};

pub struct NewArrayRule {}

impl fmt::Display for NewArrayRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NewArray")
    }
}

impl ParseRule<ASTWrapper<NewArrayExpr>> for NewArrayRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::Alloc).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<NewArrayExpr>> {
        let alloc_token = parser.try_consume(TokenType::Alloc)?;

        let array_type = parser.apply_rule(ParsedTypeRule {}, "array type", None)?;
        
        parser.consume_or_diagnostic(TokenType::LeftSquare);
        
        let array_size = parser.apply_rule(ExprRule {}, "size expression", Some(ErrMsg::ExpectedExpression))?;
        
        parser.consume_or_diagnostic(TokenType::RightSquare);
    
        let position = PositionRange::concat(&alloc_token.position, &parser.prev().position);
    
        Some(ASTWrapper::new_new_array_expr(array_size, array_type, position))
    }
}