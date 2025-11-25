use crate::{ast::{ASTNode, ASTWrapper}, logger::Log, parser::{ExprParser, ParseRule, ParserCursor, rules::{assignment::AssignmentRule, block::BlockRule, break_expr::BreakRule, declaration::DeclarationRule, expr::ExprRule, for_loop::ForLoopRule, if_block::IfBlockRule, loop_expr::LoopRule, putc::PutcRule, while_loop::WhileLoopRule}}, token::TokenType};
use std::fmt;

pub struct StatementRule {}

impl fmt::Display for StatementRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Statement")
    }
}

impl ParseRule<Box<dyn ASTNode>> for StatementRule {
    fn check_match(&self, _cursor: ParserCursor) -> bool {
        true
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        if (LoopRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(LoopRule {}, "statement loop", None);
        }
        
        if (IfBlockRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(IfBlockRule {}, "statement if", None);
        }

        if (BlockRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(BlockRule {}, "statement block", None);
        }

        if (ForLoopRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(ForLoopRule {}, "statement for", None);
        }

        if (WhileLoopRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(WhileLoopRule {}, "statement while", None);
        }

        if (BreakRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(BreakRule {}, "statement break", None);
        }

        if (DeclarationRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(DeclarationRule {}, "statement declaration", None);
        }

        if (PutcRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(PutcRule {}, "statement putc", None);
        }

        if (AssignmentRule {}).check_match(parser.get_cursor()) {
            let result = parser.apply_rule_boxed(AssignmentRule {}, "statement assignment", None)?;
            parser.consume_or_diagnostic(TokenType::Semicolon);

            return Some(result);
        }

        None
    }
}


