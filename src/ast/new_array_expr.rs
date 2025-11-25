use serde::Serialize;

use crate::{ast::{ASTNode, ASTWrapper, parsed_type::{ParsedType, ParsedUnitType}}, token::PositionRange};

#[derive(Serialize)]
pub struct NewArrayExpr {
    pub dimension: usize,
    pub sizes: Vec<Box<dyn ASTNode>>,
    pub unit_type: ASTWrapper<ParsedUnitType>,
}

impl ASTWrapper<NewArrayExpr> {
    pub fn new_new_array_expr(sizes: Vec<Box<dyn ASTNode>>, unit_type: ASTWrapper<ParsedUnitType>, position: PositionRange) -> Self {
        ASTWrapper {
            data: NewArrayExpr {
                dimension: sizes.len(),
                sizes, 
                unit_type,
            },
            position
        }
    }
}

crate::impl_ast_node!(NewArrayExpr, visit_new_array);