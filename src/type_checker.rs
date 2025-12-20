use std::rc::Rc;

use crate::{ast::*, diagnostic::{Diagnostic, ErrMsg}, lexer::token::Positioned, logger::Log, resolver::{ResolvedType, SymbolTable}};

pub struct TypeChecker<'ctx> {
    diagnostics: &'ctx mut Vec<Diagnostic>,
    symbol_table: &'ctx mut SymbolTable,
}

impl Log for TypeChecker<'_> {
    fn get_source(&self) -> String {
        "TypeChecker".to_string()
    }
}

impl<'ctx> TypeChecker<'ctx> {
    pub fn new(diagnostics: &'ctx mut Vec<Diagnostic>, symbol_table: &'ctx mut SymbolTable) -> Self {
        Self {
            diagnostics,
            symbol_table,
        }
    }
}

impl ASTVisitorMut<'_, Option<ResolvedType>> for TypeChecker<'_> {
    fn visit_binary(&mut self, node: &mut BinaryExpr) -> Option<ResolvedType> {
        let left_type = node.left.accept_visitor_mut(self)?;
        let right_type = node.right.accept_visitor_mut(self)?;

        let mut valid = true;

        match node.operator {
            BinaryOperator::Plus | BinaryOperator::Minus | BinaryOperator::Times | BinaryOperator::Divide
            | BinaryOperator::Greater | BinaryOperator::GreaterEqual | BinaryOperator::Less | BinaryOperator::LessEqual => {
                if left_type != right_type {
                    valid = false;
                } else {
                    match left_type {
                        ResolvedType::Integer | ResolvedType::Double | ResolvedType::Char => {
                            node.set_type(left_type.clone());
                        },
                        _ => {
                            valid = false;
                        }
                    }
                }
            },
            BinaryOperator::NotEqual | BinaryOperator::Equal => {
                if left_type != right_type {
                    valid = false;
                } else {
                    node.set_type(ResolvedType::Boolean);
                }
            },
            BinaryOperator::And | BinaryOperator::Or => {
                if left_type != right_type || left_type != ResolvedType::Boolean {
                    valid = false;
                } else {
                    node.set_type(left_type.clone());
                }
            },
            BinaryOperator::BitwiseAnd | BinaryOperator::BitwiseOr | BinaryOperator::BitwiseXor
            | BinaryOperator::LeftShift | BinaryOperator::RightShift | BinaryOperator::Modulo => {
                if left_type != right_type || left_type != ResolvedType::Integer {
                    valid = false;
                } else {
                    node.set_type(left_type.clone());
                }
            },
        }

        if !valid {
            self.diagnostics.push(
                ErrMsg::IncompatibleBinaryTypes(left_type, right_type, node.operator)
                .make_diagnostic(*node.get_position())
            );

            return None;
        }

        Some(node.get_type().clone().unwrap())
    }

    fn visit_cast(&mut self, node: &mut CastExpr) -> Option<ResolvedType> {
        node.expr.accept_visitor_mut(self);
        self.symbol_table.resolve_type(&node.cast_type)
    }

    fn visit_unary(&mut self, node: &mut UnaryExpr) -> Option<ResolvedType> { 
        let expr_type = node.expr.accept_visitor_mut(self)?;
        let mut valid = true;

        match node.operator {
            UnaryOperator::Negative => {
                match expr_type {
                    ResolvedType::Integer | ResolvedType::Double => {
                        node.set_type(expr_type.clone());
                    },
                    _ => {
                        valid = false;
                    }
                }
            },
            UnaryOperator::Not => {
                if expr_type != ResolvedType::Boolean {
                    valid = false;
                } else {
                    node.set_type(expr_type.clone());
                }
            },
            UnaryOperator::BitwiseNot => {
                if expr_type != ResolvedType::Integer {
                    valid = false;
                } else {
                    node.set_type(expr_type.clone());
                }
            },
            UnaryOperator::AddressOf => {
                node.set_type(ResolvedType::Pointer(Rc::new(expr_type.clone())));
            },
            UnaryOperator::Deref => {
                match expr_type.clone() {
                    ResolvedType::Pointer(inner_type) => {
                        node.set_type((*inner_type).clone());
                    },
                    _ => {
                        valid = false;
                    }
                }
            },
        }

        if !valid {
            self.diagnostics.push(
                ErrMsg::IncompatibleUnaryType(expr_type, node.operator)
                .make_diagnostic(*node.get_position())
            )
        }

        Some(node.get_type().clone().unwrap())
    }

    fn visit_literal(&mut self, node: &mut LiteralExpr) -> Option<ResolvedType> {
        match &node.value {
            Literal::Int(_) => {
                node.set_type(ResolvedType::Integer);
            },
            Literal::Double(_) => {
                node.set_type(ResolvedType::Double);
            },
            Literal::Bool(_) => {
                node.set_type(ResolvedType::Boolean);
            },
            Literal::Char(_) => {
                node.set_type(ResolvedType::Char);
            },
            Literal::String(_) => {
                node.set_type(ResolvedType::Pointer(Rc::new(ResolvedType::Char)));
            },
        }

        Some(node.get_type().clone().unwrap())
    }

