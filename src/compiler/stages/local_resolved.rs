use crate::{ast::*, compiler::{SymbolTable, stages::{Parsed, VisitConfirmation}}};

impl ArrayAccess<Parsed> {
    pub(in crate::compiler) fn local_resolved(&self, expr: &VisitConfirmation<Parsed>, index: &VisitConfirmation<Parsed>) -> VisitConfirmation<Parsed> {
        assert!(expr.verify(&self.expr));
        assert!(index.verify(&self.index));

        VisitConfirmation::new(self.get_id())
    }
}

impl AssignmentExpr<Parsed> {
    pub(in crate::compiler) fn local_resolved(&self, assignee: &VisitConfirmation<Parsed>, expr: &VisitConfirmation<Parsed>) -> VisitConfirmation<Parsed> {
        assert!(assignee.verify(&self.assignee));
        assert!(expr.verify(&self.expr));

        VisitConfirmation::new(self.get_id())
    }
}

impl BinaryExpr<Parsed> {
    pub(in crate::compiler) fn local_resolved(&self, left: &VisitConfirmation<Parsed>, right: &VisitConfirmation<Parsed>) -> VisitConfirmation<Parsed> {
        assert!(left.verify(&self.left));
        assert!(right.verify(&self.right));

        VisitConfirmation::new(self.get_id())
    }
}

impl BlockExpr<Parsed> {
    pub(in crate::compiler) fn local_resolved<'itm>(&self, exprs: impl Iterator<Item = &'itm VisitConfirmation<Parsed>>) -> VisitConfirmation<Parsed> {
        for (expr, confirmation) in self.exprs.iter().zip(exprs) {
            assert!(confirmation.verify(expr));
        }

        VisitConfirmation::new(self.get_id())
    }
}

impl CastExpr<Parsed> {
    pub(in crate::compiler) fn local_resolved(&self, expr: &VisitConfirmation<Parsed>) -> VisitConfirmation<Parsed> {
        assert!(expr.verify(&self.expr));

        VisitConfirmation::new(self.get_id())
    }
}

impl ConstructorCallExpr<Parsed> {
    pub(in crate::compiler) fn local_resolved<'itm>(&self, arguments: impl Iterator<Item = &'itm VisitConfirmation<Parsed>>) -> VisitConfirmation<Parsed> {
        for (arg, confirmation) in self.arguments.iter().zip(arguments) {
            assert!(confirmation.verify(arg));
        }

        VisitConfirmation::new(self.get_id())
    }
}

impl ConstructorItem<Parsed> {
    pub(in crate::compiler) fn local_resolved<'itm>(&self, parameters: impl Iterator<Item = &'itm VisitConfirmation<Parsed>>, body: &VisitConfirmation<Parsed>) -> VisitConfirmation<Parsed> {
        for (param, confirmation) in self.parameters.iter().zip(parameters) {
            assert!(confirmation.verify(param));
        }
        assert!(body.verify(&self.body));

        VisitConfirmation::new(self.get_id())
    }
}

impl DeclarationExpr<Parsed> {
    pub(in crate::compiler) fn local_resolved(&self, expr: Option<&VisitConfirmation<Parsed>>) -> VisitConfirmation<Parsed> {
        match (&self.expr, expr) {
            (Some(self_expr), Some(confirmation)) => assert!(confirmation.verify(self_expr)),
            (None, None) => {},
            _ => panic!("Mismatch between self.expr and expr parameter"),
        }

        VisitConfirmation::new(self.get_id())
    }
}

impl DeleteExpr<Parsed> {
    pub(in crate::compiler) fn local_resolved(&self, expr: &VisitConfirmation<Parsed>) -> VisitConfirmation<Parsed> {
        assert!(expr.verify(&self.expr));

        VisitConfirmation::new(self.get_id())
    }
}

impl ExitExpr<Parsed> {
    pub(in crate::compiler) fn local_resolved(&self, expr: Option<&VisitConfirmation<Parsed>>) -> VisitConfirmation<Parsed> {
        match (&self.expr, expr) {
            (Some(self_expr), Some(confirmation)) => assert!(confirmation.verify(self_expr)),
            (None, None) => {},
            _ => panic!("Mismatch between self.expr and expr parameter"),
        }

        VisitConfirmation::new(self.get_id())
    }
}

impl FunctionAccess<Parsed> {
    pub(in crate::compiler) fn local_resolved<'itm>(&self, expr: &VisitConfirmation<Parsed>, arguments: impl Iterator<Item = &'itm VisitConfirmation<Parsed>>) -> VisitConfirmation<Parsed> {
        assert!(expr.verify(&self.expr));
        for (arg, confirmation) in self.arguments.iter().zip(arguments) {
            assert!(confirmation.verify(arg));
        }

        VisitConfirmation::new(self.get_id())
    }
}

impl FunctionItem<Parsed> {
    // Register function name in symbol table
    pub(in crate::compiler) fn local_resolved<'ast, 'itm>(
        &'ast self, 
        parameters: impl Iterator<Item = &'itm VisitConfirmation<Parsed>>, 
        body: Option<&VisitConfirmation<Parsed>>,
        symbol_table: &mut SymbolTable<'ast>
    ) -> VisitConfirmation<Parsed> {
        for (param, confirmation) in self.parameters.iter().zip(parameters) {
            assert!(confirmation.verify(param));
        }

        match (&self.body, body) {
            (Some(self_body), Some(confirmation)) => assert!(confirmation.verify(self_body)),
            (None, None) => {},
            _ => panic!("Mismatch between self.body and body parameter"),
        }

        symbol_table.function_names.insert(&self.name);

        VisitConfirmation::new(self.get_id())
    }
}

