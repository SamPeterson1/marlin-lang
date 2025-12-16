use std::collections::HashMap;

use crate::{ast::*, diagnostic::{Diagnostic, ErrMsg}, lexer::token::Positioned, logger::Log, resolver::{ResolvedBaseType, StructType, SymbolTable}};

pub struct TypeResolver<'ast> {
    struct_declarations: HashMap<String, &'ast StructItem>,
}

impl Log for TypeResolver<'_> {
    fn get_source(&self) -> String {
        return "TypeResolver".to_string()
    }
}

impl<'ast> TypeResolver<'ast> {
    pub fn new() -> Self {
        Self { 
            struct_declarations: HashMap::new(),
        }
    }

    pub fn resolve(mut self, symbol_table: &mut SymbolTable, diagnostics: &mut Vec<Diagnostic>, program: &'ast Program) {
        program.accept_visitor(&mut self);

        for struct_item in self.struct_declarations.values() {
            let mut members = HashMap::new();
            let mut valid = true;

            for (member_type, member_name) in &struct_item.members {
                let base_type = &member_type.unit_type.base_type;
                match &base_type.data {
                    ParsedBaseType::TypeName(type_name) => {
                        if !self.struct_declarations.contains_key(&**type_name) {
                            self.log_error(&format!("Unknown type name: {}", type_name));
                            diagnostics.push(ErrMsg::UnknownTypeName(type_name.to_string()).make_diagnostic(*base_type.get_position()));
                            valid = false;

                            continue;
                        }
                    },
                    _ => {}
                }

                members.insert(member_name.data.to_string(), member_type.clone());
            }

            if valid {
                self.log_debug(&format!("Resolved struct {}", struct_item.name.data));
                let struct_type = StructType { members };
                symbol_table.insert_type(struct_item.name.data.to_string(), ResolvedBaseType::Struct(struct_type));
            } else {
                self.log_error(&format!("Failed to resolve struct {}", struct_item.name.data));
            }
        }
    }
}

