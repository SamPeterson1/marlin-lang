use std::rc::Rc;

use serde::Serialize;

use crate::{ast::ASTWrapper, token::PositionRange};

/* Unresolved type parsed from tokens */
#[derive(Clone, Serialize)]
pub enum ParsedUnitType {
    Integer, Double, Boolean,
    TypeName(Rc<String>),
}

#[derive(Clone, Serialize)]
pub enum ParsedType {
    Reference(ParsedUnitType),
    Unit(ParsedUnitType),
    Array(ParsedUnitType, u32)
}

impl ASTWrapper<ParsedType> {
    pub fn new_parsed_type(parsed_type: ParsedType, position: PositionRange) -> Self {
        ASTWrapper {
            data: parsed_type,
            position
        }
    }
}