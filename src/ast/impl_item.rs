use serde::Serialize;

use crate::ast::{function_item::FunctionItem, AstId};
use crate::{impl_ast_node, impl_positioned, new_ast_id};
use crate::lexer::token::{Located, PositionRange};

#[derive(Serialize)]
pub struct ImplItem {
    pub identifier: Located<String>,
    pub functions: Vec<FunctionItem>,
    position: PositionRange,
    id: AstId,
}


impl ImplItem {
    pub fn new(identifier: Located<String>, functions: Vec<FunctionItem>, position: PositionRange) -> Self {
        Self {
            identifier,
            functions,
            position,
            id: new_ast_id!(),
        }
    }
}

impl_positioned!(ImplItem);
impl_ast_node!(ImplItem, visit_impl);