use serde::Serialize;

use crate::{ast::{ASTNode, ASTWrapper, parsed_type::ParsedType}, token::PositionRange};

#[derive(Serialize)]
pub struct NewArrayExpr {
    pub len: Box<dyn ASTNode>,
    pub declaration_type: ASTWrapper<ParsedType>,
}

impl ASTWrapper<NewArrayExpr> {
    pub fn new_new_array_expr(len: Box<dyn ASTNode>, declaration_type: ASTWrapper<ParsedType>, position: PositionRange) -> Self {
        ASTWrapper {
            data: NewArrayExpr {
                len, 
                declaration_type,
            },
            position
        }
    }
}

crate::impl_ast_node!(NewArrayExpr, visit_new_array);