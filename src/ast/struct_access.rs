use serde::Serialize;

use crate::ast::{ASTEnum, ASTNode, AstId};
use crate::compiler::visit::{Parsed, Phase};
use crate::{impl_ast_node, new_ast_id};
use crate::lexer::token::{Located, PositionRange};

#[derive(Serialize)]
pub struct StructAccess<P: Phase = Parsed> {
    pub expr: ASTEnum<P>,
    pub member_name: Located<String>,
    pub is_direct: bool,
    position: PositionRange,
    id: AstId,
}

impl StructAccess {
    pub fn new(expr: ASTEnum, member_name: Located<String>, is_direct: bool, position: PositionRange) -> Self {
        Self {
            expr,
            member_name,
            is_direct,
            position,
            id: new_ast_id!(),
        }
    }
}

impl_ast_node!(StructAccess, visit_struct_access);
