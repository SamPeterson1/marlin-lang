use serde::Serialize;

use crate::ast::{ASTEnum, ASTNode, AstId};
use crate::compiler::stages::{Parsed, Phase};
use crate::{impl_ast_node, new_ast_id};
use crate::lexer::token::PositionRange;

#[derive(Serialize)]
pub struct BlockExpr<P: Phase = Parsed> {
    pub exprs: Vec<ASTEnum<P>>,
    position: PositionRange,
    id: AstId,
}

impl BlockExpr {
    pub fn new(exprs: Vec<ASTEnum>, position: PositionRange) -> Self {
        Self {
            exprs,
            position,
            id: new_ast_id!(),
        }
    }    
}

impl_ast_node!(BlockExpr, visit_block);