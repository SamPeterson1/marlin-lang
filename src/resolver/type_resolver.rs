use std::{collections::{HashMap, HashSet}, sync::MutexGuard};

use crate::{ast::*, diagnostic::{Diagnostic, ErrMsg}, lexer::token::{Located, PositionRange, Positioned}, logger::{Log, LogTarget}, resolver::{FunctionType, GlobalSymbolTable, ResolvedType, StructType, SymbolTable, TypeArena, TypeId}};

pub struct TypeResolver<'ctx> {
    log_target: &'ctx dyn LogTarget,
    global_table: &'ctx GlobalSymbolTable,
    symbol_table: &'ctx mut SymbolTable,
    diagnostics: &'ctx mut Vec<Diagnostic>,
    unresolved_types: HashMap<String, (TypeId, PositionRange)>,
    partial_structs: HashMap<String, (StructType, TypeId)>,
    impl_blocks: HashMap<String, Vec<&'ctx ImplItem>>,
}

impl Log for TypeResolver<'_> {
    fn get_source(&self) -> String {
        return "TypeResolver".to_string()
    }
}

impl<'ctx> TypeResolver<'ctx> {
    pub fn new(log_target: &'ctx dyn LogTarget, global_table: &'ctx GlobalSymbolTable, symbol_table: &'ctx mut SymbolTable, diagnostics: &'ctx mut Vec<Diagnostic>) -> Self {
        Self { 
            log_target,
            global_table,
            symbol_table,
            diagnostics,
            unresolved_types: HashMap::new(),
            impl_blocks: HashMap::new(),
            partial_structs: HashMap::new(),
        }
    }

    pub fn resolve(mut self, scope: &'ctx Scope) {
        scope.accept_visitor(&mut self);
 
        let partial_structs = std::mem::take(&mut self.partial_structs);
        

        for (struct_name, (mut struct_type, struct_type_id)) in partial_structs {
            if let Some(impl_blocks) = self.impl_blocks.remove(&struct_name) {
                for impl_block in impl_blocks {
                    for function in &impl_block.functions {
                        let function_type = self.get_fn_type(function);
                        let function_type_id = self.global_table.type_arena.make_function(function_type);
                        struct_type.members.insert(function.name.data.clone(), function_type_id);
                    }
                }
            }

            self.global_table.type_arena.set_type(&struct_type_id, ResolvedType::Struct(struct_type));
            self.unresolved_types.remove(&struct_name);
            self.symbol_table.types.insert(struct_name, struct_type_id);
        }
        
        for (type_name, (_, position)) in self.unresolved_types {
            self.diagnostics.push(ErrMsg::UnknownTypeName(type_name).make_diagnostic(position));
        }
    }

    pub fn resolve_type(&mut self, parsed_type: &ParsedType) -> TypeId {
        

        match &parsed_type.parsed_type {
            ParsedTypeEnum::Void => self.global_table.type_arena.void(),
            ParsedTypeEnum::Integer => self.global_table.type_arena.int(),
            ParsedTypeEnum::Double => self.global_table.type_arena.double(),
            ParsedTypeEnum::Boolean => self.global_table.type_arena.bool(),
            ParsedTypeEnum::Char => self.global_table.type_arena.char(),
            ParsedTypeEnum::TypeName(name) => {
                match self.symbol_table.types.get(name) {
                    Some(type_id) => *type_id,
                    None => {
                        self.unresolved_types.entry(name.clone())
                            .or_insert_with(|| (self.global_table.type_arena.reserve(), *parsed_type.get_position())).0
                    }
                }
            }
            ParsedTypeEnum::Pointer(inner) => {
                let base_type = self.resolve_type(inner.as_ref());
                self.global_table.type_arena.make_ptr(base_type)
            }
            ParsedTypeEnum::Reference(inner) => {
                let base_type = self.resolve_type(inner.as_ref());
                self.global_table.type_arena.make_ref(base_type)
            }
            ParsedTypeEnum::Array(inner) => {
                let base_type = self.resolve_type(inner.as_ref());
                self.global_table.type_arena.make_array(base_type)
            }
        }
    }

    fn get_fn_type(&mut self, function: &FunctionItem) -> FunctionType {
        let mut param_types = Vec::new();

        for parameter in &function.parameters {
            param_types.push(self.resolve_type(&parameter.declaration_type));
        }

        FunctionType {
            param_types,
            return_type: self.resolve_type(&function.return_type),
        }
    }
}

impl<'ast> ASTVisitor<'ast, ()> for TypeResolver<'ast> {
    fn visit_impl(&mut self, node: &'ast ImplItem) { 
        self.impl_blocks.entry(node.identifier.data.clone())
            .or_insert_with(|| Vec::new())
            .push(node);
    }

    fn visit_function(&mut self, node: &FunctionItem) { 
        

        let fn_type = self.get_fn_type(node);
        let fn_type_id = self.global_table.type_arena.make_function(fn_type);
        self.symbol_table.functions.insert(node.name.data.to_string(), fn_type_id);
    }

    fn visit_struct(&mut self, node: &'ast StructItem) {
        
        let mut members = HashMap::new();

        for (member_type, Located {data: member_name, ..} ) in &node.members {
            let member_type_id = self.resolve_type(member_type);
            members.insert(member_name.clone(), member_type_id);
        }

        let mut constructors = HashSet::new();
        let struct_type_id = self.global_table.type_arena.reserve();

        for constructor in &node.constructors {
            let mut param_types = Vec::new();

            for parameter in &constructor.parameters {
                param_types.push(self.resolve_type(&parameter.declaration_type));
            }

            let constructor_type = FunctionType {
                param_types,
                return_type: struct_type_id,
            };

            let constructor_type_id = self.global_table.type_arena.make_function(constructor_type);
            constructors.insert(constructor_type_id);
        }

        let struct_type = StructType {
            name: node.name.data.clone(),
            members,
            constructors
        };

        self.partial_structs.insert(node.name.data.clone(), (struct_type, struct_type_id));
    }

    fn visit_scope(&mut self, node: &'ast Scope) -> () {
        for item in &node.items {
            item.accept_visitor(self);
        }
    }
}

#[cfg(test)]
mod tests {
    
}