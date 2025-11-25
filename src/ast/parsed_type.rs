use std::rc::Rc;

use serde::Serialize;

use crate::{ast::ASTWrapper, token::PositionRange};

#[derive(Serialize)]
pub enum ParsedBaseType {
    Integer, Double, Boolean,
    TypeName(Rc<String>),
}

#[derive(Serialize)]
pub struct ParsedUnitType {
    pub base_type: ASTWrapper<ParsedBaseType>,
    pub is_reference: bool,
}

#[derive(Serialize)]
pub struct ParsedType {
    pub is_reference: bool,
    pub unit_type: ASTWrapper<ParsedUnitType>,
    pub array_dimension: u32,
}

impl ASTWrapper<ParsedBaseType> {
    pub fn new_parsed_base_type(base_type: ParsedBaseType, position: PositionRange) -> Self {
        ASTWrapper {
            data: base_type,
            position
        }
    }
}

impl ASTWrapper<ParsedType> {
    pub fn new_parsed_type(is_reference: bool, unit_type: ASTWrapper<ParsedUnitType>, array_dimension: u32, position: PositionRange) -> Self {
        ASTWrapper {
            data: ParsedType {
                is_reference,
                unit_type,
                array_dimension
            },
            position
        }
    }
}

impl ASTWrapper<ParsedUnitType> {
    pub fn new_parsed_unit_type(base_type: ASTWrapper<ParsedBaseType>, is_reference: bool, position: PositionRange) -> Self {
        ASTWrapper {
            data: ParsedUnitType {
                base_type,
                is_reference
            },
            position
        }
    }
}