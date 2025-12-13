use std::fmt;

use crate::ast::{ASTNode, block_expr::BlockExpr};
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::statement::StatementRule;
use crate::lexer::token::TokenType;

pub struct BlockRule {}

impl fmt::Display for BlockRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Block")
    }
}

impl ParseRule<BlockExpr> for BlockRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::LeftCurly).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<BlockExpr> {
        parser.begin_range();
        
        parser.consume_or_diagnostic(TokenType::LeftCurly);
        let mut exprs: Vec<Box<dyn ASTNode>> = Vec::new();

        while parser.try_consume_match(&[TokenType::EOF, TokenType::RightCurly]).is_none() {
            let statement = parser.apply_rule(StatementRule {}, "block statement", Some(ErrMsg::ExpectedStatement));

            if let Some(statement) = statement {
                exprs.push(statement);
            }
        }

        Some(BlockExpr::new(exprs, parser.end_range()))
    }
}
