use std::fmt;

use crate::ast::{ASTNode, BinaryExpr, BinaryOperator};
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::equality::EqualityRule;
use crate::lexer::token::TokenType;

pub struct BooleanFactorRule {}

impl fmt::Display for BooleanFactorRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BooleanFactor")
    }
}

impl ParseRule<Box<dyn ASTNode>> for BooleanFactorRule {
    fn check_match(&self, _cursor: ParserCursor) -> bool {
        true
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {

        let mut equality = parser.apply_rule(EqualityRule {}, "equality expression", Some(ErrMsg::ExpectedExpression));
        let mut expr = equality?;

        while let Some(operator) = parser.try_consume(TokenType::And) {
            let binary_operator: BinaryOperator = operator.value.try_into().unwrap();

            equality = parser.apply_rule(EqualityRule {}, "equality expression", Some(ErrMsg::ExpectedExpression));
            expr = Box::new(BinaryExpr::new(expr, equality?, binary_operator));
        }

        Some(expr)
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
    fn test_boolean_factor_rule_check_match_always_true() {
        let rule = BooleanFactorRule {};
        let tokens = vec![create_token(TokenType::EOF)];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        // Boolean factor rule always returns true for check_match
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_parse_simple_equality() {
        let rule = BooleanFactorRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Equal),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for simple equality");
    }

    #[test]
    fn test_parse_simple_and_operation() {
        let rule = BooleanFactorRule {};
        let tokens = vec![
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::And),
            create_token(TokenType::BoolLiteral(false)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for simple AND operation");
    }

    #[test]
    fn test_parse_chained_and_operations() {
        let rule = BooleanFactorRule {};
        let tokens = vec![
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::And),
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::And),
            create_token(TokenType::BoolLiteral(false)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for chained AND operations");
    }

    #[test]
    fn test_parse_comparisons_with_and() {
        let rule = BooleanFactorRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Greater),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::And),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::Less),
            create_token(TokenType::IntLiteral(20)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for comparisons with AND");
    }

    #[test]
    fn test_parse_equalities_with_and() {
        let rule = BooleanFactorRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Equal),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::And),
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::NotEqual),
            create_token(TokenType::BoolLiteral(false)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for equalities with AND");
    }

    #[test]
    fn test_parse_variables_with_and() {
        let rule = BooleanFactorRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::And),
            create_token(TokenType::Identifier("y".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for variables with AND");
    }

    #[test]
    fn test_parse_complex_expressions_with_and() {
        let rule = BooleanFactorRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::RightParen),
            create_token(TokenType::Equal),
            create_token(TokenType::IntLiteral(8)),
            create_token(TokenType::And),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::Greater),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for complex expressions with AND");
    }

    #[test]
    fn test_parse_missing_right_operand() {
        let rule = BooleanFactorRule {};
        let tokens = vec![
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::And),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because right operand is missing
        assert!(result.is_none());
        // Should have diagnostic for expected expression
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing right operand");
        assert!(diagnostics.iter().any(|d| d.message.contains("expected expression")));
    }

    #[test]
    fn test_parse_invalid_left_operand() {
        let rule = BooleanFactorRule {};
        let tokens = vec![
            create_token(TokenType::RightCurly), // Invalid token for equality
            create_token(TokenType::And),
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because left operand is invalid
        assert!(result.is_none());
        // Should have diagnostic for expected expression
        assert!(!diagnostics.is_empty(), "Expected diagnostic for invalid left operand");
        assert!(diagnostics.iter().any(|d| d.message.contains("expected expression")));
    }

    #[test]
    fn test_parse_invalid_right_operand() {
        let rule = BooleanFactorRule {};
        let tokens = vec![
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::And),
            create_token(TokenType::RightCurly), // Invalid token for equality
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because right operand is invalid
        assert!(result.is_none());
        // Should have diagnostic for expected expression
        assert!(!diagnostics.is_empty(), "Expected diagnostic for invalid right operand");
        assert!(diagnostics.iter().any(|d| d.message.contains("expected expression")));
    }

    #[test]
    fn test_parse_no_and_operator() {
        let rule = BooleanFactorRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Equal),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should succeed and return just the equality expression
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for simple equality without AND");
    }

    #[test]
    fn test_parse_multiple_and_operators() {
        let rule = BooleanFactorRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::Equal),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::And),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::Equal),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::And),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::Equal),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for multiple AND operators");
    }

    #[test]
    fn test_parse_nested_expressions_with_and() {
        let rule = BooleanFactorRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Greater),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::RightParen),
            create_token(TokenType::And),
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::Less),
            create_token(TokenType::IntLiteral(20)),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for nested expressions with AND");
    }

    #[test]
    fn test_parse_function_calls_with_and() {
        let rule = BooleanFactorRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("isValid".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::And),
            create_token(TokenType::Identifier("isReady".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Identifier("y".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for function calls with AND");
    }

    #[test]
    fn test_parse_member_access_with_and() {
        let rule = BooleanFactorRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("obj".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("isValid".to_string())),
            create_token(TokenType::And),
            create_token(TokenType::Identifier("obj".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("isReady".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for member access with AND");
    }

    #[test]
    fn test_parse_mixed_operators_and_precedence() {
        let rule = BooleanFactorRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Greater),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::And),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::Equal),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::And),
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::NotEqual),
            create_token(TokenType::BoolLiteral(false)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for mixed operators with proper precedence");
    }

    #[test]
    fn test_parse_string_equality_with_and() {
        let rule = BooleanFactorRule {};
        let tokens = vec![
            create_token(TokenType::StringLiteral("hello".to_string())),
            create_token(TokenType::Equal),
            create_token(TokenType::StringLiteral("hello".to_string())),
            create_token(TokenType::And),
            create_token(TokenType::StringLiteral("world".to_string())),
            create_token(TokenType::NotEqual),
            create_token(TokenType::StringLiteral("hello".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for string equality with AND");
    }

    #[test]
    fn test_parse_char_comparison_with_and() {
        let rule = BooleanFactorRule {};
        let tokens = vec![
            create_token(TokenType::CharLiteral('a')),
            create_token(TokenType::Less),
            create_token(TokenType::CharLiteral('z')),
            create_token(TokenType::And),
            create_token(TokenType::CharLiteral('A')),
            create_token(TokenType::Greater),
            create_token(TokenType::CharLiteral('@')),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for char comparison with AND");
    }

    #[test]
    fn test_parse_array_access_with_and() {
        let rule = BooleanFactorRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("arr".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::Equal),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::And),
            create_token(TokenType::Identifier("arr".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::NotEqual),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for array access with AND");
    }
}

