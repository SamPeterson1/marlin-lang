use std::fmt;

use crate::ast::{AssignmentExpr, ASTNode};
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::expr::ExprRule;
use crate::lexer::token::TokenType;

pub struct AssignmentRule {}

impl fmt::Display for AssignmentRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Assignment")
    }
}

impl ParseRule<Box<dyn ASTNode>> for AssignmentRule {
    fn check_match(&self, _cursor: ParserCursor) -> bool {
        true
    }
    
    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        parser.begin_range();

        let assignee = parser.apply_rule(ExprRule {}, "assignee expression", Some(ErrMsg::ExpectedExpression))?;
        
        if parser.try_consume(TokenType::Assignment).is_some() {
            let expr = parser.apply_rule(ExprRule {}, "assignment expression", Some(ErrMsg::ExpectedExpression))?;
            Some(Box::new(AssignmentExpr::new(assignee, expr, parser.end_range())))
        } else {
            Some(assignee)
        }
    }
}

use crate::logger::DYN_CONSOLE_LOGGER;
#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::{Token, TokenType, PositionRange};

    fn create_token(token_type: TokenType) -> Token {
        Token::new(token_type, PositionRange::zero())
    }

    #[test]
    fn test_assignment_rule_check_match_always_true() {
        let rule = AssignmentRule {};
        let tokens = vec![create_token(TokenType::EOF)];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        // Assignment rule always returns true for check_match
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_parse_simple_assignment() {
        let rule = AssignmentRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid assignment");
    }

    #[test]
    fn test_parse_singular_identifier() {
        let rule = AssignmentRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should successfully parse as a singular expression (not an assignment)
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid singular identifier");
    }

    #[test]
    fn test_parse_singular_literal() {
        let rule = AssignmentRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should successfully parse as a singular literal expression
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid literal expression");
    }

    #[test]
    fn test_parse_singular_arithmetic_expression() {
        let rule = AssignmentRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::Star),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should successfully parse as a singular arithmetic expression
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid arithmetic expression");
    }

    #[test]
    fn test_parse_singular_member_access() {
        let rule = AssignmentRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("obj".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("field".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should successfully parse as a singular member access expression
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid member access expression");
    }

    #[test]
    fn test_parse_complex_assignment() {
        let rule = AssignmentRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("array".to_string())),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid complex assignment");
    }

    #[test]
    fn test_parse_missing_right_hand_side() {
        let rule = AssignmentRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because assignment expression is required after '='
        assert!(result.is_none());
        // Should have diagnostic for missing right-hand side expression
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing assignment expression");
    }

    #[test]
    fn test_parse_chained_assignment() {
        let rule = AssignmentRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("y".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should parse x = (y = 42) where the right side is also an assignment
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid chained assignment");
    }

    #[test]
    fn test_parse_member_access_assignment() {
        let rule = AssignmentRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("obj".to_string())),
            create_token(TokenType::Dot),
            create_token(TokenType::Identifier("field".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::StringLiteral("value".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid member assignment");
    }

    #[test]
    fn test_parse_invalid_right_hand_side() {
        let rule = AssignmentRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::RightCurly), // Invalid token for expression
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because right-hand side expression fails
        assert!(result.is_none());
        // Should have diagnostic for invalid expression after assignment
        assert!(!diagnostics.is_empty(), "Expected diagnostic for invalid assignment expression");
    }

    #[test]
    fn test_parse_double_assignment_operator() {
        let rule = AssignmentRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Assignment), // Double assignment - invalid
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail due to invalid expression after first assignment
        assert!(result.is_none());
        // Should have diagnostic for expected expression after assignment
        assert!(!diagnostics.is_empty(), "Expected diagnostic for invalid double assignment");
    }

    #[test]
    fn test_parse_singular_function_call() {
        let rule = AssignmentRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("func".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::Comma),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should successfully parse as a singular function call expression
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid function call expression");
    }
}