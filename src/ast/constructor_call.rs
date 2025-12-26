use serde::Serialize;

use crate::ast::{arguments::Arguments, AstId};
use crate::{impl_ast_node, impl_positioned, new_ast_id};
use crate::lexer::token::{Located, PositionRange};

#[derive(Serialize)]
pub struct ConstructorCallExpr {
    pub type_name: Located<String>,
    pub arguments: Arguments,
    pub is_heap: bool,
    position: PositionRange,
    id: AstId,
}

impl ConstructorCallExpr {
    pub fn new(type_name: Located<String>, arguments: Arguments, is_heap: bool, position: PositionRange) -> Self {        
        Self {
            type_name,
            arguments,
            is_heap,
            position,
            id: new_ast_id!(),
        }
    }    
}

impl_positioned!(ConstructorCallExpr);
impl_ast_node!(ConstructorCallExpr, visit_constructor_call);