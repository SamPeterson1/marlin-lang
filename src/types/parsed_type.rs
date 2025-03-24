use std::{fmt, rc::Rc};

use crate::token::PositionRange;

/* Unresolved type parsed from tokens */
#[derive(Clone)]
pub enum ParsedType {
    Integer, Double, Boolean,
    TypeName(ParsedTypeName),
    Function(ParsedFunctionType),
    Pointer(ParsedPointerType),
    Empty,
}

impl fmt::Display for ParsedType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParsedType::Integer => write!(f, "{{\"type\": \"Integer\"}}"),
            ParsedType::Double => write!(f, "{{\"type\": \"Double\"}}"),
            ParsedType::Boolean => write!(f, "{{\"type\": \"Boolean\"}}"),
            ParsedType::TypeName(name) => write!(f, "{{\"type\": \"TypeName\", \"name\": \"{}\"}}", name),
            ParsedType::Function(func) => write!(f, "{{\"type\": \"Function\", \"function\": {}}}", func),
            ParsedType::Pointer(ptr) => write!(f, "{{\"type\": \"Pointer\", \"pointer\": {}}}", ptr),
            ParsedType::Empty => write!(f, "{{\"type\": \"Empty\"}}")
        }
    }
}

#[derive(Clone)]
pub struct ParsedTypeName {
    pub name: Rc<String>,
    pub position: PositionRange,
}

impl fmt::Display for ParsedTypeName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"name\": \"{}\", \"position\": \"{}\"}}", self.name, self.position)
    }
}

#[derive(Clone)]
pub struct ParsedPointerType {
    pub pointee: Rc<ParsedType>
}

impl fmt::Display for ParsedPointerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"pointee\": {}}}", self.pointee)
    }
}

#[derive(Clone)]
pub struct ParsedFunctionType {
    pub arg_types: Rc<Vec<ParsedType>>,
    pub ret_type: Rc<ParsedType>
}

impl fmt::Display for ParsedFunctionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"arg_types\": [")?;

        for arg in self.arg_types.iter() {
            write!(f, "{},", arg)?;
        }

        write!(f, "], \"ret_type\": {}}}", self.ret_type)
    }
}