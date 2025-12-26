use std::fmt;

use crate::ast::{ASTNode, BinaryExpr, BinaryOperator};
use crate::parser::rules::cast::CastRule;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::lexer::token::TokenType;

pub struct BinaryExprRule {}

impl fmt::Display for BinaryExprRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BinaryExpression")
    }
}

impl ParseRule<Box<dyn ASTNode>> for BinaryExprRule {
    fn check_match(&self, _cursor: ParserCursor) -> bool {
        true
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        self.parse_condition(parser)
    }
}

impl BinaryExprRule {
    // condition: boolean-factor ("or" boolean-factor)*
    fn parse_condition(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        let mut expr = self.parse_boolean_factor(parser)?;
        
        while let Some(operator) = parser.try_consume(TokenType::Or) {
            let binary_operator: BinaryOperator = operator.value.try_into().unwrap();
            let right = self.parse_boolean_factor(parser)?;
            expr = Box::new(BinaryExpr::new(expr, right, binary_operator));
        }
        
        Some(expr)
    }

    // boolean-factor: bitwise-term ("and" bitwise-term)*
    fn parse_boolean_factor(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        let mut expr = self.parse_bitwise_term(parser)?;
        
        while let Some(operator) = parser.try_consume(TokenType::And) {
            let binary_operator: BinaryOperator = operator.value.try_into().unwrap();
            let right = self.parse_bitwise_term(parser)?;
            expr = Box::new(BinaryExpr::new(expr, right, binary_operator));
        }
        
        Some(expr)
    }

    // bitwise-term: bitwise-xor ("|" bitwise-xor)*
    fn parse_bitwise_term(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        let mut expr = self.parse_bitwise_xor(parser)?;
        
        while let Some(operator) = parser.try_consume(TokenType::Bar) {
            let binary_operator: BinaryOperator = operator.value.try_into().unwrap();
            let right = self.parse_bitwise_xor(parser)?;
            expr = Box::new(BinaryExpr::new(expr, right, binary_operator));
        }
        
        Some(expr)
    }

    // bitwise-xor: bitwise-factor ("^" bitwise-factor)*
    fn parse_bitwise_xor(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        let mut expr = self.parse_bitwise_factor(parser)?;
        
        while let Some(operator) = parser.try_consume(TokenType::Carat) {
            let binary_operator: BinaryOperator = operator.value.try_into().unwrap();
            let right = self.parse_bitwise_factor(parser)?;
            expr = Box::new(BinaryExpr::new(expr, right, binary_operator));
        }
        
        Some(expr)
    }

    // bitwise-factor: equality ("&" equality)*
    fn parse_bitwise_factor(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        let mut expr = self.parse_equality(parser)?;
        
        while let Some(operator) = parser.try_consume(TokenType::Ampersand) {
            let binary_operator: BinaryOperator = operator.value.try_into().unwrap();
            let right = self.parse_equality(parser)?;
            expr = Box::new(BinaryExpr::new(expr, right, binary_operator));
        }
        
        Some(expr)
    }

    // equality: comparison (("==" | "!=") comparison)*
    fn parse_equality(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        let mut expr = self.parse_comparison(parser)?;
        
        while let Some(operator) = parser.try_consume_match(&[TokenType::Equal, TokenType::NotEqual]) {
            let binary_operator: BinaryOperator = operator.value.try_into().unwrap();
            let right = self.parse_comparison(parser)?;
            expr = Box::new(BinaryExpr::new(expr, right, binary_operator));
        }
        
        Some(expr)
    }

    // comparison: bitwise-shift (("<" | "<=" | ">" | ">=") bitwise-shift)*
    fn parse_comparison(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        let mut expr = self.parse_bitwise_shift(parser)?;
        
        while let Some(operator) = parser.try_consume_match(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual
        ]) {
            let binary_operator: BinaryOperator = operator.value.try_into().unwrap();
            let right = self.parse_bitwise_shift(parser)?;
            expr = Box::new(BinaryExpr::new(expr, right, binary_operator));
        }
        
