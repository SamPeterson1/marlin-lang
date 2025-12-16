mod type_resolver;
mod var_resolver;

use serde::Serialize;
pub use type_resolver::TypeResolver;

use std::collections::HashMap;
use crate::ast::{ArrayModifier, DeclarationId, ParsedType, VarId};

pub struct SymbolTable {
    types: HashMap<String, ResolvedBaseType>,
    variables: HashMap<VarId, DeclarationId>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
            variables: HashMap::new(),
        }
    }

    pub fn has_type(&self, type_name: &str) -> bool {
        self.types.contains_key(type_name)
    }

    pub fn get_type(&self, type_name: &str) -> Option<&ResolvedBaseType> {
        self.types.get(type_name)
    }

    pub fn insert_type(&mut self, type_name: String, base_type: ResolvedBaseType) {
        self.types.insert(type_name, base_type);
    }
}

#[derive(Clone, Serialize)]
pub enum ResolvedBaseType {
    Integer, Double, Boolean, Char,
    Struct(StructType)
}

#[derive(Clone, Serialize)]
pub struct StructType {
    pub members: HashMap<String, ParsedType>,
}