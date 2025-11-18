use std::fmt;

use crate::{ast::{ASTNode, ASTWrapper, block_expr::BlockExpr}, logger::Log, parser::{ExprParser, ParseRule, diagnostic, rules::statement::StatementRule}, token::{Position, PositionRange, TokenType}};

pub struct BlockRule {}

impl fmt::Display for BlockRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Block")
    }
}

//block: LEFT_CURLY [statement]* RIGHT_CURLY
impl ParseRule<ASTWrapper<BlockExpr>> for BlockRule {
    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<BlockExpr>> {
        parser.log_debug(&format!("Entering block parser. Current token {:?}", parser.cur()));
        let start_token = parser.cur();
        parser.consume_or_diagnostic(TokenType::LeftCurly, diagnostic::err_expected_token(PositionRange::new(Position::new(0, 0)), TokenType::LeftCurly));
        let mut exprs: Vec<Box<dyn ASTNode>> = Vec::new();

        while parser.try_match(&[TokenType::EOF, TokenType::RightCurly]).is_none() {
            let statement = parser.apply_rule(StatementRule {});
            parser.log_parse_result(&statement, "block statement");

            if let Some(statement) = statement {
                exprs.push(statement);
            }
        }

        let position = PositionRange::concat(&start_token.position, &parser.prev().position);

        Some(ASTWrapper::new_block(exprs, position))
    }
}
