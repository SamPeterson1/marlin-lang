use std::{fmt, rc::Rc};

use serde::Serialize;

use crate::token::PositionRange;

/* Unresolved type parsed from tokens */
#[derive(Clone, Serialize)]
pub enum ParsedType {
    Integer, Double, Boolean,
    TypeName(ParsedTypeName),
    Function(ParsedFunctionType),
    Pointer(ParsedPointerType),
    Empty,
}

#[derive(Clone, Serialize)]
pub struct ParsedTypeName {
    pub name: Rc<String>,
    pub position: PositionRange,
}

#[derive(Clone, Serialize)]
pub struct ParsedPointerType {
    pub pointee: Rc<ParsedType>
}

#[derive(Clone, Serialize)]
pub struct ParsedFunctionType {
    pub arg_types: Rc<Vec<ParsedType>>,
    pub ret_type: Rc<ParsedType>
}