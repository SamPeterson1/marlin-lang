use serde::Serialize;

use crate::ast::{ASTNode, ParsedType, AstId};
use crate::{impl_ast_node, impl_positioned, new_ast_id};
use crate::lexer::token::PositionRange;

#[derive(Serialize)]
pub struct NewArrayExpr {
    pub dimension: usize,
    pub sizes: Vec<Box<dyn ASTNode>>,
    pub array_type: ParsedType,
    position: PositionRange,
    id: AstId,
}

impl NewArrayExpr {
    pub fn new(sizes: Vec<Box<dyn ASTNode>>, array_type: ParsedType, position: PositionRange) -> Self {
        Self {
            dimension: sizes.len(),
            sizes,
            array_type,
            position,
            id: new_ast_id!(),
        }
    }
}

impl_positioned!(NewArrayExpr);
impl_ast_node!(NewArrayExpr, visit_new_array);