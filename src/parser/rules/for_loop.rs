use std::fmt;

use crate::{expr::{Expr, binary_expr::BinaryExpr, declaration_expr::DeclarationExpr, loop_expr::LoopExpr}, logger::Log, parser::{ExprParser, ParseRule, diagnostic, rules::{assignment::AssignmentRule, block::BlockRule, boolean_factor::BooleanFactorRule, comparison::ComparisonRule, declaration::DeclarationRule, expr::ExprRule, inline_expr::InlineExprRule}}, token::{Position, PositionRange, TokenType}};

pub struct ForLoopRule {}

impl fmt::Display for ForLoopRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ForLoop")
    }
}

//for: FOR LEFT_PAREN [declaration] [inline_expression] SEMICOLON [assignment] RIGHT_PAREN [block]
impl ParseRule<LoopExpr> for ForLoopRule {
    fn parse(&self, parser: &mut ExprParser) -> Option<LoopExpr> {
        parser.log_debug(&format!("Entering for parser. Current token {:?}", parser.cur()));
        let for_token = parser.advance();

        parser.consume_or_diagnostic(TokenType::LeftParen, diagnostic::err_expected_token(PositionRange::new(Position::new(0, 0)), TokenType::LeftParen));

        let initial = parser.apply_rule(DeclarationRule {});
        parser.log_parse_result(&initial, "for initial");

        let condition = parser.apply_rule(InlineExprRule {});
        parser.log_parse_result(&condition, "for condition");

        parser.consume_or_diagnostic(TokenType::Semicolon, diagnostic::err_expected_token(PositionRange::new(Position::new(0, 0)), TokenType::Semicolon));

        let increment = parser.apply_rule(AssignmentRule {});
        parser.log_parse_result(&increment, "for increment");

        parser.consume_or_diagnostic(TokenType::RightParen, diagnostic::err_expected_token(PositionRange::new(Position::new(0, 0)), TokenType::RightParen));


        let body = parser.apply_rule_boxed(BlockRule {});    
        parser.log_parse_result(&body, "for body");
        
        let position= PositionRange::concat(&for_token.position, &parser.prev().position);

        let result = LoopExpr::new_for(initial?, condition?, increment?, body?, position);
        
        Some(result)
    }
}
