use std::fmt;

use crate::ast::{ASTNode, BinaryExpr, BinaryOperator};
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::term::TermRule;
use crate::lexer::token::TokenType;

pub struct ComparisonRule {}

impl fmt::Display for ComparisonRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Comparison")
    }
}

impl ParseRule<Box<dyn ASTNode>> for ComparisonRule {
    fn check_match(&self, _cursor: ParserCursor) -> bool {
        true
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {

        let mut expr = parser.apply_rule(TermRule {}, "term expression", None)?;
        let matches = [TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual];
        
        while let Some(operator) = parser.try_consume_match(&matches) {
            let binary_operator: BinaryOperator = operator.value.try_into().unwrap();

            let term = parser.apply_rule(TermRule {}, "term expression", None)?;            
            expr = Box::new(BinaryExpr::new(expr, term, binary_operator));
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
    fn test_comparison_rule_check_match_always_true() {
        let rule = ComparisonRule {};
        let tokens = vec![create_token(TokenType::EOF)];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        // Comparison rule always returns true for check_match
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_parse_simple_term() {
        let rule = ComparisonRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for simple term");
    }

    #[test]
    fn test_parse_less_than_comparison() {
        let rule = ComparisonRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Less),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for less than comparison");
    }

    #[test]
    fn test_parse_less_equal_comparison() {
        let rule = ComparisonRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::LessEqual),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for less equal comparison");
    }

    #[test]
    fn test_parse_greater_than_comparison() {
        let rule = ComparisonRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::Greater),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for greater than comparison");
    }

    #[test]
    fn test_parse_greater_equal_comparison() {
        let rule = ComparisonRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::GreaterEqual),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for greater equal comparison");
    }

    #[test]
    fn test_parse_chained_comparisons() {
        let rule = ComparisonRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::Less),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Less),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for chained comparisons");
    }

    #[test]
    fn test_parse_mixed_comparison_operators() {
        let rule = ComparisonRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::Less),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Greater),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for mixed comparison operators");
    }

    #[test]
    fn test_parse_comparison_with_variables() {
        let rule = ComparisonRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Less),
            create_token(TokenType::Identifier("y".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for comparison with variables");
    }

    #[test]
    fn test_parse_comparison_with_expressions() {
        let rule = ComparisonRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::Greater),
            create_token(TokenType::IntLiteral(4)),
            create_token(TokenType::Star),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for comparison with expressions");
    }

    #[test]
    fn test_parse_missing_right_operand() {
        let rule = ComparisonRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Less),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because right operand is missing
        assert!(result.is_none());
        // Should have diagnostic or parser error from term rule
    }

    #[test]
    fn test_parse_invalid_left_operand() {
        let rule = ComparisonRule {};
        let tokens = vec![
            create_token(TokenType::RightCurly), // Invalid token for term
            create_token(TokenType::Less),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because left operand is invalid
        assert!(result.is_none());
        // Should have diagnostic from term rule
    }

    #[test]
    fn test_parse_invalid_right_operand() {
        let rule = ComparisonRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Less),
            create_token(TokenType::RightCurly), // Invalid token for term
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because right operand is invalid
        assert!(result.is_none());
        // Should have diagnostic from term rule
    }

    #[test]
    fn test_parse_no_comparison_operator() {
        let rule = ComparisonRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Plus), // Not a comparison operator
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should succeed and return just the left term (5 + 3)
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for arithmetic expression");
    }

    #[test]
    fn test_parse_double_literals_comparison() {
        let rule = ComparisonRule {};
        let tokens = vec![
            create_token(TokenType::DoubleLiteral(3.14)),
            create_token(TokenType::Greater),
            create_token(TokenType::DoubleLiteral(2.71)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for double literals comparison");
    }

    #[test]
    fn test_parse_bool_literals_comparison() {
        let rule = ComparisonRule {};
        let tokens = vec![
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::GreaterEqual),
            create_token(TokenType::BoolLiteral(false)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for bool literals comparison");
    }

    #[test]
    fn test_parse_complex_chained_comparisons() {
        let rule = ComparisonRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::Less),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::LessEqual),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::Greater),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::GreaterEqual),
            create_token(TokenType::IntLiteral(-1)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for complex chained comparisons");
    }

    #[test]
    fn test_parse_parenthesized_comparisons() {
        let rule = ComparisonRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::RightParen),
            create_token(TokenType::Greater),
            create_token(TokenType::IntLiteral(7)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for parenthesized comparisons");
    }
}
