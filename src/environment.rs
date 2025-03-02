use std::{collections::HashMap, rc::Rc};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ParsedType {
    Integer, Double, 
    Boolean, String,
    TypeName(String),
    Function(ParsedFunctionType),
    Empty,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ParsedFunctionType {
    pub arg_types: Rc<Vec<ParsedType>>,
    pub ret_type: Rc<ParsedType>
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ResolvedType {
    Integer, Double, 
    Boolean, String,
    Function(FunctionType),
    Struct(StructType),
    Empty,
}

impl ResolvedType {
    pub fn is_numeric(&self) -> bool {
        *self == ResolvedType::Integer || *self == ResolvedType::Double
    }

    pub fn n_bytes(&self) -> usize {
        match self {
            ResolvedType::Integer => 8,
            ResolvedType::Double => 8,
            ResolvedType::Boolean => 8,
            ResolvedType::String => 8,
            ResolvedType::Function(_) => 8,
            ResolvedType::Struct(struct_type) => struct_type.n_bytes(),
            ResolvedType::Empty => 0
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StructType {
    pub member_types: Rc<HashMap<String, ResolvedType>>
}

impl StructType {
    pub fn n_bytes(&self) -> usize {
        self.member_types.iter().fold(0, |acc, (_, t)| acc + t.n_bytes())
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FunctionType {
    pub arg_types: Rc<Vec<ResolvedType>>,
    pub ret_type: Rc<ResolvedType>
}

#[derive(Debug)]
pub enum Literal {
    Int (i64),
    Double (f64),
    Bool (bool),
    String (String),
}