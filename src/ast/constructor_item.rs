use serde::Serialize;

use crate::ast::{ASTNode, AstId, BlockExpr, DeclarationExpr};
use crate::compiler::visit::{Parsed, Phase};
use crate::{impl_ast_node, new_ast_id};
use crate::lexer::token::PositionRange;

#[derive(Serialize)]
pub struct ConstructorItem<P: Phase = Parsed> {
    pub parameters: Vec<DeclarationExpr<P>>,
    pub body: BlockExpr<P>,
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

impl_ast_node!(ConstructorItem, visit_constructor);