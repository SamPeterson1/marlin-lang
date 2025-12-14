use std::fmt;

use crate::ast::main_item::MainItem;
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::block::BlockRule;
use crate::lexer::token::TokenType;

pub struct MainItemRule {}

impl fmt::Display for MainItemRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MainItem")
    }
}

impl ParseRule<MainItem> for MainItemRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::Main).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<MainItem> {
        parser.begin_range();
        parser.try_consume(TokenType::Main)?;

        let block = parser.apply_rule(BlockRule {}, "main block", Some(ErrMsg::ExpectedBlock))?;

        Some(MainItem::new(block, parser.end_range()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::{Token, TokenType, PositionRange, Position};

    fn create_token(token_type: TokenType) -> Token {
        Token::new(token_type, PositionRange::new(Position::new(1, 1)))
    }

    #[test]
    fn test_main_item_rule_check_match_with_main() {
        let rule = MainItemRule {};
        let tokens = vec![
            create_token(TokenType::Main),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_main_item_rule_check_match_without_main() {
        let rule = MainItemRule {};
        let tokens = vec![
            create_token(TokenType::Fn),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(!rule.check_match(cursor));
    }

    #[test]
    fn test_parse_simple_main() {
        let rule = MainItemRule {};
        let tokens = vec![
            create_token(TokenType::Main),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid main item");
    }

    #[test]
    fn test_parse_main_with_statements() {
        let rule = MainItemRule {};
        let tokens = vec![
            create_token(TokenType::Main),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for main with statements");
    }

    #[test]
    fn test_parse_missing_main_keyword() {
        let rule = MainItemRule {};
        let tokens = vec![
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because main keyword is required
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_missing_block() {
        let rule = MainItemRule {};
        let tokens = vec![
            create_token(TokenType::Main),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because block is required
        assert!(result.is_none());
        // Should have diagnostic for missing block
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing block");
    }

    #[test]
    fn test_parse_main_with_return() {
        let rule = MainItemRule {};
        let tokens = vec![
            create_token(TokenType::Main),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Return),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for main with return");
    }
}