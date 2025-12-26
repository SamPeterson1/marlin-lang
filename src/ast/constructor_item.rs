use serde::Serialize;

use crate::ast::{AstId, BlockExpr, DeclarationExpr};
use crate::{impl_ast_node, impl_positioned, new_ast_id};
use crate::lexer::token::PositionRange;

#[derive(Serialize)]
pub struct ConstructorItem {
    pub parameters: Vec<DeclarationExpr>,
    pub body: BlockExpr,
    position: PositionRange,
    id: AstId,
}

impl ConstructorItem {
    pub fn new(parameters: Vec<DeclarationExpr>, body: BlockExpr, position: PositionRange) -> Self {
        Self {
            parameters,
            body,
            position,
            id: new_ast_id!(),
        }
    }
}

impl_positioned!(ConstructorItem);
impl_ast_node!(ConstructorItem, visit_constructor);