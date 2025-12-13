use std::fmt;

use crate::ast::new_array_expr::NewArrayExpr;
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::{expr::ExprRule, parsed_type::ParsedTypeRule, parsed_unit_type::ParsedUnitTypeRule};
use crate::lexer::token::TokenType;

pub struct NewArrayRule {}

impl fmt::Display for NewArrayRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NewArray")
    }
}

impl ParseRule<NewArrayExpr> for NewArrayRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::New).is_some() && (ParsedTypeRule {}.check_match(cursor))
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<NewArrayExpr> {
        parser.begin_range();
        parser.try_consume(TokenType::New)?;

        let array_type = parser.apply_rule(ParsedUnitTypeRule {}, "array type", None)?;
        
        let mut sizes = Vec::new();

        while let Some(_) = parser.try_consume(TokenType::LeftSquare) {
            let size_expr = parser.apply_rule(ExprRule {}, "size expression", Some(ErrMsg::ExpectedExpression))?;
            sizes.push(size_expr);
            parser.consume_or_diagnostic(TokenType::RightSquare);
        }
    
        Some(NewArrayExpr::new(sizes, array_type, parser.end_range()))
    }
}