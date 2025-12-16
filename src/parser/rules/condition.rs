use std::fmt;

use crate::ast::{ASTNode, BinaryExpr, BinaryOperator};
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::boolean_factor::BooleanFactorRule;
use crate::lexer::token::TokenType;

pub struct ConditionRule {}

impl fmt::Display for ConditionRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Condition")
    }
}

impl ParseRule<Box<dyn ASTNode>> for ConditionRule {
    fn check_match(&self, _cursor: ParserCursor) -> bool {
        true
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {    
        let mut expr = parser.apply_rule(BooleanFactorRule {}, "boolean factor", None)?;
    
        while let Some(operator) = parser.try_consume(TokenType::Or) {
            let binary_operator: BinaryOperator = operator.value.try_into().unwrap();

            let boolean_factor = parser.apply_rule(BooleanFactorRule {}, "boolean factor", None);    
            expr = Box::new(BinaryExpr::new(expr, boolean_factor?, binary_operator));
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
    fn test_condition_rule_check_match_always_true() {
        let rule = ConditionRule {};
        let tokens = vec![create_token(TokenType::EOF)];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        // Condition rule always returns true for check_match
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_parse_simple_boolean_factor() {
        let rule = ConditionRule {};
        let tokens = vec![
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for simple boolean factor");
    }

    #[test]
    fn test_parse_simple_or_operation() {
        let rule = ConditionRule {};
        let tokens = vec![
            create_token(TokenType::BoolLiteral(false)),
            create_token(TokenType::Or),
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for simple OR operation");
    }

    #[test]
    fn test_parse_chained_or_operations() {
        let rule = ConditionRule {};
        let tokens = vec![
            create_token(TokenType::BoolLiteral(false)),
            create_token(TokenType::Or),
            create_token(TokenType::BoolLiteral(false)),
            create_token(TokenType::Or),
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for chained OR operations");
    }

    #[test]
    fn test_parse_comparisons_with_or() {
        let rule = ConditionRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Greater),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::Or),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::Less),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for comparisons with OR");
    }

    #[test]
    fn test_parse_equalities_with_or() {
        let rule = ConditionRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Equal),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::Or),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::NotEqual),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for equalities with OR");
    }

    #[test]
    fn test_parse_and_with_or() {
        let rule = ConditionRule {};
        let tokens = vec![
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::And),
            create_token(TokenType::BoolLiteral(false)),
            create_token(TokenType::Or),
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::And),
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for AND with OR (proper precedence)");
    }

    #[test]
    fn test_parse_variables_with_or() {
        let rule = ConditionRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("isValid".to_string())),
            create_token(TokenType::Or),
            create_token(TokenType::Identifier("isReady".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for variables with OR");
    }

    #[test]
    fn test_parse_function_calls_with_or() {
        let rule = ConditionRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("checkA".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::Or),
            create_token(TokenType::Identifier("checkB".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for function calls with OR");
    }

    #[test]
    fn test_parse_parenthesized_conditions() {
        let rule = ConditionRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Greater),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::RightParen),
            create_token(TokenType::Or),
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::Less),
            create_token(TokenType::IntLiteral(8)),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for parenthesized conditions");
    }

    #[test]
    fn test_parse_complex_boolean_expression() {
        let rule = ConditionRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Greater),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::And),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Less),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::Or),
            create_token(TokenType::Identifier("y".to_string())),
            create_token(TokenType::Equal),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for complex boolean expression");
    }

    #[test]
    fn test_parse_unary_with_or() {
        let rule = ConditionRule {};
        let tokens = vec![
            create_token(TokenType::Not),
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::Or),
            create_token(TokenType::Not),
            create_token(TokenType::BoolLiteral(false)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for unary expressions with OR");
    }

    #[test]
    fn test_parse_missing_right_operand() {
        let rule = ConditionRule {};
        let tokens = vec![
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::Or),
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
        let rule = ConditionRule {};
        let tokens = vec![
            create_token(TokenType::RightCurly), // Invalid token for boolean factor
            create_token(TokenType::Or),
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
        let rule = ConditionRule {};
        let tokens = vec![
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::Or),
            create_token(TokenType::RightCurly), // Invalid token for boolean factor
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
    fn test_parse_no_or_operator() {
        let rule = ConditionRule {};
        let tokens = vec![
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::And),
            create_token(TokenType::BoolLiteral(false)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should succeed and return just the boolean factor (true AND false)
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for boolean factor without OR");
    }

    #[test]
    fn test_parse_multiple_or_operators() {
        let rule = ConditionRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::Equal),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::Or),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::Equal),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::Or),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::Equal),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for multiple OR operators");
    }

    #[test]
    fn test_parse_mixed_logical_operators() {
        let rule = ConditionRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Greater),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::And),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::Less),
            create_token(TokenType::IntLiteral(20)),
            create_token(TokenType::Or),
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::Equal),
            create_token(TokenType::BoolLiteral(false)),
            create_token(TokenType::And),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::NotEqual),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for mixed logical operators");
    }

    #[test]
    fn test_parse_member_access_conditions() {
        let rule = ConditionRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("obj".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("isValid".to_string())),
            create_token(TokenType::Or),
            create_token(TokenType::Identifier("obj".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("isReady".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for member access conditions");
    }

    #[test]
    fn test_parse_array_conditions() {
        let rule = ConditionRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("flags".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::Or),
            create_token(TokenType::Identifier("flags".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for array access conditions");
    }

    #[test]
    fn test_parse_string_conditions() {
        let rule = ConditionRule {};
        let tokens = vec![
            create_token(TokenType::StringLiteral("name".to_string())),
            create_token(TokenType::Equal),
            create_token(TokenType::StringLiteral("John".to_string())),
            create_token(TokenType::Or),
            create_token(TokenType::StringLiteral("name".to_string())),
            create_token(TokenType::Equal),
            create_token(TokenType::StringLiteral("Jane".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for string equality conditions");
    }

    #[test]
    fn test_parse_constructor_conditions() {
        let rule = ConditionRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("result".to_string())),
            create_token(TokenType::Equal),
            create_token(TokenType::DollarSign),
            create_token(TokenType::Identifier("Success".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::Or),
            create_token(TokenType::Identifier("retry".to_string())),
            create_token(TokenType::Equal),
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for constructor conditions");
    }
}

