use std::rc::Rc;

use serde::Serialize;

use crate::{ast::{ASTWrapper, constructor_item::ConstructorItem, parsed_type::ParsedType}, impl_ast_node, token::PositionRange};

#[derive(Serialize)]
pub struct StructItem {
    pub name: Rc<String>,
    pub members: Vec<(ASTWrapper<ParsedType>, String)>,
    pub constructors: Vec<ASTWrapper<ConstructorItem>>,
}


impl ASTWrapper<StructItem> {
    pub fn new_struct_item(name: String, members: Vec<(ASTWrapper<ParsedType>, String)>, constructors: Vec<ASTWrapper<ConstructorItem>>, position: PositionRange) -> Self {
        ASTWrapper {
            data: StructItem {
                name: Rc::new(name),
                members,
                constructors,
            },
            position
        }
    }
}

impl_ast_node!(StructItem, visit_struct);