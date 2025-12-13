use serde::Serialize;

use crate::ast::{constructor_item::ConstructorItem, parsed_type::ParsedType};
use crate::{impl_ast_node, impl_positioned};
use crate::lexer::token::{Located, PositionRange};

#[derive(Serialize)]
pub struct StructItem {
    pub name: Located<String>,
    pub members: Vec<(ParsedType, Located<String>)>,
    pub constructors: Vec<ConstructorItem>,
    position: PositionRange,
}

impl StructItem {
    pub fn new(
        name: Located<String>,
        members: Vec<(ParsedType, Located<String>)>,
        constructors: Vec<ConstructorItem>,
        position: PositionRange,
    ) -> Self {
        Self {
            name,
            members,
            constructors,
            position,
        }
    }
}

impl_positioned!(StructItem);
impl_ast_node!(StructItem, visit_struct);