impl<'ast> ASTVisitor<'ast, ()> for TypeResolver<'ast> {
    fn visit_impl(&mut self, _node: &ImplItem) { }

    fn visit_function(&mut self, _node: &FunctionItem) { }

    fn visit_struct(&mut self, node: &'ast StructItem) {
        self.struct_declarations.insert(node.name.data.to_string(), node);
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
    use crate::ast::ParsedBaseType;
    use crate::diagnostic::Diagnostic;
    use crate::lexer::Lexer;
    use crate::parser::ExprParser;
    use crate::resolver::{ResolvedBaseType, SymbolTable, TypeResolver};

    fn parse_and_resolve(source: &str) -> (SymbolTable, Vec<Diagnostic>) {
        let mut diagnostics = Vec::new();
        
        let lexer = Lexer::new(source, &mut diagnostics);
        let tokens = lexer.parse();
        
        let parser = ExprParser::new(tokens, &mut diagnostics);
        let program = parser.parse();
        
        let mut symbol_table = SymbolTable::new();
        let type_resolver = TypeResolver::new();
        type_resolver.resolve(&mut symbol_table, &mut diagnostics, &program);
        
        (symbol_table, diagnostics)
    }

    #[test]
    fn test_empty_struct() {
        let source = include_str!("../tests/type_resolver/valid_simple.mar");
        let (symbol_table, diagnostics) = parse_and_resolve(source);
        
        assert_eq!(diagnostics.len(), 0, "Should have no diagnostics");
        assert!(symbol_table.has_type("Empty"), "Should have Empty type");
        
        if let Some(ResolvedBaseType::Struct(struct_type)) = symbol_table.get_type("Empty") {
            assert_eq!(struct_type.members.len(), 0, "Empty struct should have 0 members");
        } else {
            panic!("Empty should be a struct type");
        }
    }

    #[test]
    fn test_primitives_only() {
        let source = include_str!("../tests/type_resolver/valid_simple.mar");
        let (symbol_table, diagnostics) = parse_and_resolve(source);
        
        assert_eq!(diagnostics.len(), 0, "Should have no diagnostics");
        assert!(symbol_table.has_type("PrimitivesOnly"), "Should have PrimitivesOnly type");
        
        if let Some(ResolvedBaseType::Struct(struct_type)) = symbol_table.get_type("PrimitivesOnly") {
            assert_eq!(struct_type.members.len(), 4, "Should have 4 members");
            assert!(struct_type.members.contains_key("x"), "Should have x member");
            assert!(struct_type.members.contains_key("y"), "Should have y member");
            assert!(struct_type.members.contains_key("flag"), "Should have flag member");
            assert!(struct_type.members.contains_key("c"), "Should have c member");
            
            // Check member types
            let x_type = &struct_type.members.get("x").unwrap().unit_type.base_type.data;
            assert!(matches!(x_type, ParsedBaseType::Integer), "x should be Integer");
            
            let y_type = &struct_type.members.get("y").unwrap().unit_type.base_type.data;
            assert!(matches!(y_type, ParsedBaseType::Double), "y should be Double");
            
            let flag_type = &struct_type.members.get("flag").unwrap().unit_type.base_type.data;
            assert!(matches!(flag_type, ParsedBaseType::Boolean), "flag should be Boolean");
            
            let c_type = &struct_type.members.get("c").unwrap().unit_type.base_type.data;
            assert!(matches!(c_type, ParsedBaseType::Char), "c should be Char");
        } else {
            panic!("PrimitivesOnly should be a struct type");
        }
    }

    #[test]
    fn test_with_pointers() {
        let source = include_str!("../tests/type_resolver/valid_simple.mar");
        let (symbol_table, diagnostics) = parse_and_resolve(source);
        
        assert_eq!(diagnostics.len(), 0, "Should have no diagnostics");
        assert!(symbol_table.has_type("WithPointers"), "Should have WithPointers type");
        
        if let Some(ResolvedBaseType::Struct(struct_type)) = symbol_table.get_type("WithPointers") {
            assert_eq!(struct_type.members.len(), 3, "Should have 3 members");
            
            // Check pointer levels
            let ptr = struct_type.members.get("ptr").unwrap();
            assert_eq!(ptr.unit_type.n_pointers, 1, "ptr should have 1 level of indirection");
            
            let double_ptr = struct_type.members.get("double_ptr").unwrap();
            assert_eq!(double_ptr.unit_type.n_pointers, 2, "double_ptr should have 2 levels of indirection");
            
            let ref_val = struct_type.members.get("ref_val").unwrap();
            assert!(ref_val.unit_type.is_reference, "ref_val should be a reference");
        } else {
            panic!("WithPointers should be a struct type");
        }
    }

    #[test]
    fn test_with_arrays() {
        let source = include_str!("../tests/type_resolver/valid_simple.mar");
        let (symbol_table, diagnostics) = parse_and_resolve(source);
        
        assert_eq!(diagnostics.len(), 0, "Should have no diagnostics");
        assert!(symbol_table.has_type("WithArrays"), "Should have WithArrays type");
        
        if let Some(ResolvedBaseType::Struct(struct_type)) = symbol_table.get_type("WithArrays") {
            assert_eq!(struct_type.members.len(), 3, "Should have 3 members");
            
            let arr = struct_type.members.get("arr").unwrap();
            assert_eq!(arr.array_modifiers.len(), 1, "arr should have 1 array dimension");
            
            let matrix = struct_type.members.get("matrix").unwrap();
            assert_eq!(matrix.array_modifiers.len(), 2, "matrix should have 2 array dimensions");
            
            let buffer = struct_type.members.get("buffer").unwrap();
            assert_eq!(buffer.array_modifiers.len(), 1, "buffer should have 1 array dimension");
        } else {
            panic!("WithArrays should be a struct type");
        }
    }

    #[test]
    fn test_forward_references() {
        let source = include_str!("../tests/type_resolver/valid_forward_ref.mar");
        let (symbol_table, diagnostics) = parse_and_resolve(source);
        
        assert_eq!(diagnostics.len(), 0, "Should have no diagnostics for forward references");
        assert!(symbol_table.has_type("A"), "Should have type A");
        assert!(symbol_table.has_type("B"), "Should have type B");
        assert!(symbol_table.has_type("C"), "Should have type C");
        
        // Verify A has references to B and C
        if let Some(ResolvedBaseType::Struct(struct_type)) = symbol_table.get_type("A") {
            assert!(struct_type.members.contains_key("b_ptr"), "A should have b_ptr");
            assert!(struct_type.members.contains_key("c_ref"), "A should have c_ref");
        } else {
            panic!("A should be a struct type");
        }
    }

    #[test]
    fn test_self_referential() {
        let source = include_str!("../tests/type_resolver/valid_self_ref.mar");
        let (symbol_table, diagnostics) = parse_and_resolve(source);
        
        assert_eq!(diagnostics.len(), 0, "Should have no diagnostics for self-referential structs");
        assert!(symbol_table.has_type("LinkedList"), "Should have LinkedList type");
        assert!(symbol_table.has_type("BinaryTree"), "Should have BinaryTree type");
        
        // Verify LinkedList has self-reference through pointer
        if let Some(ResolvedBaseType::Struct(struct_type)) = symbol_table.get_type("LinkedList") {
            assert_eq!(struct_type.members.len(), 2, "LinkedList should have 2 members");
            assert!(struct_type.members.contains_key("next"), "Should have next pointer");
            
            let next = struct_type.members.get("next").unwrap();
            assert_eq!(next.unit_type.n_pointers, 1, "next should be a pointer");
        } else {
            panic!("LinkedList should be a struct type");
        }
        
        // Verify BinaryTree has self-references
        if let Some(ResolvedBaseType::Struct(struct_type)) = symbol_table.get_type("BinaryTree") {
            assert_eq!(struct_type.members.len(), 3, "BinaryTree should have 3 members");
            assert!(struct_type.members.contains_key("left"), "Should have left pointer");
            assert!(struct_type.members.contains_key("right"), "Should have right pointer");
        } else {
            panic!("BinaryTree should be a struct type");
        }
    }

    #[test]
    fn test_nested_value_types() {
        let source = include_str!("../tests/type_resolver/valid_nested.mar");
        let (symbol_table, diagnostics) = parse_and_resolve(source);
        
        assert_eq!(diagnostics.len(), 0, "Should have no diagnostics for nested value types");
        assert!(symbol_table.has_type("Point"), "Should have Point type");
        assert!(symbol_table.has_type("Line"), "Should have Line type");
        assert!(symbol_table.has_type("Rectangle"), "Should have Rectangle type");
        assert!(symbol_table.has_type("Scene"), "Should have Scene type");
        
        // Verify Line has Point members
        if let Some(ResolvedBaseType::Struct(struct_type)) = symbol_table.get_type("Line") {
            assert_eq!(struct_type.members.len(), 2, "Line should have 2 members");
            
            let start = struct_type.members.get("start").unwrap();
            if let ParsedBaseType::TypeName(type_name) = &start.unit_type.base_type.data {
                assert_eq!(&**type_name, "Point", "start should be of type Point");
                assert_eq!(start.unit_type.n_pointers, 0, "start should be a value member");
            } else {
                panic!("start should be a TypeName");
            }
        } else {
            panic!("Line should be a struct type");
        }
    }

    #[test]
    fn test_unknown_type_single() {
        let source = include_str!("../tests/type_resolver/invalid_unknown_type.mar");
        let (symbol_table, diagnostics) = parse_and_resolve(source);
        
        assert!(diagnostics.len() > 0, "Should have diagnostics for unknown types");
        
        // BadStruct should not be resolved
        assert!(!symbol_table.has_type("BadStruct"), "BadStruct should not be in symbol table");
        
        // Count unknown type errors
        let unknown_type_errors: Vec<_> = diagnostics.iter()
            .filter(|d| d.message.contains("Unknown type name") || d.message.contains("UnknownType"))
            .collect();
        assert!(unknown_type_errors.len() >= 1, "Should have at least 1 unknown type error");
    }

    #[test]
    fn test_unknown_type_multiple() {
        let source = include_str!("../tests/type_resolver/invalid_unknown_type.mar");
        let (symbol_table, diagnostics) = parse_and_resolve(source);
        
        // MultipleUnknown should not be resolved
        assert!(!symbol_table.has_type("MultipleUnknown"), "MultipleUnknown should not be in symbol table");
        
        // Should have errors for TypeA, TypeB, TypeC
        let unknown_type_errors: Vec<_> = diagnostics.iter()
            .filter(|d| d.message.contains("TypeA") || d.message.contains("TypeB") || d.message.contains("TypeC"))
            .collect();
        assert!(unknown_type_errors.len() >= 3, "Should have at least 3 unknown type errors");
    }

    #[test]
    fn test_mixed_known_unknown() {
        let source = include_str!("../tests/type_resolver/invalid_unknown_type.mar");
        let (symbol_table, diagnostics) = parse_and_resolve(source);
        
        // MixedKnown should not be resolved due to invalid member
        assert!(!symbol_table.has_type("MixedKnown"), "MixedKnown should not be in symbol table");
        
        let unknown_type_errors: Vec<_> = diagnostics.iter()
            .filter(|d| d.message.contains("UnknownType"))
            .collect();
        assert!(unknown_type_errors.len() >= 1, "Should have at least 1 unknown type error");
    }

    #[test]
    fn test_complex_valid() {
        let source = include_str!("../tests/type_resolver/valid_complex.mar");
        let (symbol_table, diagnostics) = parse_and_resolve(source);
        
        assert_eq!(diagnostics.len(), 0, "Complex valid structs should have no diagnostics");
        assert!(symbol_table.has_type("Node"), "Should have Node type");
        assert!(symbol_table.has_type("Container"), "Should have Container type");
        assert!(symbol_table.has_type("AllTypes"), "Should have AllTypes type");
        
        // Verify AllTypes has all the members
        if let Some(ResolvedBaseType::Struct(struct_type)) = symbol_table.get_type("AllTypes") {
            assert_eq!(struct_type.members.len(), 9, "AllTypes should have 9 members");
            assert!(struct_type.members.contains_key("int_val"), "Should have int_val");
            assert!(struct_type.members.contains_key("double_val"), "Should have double_val");
            assert!(struct_type.members.contains_key("bool_val"), "Should have bool_val");
            assert!(struct_type.members.contains_key("char_val"), "Should have char_val");
            assert!(struct_type.members.contains_key("int_ptr"), "Should have int_ptr");
            assert!(struct_type.members.contains_key("double_ref"), "Should have double_ref");
            assert!(struct_type.members.contains_key("int_array"), "Should have int_array");
            assert!(struct_type.members.contains_key("node_ptr"), "Should have node_ptr");
            assert!(struct_type.members.contains_key("container"), "Should have container");
            
            // Verify container is a value member of type Container
            let container = struct_type.members.get("container").unwrap();
            if let ParsedBaseType::TypeName(type_name) = &container.unit_type.base_type.data {
                assert_eq!(&**type_name, "Container", "container should be of type Container");
                assert_eq!(container.unit_type.n_pointers, 0, "container should be a value member");
            } else {
                panic!("container should be a TypeName");
            }
        } else {
            panic!("AllTypes should be a struct type");
        }
    }

    #[test]
    fn test_all_simple_structs_resolved() {
        let source = include_str!("../tests/type_resolver/valid_simple.mar");
        let (symbol_table, diagnostics) = parse_and_resolve(source);
        
        assert_eq!(diagnostics.len(), 0, "Should have no diagnostics");
        
        // All structs should be resolved
        assert!(symbol_table.has_type("Empty"), "Should have Empty");
        assert!(symbol_table.has_type("PrimitivesOnly"), "Should have PrimitivesOnly");
        assert!(symbol_table.has_type("WithPointers"), "Should have WithPointers");
        assert!(symbol_table.has_type("WithArrays"), "Should have WithArrays");
    }

    // Circular dependency tests: These should RESOLVE successfully at the type resolution stage
    // Circular dependencies will be caught later during struct layout/size calculation
    #[test]
    fn test_circular_dependency_resolves() {
        let source = include_str!("../tests/type_resolver/circular.mar");
        let (symbol_table, diagnostics) = parse_and_resolve(source);
        
        // Type resolution should succeed
        assert_eq!(diagnostics.len(), 0, "Type resolution should succeed for circular types");
        
        // All types should be resolved successfully
        assert!(symbol_table.has_type("CircularA"), "CircularA should be resolved");
        assert!(symbol_table.has_type("CircularB"), "CircularB should be resolved");
        
        // Verify the circular references are recorded
        if let Some(ResolvedBaseType::Struct(struct_type)) = symbol_table.get_type("CircularA") {
            assert!(struct_type.members.contains_key("b"), "CircularA should have member b");
            let b_member = struct_type.members.get("b").unwrap();
            if let ParsedBaseType::TypeName(type_name) = &b_member.unit_type.base_type.data {
                assert_eq!(&**type_name, "CircularB", "b should be of type CircularB");
            }
        }
    }

    #[test]
    fn test_indirect_circular_dependency_resolves() {
        let source = include_str!("../tests/type_resolver/circular.mar");
        let (symbol_table, diagnostics) = parse_and_resolve(source);
        
        // Type resolution should succeed
        assert_eq!(diagnostics.len(), 0, "Type resolution should succeed for indirect circular types");
        
        // All types should be resolved
        assert!(symbol_table.has_type("IndirectCircular1"), "IndirectCircular1 should be resolved");
        assert!(symbol_table.has_type("IndirectCircular2"), "IndirectCircular2 should be resolved");
        assert!(symbol_table.has_type("IndirectCircular3"), "IndirectCircular3 should be resolved");
    }
}