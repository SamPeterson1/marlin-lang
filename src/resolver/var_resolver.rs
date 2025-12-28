use std::collections::{HashMap, VecDeque};

use crate::ast::*;
use crate::diagnostic::{Diagnostic, ErrMsg};
use crate::lexer::token::Positioned;
use crate::logger::{Log, LogTarget};
use crate::resolver::SymbolTable;

pub struct VarResolver<'ctx> {
    log_targets: &'ctx [&'ctx dyn LogTarget],
    symbol_table: &'ctx mut SymbolTable,
    diagnostics: &'ctx mut Vec<Diagnostic>,
    scopes: VecDeque<HashMap<&'ctx str, AstId>>,
}

impl Log for VarResolver<'_> {
    fn get_source(&self) -> String {
        "VarResolver".to_string()
    }
}

impl<'ctx> VarResolver<'ctx> {
    pub fn new(log_target: &'ctx &'ctx dyn LogTarget, symbol_table: &'ctx mut SymbolTable, diagnostics: &'ctx mut Vec<Diagnostic>) -> Self {
        Self {
            log_targets: std::slice::from_ref(log_target),
            symbol_table,
            diagnostics,
            scopes: vec![HashMap::new()].into()
        }
    }

    pub fn resolve_vars(mut self, program: &'ctx Program) {
        program.accept_visitor(&mut self);
    }
}

