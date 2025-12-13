use std::fmt;

use crate::ast::assignment_expr::AssignmentExpr;
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor};
use crate::parser::rules::expr::ExprRule;
use crate::lexer::token::TokenType;

pub struct AssignmentRule {}

impl fmt::Display for AssignmentRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Assignment")
    }
}

impl ParseRule<AssignmentExpr> for AssignmentRule {
    fn check_match(&self, _cursor: ParserCursor) -> bool {
        true
    }
    
    fn parse(&self, parser: &mut ExprParser) -> Option<AssignmentExpr> {
        parser.begin_range();

        let assignee = parser.apply_rule(ExprRule {}, "assignee expression", Some(ErrMsg::ExpectedExpression))?;
        
        parser.consume_or_diagnostic(TokenType::Assignment);

        let expr = parser.apply_rule(ExprRule {}, "assignment expression", Some(ErrMsg::ExpectedExpression))?;

        Some(AssignmentExpr::new(assignee, expr, parser.end_range()))
    }
}