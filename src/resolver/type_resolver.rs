use std::{collections::{HashMap, HashSet}, hash::Hash};

use crate::{ast::*, diagnostic::{Diagnostic, ErrMsg}, lexer::token::{Located, PositionRange, Positioned}, logger::Log, resolver::{FunctionType, ResolvedType, StructType, SymbolTable, TypeId}};

pub struct TypeResolver<'ctx> {
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
    pub fn new(symbol_table: &'ctx mut SymbolTable, diagnostics: &'ctx mut Vec<Diagnostic>) -> Self {
        Self { 
            symbol_table,
            diagnostics,
            unresolved_types: HashMap::new(),
            impl_blocks: HashMap::new(),
            partial_structs: HashMap::new()
        }
    }

    pub fn resolve(mut self, program: &'ctx Program) {
        program.accept_visitor(&mut self);
 
        let partial_structs = std::mem::take(&mut self.partial_structs);

        for (struct_name, (mut struct_type, struct_type_id)) in partial_structs {
            if let Some(impl_blocks) = self.impl_blocks.remove(&struct_name) {
                for impl_block in impl_blocks {
                    for function in &impl_block.functions {
                        let function_type = self.get_fn_type(function);
                        let function_type_id = self.symbol_table.type_arena.make_function(function_type);
                        struct_type.members.insert(function.name.data.clone(), function_type_id);
                    }
                }
            }

            self.symbol_table.type_arena.set_type(&struct_type_id, ResolvedType::Struct(struct_type));
            self.unresolved_types.remove(&struct_name);
            self.symbol_table.types.insert(struct_name, struct_type_id);
        }

        let getchar_type = FunctionType {
            param_types: vec![],
            return_type: self.symbol_table.type_arena.char(),
        };
        let getchar_type_id = self.symbol_table.type_arena.make_function(getchar_type);

        self.symbol_table.functions.insert("getchar".to_string(), getchar_type_id);

        let putchar_type = FunctionType {
            param_types: vec![self.symbol_table.type_arena.char()],
            return_type: self.symbol_table.type_arena.int(),
        };
        let putchar_type_id = self.symbol_table.type_arena.make_function(putchar_type);

        self.symbol_table.functions.insert("putchar".to_string(), putchar_type_id);

        for (type_name, (_, position)) in self.unresolved_types {
            self.diagnostics.push(ErrMsg::UnknownTypeName(type_name).make_diagnostic(position));
        }
    }

    pub fn resolve_type(&mut self, parsed_type: &ParsedType) -> TypeId {
        match &parsed_type.parsed_type {
            ParsedTypeEnum::Void => self.symbol_table.type_arena.void(),
            ParsedTypeEnum::Integer => self.symbol_table.type_arena.int(),
            ParsedTypeEnum::Double => self.symbol_table.type_arena.double(),
            ParsedTypeEnum::Boolean => self.symbol_table.type_arena.bool(),
            ParsedTypeEnum::Char => self.symbol_table.type_arena.char(),
            ParsedTypeEnum::TypeName(name) => {
                match self.symbol_table.types.get(name) {
                    Some(type_id) => *type_id,
                    None => {
                        self.unresolved_types.entry(name.clone())
                            .or_insert_with(|| (self.symbol_table.type_arena.reserve(), *parsed_type.get_position())).0
                    }
                }
            }
            ParsedTypeEnum::Pointer(inner) => {
                let base_type = self.resolve_type(inner.as_ref());
                self.symbol_table.type_arena.make_ptr(base_type)
            }
            ParsedTypeEnum::Reference(inner) => {
                let base_type = self.resolve_type(inner.as_ref());
                self.symbol_table.type_arena.make_ref(base_type)
            }
            ParsedTypeEnum::Array(inner) => {
                let base_type = self.resolve_type(inner.as_ref());
                self.symbol_table.type_arena.make_array(base_type)
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
        let fn_type_id = self.symbol_table.type_arena.make_function(fn_type);
        self.symbol_table.functions.insert(node.name.data.to_string(), fn_type_id);
    }

    fn visit_struct(&mut self, node: &'ast StructItem) {
        let mut members = HashMap::new();

        for (member_type, Located {data: member_name, ..} ) in &node.members {
            let member_type_id = self.resolve_type(member_type);
            members.insert(member_name.clone(), member_type_id);
        }

        let mut constructors = HashSet::new();
        let struct_type_id = self.symbol_table.type_arena.reserve();

        for constructor in &node.constructors {
            let mut param_types = Vec::new();

            for parameter in &constructor.parameters {
                param_types.push(self.resolve_type(&parameter.declaration_type));
            }

            let constructor_type = FunctionType {
                param_types,
                return_type: struct_type_id,
            };

            let constructor_type_id = self.symbol_table.type_arena.make_function(constructor_type);
            constructors.insert(constructor_type_id);
        }

        let struct_type = StructType {
            name: node.name.data.clone(),
            members,
            constructors
        };

        self.partial_structs.insert(node.name.data.clone(), (struct_type, struct_type_id));
    }

    fn visit_main(&mut self, _node: &MainItem) { }

    fn visit_program(&mut self, node: &'ast Program) -> () {
        for item in &node.items {
            item.accept_visitor(self);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::diagnostic::Diagnostic;
    use crate::lexer::Lexer;
    use crate::parser::ExprParser;
    use crate::resolver::{SymbolTable, TypeResolver};

    fn parse_and_resolve(source: &str) -> (SymbolTable, Vec<Diagnostic>) {
        let mut diagnostics = Vec::new();
        let lexer = Lexer::new(source, &mut diagnostics);
        let tokens = lexer.parse();
        
        let parser = ExprParser::new(tokens, &mut diagnostics);
        let program = parser.parse();
        
        let mut symbol_table = SymbolTable::new();
        
        let resolver = TypeResolver::new(&mut symbol_table, &mut diagnostics);
        resolver.resolve(&program);
        
        (symbol_table, diagnostics)
    }

    #[test]
    fn test_self_referential_structs() {
        let source = std::fs::read_to_string("src/tests/type_resolver/self_referential.mar")
            .expect("Failed to read test file");
        
        let (symbol_table, diagnostics) = parse_and_resolve(&source);
        
        // Should have no errors - self-referential structs are valid
        assert_eq!(diagnostics.len(), 0, "Self-referential structs should resolve without errors");
        
        // Check that structs are registered
        assert!(symbol_table.types.contains_key("Node"), "Node struct should be registered");
        assert!(symbol_table.types.contains_key("LinkedNode"), "LinkedNode struct should be registered");
    }

    #[test]
    fn test_circular_dependencies() {
        let source = std::fs::read_to_string("src/tests/type_resolver/circular.mar")
            .expect("Failed to read test file");
        
        let (symbol_table, diagnostics) = parse_and_resolve(&source);
        
        // Should have no errors - circular dependencies through pointers/references are valid
        assert_eq!(diagnostics.len(), 0, "Circular dependencies should resolve without errors");
        
        // Check all structs are registered
        assert!(symbol_table.types.contains_key("A"), "Struct A should be registered");
        assert!(symbol_table.types.contains_key("B"), "Struct B should be registered");
        assert!(symbol_table.types.contains_key("X"), "Struct X should be registered");
        assert!(symbol_table.types.contains_key("Y"), "Struct Y should be registered");
        assert!(symbol_table.types.contains_key("Z"), "Struct Z should be registered");
    }

    #[test]
    fn test_nested_pointer_types() {
        let source = std::fs::read_to_string("src/tests/type_resolver/nested_types.mar")
            .expect("Failed to read test file");
        
        let (symbol_table, diagnostics) = parse_and_resolve(&source);
        
        // Should have no errors - just registering the struct definition
        assert_eq!(diagnostics.len(), 0, "Nested types should resolve without errors");
        
        assert!(symbol_table.types.contains_key("Container"), "Container struct should be registered");
    }

    #[test]
    fn test_unknown_types_in_struct_fields() {
        let source = std::fs::read_to_string("src/tests/type_resolver/unknown_types.mar")
            .expect("Failed to read test file");
        
        let (symbol_table, diagnostics) = parse_and_resolve(&source);
        
        // Type resolver catches unknown types in struct fields
        assert!(diagnostics.len() >= 3, "Unknown types in struct fields should generate errors");
        
        let diagnostic_messages: Vec<String> = diagnostics.iter()
            .map(|d| d.message.clone())
            .collect();
        
        // These are unknown types used in struct fields
        let unknown_in_structs = vec!["UnknownType", "NonExistent", "MissingType"];
        
        for unknown_type in unknown_in_structs {
            assert!(diagnostic_messages.iter().any(|msg| msg.contains(unknown_type)),
                    "Should have error for unknown type in struct field: {}", unknown_type);
        }
        
        // Structs should still be registered even if they have invalid fields
        assert!(symbol_table.types.contains_key("BadStruct"), "BadStruct should be registered");
        assert!(symbol_table.types.contains_key("BadPointer"), "BadPointer should be registered");
        assert!(symbol_table.types.contains_key("BadReference"), "BadReference should be registered");
    }

    #[test]
    fn test_forward_references() {
        let source = std::fs::read_to_string("src/tests/type_resolver/forward_references.mar")
            .expect("Failed to read test file");
        
        let (symbol_table, diagnostics) = parse_and_resolve(&source);
        
        // Forward references should work - type resolver processes all structs first
        assert_eq!(diagnostics.len(), 0, "Forward references should resolve without errors");
        
        assert!(symbol_table.types.contains_key("A"), "Struct A should be registered");
        assert!(symbol_table.types.contains_key("B"), "Struct B should be registered");
        assert!(symbol_table.types.contains_key("C"), "Struct C should be registered");
    }

    #[test]
    fn test_array_types() {
        let source = std::fs::read_to_string("src/tests/type_resolver/array_types.mar")
            .expect("Failed to read test file");
        
        let (symbol_table, diagnostics) = parse_and_resolve(&source);
        
        // Should have no errors - just registering struct definitions
        assert_eq!(diagnostics.len(), 0, "Array types should resolve without errors");
        
        assert!(symbol_table.types.contains_key("Point"), "Point struct should be registered");
    }

    #[test]
    fn test_function_parameter_types() {
        let source = std::fs::read_to_string("src/tests/type_resolver/function_types.mar")
            .expect("Failed to read test file");
        
        let (symbol_table, diagnostics) = parse_and_resolve(&source);
        
        // Type resolver catches unknown types in function signatures
        assert!(diagnostics.len() >= 2, "Functions with unknown types should generate errors");
        
        // Valid functions should still be registered
        assert!(symbol_table.functions.contains_key("process"), "process function should be registered");
        assert!(symbol_table.functions.contains_key("processPtr"), "processPtr function should be registered");
        assert!(symbol_table.functions.contains_key("processRef"), "processRef function should be registered");
        
        let diagnostic_messages: Vec<String> = diagnostics.iter()
            .map(|d| d.message.clone())
            .collect();
        
        assert!(diagnostic_messages.iter().any(|msg| msg.contains("UnknownType")),
                "Should have error for UnknownType parameter");
        assert!(diagnostic_messages.iter().any(|msg| msg.contains("NonExistent")),
                "Should have error for NonExistent return type");
    }

    #[test]
    fn test_impl_types() {
        let source = std::fs::read_to_string("src/tests/type_resolver/impl_types.mar")
            .expect("Failed to read test file");
        
        let (symbol_table, diagnostics) = parse_and_resolve(&source);
        
        // Type resolver catches unknown types in impl method signatures
        assert!(diagnostics.len() >= 2, "Impl blocks with unknown types should generate errors");
        
        assert!(symbol_table.types.contains_key("Point"), "Point struct should be registered");
        assert!(symbol_table.types.contains_key("Line"), "Line struct should be registered");
        
        let diagnostic_messages: Vec<String> = diagnostics.iter()
            .map(|d| d.message.clone())
            .collect();
        
        assert!(diagnostic_messages.iter().any(|msg| msg.contains("UnknownType")),
                "Should have error for UnknownType parameter");
        assert!(diagnostic_messages.iter().any(|msg| msg.contains("MissingType")),
                "Should have error for MissingType return type");
    }

    #[test]
    fn test_empty_structs() {
        let source = std::fs::read_to_string("src/tests/type_resolver/empty_structs.mar")
            .expect("Failed to read test file");
        
        let (symbol_table, diagnostics) = parse_and_resolve(&source);
        
        // Empty structs are valid
        assert_eq!(diagnostics.len(), 0, "Empty structs should resolve without errors");
        
        assert!(symbol_table.types.contains_key("Empty"), "Empty struct should be registered");
        assert!(symbol_table.types.contains_key("Single"), "Single struct should be registered");
        assert!(symbol_table.types.contains_key("OnlyPointers"), "OnlyPointers struct should be registered");
    }

    #[test]
    fn test_complex_circular_dependencies() {
        let source = std::fs::read_to_string("src/tests/type_resolver/complex_circular.mar")
            .expect("Failed to read test file");
        
        let (symbol_table, diagnostics) = parse_and_resolve(&source);
        
        // Complex circular dependencies should resolve fine
        assert_eq!(diagnostics.len(), 0, "Complex circular dependencies should resolve without errors");
        
        // Check all structs in complex circular pattern
        assert!(symbol_table.types.contains_key("A"), "Struct A should be registered");
        assert!(symbol_table.types.contains_key("B"), "Struct B should be registered");
        assert!(symbol_table.types.contains_key("C"), "Struct C should be registered");
        
        // Check diamond pattern structs
        assert!(symbol_table.types.contains_key("D"), "Struct D should be registered");
        assert!(symbol_table.types.contains_key("E"), "Struct E should be registered");
        assert!(symbol_table.types.contains_key("F"), "Struct F should be registered");
        assert!(symbol_table.types.contains_key("G"), "Struct G should be registered");
    }

    #[test]
    fn test_mixed_valid_and_invalid_struct_fields() {
        let source = std::fs::read_to_string("src/tests/type_resolver/mixed_valid_invalid.mar")
            .expect("Failed to read test file");
        
        let (symbol_table, diagnostics) = parse_and_resolve(&source);
        
        // Should have error for InvalidType in struct field
        assert_eq!(diagnostics.len(), 1, "Should have 1 error for InvalidType in struct field");
        
        // Both structs should still be registered
        assert!(symbol_table.types.contains_key("Valid"), "Valid struct should be registered even with invalid field");
        assert!(symbol_table.types.contains_key("AlsoValid"), "AlsoValid struct should be registered");
        
        let diagnostic_messages: Vec<String> = diagnostics.iter()
            .map(|d| d.message.clone())
            .collect();
        
        assert!(diagnostic_messages.iter().any(|msg| msg.contains("InvalidType")),
                "Should have error for InvalidType");
    }
}