use serde::Serialize;

use crate::{ast::{ASTWrapper, function_item::FunctionItem, parsed_type::ParsedType}, impl_ast_node, token::PositionRange};

#[derive(Serialize)]
pub struct ImplItem {
    pub impl_type: ASTWrapper<ParsedType>,
    pub functions: Vec<ASTWrapper<FunctionItem>>,
}


impl ASTWrapper<ImplItem> {
    pub fn new_impl_item(impl_type: ASTWrapper<ParsedType>, functions: Vec<ASTWrapper<FunctionItem>>, position: PositionRange) -> Self {
        
        ASTWrapper {
            data: ImplItem {
                impl_type,
                functions
            },
            position
        }
    }
}

impl_ast_node!(ImplItem, visit_impl);