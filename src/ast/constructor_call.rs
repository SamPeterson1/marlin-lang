use serde::Serialize;

use crate::ast::{ASTEnum, ASTNode, AstId};
use crate::compiler::visit::{Parsed, Phase};
use crate::{impl_ast_node, new_ast_id};
use crate::lexer::token::{Located, PositionRange};

#[derive(Serialize)]
pub struct ConstructorCallExpr<P: Phase = Parsed> {
    pub type_name: Located<String>,
    pub arguments: Vec<ASTEnum<P>>,
    pub is_heap: bool,
    position: PositionRange,
    id: AstId,
}

impl ConstructorCallExpr {
    pub fn new(type_name: Located<String>, arguments: Vec<ASTEnum>, is_heap: bool, position: PositionRange) -> Self {        
        Self {
            type_name,
            arguments,
            is_heap,
            position,
            id: new_ast_id!(),
        }
    }    
}

impl_ast_node!(ConstructorCallExpr, visit_constructor_call);