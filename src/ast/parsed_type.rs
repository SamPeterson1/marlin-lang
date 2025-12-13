use std::rc::Rc;

use serde::Serialize;

use crate::impl_positioned;
use crate::lexer::token::{Located, PositionRange};

#[derive(Serialize, Clone)]
pub enum ParsedBaseType {
    Integer, Double, Boolean, Char,
    TypeName(Rc<String>),
}

#[derive(Serialize, Clone)]
pub enum ParsedUnitModifier {
    None,
    Reference,
    Pointer(u32),
}

#[derive(Serialize)]
pub struct ParsedUnitType {
    pub base_type: Located<ParsedBaseType>,
    pub modifier: ParsedUnitModifier,
    position: PositionRange,
}

#[derive(Serialize)]
pub struct ParsedType {
    pub is_reference: bool,
    pub unit_type: ParsedUnitType,
    pub array_dimension: u32,
    position: PositionRange,
}

impl ParsedType {
    pub fn new(is_reference: bool, unit_type: ParsedUnitType, array_dimension: u32, position: PositionRange) -> Self {
        Self {
            is_reference,
            unit_type,
            array_dimension,
            position,
        }
    }
}

impl ParsedUnitType {
    pub fn new(base_type: Located<ParsedBaseType>, modifier: ParsedUnitModifier, position: PositionRange) -> Self {
        Self {
            base_type,
            modifier,
            position,
        }
    }
}

impl_positioned!(ParsedType);
impl_positioned!(ParsedUnitType);