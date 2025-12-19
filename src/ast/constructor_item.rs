use serde::Serialize;

use crate::ast::{block_expr::BlockExpr, parameters::Parameters};
use crate::{impl_ast_node, impl_positioned};
use crate::lexer::token::PositionRange;

#[derive(Serialize)]
pub struct ConstructorItem {
    pub parameters: Parameters,
    pub body: BlockExpr,
    position: PositionRange,
}

impl ConstructorItem {
    pub fn new(parameters: Parameters, body: BlockExpr, position: PositionRange) -> Self {
        Self {
            parameters,
            body,
            position,
        }
    }
}

impl_positioned!(ConstructorItem);
impl_ast_node!(ConstructorItem, visit_constructor);