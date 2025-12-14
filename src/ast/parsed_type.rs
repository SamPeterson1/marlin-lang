use std::rc::Rc;

use serde::Serialize;

use crate::impl_positioned;
use crate::lexer::token::{Located, PositionRange};

#[derive(Serialize, Clone)]
pub enum ParsedBaseType {
    Integer, Double, Boolean, Char,
    TypeName(Rc<String>),
}


#[derive(Serialize)]
pub struct ParsedUnitType {
    pub base_type: Located<ParsedBaseType>,
    pub n_pointers: u32,
    pub is_reference: bool,
    position: PositionRange,
}

#[derive(Serialize)]
pub struct ArrayModifier {
    pub is_reference: bool,
}

#[derive(Serialize)]
pub struct ParsedType {
    pub unit_type: ParsedUnitType,
    pub array_modifiers: Vec<ArrayModifier>,

    position: PositionRange,
}

impl ParsedType {
    pub fn new(unit_type: ParsedUnitType, array_modifiers: Vec<ArrayModifier>, position: PositionRange) -> Self {
        Self {
            unit_type,
            array_modifiers,
            position,
        }
    }
}

impl ParsedUnitType {
    pub fn new(base_type: Located<ParsedBaseType>, n_pointers: u32, is_reference: bool, position: PositionRange) -> Self {
        Self {
            base_type,
            n_pointers,
            is_reference,
            position,
        }
    }
}

impl_positioned!(ParsedType);
impl_positioned!(ParsedUnitType);