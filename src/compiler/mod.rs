pub mod local_resolver;
pub mod visit;

use std::{collections::{HashMap, HashSet}, sync::{MappedRwLockReadGuard, RwLock, RwLockReadGuard}};

use dashmap::{DashMap, DashSet};
use serde::Serialize;

use crate::{ast::{AstId, ParsedType, ParsedTypeEnum, Scope}, diagnostic::Diagnostic};

pub struct Compiler<'ast> {
    pub type_arena: TypeArena,
    pub symbol_tables: HashMap<&'ast[String], SymbolTable<'ast>>,
    pub diagnostics: Vec<Diagnostic>,
}

impl<'ast> Compiler<'ast> {
    pub fn new(scopes: impl Iterator<Item = &'ast Scope>) -> Self {
        let mut symbol_tables = HashMap::new();

        for scope in scopes {
            symbol_tables.insert(scope.path.segments.as_slice(), SymbolTable::new());
        }

        Self {
            type_arena: TypeArena::new(),
            symbol_tables,
            diagnostics: Vec::new(),
        }
    }
}

pub struct SymbolTable<'ast> {
    pub types: DashMap<String, TypeId>,
    pub function_names: DashSet<&'ast String>,
    pub functions: DashMap<String, TypeId>,
    pub ast_types: DashMap<AstId, TypeId>,
    pub declaration_types: DashMap<AstId, TypeId>,
    pub variables: DashMap<AstId, AstId>,
}

impl SymbolTable<'_> {
    pub fn new() -> Self {        
        Self {
            types: DashMap::new(),
            function_names: DashSet::new(),
            functions: DashMap::new(),
            ast_types: DashMap::new(),
            declaration_types: DashMap::new(),
            variables: DashMap::new(),
        }
    }

    pub fn resolve_type(&self, type_arena: &TypeArena, parsed_type: &ParsedType) -> Option<TypeId> {
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

pub struct TypeArena {
    types: RwLock<Vec<Option<ResolvedType>>>,

    ref_ids: DashMap<TypeId, TypeId>,
    ptr_ids: DashMap<TypeId, TypeId>,
    array_ids: DashMap<TypeId, TypeId>,
    function_ids: DashMap<FunctionType, TypeId>,

    int_type_id: TypeId,
    double_type_id: TypeId,
    bool_type_id: TypeId,
    char_type_id: TypeId,
    void_type_id: TypeId,
}

impl TypeArena {
    pub fn new() -> Self {
        let mut types = Vec::new();

        types.push(Some(ResolvedType::Integer));
        types.push(Some(ResolvedType::Double));
        types.push(Some(ResolvedType::Boolean));
        types.push(Some(ResolvedType::Char));
        types.push(Some(ResolvedType::Void));

        Self {
            types: RwLock::new(types),
            ref_ids: DashMap::new(),
            ptr_ids: DashMap::new(),
            array_ids: DashMap::new(),
            function_ids: DashMap::new(),
            int_type_id: TypeId(0),
            double_type_id: TypeId(1),
            bool_type_id: TypeId(2),
            char_type_id: TypeId(3),
            void_type_id: TypeId(4),
        }
    }

    pub fn int(&self) -> TypeId {
        return self.int_type_id;
    }

    pub fn double(&self) -> TypeId {
        return self.double_type_id;
    }

    pub fn bool(&self) -> TypeId {
        return self.bool_type_id;
    }

    pub fn char(&self) -> TypeId {
        return self.char_type_id;
    }

    pub fn void(&self) -> TypeId {
        return self.void_type_id;
    }

    pub fn make_ref(&self, type_id: TypeId) -> TypeId {
        if let Some(ref_id) = self.ref_ids.get(&type_id) {
            return *ref_id;
        }

        let ref_id = self.insert(ResolvedType::Reference(type_id));
        self.ref_ids.insert(type_id, ref_id);
        ref_id
    }

    pub fn make_ptr(&self, type_id: TypeId) -> TypeId {
        if let Some(ptr_id) = self.ptr_ids.get(&type_id) {
            return *ptr_id;
        }

        let ptr_id = self.insert(ResolvedType::Pointer(type_id));
        self.ptr_ids.insert(type_id, ptr_id);
        ptr_id
    }

    pub fn make_array(&self, type_id: TypeId) -> TypeId {
        if let Some(array_id) = self.array_ids.get(&type_id) {
            return *array_id;
        }

        let array_id = self.insert(ResolvedType::Array(type_id));
        self.array_ids.insert(type_id, array_id);
        array_id
    }

    pub fn make_function(&self, function_type: FunctionType) -> TypeId {
        if let Some(function_id) = self.function_ids.get(&function_type) {
            return *function_id;
        }

        let function_id = self.insert(ResolvedType::Function(function_type.clone()));
        self.function_ids.insert(function_type, function_id);
        function_id
    }

    pub fn reserve(&self) -> TypeId {
        let mut types_lock = self.types.write().unwrap();
        let type_id = TypeId(types_lock.len());
        types_lock.push(None);
        type_id
    }

    pub fn set_type(&self, type_id: &TypeId, resolved_type: ResolvedType) {
        self.types.write().unwrap()[type_id.0] = Some(resolved_type);
    }

    pub fn insert(&self, resolved_type: ResolvedType) -> TypeId {
        let mut types_lock = self.types.write().unwrap();
        let type_id = TypeId(types_lock.len());
        types_lock.push(Some(resolved_type));
        type_id
    }

    pub fn get(&self, type_id: TypeId) -> MappedRwLockReadGuard<'_, ResolvedType> {
        let types_lock = self.types.read().unwrap();
        RwLockReadGuard::map(types_lock, |types| {
            types[type_id.0].as_ref().unwrap()
        })
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

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
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