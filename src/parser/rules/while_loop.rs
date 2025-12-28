use std::fmt;

use crate::ast::LoopExpr;
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::{block::BlockRule, expr::ExprRule};
use crate::lexer::token::TokenType;

pub struct WhileLoopRule {}

impl fmt::Display for WhileLoopRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WhileLoop")
    }
}

impl ParseRule<LoopExpr> for WhileLoopRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::While).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<LoopExpr> {
        parser.begin_range();
        parser.try_consume(TokenType::While)?;

        let label = match parser.try_consume(TokenType::Colon) {
            Some(_) => Some(parser.consume_or_diagnostic(TokenType::AnyIdentifier)?.unwrap_identifier()),
            _ => None
        };
    
        let condition = parser.apply_rule(ExprRule {}, "while condition", Some(ErrMsg::ExpectedExpression))?;
        let body = parser.apply_rule(BlockRule {}, "while body", Some(ErrMsg::ExpectedBlock))?;
    
        Some(LoopExpr::new_while(condition, body, label, parser.end_range()))
    }
}

use crate::logger::DYN_CONSOLE_LOGGER;
#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::{Token, TokenType, PositionRange};

    fn create_token(token_type: TokenType) -> Token {
        Token::new(token_type, PositionRange::zero())
    }

    #[test]
    fn test_while_loop_rule_check_match_with_while() {
        let rule = WhileLoopRule {};
        let tokens = vec![
            create_token(TokenType::While),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_while_loop_rule_check_match_without_while() {
        let rule = WhileLoopRule {};
        let tokens = vec![
            create_token(TokenType::If),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(!rule.check_match(cursor));
    }

    #[test]
    fn test_parse_simple_while_loop() {
        let rule = WhileLoopRule {};
        let tokens = vec![
            create_token(TokenType::While),
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Break),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for simple while loop");
    }

    #[test]
    fn test_parse_while_with_variable_condition() {
        let rule = WhileLoopRule {};
        let tokens = vec![
            create_token(TokenType::While),
            create_token(TokenType::Identifier("condition".to_string())),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Identifier("condition".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::BoolLiteral(false)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for while with variable condition");
    }

    #[test]
    fn test_parse_while_with_comparison_condition() {
        let rule = WhileLoopRule {};
        let tokens = vec![
            create_token(TokenType::While),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Less),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for while with comparison condition");
    }

    #[test]
    fn test_parse_while_with_complex_condition() {
        let rule = WhileLoopRule {};
        let tokens = vec![
            create_token(TokenType::While),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Greater),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::And),
            create_token(TokenType::Identifier("y".to_string())),
            create_token(TokenType::Less),
            create_token(TokenType::IntLiteral(100)),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Minus),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("y".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("y".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for while with complex condition");
    }

    #[test]
    fn test_parse_while_with_function_call_condition() {
        let rule = WhileLoopRule {};
        let tokens = vec![
            create_token(TokenType::While),
            create_token(TokenType::Identifier("hasMore".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Identifier("process".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for while with function call condition");
    }

    #[test]
    fn test_parse_while_with_member_access_condition() {
        let rule = WhileLoopRule {};
        let tokens = vec![
            create_token(TokenType::While),
            create_token(TokenType::Identifier("obj".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("isActive".to_string())),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Identifier("obj".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("update".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for while with member access condition");
    }

    #[test]
    fn test_parse_while_with_array_access_condition() {
        let rule = WhileLoopRule {};
        let tokens = vec![
            create_token(TokenType::While),
            create_token(TokenType::Identifier("buffer".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::Identifier("index".to_string())),
            create_token(TokenType::RightSquare),
            create_token(TokenType::NotEqual),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Identifier("index".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("index".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for while with array access condition");
    }

    #[test]
    fn test_parse_nested_while_loops() {
        let rule = WhileLoopRule {};
        let tokens = vec![
            create_token(TokenType::While),
            create_token(TokenType::Identifier("outer".to_string())),
            create_token(TokenType::Greater),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::While),
            create_token(TokenType::Identifier("inner".to_string())),
            create_token(TokenType::Greater),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Identifier("inner".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("inner".to_string())),
            create_token(TokenType::Minus),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::Identifier("outer".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("outer".to_string())),
            create_token(TokenType::Minus),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for nested while loops");
    }
    
    #[test]
    fn test_parse_while_with_multiple_statements() {
        let rule = WhileLoopRule {};
        let tokens = vec![
            create_token(TokenType::While),
            create_token(TokenType::Identifier("count".to_string())),
            create_token(TokenType::Greater),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("temp".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("count".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("count".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("count".to_string())),
            create_token(TokenType::Minus),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("print".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Identifier("temp".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for while with multiple statements");
    }

    #[test]
    fn test_parse_while_with_empty_body() {
        let rule = WhileLoopRule {};
        let tokens = vec![
            create_token(TokenType::While),
            create_token(TokenType::BoolLiteral(false)),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for while with empty body");
    }

    #[test]
    fn test_parse_missing_condition() {
        let rule = WhileLoopRule {};
        let tokens = vec![
            create_token(TokenType::While),
            create_token(TokenType::EOF), // EOF cannot be part of any expression
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because condition is required
        assert!(result.is_none());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing condition");
        assert!(diagnostics.iter().any(|d| d.message.contains("expected expression")));
    }

    #[test]
    fn test_parse_missing_body() {
        let rule = WhileLoopRule {};
        let tokens = vec![
            create_token(TokenType::While),
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because body is required
        assert!(result.is_none());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing body");
        assert!(diagnostics.iter().any(|d| d.message.contains("expected block")));
    }

    #[test]
    fn test_parse_invalid_condition() {
        let rule = WhileLoopRule {};
        let tokens = vec![
            create_token(TokenType::While),
            create_token(TokenType::RightCurly), // Invalid condition
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Break),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because condition is invalid
        assert!(result.is_none());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for invalid condition");
        assert!(diagnostics.iter().any(|d| d.message.contains("expected expression")));
    }

    #[test]
    fn test_parse_while_with_unary_condition() {
        let rule = WhileLoopRule {};
        let tokens = vec![
            create_token(TokenType::While),
            create_token(TokenType::Not),
            create_token(TokenType::Identifier("done".to_string())),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Identifier("work".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for while with unary condition");
    }

    #[test]
    fn test_parse_while_with_parenthesized_condition() {
        let rule = WhileLoopRule {};
        let tokens = vec![
            create_token(TokenType::While),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Greater),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::And),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Less),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::RightParen),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for while with parenthesized condition");
    }

    #[test]
    fn test_parse_infinite_loop_pattern() {
        let rule = WhileLoopRule {};
        let tokens = vec![
            create_token(TokenType::While),
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::If),
            create_token(TokenType::Identifier("exit_condition".to_string())),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Return),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::Identifier("do_work".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for infinite loop pattern");
    }

    #[test]
    fn test_parse_while_with_constructor_condition() {
        let rule = WhileLoopRule {};
        let tokens = vec![
            create_token(TokenType::While),
            create_token(TokenType::Identifier("getNext".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::NotEqual),
            create_token(TokenType::DollarSign),
            create_token(TokenType::Identifier("None".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Identifier("process".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for while with constructor condition");
    }
}