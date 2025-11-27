use serde::Serialize;

use crate::{ast::ASTWrapper, impl_ast_node, token::Token};

#[derive(Serialize)]
pub struct VarExpr {
    pub id: u32,
    pub identifier: String,
}

impl ASTWrapper<VarExpr> {
    pub fn new_var(id: u32, identifier: Token) -> Self {
        ASTWrapper {
            data: VarExpr {
                id,
                identifier: identifier.get_string().to_string(),
            },
            position: identifier.position
        }
    }
}

impl_ast_node!(VarExpr, visit_var);