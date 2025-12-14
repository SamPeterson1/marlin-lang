use std::fmt;

use crate::ast::ASTNode;
use crate::parser::{ExprParser, ParseRule, ParserCursor};
use crate::parser::rules::{impl_block::ImplBlockRule, main_item::MainItemRule};

use super::{function_item::FunctionRule, struct_item::StructRule};

pub struct ItemRule {}

impl fmt::Display for ItemRule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Item")
    }
}

impl ParseRule<Box<dyn ASTNode>> for ItemRule {
    fn check_match(&self, cursor: ParserCursor) -> bool {
        (MainItemRule {}).check_match(cursor)
            || (FunctionRule {}).check_match(cursor)
            || (StructRule {}).check_match(cursor)
            || (ImplBlockRule {}).check_match(cursor)
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        if (MainItemRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(MainItemRule {}, "main item", None);
        }

        if (FunctionRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed( FunctionRule {}, "function item", None);
        }

        if ((StructRule {})).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(StructRule {}, "struct item", None);
        }

        if (ImplBlockRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(ImplBlockRule {}, "impl item", None);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::{Token, TokenType, PositionRange};

    fn create_token(token_type: TokenType) -> Token {
        Token::new(token_type, PositionRange::zero())
    }

    #[test]
    fn test_item_rule_check_match_main() {
        let rule = ItemRule {};
        let tokens = vec![
            create_token(TokenType::Main),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_item_rule_check_match_function() {
        let rule = ItemRule {};
        let tokens = vec![
            create_token(TokenType::Fn),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_item_rule_check_match_struct() {
        let rule = ItemRule {};
        let tokens = vec![
            create_token(TokenType::Struct),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_item_rule_check_match_impl() {
        let rule = ItemRule {};
        let tokens = vec![
            create_token(TokenType::Impl),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_item_rule_check_match_no_match() {
        let rule = ItemRule {};
        let tokens = vec![
            create_token(TokenType::Let),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(!rule.check_match(cursor));
    }

    #[test]
    fn test_parse_main_item() {
        let rule = ItemRule {};
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
    fn test_parse_function_item() {
        let rule = ItemRule {};
        let tokens = vec![
            create_token(TokenType::Fn),
            create_token(TokenType::Identifier("test".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::Arrow),
            create_token(TokenType::Int),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid function item");
    }

    #[test]
    fn test_parse_struct_item() {
        let rule = ItemRule {};
        let tokens = vec![
            create_token(TokenType::Struct),
            create_token(TokenType::Identifier("TestStruct".to_string())),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid struct item");
    }

    #[test]
    fn test_parse_impl_item() {
        let rule = ItemRule {};
        let tokens = vec![
            create_token(TokenType::Impl),
            create_token(TokenType::Identifier("TestStruct".to_string())),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid impl item");
    }

    #[test]
    fn test_parse_no_match() {
        let rule = ItemRule {};
        let tokens = vec![
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_none());
    }
}