    fn visit_function_call(&mut self, node: &mut FunctionCall) -> Option<ResolvedType> {
        let func_type = node.expr.accept_visitor_mut(self)?;

        match func_type {
            ResolvedType::Function(func_sig) => {
                if func_sig.param_types.len() != node.arguments.args.len() {
                    self.diagnostics.push(
                        ErrMsg::FunctionArgumentCountMismatch(func_sig.param_types.len(), node.arguments.args.len())
                        .make_diagnostic(*node.get_position())
                    );
                    return None;
                }

                for (i, arg) in node.arguments.args.iter_mut().enumerate() {
                    let arg_type = arg.accept_visitor_mut(self)?;
                    let resolved_param = self.symbol_table.resolve_type(&func_sig.param_types[i]).unwrap();

                    if arg_type != resolved_param {
                        self.diagnostics.push(
                            ErrMsg::FunctionArgumentTypeMismatch(i, resolved_param, arg_type)
                            .make_diagnostic(*arg.get_position())
                        );
                        return None;
                    }
                }

                let resolved_return = self.symbol_table.resolve_type(&func_sig.return_type).unwrap();

                node.set_type(resolved_return.clone());
                Some(resolved_return.clone())
            },
            _ => {
                self.diagnostics.push(
                    ErrMsg::CallOnNonFunctionType(func_type.clone())
                    .make_diagnostic(*node.get_position())
                );
                None
            }
        }
    }

    fn visit_member_access(&mut self, node: &mut MemberAccess) -> Option<ResolvedType> {
        let mut expr_type = node.expr.accept_visitor_mut(self)?;

        for member_access in &mut node.member_accesses {
            match member_access {
                AccessType::Direct(field_name) => {
                    match expr_type {
                        ResolvedType::Struct(ref struct_type) => {
                            if let Some(field_type) = (*struct_type).members.get(&field_name.data) {
                                expr_type = self.symbol_table.resolve_type(field_type).unwrap();
                            } else {
                                self.diagnostics.push(
                                    ErrMsg::FieldNotFound(field_name.data.clone())
                                    .make_diagnostic(*field_name.get_position())
                                );
                                return None;
                            }
                        },
                        _ => {
                            self.diagnostics.push(
                                ErrMsg::IncompatibleMemberAccessType(expr_type.clone())
                                .make_diagnostic(*node.get_position())
                            );
                            return None;
                        }
                    }
                },
                AccessType::Indirect(field_name) => {
                    match &expr_type {
                        ResolvedType::Pointer(inner_type) => {
                            match inner_type.as_ref() {
                                ResolvedType::Struct(struct_type) => {
                                    if let Some(field_type) = struct_type.members.get(&field_name.data) {
                                        expr_type = self.symbol_table.resolve_type(field_type).unwrap();
                                    } else {
                                        self.diagnostics.push(
                                            ErrMsg::FieldNotFound(field_name.data.clone())
                                            .make_diagnostic(*field_name.get_position())
                                        );
                                        return None;
                                    }
                                },
                                _ => {
                                    self.diagnostics.push(
                                        ErrMsg::IncompatibleMemberAccessType(expr_type.clone())
                                        .make_diagnostic(*node.get_position())
                                    );
                                    return None;
                                }
                            }
                        },
                        _ => {
                            self.diagnostics.push(
                                ErrMsg::IncompatibleMemberAccessType(expr_type.clone())
                                .make_diagnostic(*node.get_position())
                            );
                            return None;
                        }
                    }
                },
                AccessType::Array(index_expr) => {
                    let index_type = index_expr.accept_visitor_mut(self)?;

                    if index_type != ResolvedType::Integer {
                        self.diagnostics.push(
                            ErrMsg::ArrayIndexNotInteger(index_type)
                            .make_diagnostic(*index_expr.get_position())
                        );
                        return None;
                    }

                    match expr_type {
                        ResolvedType::Array(inner_type) => {
                            expr_type = (*inner_type).clone();
                        },
                        _ => {
                            self.diagnostics.push(
                                ErrMsg::IncompatibleMemberAccessType(expr_type.clone())
                                .make_diagnostic(*node.get_position())
                            );
                            return None;
                        }
                    }
                },
            }
        }

        node.set_type(expr_type.clone());
        Some(expr_type)
    }

    fn visit_var(&mut self, node: &mut VarExpr) -> Option<ResolvedType> {
        if let Some(decl_id) = self.symbol_table.get_variable(&node.id) {
            let decl_type = self.symbol_table.get_declaration_type(decl_id).unwrap();
            let resolved_type = self.symbol_table.resolve_type(decl_type).unwrap();

            node.set_type(resolved_type.clone());
            return Some(resolved_type);
        } else if let Some(function_type) = self.symbol_table.get_function(&node.identifier.data) {
            let resolved_function = ResolvedType::Function(Rc::new(function_type.clone()));

            node.set_type(resolved_function.clone());
            return Some(resolved_function);
        } else {
            unreachable!("variable or function '{}' not found in symbol table", node.identifier.data);
        }
    }

