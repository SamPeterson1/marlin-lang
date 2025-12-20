use std::fmt;

use crate::ast::{ASTNode, AssignmentExpr, CastExpr};
use crate::diagnostic::ErrMsg;
use crate::parser::rules::parsed_type::ParsedTypeRule;
use crate::parser::rules::unary::UnaryRule;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::expr::ExprRule;
use crate::lexer::token::TokenType;

pub struct CastRule {}

impl fmt::Display for CastRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cast")
    }
}

impl ParseRule<Box<dyn ASTNode>> for CastRule {
    fn check_match(&self, _cursor: ParserCursor) -> bool {
        true
    }
    
    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        parser.begin_range();

        let expr = parser.apply_rule(UnaryRule {}, "cast unary expression", Some(ErrMsg::ExpectedExpression))?;
        
        if parser.try_consume(TokenType::As).is_some() {
            let cast_type = parser.apply_rule(ParsedTypeRule {}, "cast type", Some(ErrMsg::ExpectedType))?;
            Some(Box::new(CastExpr::new(expr, cast_type, parser.end_range())))
        } else {
            Some(expr)
        }
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
    fn test_simple_literal_without_cast() {
        let rule = CastRule {};
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
    fn test_cast_int_to_double() {
        let rule = CastRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::As),
            create_token(TokenType::Double),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_cast_variable_to_int() {
        let rule = CastRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::As),
            create_token(TokenType::Int),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_cast_to_bool() {
        let rule = CastRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::As),
            create_token(TokenType::Bool),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_cast_to_char() {
        let rule = CastRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(65)),
            create_token(TokenType::As),
            create_token(TokenType::Char),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_cast_to_pointer_type() {
        let rule = CastRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::As),
            create_token(TokenType::Int),
            create_token(TokenType::Star),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_cast_to_custom_type() {
        let rule = CastRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("value".to_string())),
            create_token(TokenType::As),
            create_token(TokenType::Identifier("MyType".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_cast_with_unary_operator() {
        let rule = CastRule {};
        let tokens = vec![
            create_token(TokenType::Minus),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::As),
            create_token(TokenType::Double),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_cast_dereferenced_value() {
        let rule = CastRule {};
        let tokens = vec![
            create_token(TokenType::Star),
            create_token(TokenType::Identifier("ptr".to_string())),
            create_token(TokenType::As),
            create_token(TokenType::Int),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_cast_address_of_value() {
        let rule = CastRule {};
        let tokens = vec![
            create_token(TokenType::Ampersand),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::As),
            create_token(TokenType::Int),
            create_token(TokenType::Star),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_cast_negated_value() {
        let rule = CastRule {};
        let tokens = vec![
            create_token(TokenType::Not),
            create_token(TokenType::Identifier("flag".to_string())),
            create_token(TokenType::As),
            create_token(TokenType::Int),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_cast_to_reference_type() {
        let rule = CastRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::As),
            create_token(TokenType::Int),
            create_token(TokenType::Ampersand),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_cast_to_array_type() {
        let rule = CastRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("ptr".to_string())),
            create_token(TokenType::As),
            create_token(TokenType::Int),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::RightSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_cast_parenthesized_expression() {
        let rule = CastRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::RightParen),
            create_token(TokenType::As),
            create_token(TokenType::Double),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_multiple_unary_operators_with_cast() {
        let rule = CastRule {};
        let tokens = vec![
            create_token(TokenType::Minus),
            create_token(TokenType::Minus),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::As),
            create_token(TokenType::Double),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_cast_missing_type_error() {
        let rule = CastRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::As),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_none());
        assert!(!diagnostics.is_empty());
    }

    #[test]
    fn test_expression_without_cast() {
        let rule = CastRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should parse just the identifier without the cast
        // The Plus should not be consumed by the cast rule
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_cast_to_double_pointer() {
        let rule = CastRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("ptr".to_string())),
            create_token(TokenType::As),
            create_token(TokenType::Int),
            create_token(TokenType::Star),
            create_token(TokenType::Star),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_cast_bool_literal() {
        let rule = CastRule {};
        let tokens = vec![
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::As),
            create_token(TokenType::Int),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_cast_double_literal() {
        let rule = CastRule {};
        let tokens = vec![
            create_token(TokenType::DoubleLiteral(3.14)),
            create_token(TokenType::As),
            create_token(TokenType::Int),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_cast_char_literal() {
        let rule = CastRule {};
        let tokens = vec![
            create_token(TokenType::CharLiteral('A')),
            create_token(TokenType::As),
            create_token(TokenType::Int),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }
}