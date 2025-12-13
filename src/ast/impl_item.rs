use serde::Serialize;

use crate::ast::{function_item::FunctionItem, parsed_type::ParsedType};
use crate::{impl_ast_node, impl_positioned};
use crate::lexer::token::PositionRange;

#[derive(Serialize)]
pub struct ImplItem {
    pub impl_type: ParsedType,
    pub functions: Vec<FunctionItem>,
    position: PositionRange,
}


impl ImplItem {
    pub fn new(impl_type: ParsedType, functions: Vec<FunctionItem>, position: PositionRange) -> Self {
        Self {
            impl_type,
            functions,
            position,
        }
    }
}

impl_positioned!(ImplItem);
impl_ast_node!(ImplItem, visit_impl);