use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

use crate::ast::*;
use crate::diagnostic::{Diagnostic, ErrMsg};
use crate::lexer::token::Positioned;
use crate::logger::{Log, LogTarget};
use crate::resolver::{GlobalSymbolTable, SymbolTable};

pub struct VarResolver<'ctx> {
    log_target: &'ctx dyn LogTarget,
    global_table: &'ctx GlobalSymbolTable,
    symbol_table: &'ctx mut SymbolTable,
    diagnostics: &'ctx mut Vec<Diagnostic>,
    scopes: VecDeque<HashMap<&'ctx str, AstId>>,
    unknown_variables: Vec<&'ctx Path>
}

impl Log for VarResolver<'_> {
    fn get_source(&self) -> String {
        "VarResolver".to_string()
    }
}

impl<'ctx> VarResolver<'ctx> {
    pub fn new(log_target: &'ctx dyn LogTarget, global_table: &'ctx GlobalSymbolTable, symbol_table: &'ctx mut SymbolTable, diagnostics: &'ctx mut Vec<Diagnostic>) -> Self {
        Self {
            log_target,
            global_table,
            symbol_table,
            diagnostics,
            scopes: vec![HashMap::new()].into(),
            unknown_variables: Vec::new()
        }
    }

    pub fn resolve_vars(&mut self, scope: &'ctx Scope) {
        scope.accept_visitor(self);
    }

    pub fn finish_resolving(self) {
        self.log_debug(self.log_target, &format!("Global scope list: {:?}", self.global_table.scopes.keys().collect::<Vec<_>>()));

        for path in &self.unknown_variables {
            let is_known = if path.segments.len() == 1 {
                self.symbol_table.function_names.contains(&path.to_string())
            } else {
                let parent_scope = &path.segments[0..path.segments.len() - 1];
                let scope_vec = parent_scope.iter().map(|s| s.data.clone()).collect::<Vec<_>>();

                self.log_debug(self.log_target, &format!("Checking for function '{}' in scope '{:?}'", path.to_string(), &scope_vec));

                if let Some(symbol_table) = self.global_table.scopes.get(&scope_vec) {
                    self.log_debug(self.log_target, &format!("Found scope '{:?} with functions: {:?}'", &scope_vec, symbol_table.lock().unwrap().function_names.iter().collect::<Vec<_>>()));
                    symbol_table.lock().unwrap().function_names.contains(&path.segments.last().unwrap().data)
                } else {
                    self.log_debug(self.log_target, &format!("Scope '{:?}' not found", &scope_vec));
                    false
                }
            };

            if !is_known {
                self.log_error(self.log_target, &format!("Unknown variable '{}'", path.to_string()));

                self.diagnostics.push(
                    ErrMsg::UnknownVariable(path.to_string())
                        .make_diagnostic(*path.get_position())
                );
            }
        }
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
        if node.path.segments.len() == 1 {
            for scope in self.scopes.iter().rev() {
                let identifier = node.path.segments.first().unwrap().data.as_str();
                if let Some(decl) = scope.get(identifier) {
                    self.log_info(self.log_target, &format!("Resolved variable '{}' to declaration ID {:?}", identifier, decl));
                    self.symbol_table.variables.insert(node.get_id(), *decl);
                    return;
                }
            }
        }

        self.unknown_variables.push(&node.path)
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

        let mut type_arena = self.global_table.type_arena.lock().unwrap();
        let resolved_type_id = self.symbol_table.resolve_type(&mut type_arena, &node.declaration_type);

        self.symbol_table.declaration_types.insert(node.get_id(), resolved_type_id.unwrap());

        if scope.contains_key(&node.identifier.data.as_str()) {
            self.log_error(self.log_target, &format!("Duplicate variable declaration: '{}'", node.identifier.data));

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
        self.symbol_table.function_names.insert(node.name.data.to_string());

        if let Some(body) = &node.body {
            self.scopes.push_back(HashMap::new());

            for parameter in &node.parameters {
                parameter.accept_visitor(self);
            }
            
            body.accept_visitor(self);
            self.scopes.pop_back();
        }
    }
    
    fn visit_struct(&mut self, node: &'ast StructItem) {
        for constructor in &node.constructors {
            constructor.accept_visitor(self);
        }
    }
    
    fn visit_constructor(&mut self, node: &'ast ConstructorItem) {
        node.body.accept_visitor(self);
    }

    fn visit_scope(&mut self, node: &'ast Scope) {
        for item in &node.items {
            item.accept_visitor(self);
        }
    }
}

#[cfg(test)]
mod tests {
}
