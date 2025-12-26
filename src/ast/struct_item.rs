use serde::Serialize;

use crate::ast::{constructor_item::ConstructorItem, parsed_type::ParsedType, AstId};
use crate::{impl_ast_node, impl_positioned, new_ast_id};
use crate::lexer::token::{Located, PositionRange};

#[derive(Serialize)]
pub struct StructItem {
    pub name: Located<String>,
    pub members: Vec<(ParsedType, Located<String>)>,
    pub constructors: Vec<ConstructorItem>,
    position: PositionRange,
    id: AstId,
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
            id: new_ast_id!(),
        }
    }
}

impl_positioned!(StructItem);
impl_ast_node!(StructItem, visit_struct);