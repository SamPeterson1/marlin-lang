use crate::ast::*;
use crate::diagnostic::Diagnostic;
use crate::resolver::SymbolTable;

pub struct VarResolver<'st, 'diag> {
    symbol_table: &'st mut SymbolTable,
    diagnostics: &'diag mut Vec<Diagnostic>,
}

impl<'st, 'diag> VarResolver<'st, 'diag> {
    pub fn new(symbol_table: &'st mut SymbolTable, diagnostics: &'diag mut Vec<Diagnostic>) -> Self {
        Self {
            symbol_table,
            diagnostics,
        }
    }

    pub fn resolve_vars(mut self, program: &Program) {
        program.accept_visitor(&mut self);
    }
}

impl ASTVisitor<'_, ()> for VarResolver<'_, '_> {
    fn visit_binary(&mut self, node: &BinaryExpr) {
        node.left.accept_visitor(self);
        node.right.accept_visitor(self);
    }

    fn visit_unary(&mut self, node: &UnaryExpr) {
        node.expr.accept_visitor(self);
    }
    
    fn visit_literal(&mut self, _node: &LiteralExpr) { }
    
    fn visit_member_access(&mut self, node: &MemberAccess) {
        node.expr.accept_visitor(self);
    }
    
    fn visit_var(&mut self, node: &VarExpr) {

    }
    
    fn visit_if(&mut self, node: &IfExpr) {
        node.condition.accept_visitor(self);
        node.success.accept_visitor(self);
        if let Some(else_branch) = &node.fail {
            else_branch.accept_visitor(self);
        }
    }
    
    fn visit_assignment(&mut self, node: &AssignmentExpr) {
        node.assignee.accept_visitor(self);
        node.expr.accept_visitor(self);
    }
    
    fn visit_delete(&mut self, node: &DeleteExpr) {
        node.expr.accept_visitor(self);
    }
    
    fn visit_declaration(&mut self, node: &DeclarationExpr) {
        node.expr.accept_visitor(self);
    }
    
    fn visit_block(&mut self, node: &BlockExpr) {
        for expr in &node.exprs {
            expr.accept_visitor(self);
        }
    }
    
    fn visit_loop(&mut self, node: &LoopExpr) {
        node.body.accept_visitor(self);
    }
    
    fn visit_exit(&mut self, node: &ExitExpr) {
        if let Some(expr) = &node.expr {
            expr.accept_visitor(self);
        }
    }
    
    fn visit_constructor_call(&mut self, node: &ConstructorCallExpr) {
        for arg in &node.arguments.args {
            arg.accept_visitor(self);
        }
    }
    
    fn visit_new_array(&mut self, node: &NewArrayExpr) {
        for size_expr in &node.sizes {
            size_expr.accept_visitor(self);
        }
    }
    
    fn visit_impl(&mut self, node: &ImplItem) {
        for function in &node.functions {
            function.accept_visitor(self);
        }
    }
    
    fn visit_function(&mut self, node: &FunctionItem) {
        node.body.accept_visitor(self);
    }
    
    fn visit_struct(&mut self, node: &StructItem) {
        for constructor in &node.constructors {
            constructor.accept_visitor(self);
        }
    }
    
    fn visit_constructor(&mut self, node: &ConstructorItem) {
        node.body.accept_visitor(self);
    }
    
    fn visit_main(&mut self, node: &MainItem) {
        node.body.accept_visitor(self);
    }
    
    fn visit_program(&mut self, node: &Program) {
        for item in &node.items {
            item.accept_visitor(self);
        }
    }
    
}
