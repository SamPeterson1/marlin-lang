use std::rc::Rc;

use serde::Serialize;

use crate::impl_positioned;
use crate::lexer::token::PositionRange;

#[derive(Serialize, Clone, PartialEq, Eq, Debug)]
pub enum ParsedTypeEnum {
    Integer, Double, Boolean, Char,
    TypeName(Rc<String>),
    Pointer(Rc<ParsedType>),
    Reference(Rc<ParsedType>),
    Array(Rc<ParsedType>),
}

#[derive(Serialize, Clone, Debug)]
pub struct ParsedType {
    pub parsed_type: ParsedTypeEnum,
    position: PositionRange,
}

impl PartialEq for ParsedType {
    fn eq(&self, other: &Self) -> bool {
        self.parsed_type == other.parsed_type
    }
}

impl Eq for ParsedType {}

impl ParsedType {
    pub fn new(parsed_type: ParsedTypeEnum, position: PositionRange) -> Self {
        Self {
            parsed_type,
            position,
        }
    }
}

impl_positioned!(ParsedType);
