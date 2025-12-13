use std::fmt;

use crate::ast::arguments::Arguments;
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::expr::ExprRule;
use crate::lexer::token::TokenType;

pub struct ArgumentsRule {}

impl fmt::Display for ArgumentsRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Arguments")
    }
}

impl ParseRule<Arguments> for ArgumentsRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::LeftParen).is_some()
    }
    
    fn parse(&self, parser: &mut ExprParser) -> Option<Arguments> {
        parser.begin_range();
        
        parser.try_consume(TokenType::LeftParen)?;

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
        
        Some(Arguments::new(arguments, parser.end_range()))
    }
}