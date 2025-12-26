use std::rc::Rc;

use serde::Serialize;

use crate::impl_positioned;
use crate::lexer::token::PositionRange;

#[derive(Serialize, Clone, PartialEq, Eq, Debug)]
pub enum ParsedTypeEnum {
    Integer, Double, Boolean, Char, Void,
    TypeName(String),
    Pointer(Box<ParsedType>),
    Reference(Box<ParsedType>),
    Array(Box<ParsedType>),
}

impl ParsedTypeEnum {
    pub fn inner_type_name(&self) -> Option<&str> {
        match self {
            Self::Array(t) => t.parsed_type.inner_type_name(),
            Self::Pointer(t) => t.parsed_type.inner_type_name(),
            Self::Reference(t) => t.parsed_type.inner_type_name(),
            Self::Boolean | Self::Integer | 
            Self::Double | Self::Char | Self::Void => None,
            Self::TypeName(type_name) => Some(type_name.as_str())
        }
    }
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
