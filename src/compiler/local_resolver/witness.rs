use crate::{ast::*, compiler::{local_resolver::LocalResolver, visit::{Parsed, PhaseWitness, VisitResult}}};

impl PhaseWitness<Parsed> for LocalResolverWitness {}

pub type LocalResolverResult = VisitResult<Parsed, LocalResolverWitness>;

pub struct LocalResolverWitness {
    _private: (),
}

impl LocalResolverWitness {
    fn new() -> Self {
        Self { _private: () }
    }
}

impl<'ctx, 'ast> LocalResolver<'ctx, 'ast> {
    pub fn witness_array_access(&mut self, node: &ArrayAccess<Parsed>, expr: &LocalResolverResult, index: &LocalResolverResult) -> LocalResolverResult {
        assert!(expr.verify(&node.expr));
        assert!(index.verify(&node.index));

        VisitResult::new(node.get_id(), LocalResolverWitness::new())
    }

    pub fn witness_assignment(&mut self, node: &AssignmentExpr<Parsed>, assignee: &LocalResolverResult, expr: &LocalResolverResult) -> LocalResolverResult {
        assert!(assignee.verify(&node.assignee));
        assert!(expr.verify(&node.expr));

        VisitResult::new(node.get_id(), LocalResolverWitness::new())
    }

    pub fn witness_binary(&mut self, node: &BinaryExpr<Parsed>, left: &LocalResolverResult, right: &LocalResolverResult) -> LocalResolverResult {
        assert!(left.verify(&node.left));
        assert!(right.verify(&node.right));

        VisitResult::new(node.get_id(), LocalResolverWitness::new())
    }

