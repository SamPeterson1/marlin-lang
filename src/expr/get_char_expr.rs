use serde::Serialize;

use crate::{expr::ASTWrapper, token::PositionRange};

#[derive(Serialize)]
pub struct GetCharExpr {}

impl ASTWrapper<GetCharExpr> {
    pub fn new_get_char(position: PositionRange) -> Self {
        ASTWrapper {
            data: GetCharExpr {},
            position
        }
    }
}

crate::impl_ast_node!(GetCharExpr, visit_get_char);