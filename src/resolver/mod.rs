mod type_resolver;
mod var_resolver;

use serde::Serialize;
pub use type_resolver::TypeResolver;
pub use var_resolver::VarResolver;

use std::collections::HashMap;
use crate::ast::{DeclarationId, ParsedType, VarId};

pub struct FunctionType {
    pub param_types: Vec<ParsedType>,
    pub return_type: ParsedType,
}

pub struct SymbolTable {
    types: HashMap<String, ResolvedBaseType>,
    impls: HashMap<String, Vec<FunctionType>>,
    functions: Vec<FunctionType>,
    variables: HashMap<VarId, DeclarationId>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
            impls: HashMap::new(),
            functions: Vec::new(),
            variables: HashMap::new(),
        }
    }

    pub fn has_type(&self, type_name: &str) -> bool {
        self.types.contains_key(type_name)
    }

    pub fn get_type(&self, type_name: &str) -> Option<&ResolvedBaseType> {
        self.types.get(type_name)
    }

    pub fn get_variable(&self, var_id: &VarId) -> Option<&DeclarationId> {
        self.variables.get(var_id)
    }

    pub fn insert_function(&mut self, function: FunctionType) {
        self.functions.push(function);
    }
    
    pub fn insert_impl(&mut self, impl_name: String, implementation: FunctionType) {
        self.impls.entry(impl_name).or_insert_with(Vec::new).push(implementation);
    }

    pub fn insert_type(&mut self, type_name: String, base_type: ResolvedBaseType) {
        self.types.insert(type_name, base_type);
    }

    pub fn resolve_var(&mut self, var_id: VarId, decl_id: DeclarationId) {
        self.variables.insert(var_id, decl_id);
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