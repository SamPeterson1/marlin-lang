use std::rc::Rc;

use serde::Serialize;

use crate::{ast::{ASTNode, ASTWrapper}, impl_ast_node, token::PositionRange, types::parsed_type::ParsedType};

#[derive(Serialize)]
pub struct FunctionItem {
    pub name: Rc<String>,
    pub args: Vec<(String, ParsedType)>,
    pub expr: Box<dyn ASTNode>,
    pub ret_type: ParsedType,
}


impl ASTWrapper<FunctionItem> {
    pub fn new_function(name: String, args: Vec<(String, ParsedType)>, expr: Box<dyn ASTNode>, ret_type: ParsedType, position: PositionRange) -> Self {
        ASTWrapper {
            data: FunctionItem {
                name: Rc::new(name),
                args,
                expr,
                ret_type,
            },
            position
        }
    }
}

impl_ast_node!(FunctionItem, visit_function);