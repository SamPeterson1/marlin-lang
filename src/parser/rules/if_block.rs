use std::fmt;

use crate::ast::if_expr::IfExpr;
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::{block::BlockRule, expr::ExprRule};
use crate::lexer::token::TokenType;

pub struct IfBlockRule {}

impl fmt::Display for IfBlockRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IfBlock")
    }
}

impl ParseRule<IfExpr> for IfBlockRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::If).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<IfExpr> {
        parser.begin_range();
        parser.try_consume(TokenType::If)?;
        
        let condition = parser.apply_rule(ExprRule {}, "if condition", Some(ErrMsg::ExpectedExpression))?;
    
        let success = parser.apply_rule(ExprRule {}, "if success", Some(ErrMsg::ExpectedBlock))?;
        
        let mut fail = None;

        if let Some(_) = parser.try_consume(TokenType::Else) {
            fail = parser.apply_rule_boxed(IfBlockRule {}, "else if block", None);

            if fail.is_none() {
                fail = parser.apply_rule_boxed(BlockRule {}, "else block", None);
            }
        }
        
        Some(IfExpr::new(condition, success, fail, parser.end_range()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::{Token, TokenType, PositionRange, Position};
    use crate::parser::ExprParser;

    fn create_token(token_type: TokenType) -> Token {
        Token::new(token_type, PositionRange::new(Position::new(1, 1)))
    }

    fn create_parser_with_tokens(tokens: Vec<TokenType>) -> ExprParser<'static> {
        let diagnostics = Box::leak(Box::new(Vec::new()));
        let tokens: Vec<Token> = tokens
            .into_iter()
            .map(|token_type| Token::new(token_type, PositionRange::zero()))
            .collect();
        ExprParser::new(tokens, diagnostics)
    }

    #[test]
    fn test_if_block_rule_check_match_with_if() {
        let rule = IfBlockRule {};
        let tokens = vec![
            create_token(TokenType::If),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_if_block_rule_check_match_without_if() {
        let rule = IfBlockRule {};
        let tokens = vec![
            create_token(TokenType::While),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(!rule.check_match(cursor));
    }

    #[test]
    fn test_parse_simple_if_statement() {
        let mut parser = create_parser_with_tokens(vec![
            TokenType::If,
            TokenType::BoolLiteral(true),
            TokenType::LeftCurly,
            TokenType::Let,
            TokenType::Int,
            TokenType::Identifier("x".to_string()),
            TokenType::Assignment,
            TokenType::IntLiteral(1),
            TokenType::Semicolon,
            TokenType::RightCurly,
            TokenType::EOF,
        ]);
        let rule = IfBlockRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_if_else_statement() {
        let mut parser = create_parser_with_tokens(vec![
            TokenType::If,
            TokenType::BoolLiteral(true),
            TokenType::LeftCurly,
            TokenType::IntLiteral(1),
            TokenType::RightCurly,
            TokenType::Else,
            TokenType::LeftCurly,
            TokenType::IntLiteral(2),
            TokenType::RightCurly,
            TokenType::EOF,
        ]);
        let rule = IfBlockRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_if_elif_else_statement() {
        let mut parser = create_parser_with_tokens(vec![
            TokenType::If,
            TokenType::Identifier("x".to_string()),
            TokenType::Equal,
            TokenType::IntLiteral(1),
            TokenType::LeftCurly,
            TokenType::StringLiteral("one".to_string()),
            TokenType::RightCurly,
            TokenType::Else,
            TokenType::If,
            TokenType::Identifier("x".to_string()),
            TokenType::Equal,
            TokenType::IntLiteral(2),
            TokenType::LeftCurly,
            TokenType::StringLiteral("two".to_string()),
            TokenType::RightCurly,
            TokenType::Else,
            TokenType::LeftCurly,
            TokenType::StringLiteral("other".to_string()),
            TokenType::RightCurly,
            TokenType::EOF,
        ]);
        let rule = IfBlockRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_multiple_elif_statements() {
        let mut parser = create_parser_with_tokens(vec![
            TokenType::If,
            TokenType::Identifier("grade".to_string()),
            TokenType::GreaterEqual,
            TokenType::IntLiteral(90),
            TokenType::LeftCurly,
            TokenType::StringLiteral("A".to_string()),
            TokenType::RightCurly,
            TokenType::Else,
            TokenType::If,
            TokenType::Identifier("grade".to_string()),
            TokenType::GreaterEqual,
            TokenType::IntLiteral(80),
            TokenType::LeftCurly,
            TokenType::StringLiteral("B".to_string()),
            TokenType::RightCurly,
            TokenType::Else,
            TokenType::If,
            TokenType::Identifier("grade".to_string()),
            TokenType::GreaterEqual,
            TokenType::IntLiteral(70),
            TokenType::LeftCurly,
            TokenType::StringLiteral("C".to_string()),
            TokenType::RightCurly,
            TokenType::Else,
            TokenType::LeftCurly,
            TokenType::StringLiteral("F".to_string()),
            TokenType::RightCurly,
            TokenType::EOF,
        ]);
        let rule = IfBlockRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_nested_if_statements() {
        let mut parser = create_parser_with_tokens(vec![
            TokenType::If,
            TokenType::Identifier("x".to_string()),
            TokenType::Greater,
            TokenType::IntLiteral(0),
            TokenType::LeftCurly,
            TokenType::If,
            TokenType::Identifier("x".to_string()),
            TokenType::Greater,
            TokenType::IntLiteral(10),
            TokenType::LeftCurly,
            TokenType::StringLiteral("big".to_string()),
            TokenType::RightCurly,
            TokenType::Else,
            TokenType::LeftCurly,
            TokenType::StringLiteral("small".to_string()),
            TokenType::RightCurly,
            TokenType::RightCurly,
            TokenType::EOF,
        ]);
        let rule = IfBlockRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_if_with_complex_condition() {
        let mut parser = create_parser_with_tokens(vec![
            TokenType::If,
            TokenType::Identifier("x".to_string()),
            TokenType::Greater,
            TokenType::IntLiteral(0),
            TokenType::And,
            TokenType::Identifier("y".to_string()),
            TokenType::Less,
            TokenType::IntLiteral(10),
            TokenType::Or,
            TokenType::Identifier("z".to_string()),
            TokenType::Equal,
            TokenType::IntLiteral(5),
            TokenType::LeftCurly,
            TokenType::Return,
            TokenType::BoolLiteral(true),
            TokenType::Semicolon,
            TokenType::RightCurly,
            TokenType::EOF,
        ]);
        let rule = IfBlockRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_if_with_function_call_condition() {
        let mut parser = create_parser_with_tokens(vec![
            TokenType::If,
            TokenType::Identifier("isValid".to_string()),
            TokenType::LeftParen,
            TokenType::Identifier("input".to_string()),
            TokenType::RightParen,
            TokenType::LeftCurly,
            TokenType::Identifier("process".to_string()),
            TokenType::LeftParen,
            TokenType::Identifier("input".to_string()),
            TokenType::RightParen,
            TokenType::Semicolon,
            TokenType::RightCurly,
            TokenType::EOF,
        ]);
        let rule = IfBlockRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_if_with_member_access_condition() {
        let mut parser = create_parser_with_tokens(vec![
            TokenType::If,
            TokenType::Identifier("obj".to_string()),
            TokenType::Dot,
            TokenType::Identifier("isReady".to_string()),
            TokenType::LeftCurly,
            TokenType::Identifier("obj".to_string()),
            TokenType::Dot,
            TokenType::Identifier("execute".to_string()),
            TokenType::LeftParen,
            TokenType::RightParen,
            TokenType::Semicolon,
            TokenType::RightCurly,
            TokenType::EOF,
        ]);
        let rule = IfBlockRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_missing_condition() {
        let mut parser = create_parser_with_tokens(vec![
            TokenType::If,
            TokenType::LeftCurly,
            TokenType::IntLiteral(1),
            TokenType::RightCurly,
            TokenType::EOF,
        ]);
        let rule = IfBlockRule {};
        let result = rule.parse(&mut parser);
        
        // Should fail because condition is required
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_missing_then_block() {
        let mut parser = create_parser_with_tokens(vec![
            TokenType::If,
            TokenType::BoolLiteral(true),
            TokenType::EOF,
        ]);
        let rule = IfBlockRule {};
        let result = rule.parse(&mut parser);
        
        // Should fail because then block is required
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_invalid_condition() {
        let mut parser = create_parser_with_tokens(vec![
            TokenType::If,
            TokenType::RightCurly, // Invalid condition
            TokenType::LeftCurly,
            TokenType::IntLiteral(1),
            TokenType::RightCurly,
            TokenType::EOF,
        ]);
        let rule = IfBlockRule {};
        let result = rule.parse(&mut parser);
        
        // Should fail because condition is invalid
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_if_expression_as_value() {
        let mut parser = create_parser_with_tokens(vec![
            TokenType::If,
            TokenType::Identifier("condition".to_string()),
            TokenType::LeftCurly,
            TokenType::IntLiteral(42),
            TokenType::RightCurly,
            TokenType::Else,
            TokenType::LeftCurly,
            TokenType::IntLiteral(0),
            TokenType::RightCurly,
            TokenType::EOF,
        ]);
        let rule = IfBlockRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_if_with_empty_blocks() {
        let mut parser = create_parser_with_tokens(vec![
            TokenType::If,
            TokenType::BoolLiteral(true),
            TokenType::LeftCurly,
            TokenType::RightCurly,
            TokenType::Else,
            TokenType::LeftCurly,
            TokenType::RightCurly,
            TokenType::EOF,
        ]);
        let rule = IfBlockRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_if_with_single_expression_blocks() {
        let mut parser = create_parser_with_tokens(vec![
            TokenType::If,
            TokenType::Identifier("flag".to_string()),
            TokenType::LeftCurly,
            TokenType::Return,
            TokenType::Semicolon,
            TokenType::RightCurly,
            TokenType::Else,
            TokenType::LeftCurly,
            TokenType::Break,
            TokenType::Semicolon,
            TokenType::RightCurly,
            TokenType::EOF,
        ]);
        let rule = IfBlockRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_if_with_constructor_condition() {
        let mut parser = create_parser_with_tokens(vec![
            TokenType::If,
            TokenType::Identifier("result".to_string()),
            TokenType::Equal,
            TokenType::DollarSign,
            TokenType::Identifier("Success".to_string()),
            TokenType::LeftParen,
            TokenType::RightParen,
            TokenType::LeftCurly,
            TokenType::Return,
            TokenType::BoolLiteral(true),
            TokenType::Semicolon,
            TokenType::RightCurly,
            TokenType::EOF,
        ]);
        let rule = IfBlockRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_elif_without_else() {
        let mut parser = create_parser_with_tokens(vec![
            TokenType::If,
            TokenType::Identifier("x".to_string()),
            TokenType::Equal,
            TokenType::IntLiteral(1),
            TokenType::LeftCurly,
            TokenType::StringLiteral("one".to_string()),
            TokenType::RightCurly,
            TokenType::Else,
            TokenType::If,
            TokenType::Identifier("x".to_string()),
            TokenType::Equal,
            TokenType::IntLiteral(2),
            TokenType::LeftCurly,
            TokenType::StringLiteral("two".to_string()),
            TokenType::RightCurly,
            TokenType::EOF,
        ]);
        let rule = IfBlockRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_if_with_array_access_condition() {
        let mut parser = create_parser_with_tokens(vec![
            TokenType::If,
            TokenType::Identifier("flags".to_string()),
            TokenType::LeftSquare,
            TokenType::IntLiteral(0),
            TokenType::RightSquare,
            TokenType::LeftCurly,
            TokenType::Return,
            TokenType::BoolLiteral(true),
            TokenType::Semicolon,
            TokenType::RightCurly,
            TokenType::EOF,
        ]);
        let rule = IfBlockRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
    }
}
