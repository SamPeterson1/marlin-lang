use std::fmt;

use crate::{expr::{Expr, assignment_expr::AssignmentExpr, block_expr::BlockExpr, static_array_expr::StaticArrayExpr, var_expr::VarExpr}, logger::{Log, Logger}, parser::{ExprParser, ParseRule, diagnostic, rules::{statement::StatementRule, var::VarRule}}, token::{Position, PositionRange, TokenType}};

pub struct BlockRule {}

impl fmt::Display for BlockRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Block")
    }
}

//block: LEFT_CURLY [statement]* RIGHT_CURLY
impl ParseRule<BlockExpr> for BlockRule {
    fn parse(&self, parser: &mut ExprParser) -> Option<BlockExpr> {
        parser.log_debug(&format!("Entering block parser. Current token {:?}", parser.cur()));
        let start_token = parser.cur();
        parser.consume_or_diagnostic(TokenType::LeftCurly, diagnostic::err_expected_token(PositionRange::new(Position::new(0, 0)), TokenType::LeftCurly));
        let mut exprs: Vec<Box<dyn Expr>> = Vec::new();

        while parser.try_match(&[TokenType::EOF, TokenType::RightCurly]).is_none() {
            let statement = parser.apply_rule(StatementRule {});
            parser.log_parse_result(&statement, "block statement");

            if let Some(statement) = statement {
                exprs.push(statement);
            }
        }

        let position = PositionRange::concat(&start_token.position, &parser.prev().position);

        Some(BlockExpr::new(exprs, position))
    }
}
