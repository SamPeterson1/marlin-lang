use std::fmt;

use crate::{ast::ASTNode, parser::{ExprParser, ParseRule, rules::condition::ConditionRule}};

pub struct InlineExprRule {}

impl fmt::Display for InlineExprRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "InlineExpr")
    }
}

impl ParseRule<Box<dyn ASTNode>> for InlineExprRule {
    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {    
        parser.apply_rule(ConditionRule {})
    }
}