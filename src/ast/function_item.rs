use serde::Serialize;

use crate::{ast::{ASTWrapper, block_expr::BlockExpr, parameters::Parameters, parsed_type::ParsedType}, impl_ast_node, token::{PositionRange, Positioned}};

#[derive(Serialize)]
pub struct FunctionItem {
    pub name: String,
    pub parameters: ASTWrapper<Parameters>,
    pub return_type: ASTWrapper<ParsedType>,
    pub body: ASTWrapper<BlockExpr>,
}


impl ASTWrapper<FunctionItem> {
    pub fn new_function_item(name: String, parameters: ASTWrapper<Parameters>, return_type: ASTWrapper<ParsedType>, body: ASTWrapper<BlockExpr>, position: PositionRange) -> Self {
        
        ASTWrapper {
            data: FunctionItem {
                name,
                parameters,
                return_type,
                body
            },
            position
        }
    }
}

impl_ast_node!(FunctionItem, visit_function);