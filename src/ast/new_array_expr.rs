use serde::Serialize;

use crate::ast::{ASTNode, ParsedType};
use crate::resolver::ResolvedType;
use crate::{impl_ast_node, impl_positioned, impl_typed};
use crate::lexer::token::PositionRange;

#[derive(Serialize)]
pub struct NewArrayExpr {
    pub dimension: usize,
    pub sizes: Vec<Box<dyn ASTNode>>,
    pub array_type: ParsedType,
    position: PositionRange,
    resolved_type: Option<ResolvedType>,
}

impl NewArrayExpr {
    pub fn new(sizes: Vec<Box<dyn ASTNode>>, array_type: ParsedType, position: PositionRange) -> Self {
        Self {
            dimension: sizes.len(),
            sizes,
            array_type,
            position,
            resolved_type: None,
        }
    }
}

impl_positioned!(NewArrayExpr);
impl_typed!(NewArrayExpr);
impl_ast_node!(NewArrayExpr, visit_new_array);