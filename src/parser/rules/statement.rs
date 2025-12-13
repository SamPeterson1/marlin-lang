use std::fmt;

use crate::ast::ASTNode;
use crate::parser::rules::delete_expr::DeleteRule;
use crate::parser::{ExprParser, ParseRule, ParserCursor};
use crate::parser::rules::{assignment::AssignmentRule, block::BlockRule, declaration::DeclarationRule, exit_expr::ExitRule, for_loop::ForLoopRule, if_block::IfBlockRule, loop_expr::LoopRule, while_loop::WhileLoopRule};
use crate::lexer::token::TokenType;

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

        if (ExitRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(ExitRule {}, "statement exit", None);
        }

        if (DeleteRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(DeleteRule {}, "statement delete", None);
        }

        if (DeclarationRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(DeclarationRule {}, "statement declaration", None);
        }

        if (AssignmentRule {}).check_match(parser.get_cursor()) {
            let result = parser.apply_rule_boxed(AssignmentRule {}, "statement assignment", None)?;
            parser.consume_or_diagnostic(TokenType::Semicolon);

            return Some(result);
        }

        None
    }
}


