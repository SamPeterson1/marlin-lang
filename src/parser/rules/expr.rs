use std::fmt;

use crate::ast::ASTNode;
use crate::parser::{ExprParser, ParseRule, ParserCursor};
use crate::parser::rules::binary_expr::BinaryExprRule;

pub struct ExprRule {}

impl fmt::Display for ExprRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Expr")
    }
}

impl ParseRule<Box<dyn ASTNode>> for ExprRule {
    fn check_match(&self, _cursor: ParserCursor) -> bool {
        true
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        parser.apply_rule(BinaryExprRule {}, "binary expression", None)
    }
}