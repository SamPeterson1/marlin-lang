use serde::Serialize;

use crate::{expr::ASTWrapper, token::PositionRange};

use super::var_expr::VarExpr;

#[derive(Serialize)]
pub struct GetAddressExpr {
    pub var_expr: ASTWrapper<VarExpr>,
}

impl ASTWrapper<GetAddressExpr> {
    pub fn new_get_address(var_expr: ASTWrapper<VarExpr>, position: PositionRange) -> Self {
        ASTWrapper {
            data: GetAddressExpr {
                var_expr,
            },
            position
        }
        
    }
}

crate::impl_ast_node!(GetAddressExpr, visit_get_address);