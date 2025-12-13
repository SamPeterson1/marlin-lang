use std::fmt;

use crate::ast::loop_expr::LoopExpr;
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::{assignment::AssignmentRule, block::BlockRule, declaration::DeclarationRule, expr::ExprRule};
use crate::lexer::token::TokenType;

pub struct ForLoopRule {}

impl fmt::Display for ForLoopRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ForLoop")
    }
}

impl ParseRule<LoopExpr> for ForLoopRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::For).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<LoopExpr> {
        parser.begin_range();
        parser.try_consume(TokenType::For)?;

        parser.consume_or_diagnostic(TokenType::LeftParen);

        let initial = parser.apply_rule(DeclarationRule {}, "for initial", Some(ErrMsg::ExpectedDeclaration))?;

        let condition = parser.apply_rule(ExprRule {}, "condition expression", Some(ErrMsg::ExpectedExpression))?;

        parser.consume_or_diagnostic(TokenType::Semicolon);

        let increment = parser.apply_rule(AssignmentRule {}, "for increment", Some(ErrMsg::ExpectedAssignment))?;

        parser.consume_or_diagnostic(TokenType::RightParen);

        let body = parser.apply_rule(BlockRule {}, "for body", Some(ErrMsg::ExpectedBlock))?;    
                
        Some(LoopExpr::new_for(initial, condition, increment, body, parser.end_range()))
    }
}
