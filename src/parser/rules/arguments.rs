
use std::fmt;

use crate::{ast::{ASTWrapper, arguments::Arguments}, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, diagnostic::ErrMsg, rules::expr::ExprRule}, token::{PositionRange, TokenType}};

pub struct ArgumentsRule {}

impl fmt::Display for ArgumentsRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Arguments")
    }
}

impl ParseRule<ASTWrapper<Arguments>> for ArgumentsRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::LeftParen).is_some()
    }
    
    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<Arguments>> {
        let left_paren = parser.try_consume(TokenType::LeftParen)?;

        let mut arguments = Vec::new();
        
        //Check if the next token is a right paren to allow for empty argument lists
        if parser.try_consume(TokenType::RightParen).is_none() {
            if let Some(argument) = parser.apply_rule(ExprRule {}, "first argument", None) {
                arguments.push(argument);
    
                while let Some(_) = parser.try_consume(TokenType::Comma) {
                    let argument = parser.apply_rule(ExprRule {}, "argument", Some(ErrMsg::ExpectedExpression))?;
                    arguments.push(argument);
                }
            }

            parser.consume_or_diagnostic(TokenType::RightParen);
        }
        
        let position = PositionRange::concat(&left_paren.position, &parser.prev().position);

        Some(ASTWrapper::new_arguments(arguments, position))
    }
}