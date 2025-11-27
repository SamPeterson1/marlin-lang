use std::fmt;

use crate::{ast::{ASTWrapper, assignment_expr::AssignmentExpr}, parser::{ExprParser, ParseRule, ParserCursor, diagnostic::ErrMsg, rules::expr::ExprRule}, token::TokenType};

pub struct AssignmentRule {}

impl fmt::Display for AssignmentRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Assignment")
    }
}

impl ParseRule<ASTWrapper<AssignmentExpr>> for AssignmentRule {
    fn check_match(&self, _cursor: ParserCursor) -> bool {
        true
    }
    
    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<AssignmentExpr>> {
        let assignee = parser.apply_rule(ExprRule {}, "assignee expression", Some(ErrMsg::ExpectedExpression))?;
        
        parser.consume_or_diagnostic(TokenType::Assignment);

        let expr = parser.apply_rule(ExprRule {}, "assignment expression", Some(ErrMsg::ExpectedExpression))?;

        Some(ASTWrapper::new_assignment(assignee, expr))
    }
}