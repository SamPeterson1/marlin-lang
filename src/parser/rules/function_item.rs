use std::fmt;

use crate::ast::{FunctionItem, ParsedType, ParsedTypeEnum};
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::{block::BlockRule, parameters::ParametersRule, parsed_type::ParsedTypeRule};
use crate::lexer::token::{PositionRange, TokenType};

pub struct FunctionRule;

impl fmt::Display for FunctionRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Function")
    }
}

impl ParseRule<FunctionItem> for FunctionRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::Fn).is_some()
         || cursor.try_consume(TokenType::Extern).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<FunctionItem> {
        parser.begin_range();

        let is_extern = parser.try_consume(TokenType::Extern).is_some();

        parser.try_consume(TokenType::Fn)?;

        let name = parser.consume_or_diagnostic(TokenType::AnyIdentifier)?.unwrap_identifier();

        let parameters = parser.apply_rule(ParametersRule {}, "function parameters", Some(ErrMsg::ExpectedParameters))?;

        let ret_type = if parser.try_consume(TokenType::Arrow).is_some() {
            parser.apply_rule(ParsedTypeRule {}, "return type", Some(ErrMsg::ExpectedType))?
        } else {
            ParsedType::new(ParsedTypeEnum::Void, PositionRange::zero())
        };

        let block = if !is_extern {
            let block = parser.apply_rule(BlockRule {}, "function body", Some(ErrMsg::ExpectedBlock))?;
            Some(block)
        } else {
            parser.consume_or_diagnostic(TokenType::Semicolon);
            None
        };

        Some(FunctionItem::new(name, parameters, ret_type, block, parser.end_range()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::{Token, TokenType, PositionRange};
    use crate::logger::CONSOLE_LOGGER;
    use crate::parser::ExprParser;

    fn create_parser_with_tokens(tokens: Vec<TokenType>) -> ExprParser<'static> {
        let diagnostics = Box::leak(Box::new(Vec::new()));
        let tokens: Vec<Token> = tokens
            .into_iter()
            .map(|token_type| Token::new(token_type, PositionRange::zero()))
            .collect();
        ExprParser::new(&CONSOLE_LOGGER, tokens, diagnostics)
    }

    #[test]
    fn test_function_check_match_with_fn_keyword() {
        let parser = create_parser_with_tokens(vec![
            TokenType::Fn,
            TokenType::Identifier("test".to_string()),
            TokenType::LeftParen,
            TokenType::RightParen,
            TokenType::Arrow,
            TokenType::Int,
            TokenType::LeftCurly,
            TokenType::RightCurly,
            TokenType::EOF,
        ]);
        let rule = FunctionRule;
        assert!(rule.check_match(parser.get_cursor()));
    }

    #[test]
    fn test_function_check_match_without_fn_keyword() {
        let parser = create_parser_with_tokens(vec![
            TokenType::Identifier("test".to_string()),
            TokenType::LeftParen,
            TokenType::RightParen,
            TokenType::Arrow,
            TokenType::Int,
            TokenType::LeftCurly,
            TokenType::RightCurly,
            TokenType::EOF,
        ]);
        let rule = FunctionRule;
        assert!(!rule.check_match(parser.get_cursor()));
    }

    #[test]
    fn test_parse_simple_function() {
        let mut parser = create_parser_with_tokens(vec![
            TokenType::Fn,
            TokenType::Identifier("test".to_string()),
            TokenType::LeftParen,
            TokenType::RightParen,
            TokenType::Arrow,
            TokenType::Int,
            TokenType::LeftCurly,
            TokenType::Return,
            TokenType::IntLiteral(42),
            TokenType::Semicolon,
            TokenType::RightCurly,
            TokenType::EOF,
        ]);
        let rule = FunctionRule;
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let function = result.unwrap();
        assert_eq!(*function.name, "test");
    }

    #[test]
    fn test_parse_function_with_parameters() {
        let mut parser = create_parser_with_tokens(vec![
            TokenType::Fn,
            TokenType::Identifier("add".to_string()),
            TokenType::LeftParen,
            TokenType::Int,
            TokenType::Identifier("a".to_string()),
            TokenType::Comma,
            TokenType::Int,
            TokenType::Identifier("b".to_string()),
            TokenType::RightParen,
            TokenType::Arrow,
            TokenType::Int,
            TokenType::LeftCurly,
            TokenType::Return,
            TokenType::Identifier("a".to_string()),
            TokenType::Plus,
            TokenType::Identifier("b".to_string()),
            TokenType::Semicolon,
            TokenType::RightCurly,
            TokenType::EOF,
        ]);
        let rule = FunctionRule;
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let function = result.unwrap();
        assert_eq!(*function.name, "add");
    }

    #[test]
    fn test_parse_function_missing_name() {
        let mut parser = create_parser_with_tokens(vec![
            TokenType::Fn,
            TokenType::LeftParen,
            TokenType::RightParen,
            TokenType::Arrow,
            TokenType::Int,
            TokenType::LeftCurly,
            TokenType::RightCurly,
            TokenType::EOF,
        ]);
        let rule = FunctionRule;
        let result = rule.parse(&mut parser);
        
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_function_missing_parameters() {
        let mut parser = create_parser_with_tokens(vec![
            TokenType::Fn,
            TokenType::Identifier("test".to_string()),
            TokenType::Arrow,
            TokenType::Int,
            TokenType::LeftCurly,
            TokenType::RightCurly,
            TokenType::EOF,
        ]);
        let rule = FunctionRule;
        let result = rule.parse(&mut parser);
        
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_function_missing_return_type() {
        let mut parser = create_parser_with_tokens(vec![
            TokenType::Fn,
            TokenType::Identifier("test".to_string()),
            TokenType::LeftParen,
            TokenType::RightParen,
            TokenType::LeftCurly,
            TokenType::RightCurly,
            TokenType::EOF,
        ]);
        let rule = FunctionRule;
        let result = rule.parse(&mut parser);
        
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_function_missing_body() {
        let mut parser = create_parser_with_tokens(vec![
            TokenType::Fn,
            TokenType::Identifier("test".to_string()),
            TokenType::LeftParen,
            TokenType::RightParen,
            TokenType::Arrow,
            TokenType::Int,
            TokenType::EOF,
        ]);
        let rule = FunctionRule;
        let result = rule.parse(&mut parser);
        
        assert!(result.is_none());
    }
}