use serde::Serialize;

use crate::ast::arguments::Arguments;
use crate::resolver::ResolvedType;
use crate::{impl_ast_node, impl_positioned, impl_typed};
use crate::lexer::token::{Located, PositionRange};

#[derive(Serialize)]
pub struct ConstructorCallExpr {
    pub type_name: Located<String>,
    pub arguments: Arguments,
    pub is_heap: bool,
    position: PositionRange,
    resolved_type: Option<ResolvedType>,
}

impl ConstructorCallExpr {
    pub fn new(type_name: Located<String>, arguments: Arguments, is_heap: bool, position: PositionRange) -> Self {        
        Self {
            type_name,
            arguments,
            is_heap,
            position,
            resolved_type: None,
        }
    }    
}

impl_positioned!(ConstructorCallExpr);
impl_typed!(ConstructorCallExpr);
impl_ast_node!(ConstructorCallExpr, visit_constructor_call);