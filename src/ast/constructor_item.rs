use serde::Serialize;

use crate::{ast::{ASTWrapper, block_expr::BlockExpr, parameters::Parameters}, impl_ast_node, token::PositionRange};

#[derive(Serialize)]
pub struct ConstructorItem {
    pub parameters: ASTWrapper<Parameters>,
    pub body: ASTWrapper<BlockExpr>
}


impl ASTWrapper<ConstructorItem> {
    pub fn new_constructor_item(parameters: ASTWrapper<Parameters>, body: ASTWrapper<BlockExpr>, position: PositionRange) -> Self {
        ASTWrapper {
            data: ConstructorItem {
                parameters,
                body,
            },
            position
        }
    }
}

impl_ast_node!(ConstructorItem, visit_constructor);