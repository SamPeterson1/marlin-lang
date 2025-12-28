mod type_resolver;
mod var_resolver;

use serde::Serialize;
pub use type_resolver::TypeResolver;
pub use var_resolver::VarResolver;

use std::{array, collections::{HashMap, HashSet}, hash::Hash, sync::{Condvar, Mutex, RwLock}};
use crate::ast::{AstId, ParsedType, ParsedTypeEnum};

pub struct GlobalSymbolTable {
    scopes: HashMap<Vec<String>, Mutex<SymbolTable>>,
    type_arena: Mutex<TypeArena>,
}

pub struct SymbolTable {
    pub types: HashMap<String, TypeId>,
    pub functions: HashMap<String, TypeId>,
    pub ast_types: HashMap<AstId, TypeId>,
    pub declaration_types: HashMap<AstId, TypeId>,
    pub variables: HashMap<AstId, AstId>,
}

impl SymbolTable {
    pub fn new() -> Self {        
        Self {
            types: HashMap::new(),
            functions: HashMap::new(),
            ast_types: HashMap::new(),
            declaration_types: HashMap::new(),
            variables: HashMap::new(),
        }
    }

    pub fn resolve_type(&mut self, type_arena: &mut TypeArena, parsed_type: &ParsedType) -> Option<TypeId> {
        Some(match parsed_type.parsed_type {
            ParsedTypeEnum::Void =>type_arena.void(),
            ParsedTypeEnum::Boolean => type_arena.bool(),
            ParsedTypeEnum::Char => type_arena.char(),
            ParsedTypeEnum::Integer => type_arena.int(),
            ParsedTypeEnum::Double => type_arena.double(),
            ParsedTypeEnum::Array(ref array_type) => {
                let array_type_id = self.resolve_type(type_arena, array_type)?;
                type_arena.make_array(array_type_id)
            },
            ParsedTypeEnum::Pointer(ref ptr_type) => {
                let ptr_type_id = self.resolve_type(type_arena, ptr_type)?;
                type_arena.make_ptr(ptr_type_id)
            },
            ParsedTypeEnum::Reference(ref ref_type) => {
                let ref_type_id = self.resolve_type(type_arena, ref_type)?;
                type_arena.make_ref(ref_type_id)
            },
            ParsedTypeEnum::TypeName(ref type_name) => {
                *self.types.get(type_name)?
            }
        })
    }
}

pub struct GlobalTypeArena {
    partial_arenas: Vec<TypeArena>
}

pub struct TypeArena {
    types: Vec<Option<ResolvedType>>,

    ref_ids: HashMap<TypeId, TypeId>,
    ptr_ids: HashMap<TypeId, TypeId>,
    array_ids: HashMap<TypeId, TypeId>,
    function_ids: HashMap<FunctionType, TypeId>,

    int_type_id: Option<TypeId>,
    double_type_id: Option<TypeId>,
    bool_type_id: Option<TypeId>,
    char_type_id: Option<TypeId>,
    void_type_id: Option<TypeId>,
}

impl TypeArena {
    pub fn new() -> Self {
        Self {
            types: Vec::new(),
            ref_ids: HashMap::new(),
            ptr_ids: HashMap::new(),
            array_ids: HashMap::new(),
            function_ids: HashMap::new(),
            int_type_id: None,
            double_type_id: None,
            bool_type_id: None,
            char_type_id: None,
            void_type_id: None
        }
    }

    pub fn int(&mut self) -> TypeId {
        match self.int_type_id {
            Some(id) => id,
            None => {
                self.int_type_id = Some(self.insert(ResolvedType::Integer));
                self.int_type_id.unwrap()
            }
        }
    }

    pub fn double(&mut self) -> TypeId {
        match self.double_type_id {
            Some(id) => id,
            None => {
                self.double_type_id = Some(self.insert(ResolvedType::Double));
                self.double_type_id.unwrap()
            }
        }
    }

    pub fn bool(&mut self) -> TypeId {
        match self.bool_type_id {
            Some(id) => id,
            None => {
                self.bool_type_id = Some(self.insert(ResolvedType::Boolean));
                self.bool_type_id.unwrap()
            }
        }
    }

    pub fn char(&mut self) -> TypeId {
        match self.char_type_id {
            Some(id) => id,
            None => {
                self.char_type_id = Some(self.insert(ResolvedType::Char));
                self.char_type_id.unwrap()
            }
        }
    }

    pub fn void(&mut self) -> TypeId {
        match self.void_type_id {
            Some(id) => id,
            None => {
                self.void_type_id = Some(self.insert(ResolvedType::Void));
                self.void_type_id.unwrap()
            }
        }
    }

    pub fn make_ref(&mut self, type_id: TypeId) -> TypeId {
        if let Some(ref_id) = self.ref_ids.get(&type_id) {
            return *ref_id;
        }

        let ref_id = self.insert(ResolvedType::Reference(type_id));
        self.ref_ids.insert(type_id, ref_id);
        ref_id
    }

    pub fn make_ptr(&mut self, type_id: TypeId) -> TypeId {
        if let Some(ptr_id) = self.ptr_ids.get(&type_id) {
            return *ptr_id;
        }

        let ptr_id = self.insert(ResolvedType::Pointer(type_id));
        self.ptr_ids.insert(type_id, ptr_id);
        ptr_id
    }

    pub fn make_array(&mut self, type_id: TypeId) -> TypeId {
        if let Some(array_id) = self.array_ids.get(&type_id) {
            return *array_id;
        }

        let array_id = self.insert(ResolvedType::Array(type_id));
        self.array_ids.insert(type_id, array_id);
        array_id
    }

    pub fn make_function(&mut self, function_type: FunctionType) -> TypeId {
        if let Some(function_id) = self.function_ids.get(&function_type) {
            return *function_id;
        }

        let function_id = self.insert(ResolvedType::Function(function_type.clone()));
        self.function_ids.insert(function_type, function_id);
        function_id
    }

    pub fn reserve(&mut self) -> TypeId {
        let type_id = TypeId(self.types.len());
        self.types.push(None);
        type_id
    }

    pub fn set_type(&mut self, type_id: &TypeId, resolved_type: ResolvedType) {
        self.types[type_id.0] = Some(resolved_type);
    }

    pub fn insert(&mut self, resolved_type: ResolvedType) -> TypeId {
        let type_id = TypeId(self.types.len());
        self.types.push(Some(resolved_type));
        type_id
    }

    pub fn get(&self, type_id: &TypeId) -> &ResolvedType {
        self.types.get(type_id.0).as_ref().unwrap().as_ref().unwrap()
    }
}

#[derive(Serialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TypeId(usize);

#[derive(Serialize, Debug)]
pub enum ResolvedType {
    Integer, Double, Boolean, Char, Void,
    Struct(StructType),
    Pointer(TypeId),
    Reference(TypeId),
    Array(TypeId),
    Function(FunctionType),
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
    pub members: HashMap<String, TypeId>,
    pub constructors: HashSet<TypeId>,
}


#[derive(Serialize, PartialEq, Eq, Debug, Clone, Hash)]
pub struct FunctionType {
    pub param_types: Vec<TypeId>,
    pub return_type: TypeId,
}