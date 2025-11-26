use serde::Serialize;

use crate::{ast::{ASTWrapper, block_expr::BlockExpr}, impl_ast_node, token::PositionRange};

#[derive(Serialize)]
pub struct MainItem {
    pub body: ASTWrapper<BlockExpr>
}


impl ASTWrapper<MainItem> {
    pub fn new_main(body: ASTWrapper<BlockExpr>, position: PositionRange) -> Self {
        ASTWrapper {
            data: MainItem {
                body
            },
            position
        }
    }
}

impl_ast_node!(MainItem, visit_main);