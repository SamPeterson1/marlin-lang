use serde::Serialize;

use crate::{ast::{ASTWrapper, arguments::Arguments}, impl_ast_node, token::PositionRange};


#[derive(Serialize)]
pub struct ConstructorCallExpr {
    pub type_name: String,
    pub arguments: ASTWrapper<Arguments>,
    pub is_heap: bool,
}

impl ASTWrapper<ConstructorCallExpr> {
    pub fn new_constructor_call(type_name: String, arguments: ASTWrapper<Arguments>, is_heap: bool, position: PositionRange) -> Self {        
        ASTWrapper {
            data: ConstructorCallExpr { 
                type_name, 
                arguments,
                is_heap
            },
            position
        }
    }    
}

impl_ast_node!(ConstructorCallExpr, visit_constructor_call);