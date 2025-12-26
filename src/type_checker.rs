use std::{any::Any, clone, rc::Rc};

use crate::{ast::*, diagnostic::{Diagnostic, ErrMsg}, lexer::token::Positioned, logger::Log, resolver::{FunctionType, ResolvedType, SymbolTable, TypeId}};

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

impl ASTVisitor<'_, Option<TypeId>> for TypeChecker<'_> {
    fn visit_binary(&mut self, node: &BinaryExpr) -> Option<TypeId> {
        let left_type_id = node.left.accept_visitor(self)?;
        let right_type_id = node.right.accept_visitor(self)?;
        let left_type = self.symbol_table.type_arena.get(&left_type_id);
        let right_type = self.symbol_table.type_arena.get(&right_type_id);

        let mut valid = true;

        match node.operator {
            BinaryOperator::Plus | BinaryOperator::Minus | BinaryOperator::Times | BinaryOperator::Divide
            | BinaryOperator::Greater | BinaryOperator::GreaterEqual | BinaryOperator::Less | BinaryOperator::LessEqual => {
                if left_type != right_type {
                    valid = false;
                } else {
                    match left_type {
                        ResolvedType::Integer | ResolvedType::Double | ResolvedType::Char => {
                            self.symbol_table.ast_types.insert(node.get_id(), left_type_id);
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
                    self.symbol_table.ast_types.insert(node.get_id(), self.symbol_table.type_arena.bool());
                }
            },
            BinaryOperator::And | BinaryOperator::Or => {
                if left_type != right_type || left_type != &ResolvedType::Boolean {
                    valid = false;
                } else {
                    self.symbol_table.ast_types.insert(node.get_id(), left_type_id);
                }
            },
            BinaryOperator::BitwiseAnd | BinaryOperator::BitwiseOr | BinaryOperator::BitwiseXor
            | BinaryOperator::LeftShift | BinaryOperator::RightShift | BinaryOperator::Modulo => {
                if left_type != right_type || left_type != &ResolvedType::Integer {
                    valid = false;
                } else {
                    self.symbol_table.ast_types.insert(node.get_id(), left_type_id);
                }
            },
        }

        if !valid {
            // We get the type references again because we have potentially mutated the type arena
            let left_type = self.symbol_table.type_arena.get(&left_type_id);
            let right_type = self.symbol_table.type_arena.get(&right_type_id);

            self.diagnostics.push(
                ErrMsg::IncompatibleBinaryTypes(left_type, right_type, node.operator)
                .make_diagnostic(*node.get_position())
            );

            return None;
        }

        Some(*self.symbol_table.ast_types.get(&node.get_id()).unwrap())
    }

    fn visit_cast(&mut self, node: &CastExpr) -> Option<TypeId> {
        node.expr.accept_visitor(self);
        let type_id = self.symbol_table.resolve_type(&node.cast_type)?;

        self.symbol_table.ast_types.insert(node.get_id(), type_id);

        Some(type_id)
    }

    fn visit_unary(&mut self, node: &UnaryExpr) -> Option<TypeId> { 
        let expr_type_id = node.expr.accept_visitor(self)?;
        let expr_type = self.symbol_table.type_arena.get(&expr_type_id);
        let mut valid = true;

        match node.operator {
            UnaryOperator::Negative => {
                match expr_type {
                    ResolvedType::Integer | ResolvedType::Double => {
                        self.symbol_table.ast_types.insert(node.get_id(), expr_type_id);
                    },
                    _ => {
                        valid = false;
                    }
                }
            },
            UnaryOperator::Not => {
                if expr_type != &ResolvedType::Boolean {
                    valid = false;
                } else {
                    self.symbol_table.ast_types.insert(node.get_id(), expr_type_id);
                }
            },
            UnaryOperator::BitwiseNot => {
                if expr_type != &ResolvedType::Integer {
                    valid = false;
                } else {
                    self.symbol_table.ast_types.insert(node.get_id(), expr_type_id);
                }
            },
            UnaryOperator::AddressOf => {
                let ptr_type_id = self.symbol_table.type_arena.make_ptr(expr_type_id);
                self.symbol_table.ast_types.insert(node.get_id(), ptr_type_id);
            },
            UnaryOperator::Deref => {
                match expr_type {
                    ResolvedType::Pointer(inner_type_id) => {
                        self.symbol_table.ast_types.insert(node.get_id(), *inner_type_id);
                    },
                    _ => {
                        valid = false;
                    }
                }
            },
        }

        if !valid {
            // We get the type references again because we have potentially mutated the type arena
            let expr_type = self.symbol_table.type_arena.get(&expr_type_id);
            self.diagnostics.push(
                ErrMsg::IncompatibleUnaryType(expr_type, node.operator)
                .make_diagnostic(*node.get_position())
            )
        }

        Some(*self.symbol_table.ast_types.get(&node.get_id()).unwrap())
    }

    fn visit_literal(&mut self, node: &LiteralExpr) -> Option<TypeId> {
        match &node.value {
            Literal::Int(_) => {
                let int_type_id = self.symbol_table.type_arena.int();
                self.symbol_table.ast_types.insert(node.get_id(), int_type_id);
            },
            Literal::Double(_) => {
                let double_type_id = self.symbol_table.type_arena.double();
                self.symbol_table.ast_types.insert(node.get_id(), double_type_id);
            },
            Literal::Bool(_) => {
                let bool_type_id = self.symbol_table.type_arena.bool();
                self.symbol_table.ast_types.insert(node.get_id(), bool_type_id);
            },
            Literal::Char(_) => {
                let char_type_id = self.symbol_table.type_arena.char();
                self.symbol_table.ast_types.insert(node.get_id(), char_type_id);
            },
            Literal::String(_) => {
                let char_type_id = self.symbol_table.type_arena.char();
                let string_type_id = self.symbol_table.type_arena.make_ptr(char_type_id);
                self.symbol_table.ast_types.insert(node.get_id(), string_type_id);
            },
        }

        Some(*self.symbol_table.ast_types.get(&node.get_id()).unwrap())
    }

    fn visit_member_access(&mut self, node: &MemberAccess) -> Option<TypeId> {
        let expr_type_id = node.expr.accept_visitor(self)?;

        for member_access in &node.member_accesses {
            match member_access {
                AccessType::Direct(field_name) => {
                    match self.symbol_table.type_arena.get(&expr_type_id) {
                        ResolvedType::Struct(struct_type) => {
                            if let Some(field_type) = (*struct_type).members.get(&field_name.data) {
                                self.symbol_table.ast_types.insert(node.get_id(), *field_type);
                            } else {
                                self.diagnostics.push(
                                    ErrMsg::FieldNotFound(field_name.data.clone())
                                    .make_diagnostic(*field_name.get_position())
                                );
                                return None;
                            }
                        },
                        expr_type => {
                            self.diagnostics.push(
                                ErrMsg::IncompatibleMemberAccessType(expr_type)
                                .make_diagnostic(*node.get_position())
                            );
                            return None;
                        }
                    }
                },
                AccessType::Indirect(field_name) => {
                    let expr_type = self.symbol_table.type_arena.get(&expr_type_id);
                    match expr_type {
                        ResolvedType::Pointer(inner_type) => {
                            match self.symbol_table.type_arena.get(inner_type) {
                                ResolvedType::Struct(struct_type) => {
                                    if let Some(field_type) = struct_type.members.get(&field_name.data) {
                                        self.symbol_table.ast_types.insert(node.get_id(), *field_type);
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
                                        ErrMsg::IncompatibleMemberAccessType(expr_type)
                                        .make_diagnostic(*node.get_position())
                                    );

                                    return None;
                                }
                            }
                        },
                        _ => {
                            self.diagnostics.push(
                                ErrMsg::IncompatibleMemberAccessType(expr_type)
                                .make_diagnostic(*node.get_position())
                            );

                            return None;
                        }
                    }
                },
                AccessType::Array(index_expr) => {
                    match self.symbol_table.type_arena.get(&expr_type_id) {
                        ResolvedType::Array(inner_type) => {
                            self.symbol_table.ast_types.insert(node.get_id(), *inner_type);
                        },
                        ResolvedType::Pointer(inner_type) => {
                            self.symbol_table.ast_types.insert(node.get_id(), *inner_type);
                        }
                        _ => {
                            let expr_type = self.symbol_table.type_arena.get(&expr_type_id);

                            self.diagnostics.push(
                                ErrMsg::IncompatibleMemberAccessType(expr_type)
                                .make_diagnostic(*node.get_position())
                            );
                            return None;
                        }
                    }

                    let index_type_id = index_expr.accept_visitor(self)?;
                    let index_type = self.symbol_table.type_arena.get(&index_type_id);

                    if index_type != &ResolvedType::Integer {
                        self.diagnostics.push(
                            ErrMsg::ArrayIndexNotInteger(index_type)
                            .make_diagnostic(*index_expr.get_position())
                        );
                        return None;
                    }
                },
                AccessType::Function(arguments) => {
                    match self.symbol_table.type_arena.get(&expr_type_id) {
                        ResolvedType::Function(func_sig) => {
                            if func_sig.param_types.len() != arguments.args.len() {
                                self.diagnostics.push(
                                    ErrMsg::FunctionArgumentCountMismatch(func_sig.param_types.len(), arguments.args.len())
                                    .make_diagnostic(*node.get_position())
                                );
                                return None;
                            }
                            
                           let func_sig = func_sig.clone();
            
                            for (i, arg) in arguments.args.iter().enumerate() {
                                let param_type_id = func_sig.param_types[i];
                                let arg_type_id = arg.accept_visitor(self)?;
            
                                if arg_type_id != param_type_id {
                                    let arg_type = self.symbol_table.type_arena.get(&arg_type_id);
                                    let param_type = self.symbol_table.type_arena.get(&param_type_id);
            
                                    self.diagnostics.push(
                                        ErrMsg::FunctionArgumentTypeMismatch(i, param_type, arg_type)
                                        .make_diagnostic(*arg.get_position())
                                    );
                                    return None;
                                }
                            }
            
                            self.symbol_table.ast_types.insert(node.get_id(), func_sig.return_type);
                        },
                        expr_type => {
                            self.diagnostics.push(
                                ErrMsg::CallOnNonFunctionType(expr_type.clone())
                                .make_diagnostic(*node.get_position())
                            );
                        }
                    }
                }
            }
        }

        Some(*self.symbol_table.ast_types.get(&node.get_id()).unwrap())
    }

    fn visit_var(&mut self, node: &VarExpr) -> Option<TypeId> {
        if let Some(decl_id) = self.symbol_table.variables.get(&node.get_id()) {
            let decl_type_id = *self.symbol_table.declaration_types.get(decl_id).unwrap();
            self.symbol_table.ast_types.insert(node.get_id(), decl_type_id);

            return Some(decl_type_id);
        } else if let Some(function_type_id) = self.symbol_table.functions.get(&node.identifier.data) {
            self.symbol_table.ast_types.insert(node.get_id(), *function_type_id);

            return Some(*function_type_id);
        } else {
            panic!("variable or function '{}' not found in symbol table", node.identifier.data);
        }
    }

    fn visit_if(&mut self, node: &IfExpr) -> Option<TypeId> { 
        let condition_type_id = node.condition.accept_visitor(self)?;
        let condition_type = self.symbol_table.type_arena.get(&condition_type_id);

        if condition_type != &ResolvedType::Boolean {
            self.diagnostics.push(
                ErrMsg::IncompatibleUnaryType(condition_type, UnaryOperator::Not)
                .make_diagnostic(*node.condition.get_position())
            );
            return None;
        }

        let then_type_id = node.success.accept_visitor(self)?;
        let else_type_id = if let Some(fail) = &node.fail {
            fail.accept_visitor(self)?
        } else {
            self.symbol_table.type_arena.void()
        };

        if then_type_id != else_type_id {
            let then_type = self.symbol_table.type_arena.get(&then_type_id);
            let else_type = self.symbol_table.type_arena.get(&else_type_id);

            self.diagnostics.push(
                ErrMsg::MismatchedIfBranches(then_type, else_type)
                .make_diagnostic(*node.get_position())
            );
            return None;
        }

        self.symbol_table.ast_types.insert(node.get_id(), then_type_id);
        Some(then_type_id)
    }

    fn visit_assignment(&mut self, node: &AssignmentExpr) -> Option<TypeId> { 
        let assignee_type_id = node.assignee.accept_visitor(self)?;
        let expr_type_id = node.expr.accept_visitor(self)?;

        if assignee_type_id != expr_type_id {
            let assignee_type = self.symbol_table.type_arena.get(&assignee_type_id);
            let expr_type = self.symbol_table.type_arena.get(&expr_type_id);

            self.diagnostics.push(
                ErrMsg::IncompatibleAssignment(assignee_type, expr_type)
                .make_diagnostic(*node.get_position())
            );
            return None;
        }

        self.symbol_table.ast_types.insert(node.get_id(), assignee_type_id);
        Some(assignee_type_id)
    }

    fn visit_delete(&mut self, _node: &DeleteExpr) -> Option<TypeId> { 
        unimplemented!();
    }

    fn visit_declaration(&mut self, node: &DeclarationExpr) -> Option<TypeId> {
        if let Some(expr) = &node.expr {
            let init_type_id = expr.accept_visitor(self)?;
            let declaration_type_id = *self.symbol_table.declaration_types.get(&node.get_id()).unwrap();
    
            if init_type_id != declaration_type_id {
                let init_type = self.symbol_table.type_arena.get(&init_type_id);
                let declaration_type = self.symbol_table.type_arena.get(&declaration_type_id);
    
                self.diagnostics.push(
                    ErrMsg::IncompatibleAssignment(declaration_type, init_type)
                    .make_diagnostic(*node.get_position())
                );
            }
        }

        None
    }

    fn visit_block(&mut self, node: &BlockExpr) -> Option<TypeId> { 
        for expr in &node.exprs {
            expr.accept_visitor(self);
        }

        None
    }

    fn visit_loop(&mut self, node: &LoopExpr) -> Option<TypeId> {
        node.initial.as_ref().map(|init_expr| init_expr.accept_visitor(self));
        node.condition.as_ref().map(|cond_expr| cond_expr.accept_visitor(self));
        node.increment.as_ref().map(|inc_expr| inc_expr.accept_visitor(self));

        node.body.accept_visitor(self);

        None
    }

    fn visit_exit(&mut self, node: &ExitExpr) -> Option<TypeId> {
        if let Some(expr) = &node.expr {
            expr.accept_visitor(self)
        } else {
            Some(self.symbol_table.type_arena.void())
        }
    }

    fn visit_constructor_call(&mut self, node: &ConstructorCallExpr) -> Option<TypeId> {
        let mut arg_types = Vec::new();
        
        for arg in &node.arguments.args {
            arg_types.push(arg.accept_visitor(self)?);
        }

        let resolved_type = self.symbol_table.types.get(&node.type_name.data)?;

        let fn_type = FunctionType {
            param_types: arg_types,
            return_type: *resolved_type
        };

        let fn_type_id = self.symbol_table.type_arena.make_function(fn_type);

        if let ResolvedType::Struct(struct_type) = self.symbol_table.type_arena.get(resolved_type) {
            if struct_type.constructors.contains(&fn_type_id) {
                self.symbol_table.ast_types.insert(node.get_id(), *resolved_type);
                return Some(*resolved_type);
            } else {
                self.diagnostics.push(
                    ErrMsg::ConstructorNotFound(self.symbol_table.type_arena.get(resolved_type))
                    .make_diagnostic(*node.get_position())
                );
                return None;
            }
        } else {
            self.diagnostics.push(
                ErrMsg::ConstructorNotFound(self.symbol_table.type_arena.get(resolved_type))
                .make_diagnostic(*node.get_position())
            );
            return None;
        }
    }

    fn visit_new_array(&mut self, node: &NewArrayExpr) -> Option<TypeId> {
        let mut resolved_type_id = self.symbol_table.resolve_type(&node.array_type)?;

        for _ in 0..node.dimension {
            resolved_type_id = self.symbol_table.type_arena.make_ptr(resolved_type_id);
        }

        self.symbol_table.ast_types.insert(node.get_id(), resolved_type_id);
        Some(resolved_type_id)
    }

    fn visit_impl(&mut self, node: &ImplItem) -> Option<TypeId> { 
        for function in &node.functions {
            function.accept_visitor(self);
        }

        None
    }

    fn visit_function(&mut self, node: &FunctionItem) -> Option<TypeId> { 
        node.body.accept_visitor(self)
    }

    fn visit_struct(&mut self, node: &StructItem) -> Option<TypeId> { 
        for constructor in &node.constructors {
            constructor.accept_visitor(self);
        }

        None
    }

    fn visit_constructor(&mut self, node: &ConstructorItem) -> Option<TypeId> { 
        node.body.accept_visitor(self)
    }

    fn visit_main(&mut self, node: &MainItem) -> Option<TypeId> { 
        node.body.accept_visitor(self)
    }

    fn visit_program(&mut self, node: &Program) -> Option<TypeId> {
        for item in &node.items {
            item.accept_visitor(self);
        }

        None
    }
}