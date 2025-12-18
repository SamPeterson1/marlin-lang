use std::fmt;

use crate::ast::{ExitExpr, ExitType};
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::expr::ExprRule;
use crate::lexer::token::TokenType;

pub struct ExitRule {}

impl fmt::Display for ExitRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Exit")
    }
}

impl ParseRule<ExitExpr> for ExitRule {
    fn check_match(&self, cursor: ParserCursor) -> bool {
        cursor.try_match(&[TokenType::Break, TokenType::Return, TokenType::Result]).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<ExitExpr> {
        parser.begin_range();

        let exit_type = match parser.try_consume_match(&[TokenType::Break, TokenType::Return, TokenType::Result])?.value {
            TokenType::Break => ExitType::Break,
            TokenType::Return => ExitType::Return,
            TokenType::Result => ExitType::Result,
            _ => return None,
        };

        let label = if exit_type == ExitType::Break || exit_type == ExitType::Result {
            match parser.try_consume(TokenType::Colon) {
                Some(_) => Some(parser.consume_or_diagnostic(TokenType::AnyIdentifier)?.unwrap_identifier()),
                _ => None
            }
        } else {
            None
        };

        let expr = if parser.try_consume(TokenType::Semicolon).is_none() {
            let result = parser.apply_rule(ExprRule {}, "exit expression", Some(ErrMsg::ExpectedExpression))?;
            parser.consume_or_diagnostic(TokenType::Semicolon);

            Some(result)
        } else {
            None
        };

        Some(ExitExpr::new(exit_type, expr, label, parser.end_range()))
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
    fn test_exit_rule_check_match_with_break() {
        let rule = ExitRule {};
        let tokens = vec![
            create_token(TokenType::Break),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_exit_rule_check_match_with_return() {
        let rule = ExitRule {};
        let tokens = vec![
            create_token(TokenType::Return),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_exit_rule_check_match_with_result() {
        let rule = ExitRule {};
        let tokens = vec![
            create_token(TokenType::Result),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_exit_rule_check_match_without_exit_keyword() {
        let rule = ExitRule {};
        let tokens = vec![
            create_token(TokenType::If),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(!rule.check_match(cursor));
    }

    #[test]
    fn test_parse_break_without_value() {
        let rule = ExitRule {};
        let tokens = vec![
            create_token(TokenType::Break),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for break without value");
    }

    #[test]
    fn test_parse_return_without_value() {
        let rule = ExitRule {};
        let tokens = vec![
            create_token(TokenType::Return),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for return without value");
    }

    #[test]
    fn test_parse_return_with_integer_literal() {
        let rule = ExitRule {};
        let tokens = vec![
            create_token(TokenType::Return),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for return with integer literal");
    }

    #[test]
    fn test_parse_return_with_variable() {
        let rule = ExitRule {};
        let tokens = vec![
            create_token(TokenType::Return),
            create_token(TokenType::Identifier("result".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for return with variable");
    }

    #[test]
    fn test_parse_return_with_function_call() {
        let rule = ExitRule {};
        let tokens = vec![
            create_token(TokenType::Return),
            create_token(TokenType::Identifier("calculate".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Comma),
            create_token(TokenType::Identifier("y".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for return with function call");
    }

    #[test]
    fn test_parse_return_with_arithmetic_expression() {
        let rule = ExitRule {};
        let tokens = vec![
            create_token(TokenType::Return),
            create_token(TokenType::Identifier("a".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::Identifier("b".to_string())),
            create_token(TokenType::Star),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for return with arithmetic expression");
    }

    #[test]
    fn test_parse_return_with_member_access() {
        let rule = ExitRule {};
        let tokens = vec![
            create_token(TokenType::Return),
            create_token(TokenType::Identifier("obj".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("getValue".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for return with member access");
    }

    #[test]
    fn test_parse_return_with_array_access() {
        let rule = ExitRule {};
        let tokens = vec![
            create_token(TokenType::Return),
            create_token(TokenType::Identifier("arr".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::Identifier("index".to_string())),
            create_token(TokenType::RightSquare),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for return with array access");
    }

    #[test]
    fn test_parse_return_with_boolean_expression() {
        let rule = ExitRule {};
        let tokens = vec![
            create_token(TokenType::Return),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Greater),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::And),
            create_token(TokenType::Identifier("y".to_string())),
            create_token(TokenType::Less),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for return with boolean expression");
    }

    #[test]
    fn test_parse_return_with_constructor_call() {
        let rule = ExitRule {};
        let tokens = vec![
            create_token(TokenType::Return),
            create_token(TokenType::DollarSign),
            create_token(TokenType::Identifier("Result".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::Comma),
            create_token(TokenType::StringLiteral("success".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for return with constructor call");
    }

    #[test]
    fn test_parse_result_with_expression() {
        let rule = ExitRule {};
        let tokens = vec![
            create_token(TokenType::Result),
            create_token(TokenType::Identifier("finalValue".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for result with expression");
    }

    #[test]
    fn test_parse_result_without_value() {
        let rule = ExitRule {};
        let tokens = vec![
            create_token(TokenType::Result),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for result without value");
    }

    #[test]
    fn test_parse_return_with_string_literal() {
        let rule = ExitRule {};
        let tokens = vec![
            create_token(TokenType::Return),
            create_token(TokenType::StringLiteral("Hello, World!".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for return with string literal");
    }

    #[test]
    fn test_parse_return_with_boolean_literal() {
        let rule = ExitRule {};
        let tokens = vec![
            create_token(TokenType::Return),
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for return with boolean literal");
    }

    #[test]
    fn test_parse_return_with_double_literal() {
        let rule = ExitRule {};
        let tokens = vec![
            create_token(TokenType::Return),
            create_token(TokenType::DoubleLiteral(3.14159)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for return with double literal");
    }

    #[test]
    fn test_parse_return_with_char_literal() {
        let rule = ExitRule {};
        let tokens = vec![
            create_token(TokenType::Return),
            create_token(TokenType::CharLiteral('A')),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for return with char literal");
    }

    #[test]
    fn test_parse_return_with_parenthesized_expression() {
        let rule = ExitRule {};
        let tokens = vec![
            create_token(TokenType::Return),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::Identifier("y".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::Star),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for return with parenthesized expression");
    }

    #[test]
    fn test_parse_return_with_unary_expression() {
        let rule = ExitRule {};
        let tokens = vec![
            create_token(TokenType::Return),
            create_token(TokenType::Not),
            create_token(TokenType::Identifier("flag".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for return with unary expression");
    }

    #[test]
    fn test_parse_missing_semicolon_with_expression() {
        let rule = ExitRule {};
        let tokens = vec![
            create_token(TokenType::Return),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should succeed but have diagnostic for missing semicolon
        assert!(result.is_some());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing semicolon");
        assert!(diagnostics.iter().any(|d| d.message.contains("';'")));
    }

    #[test]
    fn test_parse_invalid_expression() {
        let rule = ExitRule {};
        let tokens = vec![
            create_token(TokenType::Return),
            create_token(TokenType::RightCurly), // Invalid expression
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because expression is invalid
        assert!(result.is_none());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for invalid expression");
        assert!(diagnostics.iter().any(|d| d.message.contains("expected expression")));
    }

    #[test]
    fn test_parse_return_with_equality_expression() {
        let rule = ExitRule {};
        let tokens = vec![
            create_token(TokenType::Return),
            create_token(TokenType::Identifier("status".to_string())),
            create_token(TokenType::Equal),
            create_token(TokenType::DollarSign),
            create_token(TokenType::Identifier("Success".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for return with equality expression");
    }

    #[test]
    fn test_parse_return_with_chained_member_access() {
        let rule = ExitRule {};
        let tokens = vec![
            create_token(TokenType::Return),
            create_token(TokenType::Identifier("obj".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("field".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("getValue".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for return with chained member access");
    }

    #[test]
    fn test_parse_break_with_loop_value() {
        let rule = ExitRule {};
        let tokens = vec![
            create_token(TokenType::Break),
            create_token(TokenType::Identifier("loopResult".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for break with loop value");
    }

    #[test]
    fn test_parse_result_with_complex_calculation() {
        let rule = ExitRule {};
        let tokens = vec![
            create_token(TokenType::Result),
            create_token(TokenType::Identifier("sum".to_string())),
            create_token(TokenType::Slash),
            create_token(TokenType::Identifier("count".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::Identifier("bonus".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for result with complex calculation");
    }

    #[test]
    fn test_parse_return_with_logical_or() {
        let rule = ExitRule {};
        let tokens = vec![
            create_token(TokenType::Return),
            create_token(TokenType::Identifier("getValue".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::Or),
            create_token(TokenType::Identifier("defaultValue".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for return with logical OR");
    }
}