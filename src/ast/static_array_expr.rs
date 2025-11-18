use serde::Serialize;

use crate::{ast::ASTWrapper, token::PositionRange, types::parsed_type::ParsedType};

#[derive(Serialize)]
pub struct StaticArrayExpr {
    pub len: usize,
    pub declaration_type: ParsedType,
}

impl ASTWrapper<StaticArrayExpr> {
    pub fn new_static_array(len: usize, declaration_type: ParsedType, position: PositionRange) -> Self {
        ASTWrapper {
            data: StaticArrayExpr {
                len, 
                declaration_type,
            },
            position
        }
    }
}

crate::impl_ast_node!(StaticArrayExpr, visit_static_array);