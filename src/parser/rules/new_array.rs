use std::{array, fmt};

use crate::{ast::{ASTWrapper, new_array_expr::NewArrayExpr, parsed_type::ParsedUnitType}, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, diagnostic::{self, ErrMsg}, rules::{expr::ExprRule, parsed_type::ParsedTypeRule, parsed_unit_type::ParsedUnitTypeRule}}, token::{Position, PositionRange, TokenType}};

pub struct NewArrayRule {}

impl fmt::Display for NewArrayRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NewArray")
    }
}

impl ParseRule<ASTWrapper<NewArrayExpr>> for NewArrayRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::New).is_some() && (ParsedTypeRule {}.check_match(cursor))
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<NewArrayExpr>> {
        let new_token = parser.try_consume(TokenType::New)?;

        let array_type = parser.apply_rule(ParsedUnitTypeRule {}, "array type", None)?;
        
        let mut sizes = Vec::new();

        while let Some(_) = parser.try_consume(TokenType::LeftSquare) {
            let size_expr = parser.apply_rule(ExprRule {}, "size expression", Some(ErrMsg::ExpectedExpression))?;
            sizes.push(size_expr);
            parser.consume_or_diagnostic(TokenType::RightSquare);
        }

        let position = PositionRange::concat(&new_token.position, &parser.prev().position);
    
        Some(ASTWrapper::new_new_array_expr(sizes, array_type, position))
    }
}