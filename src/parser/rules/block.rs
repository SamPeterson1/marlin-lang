use std::fmt;

use crate::ast::{ASTNode, BlockExpr};
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::statement::StatementRule;
use crate::lexer::token::{Positioned, TokenType};

pub struct BlockRule {}

impl fmt::Display for BlockRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Block")
    }
}

impl ParseRule<BlockExpr> for BlockRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::LeftCurly).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<BlockExpr> {
        parser.begin_range();
        
        parser.consume_or_diagnostic(TokenType::LeftCurly);
        let mut exprs: Vec<Box<dyn ASTNode>> = Vec::new();

        while parser.try_consume(TokenType::RightCurly).is_none() {
            let statement = parser.apply_rule(StatementRule {}, "block statement", Some(ErrMsg::ExpectedStatement));

            if let Some(statement) = statement {
                exprs.push(statement);
            } else if !parser.try_match(&[TokenType::RightCurly]).is_some() {
                if let Some(token) = parser.try_match(&[TokenType::EOF]) {
                    parser.push_diagnostic(ErrMsg::ExpectedToken(TokenType::RightCurly).make_diagnostic(token.get_position().clone()));
                    break;
                }

                // Skip invalid tokens to avoid infinite loops
                parser.next();
            }
        }


        Some(BlockExpr::new(exprs, parser.end_range()))
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
    fn test_block_rule_check_match_with_left_curly() {
        let rule = BlockRule {};
        let tokens = vec![
            create_token(TokenType::LeftCurly),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_block_rule_check_match_without_left_curly() {
        let rule = BlockRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::EOF),
        ];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        assert!(!rule.check_match(cursor));
    }

    #[test]
    fn test_parse_empty_block() {
        let rule = BlockRule {};
        let tokens = vec![
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for empty block");
    }

    #[test]
    fn test_parse_single_statement_block() {
        let rule = BlockRule {};
        let tokens = vec![
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for single statement block");
    }

    #[test]
    fn test_parse_multiple_statements_block() {
        let rule = BlockRule {};
        let tokens = vec![
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("y".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::Identifier("y".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for multiple statements block");
    }

    #[test]
    fn test_parse_nested_blocks() {
        let rule = BlockRule {};
        let tokens = vec![
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("outer".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("inner".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for nested blocks");
    }

    #[test]
    fn test_parse_block_with_if_statement() {
        let rule = BlockRule {};
        let tokens = vec![
            create_token(TokenType::LeftCurly),
            create_token(TokenType::If),
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for block with if statement");
    }

    #[test]
    fn test_parse_block_with_loop() {
        let rule = BlockRule {};
        let tokens = vec![
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Loop),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Break),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for block with loop");
    }

    #[test]
    fn test_parse_block_with_while_loop() {
        let rule = BlockRule {};
        let tokens = vec![
            create_token(TokenType::LeftCurly),
            create_token(TokenType::While),
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Break),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for block with while loop");
    }

    #[test]
    fn test_parse_block_with_for_loop() {
        let rule = BlockRule {};
        let tokens = vec![
            create_token(TokenType::LeftCurly),
            create_token(TokenType::For),
            create_token(TokenType::LeftParen),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Less),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::RightParen),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for block with for loop");
    }

    #[test]
    fn test_parse_block_with_assignments() {
        let rule = BlockRule {};
        let tokens = vec![
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("y".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Star),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for block with assignments");
    }

    #[test]
    fn test_parse_block_with_exit_statements() {
        let rule = BlockRule {};
        let tokens = vec![
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Return),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for block with return statement");
    }

    #[test]
    fn test_parse_block_with_delete_statement() {
        let rule = BlockRule {};
        let tokens = vec![
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Delete),
            create_token(TokenType::Identifier("ptr".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for block with delete statement");
    }

    #[test]
    fn test_parse_missing_right_brace() {
        let rule = BlockRule {};
        let tokens = vec![
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should still return Some but with diagnostic for missing right brace
        assert!(result.is_some());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing right brace");
        assert!(diagnostics.iter().any(|d| d.message.contains("'}'")));
    }

    #[test]
    fn test_parse_invalid_statement_in_block() {
        let rule = BlockRule {};
        let tokens = vec![
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly), // Invalid token in statement position
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should succeed - invalid statement tokens are handled by the statement rule
        assert!(result.is_some());
        // May have diagnostics from statement parsing
    }

    #[test]
    fn test_parse_complex_nested_structure() {
        let rule = BlockRule {};
        let tokens = vec![
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::If),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Greater),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::While),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Greater),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Minus),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for complex nested structure");
    }

    #[test]
    fn test_parse_block_with_mixed_statement_types() {
        let rule = BlockRule {};
        let tokens = vec![
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("counter".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Loop),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Identifier("counter".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("counter".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::If),
            create_token(TokenType::Identifier("counter".to_string())),
            create_token(TokenType::Greater),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Break),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::Delete),
            create_token(TokenType::Identifier("counter".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for block with mixed statement types");
    }

    #[test]
    fn test_parse_deeply_nested_blocks() {
        let rule = BlockRule {};
        let tokens = vec![
            create_token(TokenType::LeftCurly),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("deep".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for deeply nested blocks");
    }

    #[test]
    fn test_parse_block_expression_result() {
        let rule = BlockRule {};
        let tokens = vec![
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Result),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Star),
            create_token(TokenType::IntLiteral(2)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for block with result statement");
    }
}
