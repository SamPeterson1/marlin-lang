use serde::Serialize;
use std::marker::PhantomData;

use crate::ast::{ASTEnum, ASTNode, AstId};
use crate::compiler::visit::{Parsed, Phase};
use crate::{impl_ast_node, new_ast_id};
use crate::lexer::token::PositionRange;

#[derive(Serialize)]
pub struct DeleteExpr<P: Phase = Parsed> {
    pub expr: ASTEnum<P>,
    position: PositionRange,

    #[serde(skip)]
    id: AstId,
    #[serde(skip)]
    _phase: PhantomData<P>,
}

impl DeleteExpr {
    pub fn new(expr: ASTEnum, position: PositionRange) -> Self {
        Self {
            expr,
            position,
            id: new_ast_id!(),
            _phase: PhantomData,
        }
    }
}

impl_ast_node!(DeleteExpr, visit_delete);