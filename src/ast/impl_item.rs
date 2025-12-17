use serde::Serialize;

use crate::ast::{function_item::FunctionItem, parsed_type::ParsedType};
use crate::{impl_ast_node, impl_positioned};
use crate::lexer::token::{Located, PositionRange};

#[derive(Serialize)]
pub struct ImplItem {
    pub identifier: Located<String>,
    pub functions: Vec<FunctionItem>,
    position: PositionRange,
}


impl ImplItem {
    pub fn new(identifier: Located<String>, functions: Vec<FunctionItem>, position: PositionRange) -> Self {
        Self {
            identifier,
            functions,
            position,
        }
    }
}

impl_positioned!(ImplItem);
impl_ast_node!(ImplItem, visit_impl);