    fn visit_if(&mut self, node: &mut IfExpr) -> Option<ResolvedType> { 
        let condition_type = node.condition.accept_visitor_mut(self)?;

        if condition_type != ResolvedType::Boolean {
            self.diagnostics.push(
                ErrMsg::IncompatibleUnaryType(condition_type, UnaryOperator::Not)
                .make_diagnostic(*node.condition.get_position())
            );
            return None;
        }

        let then_type = node.success.accept_visitor_mut(self)?;
        let else_type = if let Some(fail) = &mut node.fail {
            fail.accept_visitor_mut(self)?
        } else {
            ResolvedType::Void
        };

        if then_type != else_type {
            self.diagnostics.push(
                ErrMsg::MismatchedIfBranches(then_type.clone(), else_type.clone())
                .make_diagnostic(*node.get_position())
            );
            return None;
        }

        node.set_type(then_type.clone());
        Some(then_type)
    }

    fn visit_assignment(&mut self, node: &mut AssignmentExpr) -> Option<ResolvedType> { 
        let assignee_type = node.assignee.accept_visitor_mut(self)?;
        let expr_type = node.expr.accept_visitor_mut(self)?;

        if assignee_type != expr_type {
            self.diagnostics.push(
                ErrMsg::IncompatibleAssignment(assignee_type, expr_type)
                .make_diagnostic(*node.get_position())
            );
            return None;
        }

        node.set_type(assignee_type.clone());
        Some(assignee_type)
    }

    fn visit_delete(&mut self, _node: &mut DeleteExpr) -> Option<ResolvedType> { 
        unimplemented!();
    }

    fn visit_declaration(&mut self, node: &mut DeclarationExpr) -> Option<ResolvedType> {
        let init_type = node.expr.as_mut().accept_visitor_mut(self)?;
        let declaration_type = self.symbol_table.get_declaration_type(&node.id).unwrap();
        let declaration_type = self.symbol_table.resolve_type(declaration_type).unwrap();

        if init_type != declaration_type {
            self.diagnostics.push(
                ErrMsg::IncompatibleAssignment(declaration_type.clone(), init_type)
                .make_diagnostic(*node.get_position())
            );
            return None;
        }

        None
    }

    fn visit_block(&mut self, node: &mut BlockExpr) -> Option<ResolvedType> { 
        for expr in &mut node.exprs {
            expr.accept_visitor_mut(self);
        }

        None
    }

    fn visit_loop(&mut self, node: &mut LoopExpr) -> Option<ResolvedType> {
        node.initial.as_mut().map(|init_expr| init_expr.accept_visitor_mut(self));
        node.condition.as_mut().map(|cond_expr| cond_expr.accept_visitor_mut(self));
        node.increment.as_mut().map(|inc_expr| inc_expr.accept_visitor_mut(self));

        node.body.accept_visitor_mut(self);

        None
    }

    fn visit_exit(&mut self, node: &mut ExitExpr) -> Option<ResolvedType> {
        if let Some(expr) = &mut node.expr {
            expr.accept_visitor_mut(self)
        } else {
            Some(ResolvedType::Void)
        }
    }

    fn visit_constructor_call(&mut self, node: &mut ConstructorCallExpr) -> Option<ResolvedType> {
        let resolved_type = self.symbol_table.get_type(&node.type_name.data).map(|t| t.clone());
        
        for arg in &mut node.arguments.args {
            arg.accept_visitor_mut(self);
        }

        node.set_type(resolved_type.clone().unwrap());
        resolved_type
    }

    fn visit_new_array(&mut self, node: &mut NewArrayExpr) -> Option<ResolvedType> {
        let mut resolved_type = self.symbol_table.resolve_type(&node.array_type)?;

        for _ in 0..node.dimension {
            resolved_type = ResolvedType::Array(Rc::new(resolved_type));
        }

        node.set_type(resolved_type.clone());
        Some(resolved_type)
    }

    fn visit_impl(&mut self, node: &mut ImplItem) -> Option<ResolvedType> { 
        for function in &mut node.functions {
            function.accept_visitor_mut(self);
        }

        None
    }

    fn visit_function(&mut self, node: &mut FunctionItem) -> Option<ResolvedType> { 
        node.body.accept_visitor_mut(self)
    }

    fn visit_struct(&mut self, node: &mut StructItem) -> Option<ResolvedType> { 
        for constructor in &mut node.constructors {
            constructor.accept_visitor_mut(self);
        }

        None
    }

    fn visit_constructor(&mut self, node: &mut ConstructorItem) -> Option<ResolvedType> { 
        node.body.accept_visitor_mut(self)
    }

    fn visit_main(&mut self, node: &mut MainItem) -> Option<ResolvedType> { 
        node.body.accept_visitor_mut(self)
    }

    fn visit_program(&mut self, node: &mut Program) -> Option<ResolvedType> {
        for item in &mut node.items {
            item.accept_visitor_mut(self);
        }

        None
    }
}