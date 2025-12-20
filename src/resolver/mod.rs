mod type_resolver;
mod var_resolver;

use serde::Serialize;
pub use type_resolver::TypeResolver;
pub use var_resolver::VarResolver;

use std::{collections::HashMap, rc::{Rc, Weak}};
use crate::{ast::{DeclarationId, ParsedType, VarId}, impl_positioned, lexer::token::PositionRange};

#[derive(Serialize, PartialEq, Eq, Debug, Clone)]
pub struct FunctionType {
    pub param_types: Vec<ParsedType>,
    pub return_type: ParsedType,
}

pub struct SymbolTable {
    types: HashMap<String, ResolvedType>,
    impls: HashMap<String, Vec<FunctionType>>,
    functions: HashMap<String, FunctionType>,
    variables: HashMap<VarId, DeclarationId>,
    declaration_types: HashMap<DeclarationId, ParsedType>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
            impls: HashMap::new(),
            functions: HashMap::new(),
            variables: HashMap::new(),
            declaration_types: HashMap::new(),
        }
    }

    pub fn resolve_type(&self, parsed_type: &ParsedType) -> Option<ResolvedType> {
        match &parsed_type.parsed_type {
            crate::ast::ParsedTypeEnum::Integer => Some(ResolvedType::Integer),
            crate::ast::ParsedTypeEnum::Double => Some(ResolvedType::Double),
            crate::ast::ParsedTypeEnum::Boolean => Some(ResolvedType::Boolean),
            crate::ast::ParsedTypeEnum::Char => Some(ResolvedType::Char),
            crate::ast::ParsedTypeEnum::TypeName(name) => self.types.get(name.as_ref()).cloned(),
            crate::ast::ParsedTypeEnum::Pointer(inner) => {
                self.resolve_type(inner).map(|t| ResolvedType::Pointer(Rc::new(t)))
            }
            crate::ast::ParsedTypeEnum::Reference(inner) => {
                self.resolve_type(inner).map(|t| ResolvedType::Reference(Rc::new(t)))
            }
            crate::ast::ParsedTypeEnum::Array(inner) => {
                self.resolve_type(inner).map(|t| ResolvedType::Array(Rc::new(t)))
            }
        }
    }

    pub fn has_type(&self, type_name: &str) -> bool {
        self.types.contains_key(type_name)
    }

    pub fn get_type(&self, type_name: &str) -> Option<&ResolvedType> {
        self.types.get(type_name)
    }

    pub fn get_variable(&self, var_id: &VarId) -> Option<&DeclarationId> {
        self.variables.get(var_id)
    }

    pub fn get_function(&self, name: &str) -> Option<&FunctionType> {
        self.functions.get(name)
    }

    pub fn insert_function(&mut self, name: String, function: FunctionType) {
        self.functions.insert(name, function);
    }
    
    pub fn insert_impl(&mut self, impl_name: String, implementation: FunctionType) {
        self.impls.entry(impl_name).or_insert_with(Vec::new).push(implementation);
    }

    pub fn insert_declaration_type(&mut self, decl_id: DeclarationId, parsed_type: ParsedType) {
        self.declaration_types.insert(decl_id, parsed_type);
    }

    pub fn get_declaration_type(&self, decl_id: &DeclarationId) -> Option<&ParsedType> {
        self.declaration_types.get(decl_id)git 
    }

    pub fn insert_type(&mut self, type_name: String, base_type: ResolvedType) {
        self.types.insert(type_name, base_type);
    }

    pub fn resolve_var(&mut self, var_id: VarId, decl_id: DeclarationId) {
        self.variables.insert(var_id, decl_id);
    }
}

/*

struct A {
    B *b;
}

A has weak reference to B

struct B {
    A *a;
}

B has weak reference to A

*/

#[derive(Serialize, Clone, Debug)]
pub enum ResolvedType {
    Integer, Double, Boolean, Char, Void,
    Struct(Rc<StructType>),
    WeakStruct(Weak<StructType>),
    Pointer(Rc<ResolvedType>),
    Reference(Rc<ResolvedType>),
    Array(Rc<ResolvedType>),
    Function(Rc<FunctionType>),
}

impl PartialEq for ResolvedType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ResolvedType::Integer, ResolvedType::Integer) => true,
            (ResolvedType::Double, ResolvedType::Double) => true,
            (ResolvedType::Boolean, ResolvedType::Boolean) => true,
            (ResolvedType::Char, ResolvedType::Char) => true,
            (ResolvedType::Void, ResolvedType::Void) => true,
            (ResolvedType::Struct(a), ResolvedType::Struct(b)) => a == b,
            (ResolvedType::WeakStruct(a), ResolvedType::WeakStruct(b)) => {
                match (a.upgrade(), b.upgrade()) {
                    (Some(a_rc), Some(b_rc)) => a_rc == b_rc,
                    (None, None) => false, // Both have been dropped, consider them not equal
                    _ => false,
                }
            }
            (ResolvedType::Pointer(a), ResolvedType::Pointer(b)) => a == b,
            (ResolvedType::Reference(a), ResolvedType::Reference(b)) => a == b,
            (ResolvedType::Array(a), ResolvedType::Array(b)) => a == b,
            (ResolvedType::Function(a), ResolvedType::Function(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for ResolvedType {}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct StructType {
    pub name: String,
    pub members: HashMap<String, ResolvedType>,
}