impl<'ast> ASTVisitor<'ast, ()> for VarResolver<'ast> {
    fn visit_binary(&mut self, node: &'ast BinaryExpr) {
        node.left.accept_visitor(self);
        node.right.accept_visitor(self);
    }

    fn visit_cast(&mut self, node: &'ast CastExpr) -> () {
        node.expr.accept_visitor(self);
    }

    fn visit_unary(&mut self, node: &'ast UnaryExpr) {
        node.expr.accept_visitor(self);
    }
    
    fn visit_literal(&mut self, _node: &'ast LiteralExpr) { }
    
    fn visit_member_access(&mut self, node: &'ast MemberAccess) {
        for member_access in &node.member_accesses {
            match member_access {
                AccessType::Array(index_expr) => {
                    index_expr.accept_visitor(self);
                },
                AccessType::Function(arguments) => {
                    for arg in &arguments.args {
                        arg.accept_visitor(self);
                    }
                }
                _ => {}
            }
        }

        node.expr.accept_visitor(self);
    }
    
    fn visit_var(&mut self, node: &'ast VarExpr) {
        // Do not attempt to resolve function calls
        if self.symbol_table.functions.contains_key(node.identifier.data.as_str()) {
            return;
        }

        for scope in self.scopes.iter().rev() {
            if let Some(decl) = scope.get(node.identifier.data.as_str()) {
                self.log_info(self.log_targets, &format!("Resolved variable '{}' to declaration ID {:?}", node.identifier.data, decl));
                self.symbol_table.variables.insert(node.get_id(), *decl);
                return;
            }
        }

        self.log_error(self.log_targets, &format!("Unknown variable: '{}'", node.identifier.data));

        self.diagnostics.push(
            crate::diagnostic::ErrMsg::UnknownVariable(node.identifier.data.clone())
                .make_diagnostic(*node.get_position())
        );
    }
    
    fn visit_if(&mut self, node: &'ast IfExpr) {
        node.condition.accept_visitor(self);
        node.success.accept_visitor(self);
        if let Some(else_branch) = &node.fail {
            else_branch.accept_visitor(self);
        }
    }
    
    fn visit_assignment(&mut self, node: &'ast AssignmentExpr) {
        node.assignee.accept_visitor(self);
        node.expr.accept_visitor(self);
    }
    
    fn visit_delete(&mut self, node: &'ast DeleteExpr) {
        node.expr.accept_visitor(self);
    }
    
    fn visit_declaration(&mut self, node: &'ast DeclarationExpr) {
        if let Some(expr) = &node.expr {
            expr.accept_visitor(self);
        }

        let scope = self.scopes.back_mut().unwrap();

        let resolved_type_id = self.symbol_table.resolve_type(&node.declaration_type);

        self.symbol_table.declaration_types.insert(node.get_id(), resolved_type_id.unwrap());

        if scope.contains_key(&node.identifier.data.as_str()) {
            self.log_error(self.log_targets, &format!("Duplicate variable declaration: '{}'", node.identifier.data));

            self.diagnostics.push(
                ErrMsg::DuplicateVariable(node.identifier.data.clone())
                    .make_diagnostic(*node.get_position())
            );
        } else {
            self.scopes.back_mut().unwrap().insert(&node.identifier.data, node.get_id());
        }
    }
    
    fn visit_block(&mut self, node: &'ast BlockExpr) {
        self.scopes.push_back(HashMap::new());

        for expr in &node.exprs {
            expr.accept_visitor(self);
        }

        self.scopes.pop_back();
    }
    
    fn visit_loop(&mut self, node: &'ast LoopExpr) {
        if let Some(initial) = &node.initial {
            initial.accept_visitor(self);
        }

        if let Some(condition) = &node.condition {
            condition.accept_visitor(self);
        }

        if let Some(increment) = &node.increment {
            increment.accept_visitor(self);
        }

        node.body.accept_visitor(self);
    }
    
    fn visit_exit(&mut self, node: &'ast ExitExpr) {
        if let Some(expr) = &node.expr {
            expr.accept_visitor(self);
        }
    }
    
    fn visit_constructor_call(&mut self, node: &'ast ConstructorCallExpr) {
        for arg in &node.arguments.args {
            arg.accept_visitor(self);
        }
    }
    
    fn visit_new_array(&mut self, node: &'ast NewArrayExpr) {
        for size_expr in &node.sizes {
            size_expr.accept_visitor(self);
        }
    }
    
    fn visit_impl(&mut self, node: &'ast ImplItem) {
        for function in &node.functions {
            function.accept_visitor(self);
        }
    }
    
    fn visit_function(&mut self, node: &'ast FunctionItem) {
        self.scopes.push_back(HashMap::new());

        for parameter in &node.parameters {
            parameter.accept_visitor(self);
        }
        
        node.body.accept_visitor(self);
        self.scopes.pop_back();
    }
    
    fn visit_struct(&mut self, node: &'ast StructItem) {
        for constructor in &node.constructors {
            constructor.accept_visitor(self);
        }
    }
    
    fn visit_constructor(&mut self, node: &'ast ConstructorItem) {
        node.body.accept_visitor(self);
    }
    
    fn visit_main(&mut self, node: &'ast MainItem) {
        node.body.accept_visitor(self);
    }
    
    fn visit_program(&mut self, node: &'ast Program) {
        for item in &node.items {
            item.accept_visitor(self);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::diagnostic::Diagnostic;
    use crate::lexer::Lexer;
    use crate::logger::DYN_CONSOLE_LOGGER;
    use crate::parser::ExprParser;
    use crate::resolver::{SymbolTable, VarResolver};

    fn parse_and_resolve_vars(source: &str) -> (SymbolTable, Vec<Diagnostic>) {
        let mut diagnostics = Vec::new();
        
        let lexer = Lexer::new(&DYN_CONSOLE_LOGGER, source, &mut diagnostics);
        let tokens = lexer.parse();
        
        let parser = ExprParser::new(&DYN_CONSOLE_LOGGER, tokens, &mut diagnostics);
        let program = parser.parse();
        
        // Clear parsing diagnostics for variable resolution tests
        diagnostics.clear();
        
        let mut symbol_table = SymbolTable::new();
        let var_resolver = VarResolver::new(&DYN_CONSOLE_LOGGER, &mut symbol_table, &mut diagnostics);
        var_resolver.resolve_vars(&program);
        
        (symbol_table, diagnostics)
    }

    #[test]
    fn test_valid_basic_usage() {
        let source = include_str!("../tests/var_resolver/valid_basic.mar");
        let (_symbol_table, diagnostics) = parse_and_resolve_vars(source);
        
        assert_eq!(diagnostics.len(), 0, "Should have no diagnostics for valid basic usage");
    }

    #[test]
    fn test_valid_shadowing() {
        let source = include_str!("../tests/var_resolver/valid_shadowing.mar");
        let (_symbol_table, diagnostics) = parse_and_resolve_vars(source);
        
        assert_eq!(diagnostics.len(), 0, "Should have no diagnostics for valid shadowing");
    }

    #[test]
    fn test_valid_nested_scopes() {
        let source = include_str!("../tests/var_resolver/valid_nested_scopes.mar");
        let (_symbol_table, diagnostics) = parse_and_resolve_vars(source);
        
        assert_eq!(diagnostics.len(), 0, "Should have no diagnostics for valid nested scopes");
    }

    #[test]
    fn test_valid_control_flow() {
        let source = include_str!("../tests/var_resolver/valid_control_flow.mar");
        let (_symbol_table, diagnostics) = parse_and_resolve_vars(source);
        
        assert_eq!(diagnostics.len(), 0, "Should have no diagnostics for valid control flow");
    }

    #[test]
    fn test_invalid_duplicate_variable() {
        let source = include_str!("../tests/var_resolver/invalid_duplicate.mar");
        let (_symbol_table, diagnostics) = parse_and_resolve_vars(source);
        
        assert!(diagnostics.len() > 0, "Should have diagnostics for duplicate variable");
        
        let duplicate_errors: Vec<_> = diagnostics.iter()
            .filter(|d| d.message.contains("duplicate") || d.message.contains("Duplicate"))
            .collect();
        
        assert_eq!(duplicate_errors.len(), 1, "Should have exactly 1 duplicate variable error");
        assert!(duplicate_errors[0].message.contains("'x'"), "Error should mention variable 'x'");
    }

    #[test]
    fn test_invalid_undefined_variable() {
        let source = include_str!("../tests/var_resolver/invalid_undefined.mar");
        let (_symbol_table, diagnostics) = parse_and_resolve_vars(source);
        
        assert!(diagnostics.len() >= 2, "Should have at least 2 diagnostics for undefined variables");
        
        let undefined_errors: Vec<_> = diagnostics.iter()
            .filter(|d| d.message.contains("unknown") || d.message.contains("Unknown"))
            .collect();
        
        assert_eq!(undefined_errors.len(), 2, "Should have exactly 2 undefined variable errors");
        
        // Check that both 'y' and 'undefined_var' are reported
        let has_y_error = undefined_errors.iter().any(|d| d.message.contains("'y'"));
        let has_undefined_var_error = undefined_errors.iter().any(|d| d.message.contains("'undefined_var'"));
        
        assert!(has_y_error, "Should have error for undefined variable 'y'");
        assert!(has_undefined_var_error, "Should have error for undefined variable 'undefined_var'");
    }

    #[test]
    fn test_invalid_self_reference() {
        let source = include_str!("../tests/var_resolver/invalid_self_reference.mar");
        let (_symbol_table, diagnostics) = parse_and_resolve_vars(source);
        
        assert!(diagnostics.len() >= 3, "Should have at least 3 diagnostics for self-referencing variables");
        
        let self_ref_errors: Vec<_> = diagnostics.iter()
            .filter(|d| d.message.contains("unknown") || d.message.contains("Unknown"))
            .collect();
        
        assert_eq!(self_ref_errors.len(), 3, "Should have exactly 3 self-reference errors");
        
        // Check that x, y, and z are all reported as unknown (not yet declared when used)
        let has_x_error = self_ref_errors.iter().any(|d| d.message.contains("'x'"));
        let has_y_error = self_ref_errors.iter().any(|d| d.message.contains("'y'"));
        let has_z_error = self_ref_errors.iter().any(|d| d.message.contains("'z'"));
        
        assert!(has_x_error, "Should have error for self-referencing variable 'x'");
        assert!(has_y_error, "Should have error for self-referencing variable 'y'");
        assert!(has_z_error, "Should have error for self-referencing variable 'z'");
    }

    #[test]
    fn test_invalid_out_of_scope() {
        let source = include_str!("../tests/var_resolver/invalid_out_of_scope.mar");
        let (_symbol_table, diagnostics) = parse_and_resolve_vars(source);
        
        assert!(diagnostics.len() >= 2, "Should have at least 2 diagnostics for out-of-scope variables");
        
        let out_of_scope_errors: Vec<_> = diagnostics.iter()
            .filter(|d| d.message.contains("unknown") || d.message.contains("Unknown"))
            .collect();
        
        assert_eq!(out_of_scope_errors.len(), 2, "Should have exactly 2 out-of-scope errors");
        
        // Check that both 'x' and 'b' are reported (they're out of scope when used)
        let x_errors = out_of_scope_errors.iter().filter(|d| d.message.contains("'x'")).count();
        let b_errors = out_of_scope_errors.iter().filter(|d| d.message.contains("'b'")).count();
        
        assert_eq!(x_errors, 1, "Should have 1 error for out-of-scope variable 'x'");
        assert_eq!(b_errors, 1, "Should have 1 error for out-of-scope variable 'b'");
    }

    #[test]
    fn test_invalid_multiple_errors() {
        let source = include_str!("../tests/var_resolver/invalid_multiple_errors.mar");
        let (_symbol_table, diagnostics) = parse_and_resolve_vars(source);
        
        assert!(diagnostics.len() >= 6, "Should have at least 6 diagnostics for multiple errors");
        
        let unknown_errors: Vec<_> = diagnostics.iter()
            .filter(|d| d.message.contains("unknown") || d.message.contains("Unknown"))
            .collect();
        
        let duplicate_errors: Vec<_> = diagnostics.iter()
            .filter(|d| d.message.contains("duplicate") || d.message.contains("Duplicate"))
            .collect();
        
        // Expect errors for:
        // - y undefined
        // - x duplicate
        // - z self-reference (unknown)
        // - b undefined
        // - c duplicate
        // - c out of scope (unknown)
        assert_eq!(unknown_errors.len(), 4, "Should have 4 unknown variable errors");
        assert_eq!(duplicate_errors.len(), 2, "Should have 2 duplicate variable errors");
    }

    #[test]
    fn test_shadowing_different_variables() {
        let source = include_str!("../tests/var_resolver/valid_shadowing_partial.mar");
        let (_symbol_table, diagnostics) = parse_and_resolve_vars(source);
        
        assert_eq!(diagnostics.len(), 0, "Shadowing only affects the shadowed variable");
    }

    #[test]
    fn test_scope_isolation() {
        let source = include_str!("../tests/var_resolver/valid_scope_isolation.mar");
        let (_symbol_table, diagnostics) = parse_and_resolve_vars(source);
        
        assert_eq!(diagnostics.len(), 0, "Variables in sibling scopes should not interfere");
    }

    #[test]
    fn test_nested_shadowing_resolution() {
        let source = include_str!("../tests/var_resolver/valid_nested_shadowing.mar");
        let (_symbol_table, diagnostics) = parse_and_resolve_vars(source);
        
        assert_eq!(diagnostics.len(), 0, "Nested shadowing should resolve to nearest scope");
    }

    #[test]
    fn test_if_scope_isolation() {
        let source = include_str!("../tests/var_resolver/invalid_if_scope.mar");
        let (_symbol_table, diagnostics) = parse_and_resolve_vars(source);
        
        assert!(diagnostics.len() > 0, "Variables from if-block should not leak out");
        
        let unknown_errors: Vec<_> = diagnostics.iter()
            .filter(|d| d.message.contains("'y'"))
            .collect();
        
        assert_eq!(unknown_errors.len(), 1, "Should have error for 'y' being out of scope");
    }

    #[test]
    fn test_loop_scope_isolation() {
        let source = include_str!("../tests/var_resolver/invalid_loop_scope.mar");
        let (_symbol_table, diagnostics) = parse_and_resolve_vars(source);
        
        assert!(diagnostics.len() > 0, "Variables from loop body should not leak out");
        
        let unknown_errors: Vec<_> = diagnostics.iter()
            .filter(|d| d.message.contains("'counter'"))
            .collect();
        
        assert_eq!(unknown_errors.len(), 1, "Should have error for 'counter' being out of scope");
    }

    #[test]
    fn test_complex_expression_in_declaration() {
        let source = include_str!("../tests/var_resolver/valid_complex_expression.mar");
        let (_symbol_table, diagnostics) = parse_and_resolve_vars(source);
        
        assert_eq!(diagnostics.len(), 0, "Complex expressions should resolve all variables");
    }

    #[test]
    fn test_declaration_order_matters() {
        let source = include_str!("../tests/var_resolver/valid_declaration_order.mar");
        let (_symbol_table, diagnostics) = parse_and_resolve_vars(source);
        
        assert_eq!(diagnostics.len(), 0, "Variables should be usable after declaration");
    }

    #[test]
    fn test_cannot_use_before_declaration() {
        let source = include_str!("../tests/var_resolver/invalid_use_before_declaration.mar");
        let (_symbol_table, diagnostics) = parse_and_resolve_vars(source);
        
        assert!(diagnostics.len() > 0, "Cannot use variable before it's declared");
        
        let unknown_errors: Vec<_> = diagnostics.iter()
            .filter(|d| d.message.contains("'y'"))
            .collect();
        
        assert_eq!(unknown_errors.len(), 1, "Should have error for using 'y' before declaration");
    }
}
