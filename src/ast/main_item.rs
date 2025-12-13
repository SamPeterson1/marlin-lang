use serde::Serialize;

use crate::ast::block_expr::BlockExpr;
use crate::{impl_ast_node, impl_positioned};
use crate::lexer::token::PositionRange;

#[derive(Serialize)]
pub struct MainItem {
    pub body: BlockExpr,
    position: PositionRange,
}

impl MainItem {
    pub fn new(body: BlockExpr, position: PositionRange) -> Self {
        Self {
            body,
            position,
        }
    }
}

impl_positioned!(MainItem);
impl_ast_node!(MainItem, visit_main);