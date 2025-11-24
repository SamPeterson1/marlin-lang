use crate::{ast::ASTNode, logger::Log, parser::{ExprParser, ParseRule, ParserCursor, rules::{block::BlockRule, break_expr::BreakRule, condition::ConditionRule, declaration::DeclarationRule, for_loop::ForLoopRule, getc::GetcRule, if_block::IfBlockRule, loop_expr::LoopRule, putc::PutcRule, while_loop::WhileLoopRule}}, token::TokenType};
use std::fmt;

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
        parser.apply_rule(ConditionRule {}, "condition expression", None)
    }
}