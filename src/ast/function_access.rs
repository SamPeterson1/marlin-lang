use serde::Serialize;

use crate::ast::{ASTEnum, ASTNode, AstId};
use crate::compiler::visit::{Parsed, Phase};
use crate::{impl_ast_node, new_ast_id};
use crate::lexer::token::PositionRange;

#[derive(Serialize)]
pub struct FunctionAccess<P: Phase = Parsed> {
    pub expr: ASTEnum<P>,
    pub arguments: Vec<ASTEnum<P>>,
    position: PositionRange,
    id: AstId,
}

impl FunctionAccess {
    pub fn new(expr: ASTEnum, arguments: Vec<ASTEnum>, position: PositionRange) -> Self {
        Self {
            expr,
            arguments,
            position,
            id: new_ast_id!(),
        }
    }
}

impl_ast_node!(FunctionAccess, visit_function_access);