impl IfExpr<Parsed> {
    pub(in crate::compiler) fn local_resolved(&self, condition: &VisitConfirmation<Parsed>, success: &VisitConfirmation<Parsed>, fail: Option<&VisitConfirmation<Parsed>>) -> VisitConfirmation<Parsed> {
        assert!(condition.verify(&self.condition));
        assert!(success.verify(&self.success));
        match (&self.fail, fail) {
            (Some(self_fail), Some(confirmation)) => assert!(confirmation.verify(self_fail)),
            (None, None) => {},
            _ => panic!("Mismatch between self.fail and fail parameter"),
        }

        VisitConfirmation::new(self.get_id())
    }
}

impl ImplItem<Parsed> {
    pub(in crate::compiler) fn local_resolved<'itm>(&self, functions: impl Iterator<Item = &'itm VisitConfirmation<Parsed>>) -> VisitConfirmation<Parsed> {
        for (func, confirmation) in self.functions.iter().zip(functions) {
            assert!(confirmation.verify(func));
        }

        VisitConfirmation::new(self.get_id())
    }
}

impl LiteralExpr<Parsed> {
    pub(in crate::compiler) fn local_resolved(&self) -> VisitConfirmation<Parsed> {
        VisitConfirmation::new(self.get_id())
    }
}

impl LoopExpr<Parsed> {
    pub(in crate::compiler) fn local_resolved(&self, initial: Option<&VisitConfirmation<Parsed>>, condition: Option<&VisitConfirmation<Parsed>>, increment: Option<&VisitConfirmation<Parsed>>, body: &VisitConfirmation<Parsed>) -> VisitConfirmation<Parsed> {
        match (&self.initial, initial) {
            (Some(self_initial), Some(confirmation)) => assert!(confirmation.verify(self_initial)),
            (None, None) => {},
            _ => panic!("Mismatch between self.initial and initial parameter"),
        }
        match (&self.condition, condition) {
            (Some(self_condition), Some(confirmation)) => assert!(confirmation.verify(self_condition)),
            (None, None) => {},
            _ => panic!("Mismatch between self.condition and condition parameter"),
        }
        match (&self.increment, increment) {
            (Some(self_increment), Some(confirmation)) => assert!(confirmation.verify(self_increment)),
            (None, None) => {},
            _ => panic!("Mismatch between self.increment and increment parameter"),
        }
        assert!(body.verify(&self.body));

        VisitConfirmation::new(self.get_id())
    }
}

impl NewArrayExpr<Parsed> {
    pub(in crate::compiler) fn local_resolved<'itm>(&self, sizes: impl Iterator<Item = &'itm VisitConfirmation<Parsed>>) -> VisitConfirmation<Parsed> {
        for (size, confirmation) in self.sizes.iter().zip(sizes) {
            assert!(confirmation.verify(size));
        }

        VisitConfirmation::new(self.get_id())
    }
}

impl Scope<Parsed> {
    pub(in crate::compiler) fn local_resolved<'itm>(&self, child_scopes: impl Iterator<Item = &'itm VisitConfirmation<Parsed>>, items: impl Iterator<Item = &'itm VisitConfirmation<Parsed>>) -> VisitConfirmation<Parsed> {
        for (child, confirmation) in self.child_scopes.iter().zip(child_scopes) {
            assert!(confirmation.verify(child));
        }
        for (item, confirmation) in self.items.iter().zip(items) {
            assert!(confirmation.verify(item));
        }

        VisitConfirmation::new(self.get_id())
    }
}

impl StructAccess<Parsed> {
    pub(in crate::compiler) fn local_resolved(&self, expr: &VisitConfirmation<Parsed>) -> VisitConfirmation<Parsed> {
        assert!(expr.verify(&self.expr));

        VisitConfirmation::new(self.get_id())
    }
}

impl StructItem<Parsed> {
    pub(in crate::compiler) fn local_resolved<'itm>(&self, constructors: impl Iterator<Item = &'itm VisitConfirmation<Parsed>>) -> VisitConfirmation<Parsed> {
        for (constructor, confirmation) in self.constructors.iter().zip(constructors) {
            assert!(confirmation.verify(constructor));
        }

        VisitConfirmation::new(self.get_id())
    }
}

impl UnaryExpr<Parsed> {
    pub(in crate::compiler) fn local_resolved(&self, expr: &VisitConfirmation<Parsed>) -> VisitConfirmation<Parsed> {
        assert!(expr.verify(&self.expr));

        VisitConfirmation::new(self.get_id())
    }
}

impl VarExpr<Parsed> {
    pub(in crate::compiler) fn local_resolved(&self, decl_id: AstId, symbol_table: &mut SymbolTable) -> VisitConfirmation<Parsed> {
        symbol_table.variables.insert(self.get_id(), decl_id);
        VisitConfirmation::new(self.get_id())
    }
}