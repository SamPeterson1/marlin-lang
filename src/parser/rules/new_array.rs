use std::fmt;

use crate::ast::NewArrayExpr;
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::{expr::ExprRule, parsed_type::ParsedTypeRule, parsed_unit_type::ParsedUnitTypeRule};
use crate::lexer::token::TokenType;

pub struct NewArrayRule {}

impl fmt::Display for NewArrayRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NewArray")
    }
}

impl ParseRule<NewArrayExpr> for NewArrayRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::New).is_some() && (ParsedTypeRule {}.check_match(cursor))
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<NewArrayExpr> {
        parser.begin_range();
        parser.try_consume(TokenType::New)?;

        let array_type = parser.apply_rule(ParsedUnitTypeRule {}, "array type", None)?;
        
        let mut sizes = Vec::new();

        while let Some(_) = parser.try_consume(TokenType::LeftSquare) {
            let size_expr = parser.apply_rule(ExprRule {}, "size expression", Some(ErrMsg::ExpectedExpression))?;
            sizes.push(size_expr);
            parser.consume_or_diagnostic(TokenType::RightSquare);
        }
    
        Some(NewArrayExpr::new(sizes, array_type, parser.end_range()))
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
    fn test_new_array_rule_check_match_with_new_type() {
        let rule = NewArrayRule {};
        let tokens = vec![
            create_token(TokenType::New),
            create_token(TokenType::Int),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_new_array_rule_check_match_without_new() {
        let rule = NewArrayRule {};
        let tokens = vec![
            create_token(TokenType::Int),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(!rule.check_match(cursor));
    }

    #[test]
    fn test_new_array_rule_check_match_new_without_type() {
        let rule = NewArrayRule {};
        let tokens = vec![
            create_token(TokenType::New),
            create_token(TokenType::LeftParen),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(!rule.check_match(cursor));
    }

    #[test]
    fn test_parse_new_int_array_with_size() {
        let rule = NewArrayRule {};
        let tokens = vec![
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
        assert!(diagnostics.is_empty(), "Expected no diagnostics for new int array with size");
    }

    #[test]
    fn test_parse_new_bool_array_with_expression_size() {
        let rule = NewArrayRule {};
        let tokens = vec![
            create_token(TokenType::New),
            create_token(TokenType::Bool),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::Identifier("n".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for new bool array with expression size");
    }

    #[test]
    fn test_parse_new_double_array_with_function_call_size() {
        let rule = NewArrayRule {};
        let tokens = vec![
            create_token(TokenType::New),
            create_token(TokenType::Double),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::Identifier("getSize".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::RightSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for new double array with function call size");
    }

    #[test]
    fn test_parse_new_char_array_with_member_access_size() {
        let rule = NewArrayRule {};
        let tokens = vec![
            create_token(TokenType::New),
            create_token(TokenType::Char),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::Identifier("config".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("bufferSize".to_string())),
            create_token(TokenType::RightSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for new char array with member access size");
    }

    #[test]
    fn test_parse_new_2d_int_array() {
        let rule = NewArrayRule {};
        let tokens = vec![
            create_token(TokenType::New),
            create_token(TokenType::Int),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for new 2D int array");
    }

    #[test]
    fn test_parse_new_3d_char_array() {
        let rule = NewArrayRule {};
        let tokens = vec![
            create_token(TokenType::New),
            create_token(TokenType::Char),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(4)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for new 3D string array");
    }

    #[test]
    fn test_parse_new_array_with_zero_size() {
        let rule = NewArrayRule {};
        let tokens = vec![
            create_token(TokenType::New),
            create_token(TokenType::Int),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for new array with zero size");
    }

    #[test]
    fn test_parse_new_array_with_negative_size() {
        let rule = NewArrayRule {};
        let tokens = vec![
            create_token(TokenType::New),
            create_token(TokenType::Int),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::Minus),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for new array with negative size");
    }

    #[test]
    fn test_parse_new_array_with_complex_size_expression() {
        let rule = NewArrayRule {};
        let tokens = vec![
            create_token(TokenType::New),
            create_token(TokenType::Bool),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Identifier("width".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::Identifier("height".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::Star),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for new array with complex size expression");
    }

    #[test]
    fn test_parse_new_array_without_dimensions() {
        let rule = NewArrayRule {};
        let tokens = vec![
            create_token(TokenType::New),
            create_token(TokenType::Int),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should succeed with zero dimensions (unsized array)
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for new array without dimensions");
    }

    #[test]
    fn test_parse_new_array_missing_closing_bracket() {
        let rule = NewArrayRule {};
        let tokens = vec![
            create_token(TokenType::New),
            create_token(TokenType::Int),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should succeed but have diagnostic for missing bracket
        assert!(result.is_some());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing closing bracket");
        assert!(diagnostics.iter().any(|d| d.message.contains("']'")));
    }

    #[test]
    fn test_parse_new_array_missing_size_expression() {
        let rule = NewArrayRule {};
        let tokens = vec![
            create_token(TokenType::New),
            create_token(TokenType::Int),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::RightSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because size expression is required
        assert!(result.is_none());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing size expression");
        assert!(diagnostics.iter().any(|d| d.message.contains("expected expression")));
    }

    #[test]
    fn test_parse_new_array_invalid_size_expression() {
        let rule = NewArrayRule {};
        let tokens = vec![
            create_token(TokenType::New),
            create_token(TokenType::Int),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::RightCurly), // Invalid expression
            create_token(TokenType::RightSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because size expression is invalid
        assert!(result.is_none());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for invalid size expression");
        assert!(diagnostics.iter().any(|d| d.message.contains("expected expression")));
    }

    #[test]
    fn test_parse_new_array_with_array_access_size() {
        let rule = NewArrayRule {};
        let tokens = vec![
            create_token(TokenType::New),
            create_token(TokenType::Double),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::Identifier("sizes".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::RightSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for new array with array access size");
    }

    #[test]
    fn test_parse_new_array_with_constructor_call_size() {
        let rule = NewArrayRule {};
        let tokens = vec![
            create_token(TokenType::New),
            create_token(TokenType::Bool),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::DollarSign),
            create_token(TokenType::Identifier("Size".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(100)),
            create_token(TokenType::RightParen),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("value".to_string())),
            create_token(TokenType::RightSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for new array with constructor call size");
    }

    #[test]
    fn test_parse_new_mixed_dimension_array() {
        let rule = NewArrayRule {};
        let tokens = vec![
            create_token(TokenType::New),
            create_token(TokenType::Int),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::Identifier("dynamicSize".to_string())),
            create_token(TokenType::RightSquare),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::Identifier("getDepth".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::RightSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for new mixed dimension array");
    }

    #[test]
    fn test_parse_missing_type() {
        let rule = NewArrayRule {};
        let tokens = vec![
            create_token(TokenType::New),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because type is required
        assert!(result.is_none());
    }
}