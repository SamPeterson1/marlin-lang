use serde::Serialize;

use crate::ast::{block_expr::BlockExpr, AstId};
use crate::{impl_ast_node, impl_positioned, new_ast_id};
use crate::lexer::token::PositionRange;

#[derive(Serialize)]
pub struct MainItem {
    pub body: BlockExpr,
    position: PositionRange,
    id: AstId,
}

impl MainItem {
    pub fn new(body: BlockExpr, position: PositionRange) -> Self {
        Self {
            body,
            position,
            id: new_ast_id!(),
        }
    }
}

impl_positioned!(MainItem);
impl_ast_node!(MainItem, visit_main);