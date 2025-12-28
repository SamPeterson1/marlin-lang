use std::fmt;

use crate::ast::ASTNode;
use crate::parser::rules::delete_expr::DeleteRule;
use crate::parser::{ExprParser, ParseRule, ParserCursor};
use crate::parser::rules::{assignment::AssignmentRule, block::BlockRule, declaration::DeclarationRule, exit_expr::ExitRule, for_loop::ForLoopRule, if_block::IfBlockRule, loop_expr::LoopRule, while_loop::WhileLoopRule};
use crate::lexer::token::TokenType;

pub struct StatementRule {}

impl fmt::Display for StatementRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Statement")
    }
}

impl ParseRule<Box<dyn ASTNode>> for StatementRule {
    fn check_match(&self, _cursor: ParserCursor) -> bool {
        true
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        if (LoopRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(LoopRule {}, "statement loop", None);
        }
        
        if (IfBlockRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(IfBlockRule {}, "statement if", None);
        }

        if (BlockRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(BlockRule {}, "statement block", None);
        }

        if (ForLoopRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(ForLoopRule {}, "statement for", None);
        }

        if (WhileLoopRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(WhileLoopRule {}, "statement while", None);
        }

        if (ExitRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(ExitRule {}, "statement exit", None);
        }

        if (DeleteRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(DeleteRule {}, "statement delete", None);
        }

        let decl_rule = DeclarationRule { use_let: true };
        if decl_rule.check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(decl_rule, "statement declaration", None);
        }

        if (AssignmentRule {}).check_match(parser.get_cursor()) {
            let result = parser.apply_rule(AssignmentRule {}, "statement assignment", None)?;
            parser.consume_or_diagnostic(TokenType::Semicolon);

            return Some(result);
        }

        None
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
    fn test_statement_rule_check_match_always_true() {
        let rule = StatementRule {};
        let tokens = vec![create_token(TokenType::EOF)];
        let cursor = ParserCursor { ptr: 0, tokens: &tokens };
        
        // Statement rule always returns true for check_match
        assert!(rule.check_match(cursor));
    }

    #[test]
    fn test_parse_loop_statement() {
        let rule = StatementRule {};
        let tokens = vec![
            create_token(TokenType::Loop),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Break),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid loop statement");
    }

    #[test]
    fn test_parse_if_statement() {
        let rule = StatementRule {};
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
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid if statement");
    }

    #[test]
    fn test_parse_block_statement() {
        let rule = StatementRule {};
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
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid block statement");
    }

    #[test]
    fn test_parse_for_statement() {
        let rule = StatementRule {};
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
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid for statement");
    }

    #[test]
    fn test_parse_while_statement() {
        let rule = StatementRule {};
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
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid while statement");
    }

    #[test]
    fn test_parse_exit_statement() {
        let rule = StatementRule {};
        let tokens = vec![
            create_token(TokenType::Return),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid exit statement");
    }

    #[test]
    fn test_parse_delete_statement() {
        let rule = StatementRule {};
        let tokens = vec![
            create_token(TokenType::Delete),
            create_token(TokenType::Identifier("ptr".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid delete statement");
    }

    #[test]
    fn test_parse_declaration_statement() {
        let rule = StatementRule {};
        let tokens = vec![
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid declaration statement");
    }

    #[test]
    fn test_parse_assignment_statement() {
        let rule = StatementRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid assignment statement");
    }

    #[test]
    fn test_parse_expression_statement() {
        let rule = StatementRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("func".to_string())),
            create_token(TokenType::LeftParen),
            create_token(TokenType::RightParen),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        // Should parse as expression statement via AssignmentRule (which handles singular expressions)
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid expression statement");
    }

    #[test]
    fn test_parse_statement_missing_semicolon() {
        let rule = StatementRule {};
        let tokens = vec![
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::EOF), // Missing semicolon
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        // Should have diagnostic for missing semicolon
        assert!(!diagnostics.is_empty(), "Expected diagnostic for missing semicolon");
        assert!(diagnostics.iter().any(|d| d.message.contains("';'")));
    }

    #[test]
    fn test_parse_break_statement() {
        let rule = StatementRule {};
        let tokens = vec![
            create_token(TokenType::Break),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid break statement");
    }

    #[test]
    fn test_parse_result_statement() {
        let rule = StatementRule {};
        let tokens = vec![
            create_token(TokenType::Result),
            create_token(TokenType::StringLiteral("success".to_string())),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for valid result statement");
    }

    #[test]
    fn test_parse_invalid_statement() {
        let rule = StatementRule {};
        let tokens = vec![
            create_token(TokenType::RightCurly), // Invalid token to start statement
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_none());
    }

    #[test]
    fn test_rule_precedence() {
        // Test that more specific rules are checked before general assignment rule
        let rule = StatementRule {};
        
        // Declaration should be parsed as declaration, not as assignment
        let tokens = vec![
            create_token(TokenType::Let),
            create_token(TokenType::Int),
            create_token(TokenType::Identifier("x".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::IntLiteral(42)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for declaration precedence");
        
        // Loop should be parsed as loop, not as assignment
        let tokens = vec![
            create_token(TokenType::Loop),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for loop precedence");
    }

    #[test]
    fn test_complex_nested_statement() {
        let rule = StatementRule {};
        let tokens = vec![
            create_token(TokenType::If),
            create_token(TokenType::Identifier("condition".to_string())),
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
            create_token(TokenType::IntLiteral(5)),
            create_token(TokenType::Semicolon),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Assignment),
            create_token(TokenType::Identifier("i".to_string())),
            create_token(TokenType::Plus),
            create_token(TokenType::IntLiteral(1)),
            create_token(TokenType::RightParen),
            create_token(TokenType::LeftCurly),
            create_token(TokenType::Break),
            create_token(TokenType::Semicolon),
            create_token(TokenType::RightCurly),
            create_token(TokenType::RightCurly),
            create_token(TokenType::EOF),
        ];
        let mut diagnostics = Vec::new();
        let mut parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        
        let result = rule.parse(&mut parser);
        
        assert!(result.is_some());
        assert!(diagnostics.is_empty(), "Expected no diagnostics for complex nested statement");
    }
}


