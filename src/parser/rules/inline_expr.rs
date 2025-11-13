use std::fmt;

use crate::{expr::Expr, parser::{ExprParser, ParseRule, rules::condition::ConditionRule}};

pub struct InlineExprRule {}

impl fmt::Display for InlineExprRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "InlineExpr")
    }
}

impl ParseRule<Box<dyn Expr>> for InlineExprRule {
    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn Expr>> {    
        parser.apply_rule(ConditionRule {})
    }
}