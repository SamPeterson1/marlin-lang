use core::fmt;
use std::{collections::HashMap, rc::Rc};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ParsedType {
    Integer, Double, 
    Boolean, String,
    TypeName(String),
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
            ParsedType::String => write!(f, "{{\"type\": \"String\"}}"),
            ParsedType::TypeName(name) => write!(f, "{{\"type\": \"TypeName\", \"name\": \"{}\"}}", name),
            ParsedType::Function(func) => write!(f, "{{\"type\": \"Function\", \"function\": {}}}", func),
            ParsedType::Pointer(ptr) => write!(f, "{{\"type\": \"Pointer\", \"pointer\": {}}}", ptr),
            ParsedType::Empty => write!(f, "{{\"type\": \"Empty\"}}")
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ParsedPointerType {
    pub pointee: Rc<ParsedType>
}

impl fmt::Display for ParsedPointerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"pointee\": {}}}", self.pointee)
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
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

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ResolvedType {
    Integer, Double, 
    Boolean, String,
    Function(FunctionType),
    Struct(StructType),
    Pointer(PointerType),
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
            ResolvedType::Pointer(_) => 8,
            ResolvedType::Empty => 0
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PointerType {
    pub pointee: Rc<ResolvedType>
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StructType {
    size: usize,
    member_offsets: Rc<HashMap<String, usize>>,
    member_types: Rc<HashMap<String, ResolvedType>>,
}

impl StructType {
    pub fn new(members: Vec<(String, ResolvedType)>) -> StructType {
        let mut member_offsets = HashMap::new();
        let mut member_types = HashMap::new();
        let mut offset = 0;

        for (name, t) in members {
            member_offsets.insert(name.clone(), offset);
            offset += t.n_bytes() / 8;
            member_types.insert(name.clone(), t);
        }

        StructType {
            size: offset,
            member_offsets: Rc::new(member_offsets),
            member_types: Rc::new(member_types),
        }
    }

    pub fn get_member_type(&self, member: &str) -> &ResolvedType {
        self.member_types.get(member).unwrap()
    }

    pub fn get_member_offset(&self, member: &str) -> usize {
        *self.member_offsets.get(member).unwrap()
    }

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

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Int(i) => write!(f, "{{\"type\": \"Integer\", \"value\": {}}}", i),
            Literal::Double(d) => write!(f, "{{\"type\": \"Double\", \"value\": {}}}", d),
            Literal::Bool(b) => write!(f, "{{\"type\": \"Boolean\", \"value\": {}}}", b),
            Literal::String(s) => write!(f, "{{\"type\": \"String\", \"value\": \"{}\"}}", s),
        }
    }
}