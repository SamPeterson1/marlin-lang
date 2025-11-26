use std::fmt;

use crate::{ast::{ASTNode, ASTWrapper, assignment_expr::AssignmentExpr, lvar_expr::VarExpr}, logger::Log, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, diagnostic::{self, ErrMsg}, rules::{expr::ExprRule, var::VarRule}}, token::{Position, PositionRange, TokenType}};

pub struct AssignmentRule {}

impl fmt::Display for AssignmentRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Assignment")
    }
}

impl ParseRule<ASTWrapper<AssignmentExpr>> for AssignmentRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::Identifier).is_some() && cursor.try_consume(TokenType::Assignment).is_some()
    }
    
    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<AssignmentExpr>> {
        let assignee = parser.try_consume(TokenType::Identifier)?;
        
        parser.try_consume(TokenType::Assignment)?;

        let expr = parser.apply_rule(ExprRule {}, "assignment expression", Some(ErrMsg::ExpectedExpression))?;

        Some(ASTWrapper::new_assignment(&assignee, expr))
    }
}