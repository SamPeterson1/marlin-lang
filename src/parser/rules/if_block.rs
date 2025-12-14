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
    use crate::lexer::token::{Token, TokenType, PositionRange};
    use crate::parser::ExprParser;

    fn create_token(token_type: TokenType) -> Token {
        Token::new(token_type, PositionRange::zero())
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
        let tokens = vec![
            Token::new(TokenType::If, PositionRange::zero()),
            Token::new(TokenType::BoolLiteral(true), PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::Let, PositionRange::zero()),
            Token::new(TokenType::Int, PositionRange::zero()),
            Token::new(TokenType::Identifier("x".to_string()), PositionRange::zero()),
            Token::new(TokenType::Assignment, PositionRange::zero()),
            Token::new(TokenType::IntLiteral(1), PositionRange::zero()),
            Token::new(TokenType::Semicolon, PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::EOF, PositionRange::zero()),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        let rule = IfBlockRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_if_else_statement() {
        let tokens = vec![
            Token::new(TokenType::If, PositionRange::zero()),
            Token::new(TokenType::BoolLiteral(true), PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::IntLiteral(1), PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::Else, PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::IntLiteral(2), PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::EOF, PositionRange::zero()),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        let rule = IfBlockRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_if_elif_else_statement() {
        let tokens = vec![
            Token::new(TokenType::If, PositionRange::zero()),
            Token::new(TokenType::Identifier("x".to_string()), PositionRange::zero()),
            Token::new(TokenType::Equal, PositionRange::zero()),
            Token::new(TokenType::IntLiteral(1), PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::StringLiteral("one".to_string()), PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::Else, PositionRange::zero()),
            Token::new(TokenType::If, PositionRange::zero()),
            Token::new(TokenType::Identifier("x".to_string()), PositionRange::zero()),
            Token::new(TokenType::Equal, PositionRange::zero()),
            Token::new(TokenType::IntLiteral(2), PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::StringLiteral("two".to_string()), PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::Else, PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::StringLiteral("other".to_string()), PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::EOF, PositionRange::zero()),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        let rule = IfBlockRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_multiple_elif_statements() {
        let tokens = vec![
            Token::new(TokenType::If, PositionRange::zero()),
            Token::new(TokenType::Identifier("grade".to_string()), PositionRange::zero()),
            Token::new(TokenType::GreaterEqual, PositionRange::zero()),
            Token::new(TokenType::IntLiteral(90), PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::StringLiteral("A".to_string()), PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::Else, PositionRange::zero()),
            Token::new(TokenType::If, PositionRange::zero()),
            Token::new(TokenType::Identifier("grade".to_string()), PositionRange::zero()),
            Token::new(TokenType::GreaterEqual, PositionRange::zero()),
            Token::new(TokenType::IntLiteral(80), PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::StringLiteral("B".to_string()), PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::Else, PositionRange::zero()),
            Token::new(TokenType::If, PositionRange::zero()),
            Token::new(TokenType::Identifier("grade".to_string()), PositionRange::zero()),
            Token::new(TokenType::GreaterEqual, PositionRange::zero()),
            Token::new(TokenType::IntLiteral(70), PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::StringLiteral("C".to_string()), PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::Else, PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::StringLiteral("F".to_string()), PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::EOF, PositionRange::zero()),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        let rule = IfBlockRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_nested_if_statements() {
        let tokens = vec![
            Token::new(TokenType::If, PositionRange::zero()),
            Token::new(TokenType::Identifier("x".to_string()), PositionRange::zero()),
            Token::new(TokenType::Greater, PositionRange::zero()),
            Token::new(TokenType::IntLiteral(0), PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::If, PositionRange::zero()),
            Token::new(TokenType::Identifier("x".to_string()), PositionRange::zero()),
            Token::new(TokenType::Greater, PositionRange::zero()),
            Token::new(TokenType::IntLiteral(10), PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::StringLiteral("big".to_string()), PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::Else, PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::StringLiteral("small".to_string()), PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::EOF, PositionRange::zero()),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        let rule = IfBlockRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_if_with_complex_condition() {
        let tokens = vec![
            Token::new(TokenType::If, PositionRange::zero()),
            Token::new(TokenType::Identifier("x".to_string()), PositionRange::zero()),
            Token::new(TokenType::Greater, PositionRange::zero()),
            Token::new(TokenType::IntLiteral(0), PositionRange::zero()),
            Token::new(TokenType::And, PositionRange::zero()),
            Token::new(TokenType::Identifier("y".to_string()), PositionRange::zero()),
            Token::new(TokenType::Less, PositionRange::zero()),
            Token::new(TokenType::IntLiteral(10), PositionRange::zero()),
            Token::new(TokenType::Or, PositionRange::zero()),
            Token::new(TokenType::Identifier("z".to_string()), PositionRange::zero()),
            Token::new(TokenType::Equal, PositionRange::zero()),
            Token::new(TokenType::IntLiteral(5), PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::Return, PositionRange::zero()),
            Token::new(TokenType::BoolLiteral(true), PositionRange::zero()),
            Token::new(TokenType::Semicolon, PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::EOF, PositionRange::zero()),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        let rule = IfBlockRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_if_with_function_call_condition() {
        let tokens = vec![
            Token::new(TokenType::If, PositionRange::zero()),
            Token::new(TokenType::Identifier("isValid".to_string()), PositionRange::zero()),
            Token::new(TokenType::LeftParen, PositionRange::zero()),
            Token::new(TokenType::Identifier("input".to_string()), PositionRange::zero()),
            Token::new(TokenType::RightParen, PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::Identifier("process".to_string()), PositionRange::zero()),
            Token::new(TokenType::LeftParen, PositionRange::zero()),
            Token::new(TokenType::Identifier("input".to_string()), PositionRange::zero()),
            Token::new(TokenType::RightParen, PositionRange::zero()),
            Token::new(TokenType::Semicolon, PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::EOF, PositionRange::zero()),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        let rule = IfBlockRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_if_with_member_access_condition() {
        let tokens = vec![
            Token::new(TokenType::If, PositionRange::zero()),
            Token::new(TokenType::Identifier("obj".to_string()), PositionRange::zero()),
            Token::new(TokenType::Dot, PositionRange::zero()),
            Token::new(TokenType::Identifier("isReady".to_string()), PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::Identifier("obj".to_string()), PositionRange::zero()),
            Token::new(TokenType::Dot, PositionRange::zero()),
            Token::new(TokenType::Identifier("execute".to_string()), PositionRange::zero()),
            Token::new(TokenType::LeftParen, PositionRange::zero()),
            Token::new(TokenType::RightParen, PositionRange::zero()),
            Token::new(TokenType::Semicolon, PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::EOF, PositionRange::zero()),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        let rule = IfBlockRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_missing_condition() {
        let tokens = vec![
            Token::new(TokenType::If, PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::IntLiteral(1), PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::EOF, PositionRange::zero()),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        let rule = IfBlockRule {};
        let result = rule.parse(&mut parser);
        
        // Should fail because condition is required
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_missing_then_block() {
        let tokens = vec![
            Token::new(TokenType::If, PositionRange::zero()),
            Token::new(TokenType::BoolLiteral(true), PositionRange::zero()),
            Token::new(TokenType::EOF, PositionRange::zero()),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        let rule = IfBlockRule {};
        let result = rule.parse(&mut parser);
        
        // Should fail because then block is required
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_invalid_condition() {
        let tokens = vec![
            Token::new(TokenType::If, PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()), // Invalid condition
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::IntLiteral(1), PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::EOF, PositionRange::zero()),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        let rule = IfBlockRule {};
        let result = rule.parse(&mut parser);
        
        // Should fail because condition is invalid
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_if_expression_as_value() {
        let tokens = vec![
            Token::new(TokenType::If, PositionRange::zero()),
            Token::new(TokenType::Identifier("condition".to_string()), PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::IntLiteral(42), PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::Else, PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::IntLiteral(0), PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::EOF, PositionRange::zero()),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        let rule = IfBlockRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_if_with_empty_blocks() {
        let tokens = vec![
            Token::new(TokenType::If, PositionRange::zero()),
            Token::new(TokenType::BoolLiteral(true), PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::Else, PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::EOF, PositionRange::zero()),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        let rule = IfBlockRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_if_with_single_expression_blocks() {
        let tokens = vec![
            Token::new(TokenType::If, PositionRange::zero()),
            Token::new(TokenType::Identifier("flag".to_string()), PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::Return, PositionRange::zero()),
            Token::new(TokenType::Semicolon, PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::Else, PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::Break, PositionRange::zero()),
            Token::new(TokenType::Semicolon, PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::EOF, PositionRange::zero()),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        let rule = IfBlockRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_if_with_constructor_condition() {
        let tokens = vec![
            Token::new(TokenType::If, PositionRange::zero()),
            Token::new(TokenType::Identifier("result".to_string()), PositionRange::zero()),
            Token::new(TokenType::Equal, PositionRange::zero()),
            Token::new(TokenType::DollarSign, PositionRange::zero()),
            Token::new(TokenType::Identifier("Success".to_string()), PositionRange::zero()),
            Token::new(TokenType::LeftParen, PositionRange::zero()),
            Token::new(TokenType::RightParen, PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::Return, PositionRange::zero()),
            Token::new(TokenType::BoolLiteral(true), PositionRange::zero()),
            Token::new(TokenType::Semicolon, PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::EOF, PositionRange::zero()),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        let rule = IfBlockRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_elif_without_else() {
        let tokens = vec![
            Token::new(TokenType::If, PositionRange::zero()),
            Token::new(TokenType::Identifier("x".to_string()), PositionRange::zero()),
            Token::new(TokenType::Equal, PositionRange::zero()),
            Token::new(TokenType::IntLiteral(1), PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::StringLiteral("one".to_string()), PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::Else, PositionRange::zero()),
            Token::new(TokenType::If, PositionRange::zero()),
            Token::new(TokenType::Identifier("x".to_string()), PositionRange::zero()),
            Token::new(TokenType::Equal, PositionRange::zero()),
            Token::new(TokenType::IntLiteral(2), PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::StringLiteral("two".to_string()), PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::EOF, PositionRange::zero()),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        let rule = IfBlockRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_if_with_array_access_condition() {
        let tokens = vec![
            Token::new(TokenType::If, PositionRange::zero()),
            Token::new(TokenType::Identifier("flags".to_string()), PositionRange::zero()),
            Token::new(TokenType::LeftSquare, PositionRange::zero()),
            Token::new(TokenType::IntLiteral(0), PositionRange::zero()),
            Token::new(TokenType::RightSquare, PositionRange::zero()),
            Token::new(TokenType::LeftCurly, PositionRange::zero()),
            Token::new(TokenType::Return, PositionRange::zero()),
            Token::new(TokenType::BoolLiteral(true), PositionRange::zero()),
            Token::new(TokenType::Semicolon, PositionRange::zero()),
            Token::new(TokenType::RightCurly, PositionRange::zero()),
            Token::new(TokenType::EOF, PositionRange::zero()),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        let rule = IfBlockRule {};
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
    }
}
