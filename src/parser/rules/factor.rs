use std::fmt;

use crate::ast::{ASTNode, BinaryExpr, BinaryOperator};
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::unary::UnaryRule;
use crate::lexer::token::TokenType;

pub struct FactorRule {}

impl fmt::Display for FactorRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Factor")
    }
}

impl ParseRule<Box<dyn ASTNode>> for FactorRule {
    fn check_match(&self, _cursor: ParserCursor) -> bool {
        true
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        let mut expr = parser.apply_rule(UnaryRule {}, "unary expression", None)?;

        while let Some(operator) = parser.try_consume_match(&[TokenType::Slash, TokenType::Star]) {
            let binary_operator: BinaryOperator = operator.value.try_into().unwrap();

            let unary = parser.apply_rule(UnaryRule {}, "unary expression", None)?;
            expr = Box::new(BinaryExpr::new(expr, unary, binary_operator));
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
    fn test_factor_rule_check_match_always_true() {
        let rule = FactorRule {};
        let tokens = vec![create_token(TokenType::EOF)];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        // Factor rule always returns true for check_match
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_parse_simple_unary() {
        let rule = FactorRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for simple unary");
    }

    #[test]
    fn test_parse_multiplication() {
        let rule = FactorRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Star),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for multiplication");
    }

    #[test]
    fn test_parse_division() {
        let rule = FactorRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::Slash),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for division");
    }

    #[test]
    fn test_parse_chained_multiplication() {
        let rule = FactorRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::Star),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::Star),
            create_token(TokenType::IntLiteral(4)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for chained multiplication");
    }

    #[test]
    fn test_parse_chained_division() {
        let rule = FactorRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(24)),
            create_token(TokenType::Slash),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::Slash),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for chained division");
    }

    #[test]
    fn test_parse_mixed_multiplication_division() {
        let rule = FactorRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::Star),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Slash),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for mixed multiplication and division");
    }

    #[test]
    fn test_parse_variables_multiplication() {
        let rule = FactorRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Star),
            create_token(TokenType::Identifier("y".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for variables multiplication");
    }

    #[test]
    fn test_parse_double_multiplication() {
        let rule = FactorRule {};
        let tokens = vec![
            create_token(TokenType::DoubleLiteral(3.14)),
            create_token(TokenType::Star),
            create_token(TokenType::DoubleLiteral(2.0)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for double multiplication");
    }

    #[test]
    fn test_parse_parenthesized_multiplication() {
        let rule = FactorRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::RightParen),
            create_token(TokenType::Star),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for parenthesized multiplication");
    }

    #[test]
    fn test_parse_function_call_multiplication() {
        let rule = FactorRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("getValue".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::Star),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for function call multiplication");
    }

    #[test]
    fn test_parse_array_access_multiplication() {
        let rule = FactorRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("arr".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::Slash),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for array access division");
    }

    #[test]
    fn test_parse_member_access_multiplication() {
        let rule = FactorRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("obj".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("value".to_string())),
            create_token(TokenType::Star),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for member access multiplication");
    }

    #[test]
    fn test_parse_unary_multiplication() {
        let rule = FactorRule {};
        let tokens = vec![
            create_token(TokenType::Minus),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Star),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for unary multiplication");
    }

    #[test]
    fn test_parse_dereference_multiplication() {
        let rule = FactorRule {};
        let tokens = vec![
            create_token(TokenType::Star),
            create_token(TokenType::Identifier("ptr".to_string())),
            create_token(TokenType::Star),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for dereference multiplication");
    }

    #[test]
    fn test_parse_missing_right_operand() {
        let rule = FactorRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Star),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because right operand is missing
        assert!(result.is_none());
        // Should have diagnostic or parser error from unary rule
    }

    #[test]
    fn test_parse_invalid_left_operand() {
        let rule = FactorRule {};
        let tokens = vec![
            create_token(TokenType::RightCurly), // Invalid token for unary
            create_token(TokenType::Star),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because left operand is invalid
        assert!(result.is_none());
        // Should have diagnostic from unary rule
    }

    #[test]
    fn test_parse_invalid_right_operand() {
        let rule = FactorRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Star),
            create_token(TokenType::RightCurly), // Invalid token for unary
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because right operand is invalid
        assert!(result.is_none());
        // Should have diagnostic from unary rule
    }

    #[test]
    fn test_parse_no_multiplicative_operator() {
        let rule = FactorRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Plus), // Not a multiplication/division operator
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should succeed and return just the left unary (5)
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for single unary expression");
    }

    #[test]
    fn test_parse_division_by_zero_literal() {
        let rule = FactorRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::Slash),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should succeed syntactically (division by zero is a runtime issue)
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for division by zero literal");
    }

    #[test]
    fn test_parse_complex_multiplicative_expression() {
        let rule = FactorRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::Star),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::Slash),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::Star),
            create_token(TokenType::IntLiteral(4)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for complex multiplicative expression");
    }

    #[test]
    fn test_parse_constructor_multiplication() {
        let rule = FactorRule {};
        let tokens = vec![
            create_token(TokenType::DollarSign),
            create_token(TokenType::Identifier("Vector".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::Comma),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::RightParen),
            create_token(TokenType::Star),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for constructor multiplication");
    }

    #[test]
    fn test_parse_string_multiplication() {
        let rule = FactorRule {};
        let tokens = vec![
            create_token(TokenType::StringLiteral("hello".to_string())),
            create_token(TokenType::Star),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for string multiplication");
    }

    #[test]
    fn test_parse_negative_number_multiplication() {
        let rule = FactorRule {};
        let tokens = vec![
            create_token(TokenType::Minus),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Star),
            create_token(TokenType::Minus),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for negative number multiplication");
    }

    #[test]
    fn test_parse_pointer_arithmetic() {
        let rule = FactorRule {};
        let tokens = vec![
            create_token(TokenType::Star),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Identifier("ptr".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::RightParen),
            create_token(TokenType::Star),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for pointer arithmetic multiplication");
    }

    #[test]
    fn test_parse_long_multiplicative_chain() {
        let rule = FactorRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::Star),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::Slash),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::Star),
            create_token(TokenType::IntLiteral(4)),
            create_token(TokenType::Slash),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::Star),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for long multiplicative chain");
    }
}