use std::fmt;

use crate::ast::{ASTNode, BinaryExpr, BinaryOperator};
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::factor::FactorRule;
use crate::lexer::token::TokenType;

pub struct TermRule {}

impl fmt::Display for TermRule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Term")
    }
}

impl ParseRule<Box<dyn ASTNode>> for TermRule {
    fn check_match(&self, _cursor: ParserCursor) -> bool {
        true
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        let mut expr =  parser.apply_rule(FactorRule {}, "factor expression", None)?;

        while let Some(operator) = parser.try_consume_match(&[TokenType::Minus, TokenType::Plus]) {
            let binary_operator: BinaryOperator = operator.value.try_into().unwrap();

            let factor =  parser.apply_rule(FactorRule {}, "factor expression", None)?;
            expr = Box::new(BinaryExpr::new(expr, factor, binary_operator));
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
    fn test_term_rule_check_match_always_true() {
        let rule = TermRule {};
        let tokens = vec![create_token(TokenType::EOF)];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        // Term rule always returns true for check_match
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_parse_simple_factor() {
        let rule = TermRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for simple factor");
    }

    #[test]
    fn test_parse_addition() {
        let rule = TermRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for addition");
    }

    #[test]
    fn test_parse_subtraction() {
        let rule = TermRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::Minus),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for subtraction");
    }

    #[test]
    fn test_parse_chained_addition() {
        let rule = TermRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for chained addition");
    }

    #[test]
    fn test_parse_chained_subtraction() {
        let rule = TermRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::Minus),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::Minus),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for chained subtraction");
    }

    #[test]
    fn test_parse_mixed_addition_subtraction() {
        let rule = TermRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Minus),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for mixed addition and subtraction");
    }

    #[test]
    fn test_parse_variables_arithmetic() {
        let rule = TermRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::Identifier("y".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for variables arithmetic");
    }

    #[test]
    fn test_parse_double_arithmetic() {
        let rule = TermRule {};
        let tokens = vec![
            create_token(TokenType::DoubleLiteral(3.14)),
            create_token(TokenType::Plus),
            create_token(TokenType::DoubleLiteral(2.71)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for double arithmetic");
    }

    #[test]
    fn test_parse_multiplication_with_addition() {
        let rule = TermRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::Star),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(4)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for multiplication with addition");
    }

    #[test]
    fn test_parse_parenthesized_addition() {
        let rule = TermRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::RightParen),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for parenthesized addition");
    }

    #[test]
    fn test_parse_function_call_addition() {
        let rule = TermRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("getValue".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for function call addition");
    }

    #[test]
    fn test_parse_array_access_arithmetic() {
        let rule = TermRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("arr".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::Minus),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for array access arithmetic");
    }

    #[test]
    fn test_parse_member_access_arithmetic() {
        let rule = TermRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("obj".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("value".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for member access arithmetic");
    }

    #[test]
    fn test_parse_unary_with_arithmetic() {
        let rule = TermRule {};
        let tokens = vec![
            create_token(TokenType::Minus),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for unary with arithmetic");
    }

    #[test]
    fn test_parse_missing_right_operand() {
        let rule = TermRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Plus),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because right operand is missing
        assert!(result.is_none());
        // Should have diagnostic or parser error from factor rule
    }

    #[test]
    fn test_parse_invalid_left_operand() {
        let rule = TermRule {};
        let tokens = vec![
            create_token(TokenType::RightCurly), // Invalid token for factor
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because left operand is invalid
        assert!(result.is_none());
        // Should have diagnostic from factor rule
    }

    #[test]
    fn test_parse_invalid_right_operand() {
        let rule = TermRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Plus),
            create_token(TokenType::RightCurly), // Invalid token for factor
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because right operand is invalid
        assert!(result.is_none());
        // Should have diagnostic from factor rule
    }

    #[test]
    fn test_parse_no_arithmetic_operator() {
        let rule = TermRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Star), // Not an addition/subtraction operator
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should succeed and return just the left factor (5 * 3)
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for multiplication expression");
    }

    #[test]
    fn test_parse_complex_expression() {
        let rule = TermRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::Star),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::Minus),
            create_token(TokenType::IntLiteral(4)),
            create_token(TokenType::Slash),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for complex arithmetic expression");
    }

    #[test]
    fn test_parse_string_concatenation() {
        let rule = TermRule {};
        let tokens = vec![
            create_token(TokenType::StringLiteral("Hello".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::StringLiteral(" World".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for string concatenation");
    }

    #[test]
    fn test_parse_constructor_arithmetic() {
        let rule = TermRule {};
        let tokens = vec![
            create_token(TokenType::DollarSign),
            create_token(TokenType::Identifier("Point".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::Comma),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::RightParen),
            create_token(TokenType::Plus),
            create_token(TokenType::DollarSign),
            create_token(TokenType::Identifier("Point".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::Comma),
            create_token(TokenType::IntLiteral(4)),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for constructor arithmetic");
    }

    #[test]
    fn test_parse_long_arithmetic_chain() {
        let rule = TermRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::Minus),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(4)),
            create_token(TokenType::Minus),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(6)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for long arithmetic chain");
    }
}