    pub fn witness_block<'itm>(&mut self, node: &BlockExpr<Parsed>, exprs: impl Iterator<Item = &'itm LocalResolverResult>) -> LocalResolverResult {
        for (expr, confirmation) in node.exprs.iter().zip(exprs) {
            assert!(confirmation.verify(expr));
        }

        VisitResult::new(node.get_id(), LocalResolverWitness::new())
    }

    pub fn witness_cast(&mut self, node: &CastExpr<Parsed>, expr: &LocalResolverResult) -> LocalResolverResult {
        assert!(expr.verify(&node.expr));

        VisitResult::new(node.get_id(), LocalResolverWitness::new())
    }

    pub fn witness_constructor<'itm>(&mut self, node: &ConstructorItem<Parsed>, parameters: impl Iterator<Item = &'itm LocalResolverResult>, body: &LocalResolverResult) -> LocalResolverResult {
        for (param, confirmation) in node.parameters.iter().zip(parameters) {
            assert!(confirmation.verify(param));
        }
        assert!(body.verify(&node.body));

        VisitResult::new(node.get_id(), LocalResolverWitness::new())
    }

    pub fn witness_constructor_call<'itm>(&mut self, node: &ConstructorCallExpr<Parsed>, arguments: impl Iterator<Item = &'itm LocalResolverResult>) -> LocalResolverResult {
        for (arg, confirmation) in node.arguments.iter().zip(arguments) {
            assert!(confirmation.verify(arg));
        }

        VisitResult::new(node.get_id(), LocalResolverWitness::new())
    }

    pub fn witness_declaration(&mut self, node: &DeclarationExpr<Parsed>, expr: Option<&LocalResolverResult>) -> LocalResolverResult {
        match (&node.expr, expr) {
            (Some(self_expr), Some(confirmation)) => assert!(confirmation.verify(self_expr)),
            (None, None) => {},
            _ => panic!("Mismatch between node.expr and expr parameter"),
        }

        VisitResult::new(node.get_id(), LocalResolverWitness::new())
    }

    pub fn witness_delete(&mut self, node: &DeleteExpr<Parsed>, expr: &LocalResolverResult) -> LocalResolverResult {
        assert!(expr.verify(&node.expr));

        VisitResult::new(node.get_id(), LocalResolverWitness::new())
    }

    pub fn witness_exit(&mut self, node: &ExitExpr<Parsed>, expr: Option<&LocalResolverResult>) -> LocalResolverResult {
        match (&node.expr, expr) {
            (Some(self_expr), Some(confirmation)) => assert!(confirmation.verify(self_expr)),
            (None, None) => {},
            _ => panic!("Mismatch between node.expr and expr parameter"),
        }

        VisitResult::new(node.get_id(), LocalResolverWitness::new())
    }

    pub fn witness_function_access<'itm>(&mut self, node: &FunctionAccess<Parsed>, expr: &LocalResolverResult, arguments: impl Iterator<Item = &'itm LocalResolverResult>) -> LocalResolverResult {
        assert!(expr.verify(&node.expr));
        for (arg, confirmation) in node.arguments.iter().zip(arguments) {
            assert!(confirmation.verify(arg));
        }

        VisitResult::new(node.get_id(), LocalResolverWitness::new())
    }

    pub fn witness_function<'itm>(&mut self, node: &'ast FunctionItem<Parsed>, parameters: impl Iterator<Item = &'itm LocalResolverResult>, body: Option<&LocalResolverResult>) -> LocalResolverResult {
        for (param, confirmation) in node.parameters.iter().zip(parameters) {
            assert!(confirmation.verify(param));
        }

        match (&node.body, body) {
            (Some(self_body), Some(confirmation)) => assert!(confirmation.verify(self_body)),
            (None, None) => {},
            _ => panic!("Mismatch between node.body and body parameter"),
        }

        self.symbol_table.function_names.insert(&node.name);

        VisitResult::new(node.get_id(), LocalResolverWitness::new())
    }

    pub fn witness_if(&mut self, node: &IfExpr<Parsed>, condition: &LocalResolverResult, success: &LocalResolverResult, fail: Option<&LocalResolverResult>) -> LocalResolverResult {
        assert!(condition.verify(&node.condition));
        assert!(success.verify(&node.success));
        match (&node.fail, fail) {
            (Some(self_fail), Some(confirmation)) => assert!(confirmation.verify(self_fail)),
            (None, None) => {},
            _ => panic!("Mismatch between node.fail and fail parameter"),
        }

        VisitResult::new(node.get_id(), LocalResolverWitness::new())
    }

    pub fn witness_impl<'itm>(&mut self, node: &ImplItem<Parsed>, functions: impl Iterator<Item = &'itm LocalResolverResult>) -> LocalResolverResult {
        for (func, confirmation) in node.functions.iter().zip(functions) {
            assert!(confirmation.verify(func));
        }

        VisitResult::new(node.get_id(), LocalResolverWitness::new())
    }

    pub fn witness_literal(&mut self, node: &LiteralExpr<Parsed>) -> LocalResolverResult {
        VisitResult::new(node.get_id(), LocalResolverWitness::new())
    }

    pub fn witness_loop(&mut self, node: &LoopExpr<Parsed>, initial: Option<&LocalResolverResult>, condition: Option<&LocalResolverResult>, increment: Option<&LocalResolverResult>, body: &LocalResolverResult) -> LocalResolverResult {
        match (&node.initial, initial) {
            (Some(self_initial), Some(confirmation)) => assert!(confirmation.verify(self_initial)),
            (None, None) => {},
            _ => panic!("Mismatch between node.initial and initial parameter"),
        }
        match (&node.condition, condition) {
            (Some(self_condition), Some(confirmation)) => assert!(confirmation.verify(self_condition)),
            (None, None) => {},
            _ => panic!("Mismatch between node.condition and condition parameter"),
        }
        match (&node.increment, increment) {
            (Some(self_increment), Some(confirmation)) => assert!(confirmation.verify(self_increment)),
            (None, None) => {},
            _ => panic!("Mismatch between node.increment and increment parameter"),
        }
        assert!(body.verify(&node.body));

        VisitResult::new(node.get_id(), LocalResolverWitness::new())
    }

    pub fn witness_new_array<'itm>(&mut self, node: &NewArrayExpr<Parsed>, sizes: impl Iterator<Item = &'itm LocalResolverResult>) -> LocalResolverResult {
        for (size, confirmation) in node.sizes.iter().zip(sizes) {
            assert!(confirmation.verify(size));
        }

        VisitResult::new(node.get_id(), LocalResolverWitness::new())
    }

    pub fn witness_scope<'itm>(&mut self, node: &Scope<Parsed>, child_scopes: impl Iterator<Item = &'itm LocalResolverResult>, items: impl Iterator<Item = &'itm LocalResolverResult>) -> LocalResolverResult {
        for (child, confirmation) in node.child_scopes.iter().zip(child_scopes) {
            assert!(confirmation.verify(child));
        }
        for (item, confirmation) in node.items.iter().zip(items) {
            assert!(confirmation.verify(item));
        }

        VisitResult::new(node.get_id(), LocalResolverWitness::new())
    }

    pub fn witness_struct_access(&mut self, node: &StructAccess<Parsed>, expr: &LocalResolverResult) -> LocalResolverResult {
        assert!(expr.verify(&node.expr));

        VisitResult::new(node.get_id(), LocalResolverWitness::new())
    }

    pub fn witness_struct<'itm>(&mut self, node: &StructItem<Parsed>, constructors: impl Iterator<Item = &'itm LocalResolverResult>) -> LocalResolverResult {
        for (constructor, confirmation) in node.constructors.iter().zip(constructors) {
            assert!(confirmation.verify(constructor));
        }

        VisitResult::new(node.get_id(), LocalResolverWitness::new())
    }

    pub fn witness_unary(&mut self, node: &UnaryExpr<Parsed>, expr: &LocalResolverResult) -> LocalResolverResult {
        assert!(expr.verify(&node.expr));

        VisitResult::new(node.get_id(), LocalResolverWitness::new())
    }

    pub fn witness_var(&mut self, node: &'ast VarExpr<Parsed>, decl_id: Option<AstId>) -> LocalResolverResult {
        match decl_id {
            Some(id) => {
                self.symbol_table.variables.insert(node.get_id(), id);
            }
            None => {
                self.unknown_variables.push(&node.path);
            }
        }
        
        VisitResult::new(node.get_id(), LocalResolverWitness::new())
    }
}



