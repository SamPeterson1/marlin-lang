use std::{collections::HashMap, rc::Rc};

use serde::Serialize;

use crate::{expr::ASTWrapper, impl_ast_node, token::PositionRange, types::parsed_type::ParsedType};


#[derive(Clone, Serialize)]
pub struct StructItem {
    pub name: Rc<String>,
    pub members: HashMap<String, ParsedType>,
}

impl ASTWrapper<StructItem> {
    pub fn new_struct_item(name: String, members: HashMap<String, ParsedType>, position: PositionRange) -> Self {
        ASTWrapper {
            data: StructItem {
                name: Rc::new(name),
                members,
            },
            position
        }
    }
}

impl_ast_node!(StructItem, visit_struct);