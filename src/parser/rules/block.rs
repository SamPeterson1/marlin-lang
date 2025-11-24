use std::fmt;

use crate::{ast::{ASTNode, ASTWrapper, block_expr::BlockExpr}, logger::Log, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, diagnostic::{self, ErrMsg}, rules::statement::StatementRule}, token::{Position, PositionRange, TokenType}};

pub struct BlockRule {}

impl fmt::Display for BlockRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Block")
    }
}

impl ParseRule<ASTWrapper<BlockExpr>> for BlockRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::LeftCurly).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<BlockExpr>> {
        let start_token = &parser.cur();

        parser.consume_or_diagnostic(TokenType::LeftCurly);
        let mut exprs: Vec<Box<dyn ASTNode>> = Vec::new();

        while parser.try_match(&[TokenType::EOF, TokenType::RightCurly]).is_none() {
            let statement = parser.apply_rule(StatementRule {}, "block statement", Some(ErrMsg::ExpectedStatement));

            if let Some(statement) = statement {
                exprs.push(statement);
            }
        }

        let position = PositionRange::concat(&start_token.position, &parser.prev().position);

        Some(ASTWrapper::new_block(exprs, position))
    }
}
