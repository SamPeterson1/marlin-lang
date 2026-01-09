use std::collections::HashMap;

use crate::{ast::ASTNode, compiler::{local_resolver::{LocalResolver, witness::{LocalResolverResult, LocalResolverWitness}}, visit::{ASTVisitor, AcceptsASTVisitor, Parsed}}};

impl<'ctx, 'ast> ASTVisitor<'ast, Parsed, LocalResolverWitness> for LocalResolver<'ctx, 'ast> {
    fn visit_array_access(&mut self, node: &'ast crate::ast::ArrayAccess<Parsed>) -> LocalResolverResult {
        let expr = node.expr.accept_visitor(self);
        let index = node.index.accept_visitor(self);
        
        self.witness_array_access(node, &expr, &index)
    }

    fn visit_assignment(&mut self, node: &'ast crate::ast::AssignmentExpr<Parsed>) -> LocalResolverResult {
        let assignee = node.assignee.accept_visitor(self);
        let expr = node.expr.accept_visitor(self);
        
        self.witness_assignment(node, &assignee, &expr)
    }

    fn visit_binary(&mut self, node: &'ast crate::ast::BinaryExpr<Parsed>) -> LocalResolverResult {
        let left = node.left.accept_visitor(self);
        let right = node.right.accept_visitor(self);
        
        self.witness_binary(node, &left, &right)
    }

    fn visit_block(&mut self, node: &'ast crate::ast::BlockExpr<Parsed>) -> LocalResolverResult {
        self.scopes.push_back(HashMap::new());

        let exprs = node.exprs.iter()
            .map(|expr| expr.accept_visitor(self))
            .collect::<Vec<_>>();

        self.scopes.pop_back();
        
        self.witness_block(node, exprs.iter())
    }

    fn visit_cast(&mut self, node: &'ast crate::ast::CastExpr<Parsed>) -> LocalResolverResult {
        let expr = node.expr.accept_visitor(self);
        
        self.witness_cast(node, &expr)
    }

    fn visit_constructor(&mut self, node: &'ast crate::ast::ConstructorItem<Parsed>) -> LocalResolverResult {
        self.scopes.push_back(HashMap::new());

        let parameters: Vec<_> = node.parameters.iter()
            .map(|param| param.accept_visitor(self))
            .collect();
        
        let body = node.body.accept_visitor(self);

        self.scopes.pop_back();
        
        self.witness_constructor(node, parameters.iter(), &body)
    }

    fn visit_constructor_call(&mut self, node: &'ast crate::ast::ConstructorCallExpr<Parsed>) -> LocalResolverResult {
        let arguments: Vec<_> = node.arguments.iter()
            .map(|arg| arg.accept_visitor(self))
            .collect();
        
        self.witness_constructor_call(node, arguments.iter())
    }

    fn visit_declaration(&mut self, node: &'ast crate::ast::DeclarationExpr<Parsed>) -> LocalResolverResult {
        let expr = node.expr.as_ref().map(|e| e.accept_visitor(self));

        let scope = self.scopes.back_mut().unwrap();
        scope.insert(node.identifier.as_ref(), node.get_id());
        
        self.witness_declaration(node, expr.as_ref())
    }

    fn visit_delete(&mut self, node: &'ast crate::ast::DeleteExpr<Parsed>) -> LocalResolverResult {
        let expr = node.expr.accept_visitor(self);
        
        self.witness_delete(node, &expr)
    }

    fn visit_exit(&mut self, node: &'ast crate::ast::ExitExpr<Parsed>) -> LocalResolverResult {
        let expr = node.expr.as_ref().map(|e| e.accept_visitor(self));
        
        self.witness_exit(node, expr.as_ref())
    }

    fn visit_function(&mut self, node: &'ast crate::ast::FunctionItem<Parsed>) -> LocalResolverResult {
        self.scopes.push_back(HashMap::new());

        let parameters: Vec<_> = node.parameters.iter()
            .map(|param| param.accept_visitor(self))
            .collect();
        
        let body = node.body.as_ref().map(|body| body.accept_visitor(self));

        self.scopes.pop_back();

        self.witness_function(node, parameters.iter(), body.as_ref())
    }

    fn visit_function_access(&mut self, node: &'ast crate::ast::FunctionAccess<Parsed>) -> LocalResolverResult {
        let expr = node.expr.accept_visitor(self);
        let arguments: Vec<_> = node.arguments.iter()
            .map(|arg| arg.accept_visitor(self))
            .collect();
        
        self.witness_function_access(node, &expr, arguments.iter())
    }

    fn visit_if(&mut self, node: &'ast crate::ast::IfExpr<Parsed>) -> LocalResolverResult {
        let condition = node.condition.accept_visitor(self);
        let success = node.success.accept_visitor(self);
        let fail = node.fail.as_ref().map(|f| f.accept_visitor(self));
        
        self.witness_if(node, &condition, &success, fail.as_ref())
    }

    fn visit_impl(&mut self, node: &'ast crate::ast::ImplItem<Parsed>) -> LocalResolverResult {
        let functions: Vec<_> = node.functions.iter()
            .map(|func| func.accept_visitor(self))
            .collect();
        
        self.witness_impl(node, functions.iter())
    }

    fn visit_literal(&mut self, node: &'ast crate::ast::LiteralExpr<Parsed>) -> LocalResolverResult {
        self.witness_literal(node)
    }

    fn visit_loop(&mut self, node: &'ast crate::ast::LoopExpr<Parsed>) -> LocalResolverResult {
        let initial = node.initial.as_ref().map(|i| i.accept_visitor(self));
        let condition = node.condition.as_ref().map(|c| c.accept_visitor(self));
        let increment = node.increment.as_ref().map(|i| i.accept_visitor(self));
        let body = node.body.accept_visitor(self);
        
        self.witness_loop(node, initial.as_ref(), condition.as_ref(), increment.as_ref(), &body)
    }

    fn visit_new_array(&mut self, node: &'ast crate::ast::NewArrayExpr<Parsed>) -> LocalResolverResult {
        let sizes: Vec<_> = node.sizes.iter()
            .map(|size| size.accept_visitor(self))
            .collect();
        
        self.witness_new_array(node, sizes.iter())
    }

    fn visit_scope(&mut self, node: &'ast crate::ast::Scope<Parsed>) -> LocalResolverResult {
        let child_scopes: Vec<_> = node.child_scopes.iter()
            .map(|scope| scope.accept_visitor(self))
            .collect();
        
        let items: Vec<_> = node.items.iter()
            .map(|item| item.accept_visitor(self))
            .collect();
        
        self.witness_scope(node, child_scopes.iter(), items.iter())
    }

    fn visit_struct(&mut self, node: &'ast crate::ast::StructItem<Parsed>) -> LocalResolverResult {
        let constructors: Vec<_> = node.constructors.iter()
            .map(|ctor| ctor.accept_visitor(self))
            .collect();
        
        self.witness_struct(node, constructors.iter())
    }

    fn visit_struct_access(&mut self, node: &'ast crate::ast::StructAccess<Parsed>) -> LocalResolverResult {
        let expr = node.expr.accept_visitor(self);
        
        self.witness_struct_access(node, &expr)
    }

    fn visit_unary(&mut self, node: &'ast crate::ast::UnaryExpr<Parsed>) -> LocalResolverResult {
        let expr = node.expr.accept_visitor(self);
        
        self.witness_unary(node, &expr)
    }

    fn visit_var(&mut self, node: &'ast crate::ast::VarExpr<Parsed>) -> LocalResolverResult {
        if node.path.segments.len() == 1 {
            for scope in self.scopes.iter().rev() {
                let identifier = node.path.segments.first().unwrap();
                if let Some(decl_id) = scope.get(identifier) {
                    return self.witness_var(node, Some(*decl_id));
                }
            }
        }

        self.witness_var(node, None)
    }
}