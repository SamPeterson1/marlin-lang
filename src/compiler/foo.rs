use std::{collections::{HashMap, VecDeque}, ops::Deref};

use crate::{ast::{AstId, ASTNode, Path}, compiler::{Compiler, SymbolTable, stages::{AcceptsASTVisitor, ASTVisitor, Parsed, VisitConfirmation, VisitResult}}, logger::LogTarget};

pub struct LocalResolver<'ctx> {
    log_target: &'ctx dyn LogTarget,
    compiler: &'ctx Compiler<'ctx>,
    symbol_table: &'ctx mut SymbolTable<'ctx>,
    scopes: VecDeque<HashMap<&'ctx String, AstId>>,
    unknown_variables: Vec<&'ctx Path>
}

impl<'ctx> LocalResolver<'ctx> {
    pub fn new(log_target: &'ctx dyn LogTarget, compiler: &'ctx Compiler<'ctx>, symbol_table: &'ctx mut SymbolTable<'ctx>) -> Self {
        Self {
            log_target,
            compiler,
            symbol_table,
            scopes: VecDeque::new(),
            unknown_variables: Vec::new(),
        }
    }
}

impl<'ctx> ASTVisitor<'ctx, Parsed> for LocalResolver<'ctx> {
    fn visit_array_access(&mut self, node: &'ctx crate::ast::ArrayAccess<Parsed>) -> VisitResult<Parsed> {
        let expr = node.expr.accept_visitor(self);
        let index = node.index.accept_visitor(self);
        
        node.local_resolved(&expr, &index).into()
    }

    fn visit_assignment(&mut self, node: &'ctx crate::ast::AssignmentExpr<Parsed>) -> VisitResult<Parsed> {
        let assignee = node.assignee.accept_visitor(self);
        let expr = node.expr.accept_visitor(self);
        
        node.local_resolved(&assignee, &expr).into()
    }

    fn visit_binary(&mut self, node: &'ctx crate::ast::BinaryExpr<Parsed>) -> VisitResult<Parsed> {
        let left = node.left.accept_visitor(self);
        let right = node.right.accept_visitor(self);
        
        node.local_resolved(&left, &right).into()
    }

    fn visit_block(&mut self, node: &'ctx crate::ast::BlockExpr<Parsed>) -> VisitResult<Parsed> {
        self.scopes.push_back(HashMap::new());

        let exprs = node.exprs.iter()
            .map(|expr| expr.accept_visitor(self))
            .collect::<Vec<_>>();

        self.scopes.pop_back();
        
        node.local_resolved(exprs.iter().map(|e| e.deref())).into()
    }

    fn visit_cast(&mut self, node: &'ctx crate::ast::CastExpr<Parsed>) -> VisitResult<Parsed> {
        let expr = node.expr.accept_visitor(self);
        
        node.local_resolved(&expr).into()
    }

    fn visit_constructor(&mut self, node: &'ctx crate::ast::ConstructorItem<Parsed>) -> VisitResult<Parsed> {
        self.scopes.push_back(HashMap::new());

        let parameters: Vec<_> = node.parameters.iter()
            .map(|param| param.accept_visitor(self))
            .collect();
        
        let body = node.body.accept_visitor(self);

        self.scopes.pop_back();
        
        node.local_resolved(parameters.iter().map(|p| p.deref()), &body).into()
    }

    fn visit_constructor_call(&mut self, node: &'ctx crate::ast::ConstructorCallExpr<Parsed>) -> VisitResult<Parsed> {
        let arguments: Vec<_> = node.arguments.iter()
            .map(|arg| arg.accept_visitor(self))
            .collect();
        
        node.local_resolved(arguments.iter().map(|a| a.deref())).into()
    }

    fn visit_declaration(&mut self, node: &'ctx crate::ast::DeclarationExpr<Parsed>) -> VisitResult<Parsed> {
        let expr = node.expr.as_ref().map(|e| e.accept_visitor(self));

        let scope = self.scopes.back_mut().unwrap();
        scope.insert(node.identifier.as_ref(), node.get_id());
        
        node.local_resolved(expr.as_ref().map(|e| e.deref())).into()
    }

    fn visit_delete(&mut self, node: &'ctx crate::ast::DeleteExpr<Parsed>) -> VisitResult<Parsed> {
        let expr = node.expr.accept_visitor(self);
        
        node.local_resolved(&expr).into()
    }

    fn visit_exit(&mut self, node: &'ctx crate::ast::ExitExpr<Parsed>) -> VisitResult<Parsed> {
        let expr = node.expr.as_ref().map(|e| e.accept_visitor(self));
        
        node.local_resolved(expr.as_ref().map(|e| e.deref())).into()
    }

    fn visit_function(&mut self, node: &'ctx crate::ast::FunctionItem<Parsed>) -> VisitResult<Parsed> {
        self.scopes.push_back(HashMap::new());

        let parameters: Vec<_> = node.parameters.iter()
            .map(|param| param.accept_visitor(self))
            .collect();
        
        let body = node.body.as_ref().map(|body| body.accept_visitor(self));

        self.scopes.pop_back();

        node.local_resolved(
            parameters.iter().map(|p| p.deref()),
            body.as_ref().map(|body| body.deref()),
            self.symbol_table
        ).into()
    }

    fn visit_function_access(&mut self, node: &'ctx crate::ast::FunctionAccess<Parsed>) -> VisitResult<Parsed> {
        let expr = node.expr.accept_visitor(self);
        let arguments: Vec<_> = node.arguments.iter()
            .map(|arg| arg.accept_visitor(self))
            .collect();
        
        node.local_resolved(&expr, arguments.iter().map(|a| a.deref())).into()
    }

    fn visit_if(&mut self, node: &'ctx crate::ast::IfExpr<Parsed>) -> VisitResult<Parsed> {
        let condition = node.condition.accept_visitor(self);
        let success = node.success.accept_visitor(self);
        let fail = node.fail.as_ref().map(|f| f.accept_visitor(self));
        
        node.local_resolved(&condition, &success, fail.as_ref().map(|f| f.deref())).into()
    }

    fn visit_impl(&mut self, node: &'ctx crate::ast::ImplItem<Parsed>) -> VisitResult<Parsed> {
        let functions: Vec<_> = node.functions.iter()
            .map(|func| func.accept_visitor(self))
            .collect();
        
        node.local_resolved(functions.iter().map(|f| f.deref())).into()
    }

    fn visit_literal(&mut self, node: &'ctx crate::ast::LiteralExpr<Parsed>) -> VisitResult<Parsed> {
        node.local_resolved().into()
    }

    fn visit_loop(&mut self, node: &'ctx crate::ast::LoopExpr<Parsed>) -> VisitResult<Parsed> {
        let initial = node.initial.as_ref().map(|i| i.accept_visitor(self));
        let condition = node.condition.as_ref().map(|c| c.accept_visitor(self));
        let increment = node.increment.as_ref().map(|i| i.accept_visitor(self));
        let body = node.body.accept_visitor(self);
        
        node.local_resolved(
            initial.as_ref().map(|i| i.deref()), 
            condition.as_ref().map(|c| c.deref()), 
            increment.as_ref().map(|i| i.deref()), 
            &body
        ).into()
    }

    fn visit_new_array(&mut self, node: &'ctx crate::ast::NewArrayExpr<Parsed>) -> VisitResult<Parsed> {
        let sizes: Vec<_> = node.sizes.iter()
            .map(|size| size.accept_visitor(self))
            .collect();
        
        node.local_resolved(sizes.iter().map(|s| s.deref())).into()
    }

    fn visit_scope(&mut self, node: &'ctx crate::ast::Scope<Parsed>) -> VisitResult<Parsed> {
        let child_scopes: Vec<_> = node.child_scopes.iter()
            .map(|scope| scope.accept_visitor(self))
            .collect();
        
        let items: Vec<_> = node.items.iter()
            .map(|item| item.accept_visitor(self))
            .collect();
        
        node.local_resolved(
            child_scopes.iter().map(|s| s.deref()),
            items.iter().map(|i| i.deref())
        ).into()
    }

    fn visit_struct(&mut self, node: &'ctx crate::ast::StructItem<Parsed>) -> VisitResult<Parsed> {
        let constructors: Vec<_> = node.constructors.iter()
            .map(|ctor| ctor.accept_visitor(self))
            .collect();
        
        node.local_resolved(constructors.iter().map(|c| c.deref())).into()
    }

    fn visit_struct_access(&mut self, node: &'ctx crate::ast::StructAccess<Parsed>) -> VisitResult<Parsed> {
        let expr = node.expr.accept_visitor(self);
        
        node.local_resolved(&expr).into()
    }

    fn visit_unary(&mut self, node: &'ctx crate::ast::UnaryExpr<Parsed>) -> VisitResult<Parsed> {
        let expr = node.expr.accept_visitor(self);
        
        node.local_resolved(&expr).into()
    }

    fn visit_var(&mut self, node: &'ctx crate::ast::VarExpr<Parsed>) -> VisitResult<Parsed> {
        if node.path.segments.len() == 1 {
            for scope in self.scopes.iter().rev() {
                let identifier = node.path.segments.first().unwrap();
                if let Some(decl_id) = scope.get(identifier) {
                    return node.local_resolved(*decl_id, self.symbol_table).into();
                }
            }
        }

        // Variable not found in local scopes, add to unknown variables for later resolution
        self.unknown_variables.push(&node.path);
        
        // Still need to return a confirmation even though we haven't resolved it yet
        VisitConfirmation::new(node.get_id()).into()
    }
}