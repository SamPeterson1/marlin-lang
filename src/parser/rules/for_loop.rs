use std::fmt;

use crate::{ast::{ASTWrapper, loop_expr::LoopExpr}, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, diagnostic::ErrMsg, rules::{assignment::AssignmentRule, block::BlockRule, declaration::DeclarationRule, expr::ExprRule}}, token::{PositionRange, Positioned, TokenType}};

pub struct ForLoopRule {}

impl fmt::Display for ForLoopRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ForLoop")
    }
}

impl ParseRule<ASTWrapper<LoopExpr>> for ForLoopRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::For).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<LoopExpr>> {
        let for_token = parser.try_consume(TokenType::For)?;

        parser.consume_or_diagnostic(TokenType::LeftParen);

        let initial = parser.apply_rule(DeclarationRule {}, "for initial", Some(ErrMsg::ExpectedDeclaration))?;

        let condition = parser.apply_rule(ExprRule {}, "condition expression", Some(ErrMsg::ExpectedExpression))?;

        parser.consume_or_diagnostic(TokenType::Semicolon);

        let increment = parser.apply_rule(AssignmentRule {}, "for increment", Some(ErrMsg::ExpectedAssignment))?;

        parser.consume_or_diagnostic(TokenType::RightParen);

        let body = parser.apply_rule(BlockRule {}, "for body", Some(ErrMsg::ExpectedBlock))?;    
        
        let position= PositionRange::concat(&for_token.position, body.get_position());
        
        Some(ASTWrapper::new_for(initial, condition, increment, body, position))
    }
}
