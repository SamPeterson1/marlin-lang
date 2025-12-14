use std::fmt;

use crate::ast::loop_expr::LoopExpr;
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::{assignment::AssignmentRule, block::BlockRule, declaration::DeclarationRule, expr::ExprRule};
use crate::lexer::token::TokenType;

pub struct ForLoopRule {}

impl fmt::Display for ForLoopRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ForLoop")
    }
}

impl ParseRule<LoopExpr> for ForLoopRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::For).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<LoopExpr> {
        parser.begin_range();
        parser.try_consume(TokenType::For)?;

        parser.consume_or_diagnostic(TokenType::LeftParen);

        let initial = parser.apply_rule(DeclarationRule {}, "for initial declaration", Some(ErrMsg::ExpectedDeclaration))?;

        let condition = parser.apply_rule(ExprRule {}, "for condition expression", Some(ErrMsg::ExpectedExpression))?;

        parser.consume_or_diagnostic(TokenType::Semicolon);

        let increment = parser.apply_rule(AssignmentRule {}, "for increment assignment", Some(ErrMsg::ExpectedAssignment))?;

        parser.consume_or_diagnostic(TokenType::RightParen);

        let body = parser.apply_rule(BlockRule {}, "for body", Some(ErrMsg::ExpectedBlock))?;    
                
        Some(LoopExpr::new_for(initial, condition, increment, body, parser.end_range()))
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
    fn test_for_loop_rule_check_match_with_for() {
        let rule = ForLoopRule {};
        let tokens = vec![
            create_token(TokenType::For),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_for_loop_rule_check_match_without_for() {
        let rule = ForLoopRule {};
        let tokens = vec![
            create_token(TokenType::While),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(!rule.check_match(cursor));
    }

    #[test]
    fn test_parse_simple_for_loop() {
        let rule = ForLoopRule {};
        let tokens = vec![
            create_token(TokenType::For),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Less),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::RightParen),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Identifier("print".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for simple for loop");
    }

    #[test]
    fn test_parse_for_loop_with_bool_type() {
        let rule = ForLoopRule {};
        let tokens = vec![
            create_token(TokenType::For),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Let),
            create_token(TokenType::Bool),
            create_token(TokenType::Identifier("running".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("running".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("running".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("checkCondition".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Identifier("doWork".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for for loop with bool type");
    }

    #[test]
    fn test_parse_for_loop_with_complex_condition() {
        let rule = ForLoopRule {};
        let tokens = vec![
            create_token(TokenType::For),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Less),
            create_token(TokenType::IntLiteral(100)),
            create_token(TokenType::And),
            create_token(TokenType::Identifier("keepGoing".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::RightParen),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for for loop with complex condition");
    }

    #[test]
    fn test_parse_nested_for_loops() {
        let rule = ForLoopRule {};
        let tokens = vec![
            create_token(TokenType::For),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Less),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::RightParen),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::For),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("j".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("j".to_string())),
            create_token(TokenType::Less),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("j".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("j".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::RightParen),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::Break),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for nested for loops");
    }

    #[test]
    fn test_parse_for_loop_with_function_calls_in_assignment() {
        let rule = ForLoopRule {};
        let tokens = vec![
            create_token(TokenType::For),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("counter".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("getInitialValue".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("shouldContinue".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Identifier("counter".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("counter".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("update".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Identifier("counter".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Identifier("doWork".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for for loop with function calls");
    }

    #[test]
    fn test_parse_for_loop_with_break_continue() {
        let rule = ForLoopRule {};
        let tokens = vec![
            create_token(TokenType::For),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Less),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::RightParen),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::If),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Equal),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Break),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for for loop with break");
    }

    #[test]
    fn test_parse_for_loop_decrement() {
        let rule = ForLoopRule {};
        let tokens = vec![
            create_token(TokenType::For),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Greater),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Minus),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::RightParen),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for decrementing for loop");
    }

    #[test]
    fn test_parse_for_loop_with_array_iteration() {
        let rule = ForLoopRule {};
        let tokens = vec![
            create_token(TokenType::For),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Less),
            create_token(TokenType::Identifier("arr".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("length".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::RightParen),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Identifier("process".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Identifier("arr".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::RightSquare),
            create_token(TokenType::RightParen),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for array iteration for loop");
    }

    #[test]
    fn test_parse_for_loop_with_char_type() {
        let rule = ForLoopRule {};
        let tokens = vec![
            create_token(TokenType::For),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Let),
            create_token(TokenType::Char),
            create_token(TokenType::Identifier("c".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::CharLiteral('a')),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("c".to_string())),
            create_token(TokenType::LessEqual),
            create_token(TokenType::CharLiteral('z')),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("c".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("getNextChar".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Identifier("c".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for for loop with char type");
    }

    #[test]
    fn test_parse_for_loop_with_double_type() {
        let rule = ForLoopRule {};
        let tokens = vec![
            create_token(TokenType::For),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Let),
            create_token(TokenType::Double),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::DoubleLiteral(0.0)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Less),
            create_token(TokenType::DoubleLiteral(10.0)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::DoubleLiteral(0.1)),
            create_token(TokenType::RightParen),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for for loop with double type");
    }

    #[test]
    fn test_parse_for_loop_with_assignment_only_increment() {
        let rule = ForLoopRule {};
        let tokens = vec![
            create_token(TokenType::For),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Less),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("increment".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for for loop with function call as increment");
    }

    // Error cases
    #[test]
    fn test_parse_missing_parentheses() {
        let rule = ForLoopRule {};
        let tokens = vec![
            create_token(TokenType::For),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Less),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(!result.is_none());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing parentheses");
        assert!(diagnostics.iter().any(|d| d.message.contains("'('")));
        assert!(diagnostics.iter().any(|d| d.message.contains("')'")));
    }

    #[test]
    fn test_parse_missing_first_semicolon() {
        let rule = ForLoopRule {};
        let tokens = vec![
            create_token(TokenType::For),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Less),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::RightParen),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing semicolon after declaration");
    }

    #[test]
    fn test_parse_missing_second_semicolon() {
        let rule = ForLoopRule {};
        let tokens = vec![
            create_token(TokenType::For),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Less),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::RightParen),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(!result.is_none());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing semicolon after condition");
        assert!(diagnostics.iter().any(|d| d.message.contains("';'")));
    }

    #[test]
    fn test_parse_missing_body() {
        let rule = ForLoopRule {};
        let tokens = vec![
            create_token(TokenType::For),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Less),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because body is required
        assert!(result.is_none());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing body");
        assert!(diagnostics.iter().any(|d| d.message.contains("expected block")));
    }

    #[test]
    fn test_parse_missing_declaration() {
        let rule = ForLoopRule {};
        let tokens = vec![
            create_token(TokenType::For),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Less),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::RightParen),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because declaration is required
        assert!(result.is_none());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing declaration");
        assert!(diagnostics.iter().any(|d| d.message.contains("expected declaration")));
    }

    #[test]
    fn test_parse_missing_condition() {
        let rule = ForLoopRule {};
        let tokens = vec![
            create_token(TokenType::For),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::RightParen),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because condition expression is required
        assert!(result.is_none());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing condition");
        assert!(diagnostics.iter().any(|d| d.message.contains("expected expression")));
    }

    #[test]
    fn test_parse_missing_assignment() {
        let rule = ForLoopRule {};
        let tokens = vec![
            create_token(TokenType::For),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Less),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightParen),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because assignment is required
        assert!(result.is_none());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing assignment");
        assert!(diagnostics.iter().any(|d| d.message.contains("expected expression")));
    }
}
