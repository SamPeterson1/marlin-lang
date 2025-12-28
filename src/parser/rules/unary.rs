use std::fmt;

use crate::ast::{ASTNode, UnaryExpr, UnaryOperator};
use crate::parser::rules::member_access::MemberAccessRule;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::lexer::token::TokenType;

pub struct UnaryRule {}

impl fmt::Display for UnaryRule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Unary")
    }
}

impl ParseRule<Box<dyn ASTNode>> for UnaryRule {
    fn check_match(&self,  _cursor: ParserCursor) -> bool {
        true
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        parser.begin_range();

        if let Some(operator) = parser.try_consume_match(&[TokenType::Not, TokenType::Minus, TokenType::Star, TokenType::Ampersand, TokenType::Tilda]) {
            let unary_operator: UnaryOperator = operator.value.try_into().unwrap();
            let unary = parser.apply_rule(UnaryRule {}, "unary expression", None)?;
            Some(Box::new(UnaryExpr::new(unary, unary_operator, parser.end_range())))
        } else {
            parser.apply_rule(MemberAccessRule {}, "member access expression", None)
        }
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
    fn test_unary_rule_check_match_always_true() {
        let rule = UnaryRule {};
        let tokens = vec![create_token(TokenType::EOF)];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        // Unary rule always returns true for check_match
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_parse_simple_member_access() {
        let rule = UnaryRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("variable".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for simple member access");
    }

    #[test]
    fn test_parse_logical_not_operator() {
        let rule = UnaryRule {};
        let tokens = vec![
            create_token(TokenType::Not),
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for logical NOT operator");
    }

    #[test]
    fn test_parse_negative_operator() {
        let rule = UnaryRule {};
        let tokens = vec![
            create_token(TokenType::Minus),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for negative operator");
    }

    #[test]
    fn test_parse_dereference_operator() {
        let rule = UnaryRule {};
        let tokens = vec![
            create_token(TokenType::Star),
            create_token(TokenType::Identifier("ptr".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for dereference operator");
    }

    #[test]
    fn test_parse_address_of_operator() {
        let rule = UnaryRule {};
        let tokens = vec![
            create_token(TokenType::Ampersand),
            create_token(TokenType::Identifier("variable".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for address-of operator");
    }

    #[test]
    fn test_parse_chained_unary_operators() {
        let rule = UnaryRule {};
        let tokens = vec![
            create_token(TokenType::Not),
            create_token(TokenType::Not),
            create_token(TokenType::BoolLiteral(false)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for chained unary operators");
    }

    #[test]
    fn test_parse_mixed_unary_operators() {
        let rule = UnaryRule {};
        let tokens = vec![
            create_token(TokenType::Minus),
            create_token(TokenType::Star),
            create_token(TokenType::Identifier("ptr".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for mixed unary operators");
    }

    #[test]
    fn test_parse_unary_with_parentheses() {
        let rule = UnaryRule {};
        let tokens = vec![
            create_token(TokenType::Not),
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Greater),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for unary with parentheses");
    }

    #[test]
    fn test_parse_unary_with_function_call() {
        let rule = UnaryRule {};
        let tokens = vec![
            create_token(TokenType::Not),
            create_token(TokenType::Identifier("isValid".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for unary with function call");
    }

    #[test]
    fn test_parse_unary_with_member_access() {
        let rule = UnaryRule {};
        let tokens = vec![
            create_token(TokenType::Ampersand),
            create_token(TokenType::Identifier("obj".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("field".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for unary with member access");
    }

    #[test]
    fn test_parse_unary_with_array_access() {
        let rule = UnaryRule {};
        let tokens = vec![
            create_token(TokenType::Star),
            create_token(TokenType::Identifier("arr".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for unary with array access");
    }

    #[test]
    fn test_parse_missing_operand() {
        let rule = UnaryRule {};
        let tokens = vec![
            create_token(TokenType::Not),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because operand is missing
        assert!(result.is_none());
        // Should have diagnostic for expected expression
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing operand");
        assert!(diagnostics.iter().any(|d| d.message.contains("expected expression")));
    }

    #[test]
    fn test_parse_invalid_operand() {
        let rule = UnaryRule {};
        let tokens = vec![
            create_token(TokenType::Minus),
            create_token(TokenType::RightCurly), // Invalid token for unary operand
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because operand is invalid
        assert!(result.is_none());
        // Should have diagnostic for expected expression
        assert!(!diagnostics.is_empty(), "Expected diagnostic for invalid operand");
        assert!(diagnostics.iter().any(|d| d.message.contains("expected expression")));
    }

    #[test]
    fn test_parse_double_negative() {
        let rule = UnaryRule {};
        let tokens = vec![
            create_token(TokenType::Minus),
            create_token(TokenType::Minus),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for double negative");
    }

    #[test]
    fn test_parse_complex_nested_unary() {
        let rule = UnaryRule {};
        let tokens = vec![
            create_token(TokenType::Not),
            create_token(TokenType::Star),
            create_token(TokenType::Ampersand),
            create_token(TokenType::Identifier("variable".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for complex nested unary operators");
    }

    #[test]
    fn test_parse_unary_with_literals() {
        let rule = UnaryRule {};
        let tokens = vec![
            create_token(TokenType::Minus),
            create_token(TokenType::DoubleLiteral(3.14)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for unary with double literal");
    }

    #[test]
    fn test_parse_not_with_string_comparison() {
        let rule = UnaryRule {};
        let tokens = vec![
            create_token(TokenType::Not),
            create_token(TokenType::LeftParen),
            create_token(TokenType::StringLiteral("hello".to_string())),
            create_token(TokenType::Equal),
            create_token(TokenType::StringLiteral("world".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for NOT with string comparison");
    }

    #[test]
    fn test_parse_address_of_array_element() {
        let rule = UnaryRule {};
        let tokens = vec![
            create_token(TokenType::Ampersand),
            create_token(TokenType::Identifier("arr".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for address-of array element");
    }

    #[test]
    fn test_parse_dereference_member_access() {
        let rule = UnaryRule {};
        let tokens = vec![
            create_token(TokenType::Star),
            create_token(TokenType::Identifier("obj_ptr".to_string())),
            create_token(TokenType::Arrow),
            create_token(TokenType::Identifier("field".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for dereference with arrow access");
    }

    #[test]
    fn test_parse_unary_with_constructor_call() {
        let rule = UnaryRule {};
        let tokens = vec![
            create_token(TokenType::Ampersand),
            create_token(TokenType::DollarSign),
            create_token(TokenType::Identifier("Person".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::StringLiteral("John".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for unary with constructor call");
    }

    #[test]
    fn test_parse_unary_with_new_array() {
        let rule = UnaryRule {};
        let tokens = vec![
            create_token(TokenType::Ampersand),
            create_token(TokenType::New),
            create_token(TokenType::Int),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for unary with new array");
    }
}