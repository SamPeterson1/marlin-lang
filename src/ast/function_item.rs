use std::rc::Rc;

use serde::Serialize;

use crate::{ast::{ASTNode, ASTWrapper, block_expr::BlockExpr, function_prototype::FunctionPrototype, parsed_type::ParsedType}, impl_ast_node, token::{PositionRange, Positioned}};

#[derive(Serialize)]
pub struct FunctionItem {
    pub prototype: ASTWrapper<FunctionPrototype>,
    pub body: ASTWrapper<BlockExpr>,
    pub src_type: ASTWrapper<ParsedType>,
    pub src_identifier: String,
}


impl ASTWrapper<FunctionItem> {
    pub fn new_function_item(prototype: ASTWrapper<FunctionPrototype>, body: ASTWrapper<BlockExpr>, src_type: ASTWrapper<ParsedType>, src_identifier: String) -> Self {
        let position = PositionRange::concat(prototype.get_position(), body.get_position());
        
        ASTWrapper {
            data: FunctionItem {
                prototype,
                body,
                src_type,
                src_identifier,
            },
            position
        }
    }
}

impl_ast_node!(FunctionItem, visit_function);