        Some(expr)
    }

    // bitwise-shift: arithmetic-term (("<<" | ">>") arithmetic-term)*
    fn parse_bitwise_shift(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        let mut expr = self.parse_arithmetic_term(parser)?;
        
        while let Some(operator) = parser.try_consume_match(&[TokenType::LeftShift, TokenType::RightShift]) {
            let binary_operator: BinaryOperator = operator.value.try_into().unwrap();
            let right = self.parse_arithmetic_term(parser)?;
            expr = Box::new(BinaryExpr::new(expr, right, binary_operator));
        }
        
        Some(expr)
    }

    // arithmetic-term: arithmetic-factor (("-" | "+") arithmetic-factor)*
    fn parse_arithmetic_term(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        let mut expr = self.parse_arithmetic_factor(parser)?;
        
        while let Some(operator) = parser.try_consume_match(&[TokenType::Minus, TokenType::Plus]) {
            let binary_operator: BinaryOperator = operator.value.try_into().unwrap();
            let right = self.parse_arithmetic_factor(parser)?;
            expr = Box::new(BinaryExpr::new(expr, right, binary_operator));
        }
        
        Some(expr)
    }

    // arithmetic-factor: unary (("*" | "/" | "%") unary)*
    fn parse_arithmetic_factor(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        let mut expr = parser.apply_rule(CastRule {}, "cast expression", None)?;
        
        while let Some(operator) = parser.try_consume_match(&[TokenType::Star, TokenType::Slash, TokenType::Percentage]) {
            let binary_operator: BinaryOperator = operator.value.try_into().unwrap();
            let right = parser.apply_rule(CastRule {}, "cast expression", None)?;
            expr = Box::new(BinaryExpr::new(expr, right, binary_operator));
        }
        
        Some(expr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::{Token, PositionRange};

    fn create_token(token_type: TokenType) -> Token {
        Token::new(token_type, PositionRange::zero())
    }

    #[test]
    fn test_simple_literal() {
        let rule = BinaryExprRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_addition() {
        let rule = BinaryExprRule {};
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
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_multiplication() {
        let rule = BinaryExprRule {};
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
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_modulo() {
        let rule = BinaryExprRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::Percentage),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_left_shift() {
        let rule = BinaryExprRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::LeftShift),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_right_shift() {
        let rule = BinaryExprRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(20)),
            create_token(TokenType::RightShift),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_bitwise_xor() {
        let rule = BinaryExprRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Carat),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_bitwise_and() {
        let rule = BinaryExprRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Ampersand),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_bitwise_or() {
        let rule = BinaryExprRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Bar),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_comparison() {
        let rule = BinaryExprRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Greater),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_equality() {
        let rule = BinaryExprRule {};
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
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_logical_and() {
        let rule = BinaryExprRule {};
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
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_logical_or() {
        let rule = BinaryExprRule {};
        let tokens = vec![
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::Or),
            create_token(TokenType::BoolLiteral(false)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_precedence_multiplication_over_addition() {
        // 2 + 3 * 4 should be parsed as 2 + (3 * 4)
        let rule = BinaryExprRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::Star),
            create_token(TokenType::IntLiteral(4)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_precedence_shift_over_addition() {
        // 2 + 3 << 1 should be parsed as (2 + 3) << 1
        let rule = BinaryExprRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::LeftShift),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_precedence_comparison_over_shift() {
        // 5 << 1 > 8 should be parsed as (5 << 1) > 8
        let rule = BinaryExprRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::LeftShift),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::Greater),
            create_token(TokenType::IntLiteral(8)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_precedence_and_over_or() {
        // true or false and true should be parsed as true or (false and true)
        let rule = BinaryExprRule {};
        let tokens = vec![
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::Or),
            create_token(TokenType::BoolLiteral(false)),
            create_token(TokenType::And),
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_complex_bitwise_expression() {
        // a & b | c ^ d should be parsed as ((a & b) | c) ^ d
        let rule = BinaryExprRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("a".to_string())),
            create_token(TokenType::Ampersand),
            create_token(TokenType::Identifier("b".to_string())),
            create_token(TokenType::Bar),
            create_token(TokenType::Identifier("c".to_string())),
            create_token(TokenType::Carat),
            create_token(TokenType::Identifier("d".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_all_operators_combined() {
        // x + y * z % 2 << 1 > 10 & 1 ^ 2 | 3 and true or false
        let rule = BinaryExprRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::Identifier("y".to_string())),
            create_token(TokenType::Star),
            create_token(TokenType::Identifier("z".to_string())),
            create_token(TokenType::Percentage),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::LeftShift),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::Greater),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::Ampersand),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::Carat),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::Bar),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::And),
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::Or),
            create_token(TokenType::BoolLiteral(false)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }
}
