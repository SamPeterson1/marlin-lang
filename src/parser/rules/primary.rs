use std::fmt;

use crate::ast::{ASTNode, Literal, LiteralExpr};
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, TokenCursor};
use crate::parser::rules::{block::BlockRule, constructor_call::ConstructorCallRule, expr::ExprRule, for_loop::ForLoopRule, if_block::IfBlockRule, loop_expr::LoopRule, new_array::NewArrayRule, var::VarRule, while_loop::WhileLoopRule};
use crate::lexer::token::{Positioned, TokenType};

pub struct PrimaryRule {}

impl fmt::Display for PrimaryRule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Primary")
    }
}

impl ParseRule<Box<dyn ASTNode>> for PrimaryRule {
    fn check_match(&self, _cursor: crate::parser::ParserCursor) -> bool {
        true
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        if (ConstructorCallRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(ConstructorCallRule {}, "primary constructor call", None);
        }
        
        if (NewArrayRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(NewArrayRule {}, "primary new array", None);
        }

        if (LoopRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(LoopRule {}, "primary loop", None);
        }

        if (IfBlockRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(IfBlockRule {}, "primary if", None);
        }

        if (BlockRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(BlockRule {}, "primary block", None);
        }

        if (ForLoopRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(ForLoopRule {}, "primary for", None);
        }

        if (WhileLoopRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(WhileLoopRule {}, "primary while", None);
        }

        if parser.try_consume(TokenType::LeftParen).is_some() {
            let expr = parser.apply_rule(ExprRule {}, "primary grouped expression", Some(ErrMsg::ExpectedExpression))?;
            parser.consume_or_diagnostic(TokenType::RightParen);

            return Some(expr);
        }

        if (VarRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(VarRule {}, "primary var", None);
        }
        
        let cur = parser.cur();

        let literal = match cur.value {
            TokenType::IntLiteral(x) => LiteralExpr::new(Literal::Int(x), *cur.get_position()),
            TokenType::DoubleLiteral(x) => LiteralExpr::new(Literal::Double(x), *cur.get_position()),
            TokenType::BoolLiteral(x) => LiteralExpr::new(Literal::Bool(x), *cur.get_position()),
            TokenType::CharLiteral(x) => LiteralExpr::new(Literal::Char(x), *cur.get_position()),
            TokenType::StringLiteral(ref x) => LiteralExpr::new(Literal::String(x.clone()), *cur.get_position()),
            _ => {
                parser.push_diagnostic(ErrMsg::ExpectedExpression.make_diagnostic(*cur.get_position()));
                return None;
            }
        };

        parser.next();

        return Some(Box::new(literal));
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
    fn test_primary_rule_check_match_always_true() {
        let rule = PrimaryRule {};
        let tokens = vec![create_token(TokenType::EOF)];
        let cursor = crate::parser::ParserCursor { ptr: 0, tokens: &tokens };
        
        // Primary rule always returns true for check_match
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_parse_int_literal() {
        let rule = PrimaryRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid int literal");
    }

    #[test]
    fn test_parse_double_literal() {
        let rule = PrimaryRule {};
        let tokens = vec![
            create_token(TokenType::DoubleLiteral(3.14)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_parse_bool_literal() {
        let rule = PrimaryRule {};
        let tokens = vec![
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid bool literal");
    }

    #[test]
    fn test_parse_char_literal() {
        let rule = PrimaryRule {};
        let tokens = vec![
            create_token(TokenType::CharLiteral('a')),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid char literal");
    }

    #[test]
    fn test_parse_string_literal() {
        let rule = PrimaryRule {};
        let tokens = vec![
            create_token(TokenType::StringLiteral("hello".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid string literal");
    }

    #[test]
    fn test_parse_parenthesized_expression() {
        let rule = PrimaryRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid parenthesized expression");
    }

    #[test]
    fn test_parse_parenthesized_missing_right_paren() {
        let rule = PrimaryRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should still return Some but with diagnostic for missing right paren
        assert!(result.is_some());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing right parenthesis");
        assert!(diagnostics.iter().any(|d| d.message.contains("')'")));
    }

    #[test]
    fn test_parse_parenthesized_missing_expression() {
        let rule = PrimaryRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail because expression is required
        assert!(result.is_none());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing expression");
        assert!(diagnostics.iter().any(|d| d.message.contains("expected expression")));
    }

    #[test]
    fn test_parse_invalid_token() {
        let rule = PrimaryRule {};
        let tokens = vec![
            create_token(TokenType::RightCurly),  // Invalid token for primary
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should fail and produce diagnostic for expected expression
        assert!(result.is_none());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for invalid token");
        assert!(diagnostics.iter().any(|d| d.message.contains("expected expression")));
    }

    #[test]
    fn test_parse_identifier_via_var_rule() {
        let rule = PrimaryRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("variable".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should succeed via VarRule
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid identifier");
    }

    #[test]
    fn test_parse_complex_parenthesized_expression() {
        let rule = PrimaryRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(3)),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid complex parenthesized expression");
    }

    #[test]
    fn test_parse_nested_parentheses() {
        let rule = PrimaryRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::RightParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid nested parentheses");
    }

    #[test]
    fn test_parse_constructor_call() {
        let rule = PrimaryRule {};
        let tokens = vec![
            create_token(TokenType::DollarSign),
            create_token(TokenType::Identifier("Person".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::StringLiteral("John".to_string())),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should succeed via ConstructorCallRule
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid constructor call");
    }

    #[test]
    fn test_parse_new_array() {
        let rule = PrimaryRule {};
        let tokens = vec![
            create_token(TokenType::New),
            create_token(TokenType::Int),
            create_token(TokenType::LeftSquare),
            create_token(TokenType::IntLiteral(10)),
            create_token(TokenType::RightSquare),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should succeed via NewArrayRule
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid new array");
    }

    #[test]
    fn test_parse_block_expression() {
        let rule = PrimaryRule {};
        let tokens = vec![
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should succeed via BlockRule
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid block expression");
    }

    #[test]
    fn test_parse_if_expression() {
        let rule = PrimaryRule {};
        let tokens = vec![
            create_token(TokenType::If),
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should succeed via IfBlockRule
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid if expression");
    }

    #[test]
    fn test_parse_loop_expression() {
        let rule = PrimaryRule {};
        let tokens = vec![
            create_token(TokenType::Loop),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Break),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should succeed via LoopRule
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid loop expression");
    }

    #[test]
    fn test_parse_while_expression() {
        let rule = PrimaryRule {};
        let tokens = vec![
            create_token(TokenType::While),
            create_token(TokenType::BoolLiteral(true)),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Break),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should succeed via WhileLoopRule
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid while expression");
    }

    #[test]
    fn test_parse_for_expression() {
        let rule = PrimaryRule {};
        let tokens = vec![
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
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should succeed via ForLoopRule
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid for expression");
    }

    #[test]
    fn test_parse_grouped_expression() {
        let rule = PrimaryRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::RightParen),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_grouped_expression_missing_right_paren() {
        let rule = PrimaryRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should still parse the inner expression but add diagnostic for missing paren
        assert!(result.is_some());
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing right paren");
    }

    #[test]
    fn test_display_trait() {
        let rule = PrimaryRule {};
        assert_eq!(format!("{}", rule), "Primary");
    }

    #[test]
    fn test_parse_bool_literal_true() {
        let rule = PrimaryRule {};
        let tokens = vec![
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
    fn test_parse_bool_literal_false() {
        let rule = PrimaryRule {};
        let tokens = vec![
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
    fn test_parse_grouped_expression_missing_close_paren() {
        let rule = PrimaryRule {};
        let tokens = vec![
            create_token(TokenType::LeftParen),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        // Should have diagnostic for missing right paren
        assert!(!diagnostics.is_empty());
    }

    #[test]
    fn test_parse_negative_int() {
        let rule = PrimaryRule {};
        let tokens = vec![
            create_token(TokenType::IntLiteral(-10)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_parse_zero_values() {
        let rule = PrimaryRule {};
        
        // Test zero integer
        let tokens = vec![
            create_token(TokenType::IntLiteral(0)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
        
        // Test zero double
        let tokens = vec![
            create_token(TokenType::DoubleLiteral(0.0)),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_parse_special_chars() {
        let rule = PrimaryRule {};
        
        // Test newline character
        let tokens = vec![
            create_token(TokenType::CharLiteral('\n')),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
        
        // Test tab character
        let tokens = vec![
            create_token(TokenType::CharLiteral('\t')),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_parse_empty_string() {
        let rule = PrimaryRule {};
        let tokens = vec![
            create_token(TokenType::StringLiteral("".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_parse_multiline_string() {
        let rule = PrimaryRule {};
        let tokens = vec![
            create_token(TokenType::StringLiteral("line1\nline2".to_string())),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty());
    }
}