use std::fmt;

use crate::ast::LoopExpr;
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::block::BlockRule;
use crate::lexer::token::TokenType;

pub struct LoopRule {}

impl fmt::Display for LoopRule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Loop")
    }
}

impl ParseRule<LoopExpr> for LoopRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::Loop).is_some()
    }
    
    fn parse(&self, parser: &mut ExprParser) -> Option<LoopExpr> {
        parser.begin_range();
        parser.try_consume(TokenType::Loop)?;

        let label = match parser.try_consume(TokenType::Colon) {
            Some(_) => Some(parser.consume_or_diagnostic(TokenType::AnyIdentifier)?.unwrap_identifier()),
            _ => None
        };
    
        let body = parser.apply_rule(BlockRule {}, "loop body", Some(ErrMsg::ExpectedBlock))?;
            
        Some(LoopExpr::new_loop(body, label, parser.end_range()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::{Token, TokenType, PositionRange};
    use crate::logger::CONSOLE_LOGGER;

    fn create_token(token_type: TokenType) -> Token {
        Token::new(token_type, PositionRange::zero())
    }

    #[test]
    fn test_loop_expr_rule_check_match_with_loop() {
        let rule = LoopRule {};
        let tokens = vec![
            create_token(TokenType::Loop),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_loop_expr_rule_check_match_without_loop() {
        let rule = LoopRule {};
        let tokens = vec![
            create_token(TokenType::While),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(!rule.check_match(cursor));
    }

    #[test]
    fn test_parse_simple_infinite_loop() {
        let rule = LoopRule {};
        let tokens = vec![
            create_token(TokenType::Loop),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Break),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for simple infinite loop");
    }

    #[test]
    fn test_parse_loop_with_conditional_break() {
        let rule = LoopRule {};
        let tokens = vec![
            create_token(TokenType::Loop),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::If),
            create_token(TokenType::Identifier("condition".to_string())),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Break),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::Identifier("doWork".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for loop with conditional break");
    }

    #[test]
    fn test_parse_loop_with_multiple_statements() {
        let rule = LoopRule {};
        let tokens = vec![
            create_token(TokenType::Loop),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("counter".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("counter".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("counter".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::If),
            create_token(TokenType::Identifier("counter".to_string())),
            create_token(TokenType::Greater),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Break),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for loop with multiple statements");
    }

    #[test]
    fn test_parse_nested_loops() {
        let rule = LoopRule {};
        let tokens = vec![
            create_token(TokenType::Loop),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Loop),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::If),
            create_token(TokenType::Identifier("innerCondition".to_string())),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Break),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::If),
            create_token(TokenType::Identifier("outerCondition".to_string())),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Break),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for nested loops");
    }

    #[test]
    fn test_parse_loop_with_function_calls() {
        let rule = LoopRule {};
        let tokens = vec![
            create_token(TokenType::Loop),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("result".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("doWork".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::Semicolon),
            create_token(TokenType::If),
            create_token(TokenType::Identifier("isComplete".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Identifier("result".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Break),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for loop with function calls");
    }

    #[test]
    fn test_parse_loop_with_return() {
        let rule = LoopRule {};
        let tokens = vec![
            create_token(TokenType::Loop),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::If),
            create_token(TokenType::Identifier("found".to_string())),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Return),
            create_token(TokenType::Identifier("result".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::Identifier("search".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for loop with return");
    }

    #[test]
    fn test_parse_loop_with_complex_control_flow() {
        let rule = LoopRule {};
        let tokens = vec![
            create_token(TokenType::Loop),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("input".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("getInput".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::Semicolon),
            create_token(TokenType::If),
            create_token(TokenType::Identifier("input".to_string())),
            create_token(TokenType::Equal),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Break),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::Else),
            create_token(TokenType::If),
            create_token(TokenType::Identifier("input".to_string())),
            create_token(TokenType::Less),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::Identifier("process".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Identifier("input".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for loop with complex control flow");
    }

    #[test]
    fn test_parse_loop_with_empty_body() {
        let rule = LoopRule {};
        let tokens = vec![
            create_token(TokenType::Loop),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for loop with empty body");
    }

    #[test]
    fn test_parse_missing_body() {
        let rule = LoopRule {};
        let tokens = vec![
            create_token(TokenType::Loop),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because body is required
        assert!(result.is_none());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing body");
        assert!(diagnostics.iter().any(|d| d.message.contains("expected block")));
    }

    #[test]
    fn test_parse_loop_with_variable_declarations() {
        let rule = LoopRule {};
        let tokens = vec![
            create_token(TokenType::Loop),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Let),
            create_token(TokenType::Bool),
            create_token(TokenType::Identifier("flag".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::If),
            create_token(TokenType::Identifier("flag".to_string())),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Break),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for loop with variable declarations");
    }

    #[test]
    fn test_parse_loop_with_assignments() {
        let rule = LoopRule {};
        let tokens = vec![
            create_token(TokenType::Loop),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Identifier("counter".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("counter".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("sum".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("sum".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::Identifier("counter".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::If),
            create_token(TokenType::Identifier("sum".to_string())),
            create_token(TokenType::Greater),
            create_token(TokenType::IntLiteral(100)),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Break),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for loop with assignments");
    }

    #[test]
    fn test_parse_loop_with_match_expression() {
        let rule = LoopRule {};
        let tokens = vec![
            create_token(TokenType::Loop),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("value".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("getValue".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::Semicolon),
            create_token(TokenType::If),
            create_token(TokenType::Identifier("value".to_string())),
            create_token(TokenType::Equal),
            create_token(TokenType::DollarSign),
            create_token(TokenType::Identifier("Stop".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Break),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for loop with match expression");
    }

    #[test]
    fn test_parse_loop_with_array_operations() {
        let rule = LoopRule {};
        let tokens = vec![
            create_token(TokenType::Loop),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::If),
            create_token(TokenType::Identifier("arr".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::Identifier("index".to_string())),
            create_token(TokenType::RightSquare),
            create_token(TokenType::Equal),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Break),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
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
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for loop with array operations");
    }

    #[test]
    fn test_parse_loop_with_member_access() {
        let rule = LoopRule {};
        let tokens = vec![
            create_token(TokenType::Loop),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::If),
            create_token(TokenType::Identifier("obj".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("shouldStop".to_string())),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Break),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
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
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for loop with member access");
    }

    #[test]
    fn test_parse_loop_with_result_expression() {
        let rule = LoopRule {};
        let tokens = vec![
            create_token(TokenType::Loop),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("attempts".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("attempts".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("attempts".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::If),
            create_token(TokenType::Identifier("attempts".to_string())),
            create_token(TokenType::Greater),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Result),
            create_token(TokenType::Identifier("attempts".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for loop with result expression");
    }
}