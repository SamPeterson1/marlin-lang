use std::fmt;

use crate::ast::ASTEnum;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::expr::ExprRule;
use crate::lexer::token::TokenType;

pub struct ArgumentsRule {}

impl fmt::Display for ArgumentsRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Arguments")
    }
}

impl ParseRule<Vec<ASTEnum>> for ArgumentsRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::LeftParen).is_some()
    }
    
    fn parse(&self, parser: &mut ExprParser) -> Option<Vec<ASTEnum>> {
        parser.begin_range();
        
        parser.try_consume(TokenType::LeftParen)?;

        let mut arguments = Vec::new();
        
        //Check if the next token is a right paren to allow for empty argument lists
        if parser.try_consume(TokenType::RightParen).is_none() {
            if let Some(argument) = parser.apply_rule(ExprRule {}, "first argument", None) {
                arguments.push(argument);
    
                while let Some(_) = parser.try_consume(TokenType::Comma) {
                    let argument = parser.apply_rule(ExprRule {}, "argument", None)?;
                    arguments.push(argument);
                }
            }

            parser.consume_or_diagnostic(TokenType::RightParen);
        }
        
        Some(arguments)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::{Token, TokenType, PositionRange};
    use crate::diagnostic::DiagnosticSeverity;
    use crate::logger::CONSOLE_LOGGER;

    fn create_token(token_type: TokenType) -> Token {
        Token::new(token_type, PositionRange::zero())
    }

    #[test]
    fn test_arguments_rule_check_match_with_left_paren() {
        let rule = ArgumentsRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_arguments_rule_check_match_without_left_paren() {
        let rule = ArgumentsRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("test".to_string())),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(!rule.check_match(cursor));
    }

    #[test]
    fn test_parse_empty_arguments() {
        let rule = ArgumentsRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let arguments = result.unwrap();
        assert_eq!(arguments.len(), 0);
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid empty arguments");
    }

    #[test]
    fn test_parse_single_argument() {
        let rule = ArgumentsRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let arguments = result.unwrap();
        assert_eq!(arguments.len(), 1);
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid single argument");
    }

    #[test]
    fn test_parse_multiple_arguments() {
        let rule = ArgumentsRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::Comma),
            create_token(TokenType::StringLiteral("test".to_string())),
            create_token(TokenType::Comma),
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid multiple arguments");
    }

    #[test]
    fn test_parse_missing_right_paren() {
        let rule = ArgumentsRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should still return Some but with diagnostics
        assert!(result.is_some());
        let arguments = result.unwrap();
        assert_eq!(arguments.len(), 1);
        
        // Should have diagnostic for missing right paren
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing right paren");
        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message.contains("')'"));
        assert!(matches!(diagnostics[0].severity, DiagnosticSeverity::Error));
    }

    #[test]
    fn test_parse_trailing_comma() {
        let rule = ArgumentsRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::Comma),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because there's no expression after comma
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_invalid_expression_in_arguments() {
        let rule = ArgumentsRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightCurly), // Invalid token for expression
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should return empty arguments since first expression fails
        assert!(result.is_some());
        let arguments = result.unwrap();
        assert_eq!(arguments.len(), 0);
        
        // May have diagnostics from failed expression parsing
        // The exact diagnostic depends on the ExprRule implementation
    }

    #[test]
    fn test_parse_multiple_comma_errors() {
        let rule = ArgumentsRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::Comma),
            create_token(TokenType::Comma), // Double comma - invalid
            create_token(TokenType::IntLiteral(24)),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail due to invalid comma sequence
        assert!(result.is_none());
        
        // Should have diagnostic for expected expression after first comma
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing expression after comma");
    }

    #[test]
    fn test_parse_only_commas() {
        let rule = ArgumentsRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::Comma),
            create_token(TokenType::Comma),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should return empty arguments since no valid expressions
        assert!(result.is_some());
        let arguments = result.unwrap();
        assert_eq!(arguments.len(), 0);
        
        // Should have diagnostics for invalid comma usage
        assert!(!diagnostics.is_empty(), "Expected diagnostics for invalid comma sequence");
    }

    #[test]
    fn test_parse_mixed_argument_count() {
        let rule = ArgumentsRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(123)),
            create_token(TokenType::Comma),
            create_token(TokenType::DoubleLiteral(45.67)),
            create_token(TokenType::Comma),
            create_token(TokenType::BoolLiteral(false)),
            create_token(TokenType::Comma),
            create_token(TokenType::CharLiteral('x')),
            create_token(TokenType::Comma),
            create_token(TokenType::Identifier("variable".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        let arguments = result.unwrap();
        assert_eq!(arguments.len(), 5);
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid argument structure");
    }
}