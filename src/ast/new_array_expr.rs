use serde::Serialize;

use crate::ast::{ASTNode, parsed_type::ParsedUnitType};
use crate::{impl_ast_node, impl_positioned};
use crate::lexer::token::PositionRange;

#[derive(Serialize)]
pub struct NewArrayExpr {
    pub dimension: usize,
    pub sizes: Vec<Box<dyn ASTNode>>,
    pub unit_type: ParsedUnitType,
    position: PositionRange,
}

impl NewArrayExpr {
    pub fn new(sizes: Vec<Box<dyn ASTNode>>, unit_type: ParsedUnitType, position: PositionRange) -> Self {
        Self {
            dimension: sizes.len(),
            sizes,
            unit_type,
            position,
        }
    }
}

impl_positioned!(NewArrayExpr);
impl_ast_node!(NewArrayExpr, visit_new_array);