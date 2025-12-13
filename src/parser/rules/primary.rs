use std::fmt;

use crate::ast::{ASTNode, literal_expr::{Literal, LiteralExpr}};
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, TokenCursor};
use crate::parser::rules::{block::BlockRule, constructor_call::ConstructorCallRule, expr::ExprRule, for_loop::ForLoopRule, if_block::IfBlockRule, loop_expr::LoopRule, new_array::NewArrayRule, var::VarRule, while_loop::WhileLoopRule};
use crate::lexer::token::{Positioned, TokenType};

pub struct PrimaryRule {}

impl fmt::Display for PrimaryRule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Primary")
    }
}

impl ParseRule<Box<dyn ASTNode>> for PrimaryRule {
    fn check_match(&self, _cursor: crate::parser::ParserCursor) -> bool {
        true
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        if (ConstructorCallRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(ConstructorCallRule {}, "primary constructor call", None);
        }
        
        if (NewArrayRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(NewArrayRule {}, "primary new array", None);
        }

        if (LoopRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(LoopRule {}, "primary loop", None);
        }

        if (IfBlockRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(IfBlockRule {}, "primary if", None);
        }

        if (BlockRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(BlockRule {}, "primary block", None);
        }

        if (ForLoopRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(ForLoopRule {}, "primary for", None);
        }

        if (WhileLoopRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(WhileLoopRule {}, "primary while", None);
        }

        if parser.try_consume(TokenType::LeftParen).is_some() {
            let expr = parser.apply_rule(ExprRule {}, "primary grouped expression", Some(ErrMsg::ExpectedExpression))?;
            parser.consume_or_diagnostic(TokenType::RightParen);

            return Some(expr);
        }

        if (VarRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(VarRule {}, "primary var", None);
        }
        
        let cur = parser.cur();

        let literal = match cur.value {
            TokenType::IntLiteral(x) => LiteralExpr::new(Literal::Int(x), *cur.get_position()),
            TokenType::DoubleLiteral(x) => LiteralExpr::new(Literal::Double(x), *cur.get_position()),
            TokenType::BoolLiteral(x) => LiteralExpr::new(Literal::Bool(x), *cur.get_position()),
            TokenType::CharLiteral(x) => LiteralExpr::new(Literal::Char(x), *cur.get_position()),
            TokenType::StringLiteral(ref x) => LiteralExpr::new(Literal::String(x.clone()), *cur.get_position()),
            _ => {
                parser.push_diagnostic(ErrMsg::ExpectedExpression.make_diagnostic(*cur.get_position()));
                return None;
            }
        };

        parser.next();

        return Some(Box::new(literal));
    }
}