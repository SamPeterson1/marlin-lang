use serde::Serialize;

use crate::ast::{ASTEnum, ASTNode, AstId, ParsedType};
use crate::compiler::visit::{Parsed, Phase};
use crate::{impl_ast_node, new_ast_id};
use crate::lexer::token::PositionRange;

#[derive(Serialize)]
pub struct NewArrayExpr<P: Phase = Parsed> {
    pub dimension: usize,
    pub sizes: Vec<ASTEnum<P>>,
    pub array_type: ParsedType,
    position: PositionRange,
    id: AstId,
}

impl NewArrayExpr {
    pub fn new(sizes: Vec<ASTEnum>, array_type: ParsedType, position: PositionRange) -> Self {
        Self {
            dimension: sizes.len(),
            sizes,
            array_type,
            position,
            id: new_ast_id!(),
        }
    }
}

impl_ast_node!(NewArrayExpr, visit_new_array);