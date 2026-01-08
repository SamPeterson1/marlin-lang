use std::collections::HashMap;

use crate::{ast::*, diagnostic::{Diagnostic, ErrMsg}, lexer::token::Positioned, logger::{Log, LogTarget}, resolver::{FunctionType, GlobalSymbolTable, ResolvedType, SymbolTable, TypeId}};

pub struct TypeChecker<'ctx> {
    diagnostics: &'ctx mut Vec<Diagnostic>,
    global_table: &'ctx GlobalSymbolTable,
    symbol_table: &'ctx SymbolTable,
    functions_to_resolve: HashMap<&'ctx Path, TypeId>,
    log_target: &'ctx dyn LogTarget,
}

impl Log for TypeChecker<'_> {
    fn get_source(&self) -> String {
        "TypeChecker".to_string()
    }
}

impl<'ctx> TypeChecker<'ctx> {
    pub fn new(log_target: &'ctx dyn LogTarget, diagnostics: &'ctx mut Vec<Diagnostic>, global_table: &'ctx GlobalSymbolTable, symbol_table: &'ctx SymbolTable) -> Self {
        Self {
            diagnostics,
            global_table,
            symbol_table,
            functions_to_resolve: HashMap::new(),
            log_target,
        }
    }
}

impl<'ctx> ASTVisitor<'ctx, Option<TypeId>> for TypeChecker<'ctx> {
    fn visit_binary(&mut self, node: &'ctx BinaryExpr) -> Option<TypeId> {

        let left_type_id = node.left.accept_visitor(self)?;
        let right_type_id = node.right.accept_visitor(self)?;

        

        let left_type = self.global_table.type_arena.get(left_type_id);
        let right_type = self.global_table.type_arena.get(right_type_id);

        let mut valid = true;

        match node.operator {
            BinaryOperator::Plus | BinaryOperator::Minus | BinaryOperator::Times | BinaryOperator::Divide
            | BinaryOperator::Greater | BinaryOperator::GreaterEqual | BinaryOperator::Less | BinaryOperator::LessEqual => {
                if *left_type != *right_type {
                    valid = false;
                } else {
                    match *left_type {
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
                if *left_type != *right_type {
                    valid = false;
                } else {
                    self.symbol_table.ast_types.insert(node.get_id(), self.global_table.type_arena.bool());
                }
            },
            BinaryOperator::And | BinaryOperator::Or => {
                if *left_type != *right_type || *left_type != ResolvedType::Boolean {
                    valid = false;
                } else {
                    self.symbol_table.ast_types.insert(node.get_id(), left_type_id);
                }
            },
            BinaryOperator::BitwiseAnd | BinaryOperator::BitwiseOr | BinaryOperator::BitwiseXor
            | BinaryOperator::LeftShift | BinaryOperator::RightShift | BinaryOperator::Modulo => {
                if *left_type != *right_type || *left_type != ResolvedType::Integer {
                    valid = false;
                } else {
                    self.symbol_table.ast_types.insert(node.get_id(), left_type_id);
                }
            },
        }

        if !valid {
            // We get the type references again because we have potentially mutated the type arena
            let left_type = self.global_table.type_arena.get(left_type_id);
            let right_type = self.global_table.type_arena.get(right_type_id);

            self.diagnostics.push(
                ErrMsg::IncompatibleBinaryTypes(&left_type, &right_type, node.operator)
                .make_diagnostic(*node.get_position())
            );

            return None;
        }

        Some(*self.symbol_table.ast_types.get(&node.get_id()).unwrap())
    }

    fn visit_cast(&mut self, node: &'ctx CastExpr) -> Option<TypeId> {
        node.expr.accept_visitor(self);

        
        let type_id = self.symbol_table.resolve_type(&self.global_table.type_arena, &node.cast_type)?;

        self.symbol_table.ast_types.insert(node.get_id(), type_id);

        Some(type_id)
    }

    fn visit_unary(&mut self, node: &'ctx UnaryExpr) -> Option<TypeId> { 
        let expr_type_id = node.expr.accept_visitor(self)?;
        

        let expr_type = self.global_table.type_arena.get(expr_type_id);
        let mut valid = true;

        match node.operator {
            UnaryOperator::Negative => {
                match *expr_type {
                    ResolvedType::Integer | ResolvedType::Double => {
                        self.symbol_table.ast_types.insert(node.get_id(), expr_type_id);
                    },
                    _ => {
                        valid = false;
                    }
                }
            },
            UnaryOperator::Not => {
                if *expr_type != ResolvedType::Boolean {
                    valid = false;
                } else {
                    self.symbol_table.ast_types.insert(node.get_id(), expr_type_id);
                }
            },
            UnaryOperator::BitwiseNot => {
                if *expr_type != ResolvedType::Integer {
                    valid = false;
                } else {
                    self.symbol_table.ast_types.insert(node.get_id(), expr_type_id);
                }
            },
            UnaryOperator::AddressOf => {
                let ptr_type_id = self.global_table.type_arena.make_ptr(expr_type_id);
                self.symbol_table.ast_types.insert(node.get_id(), ptr_type_id);
            },
            UnaryOperator::Deref => {
                match *expr_type {
                    ResolvedType::Pointer(inner_type_id) => {
                        self.symbol_table.ast_types.insert(node.get_id(), inner_type_id);
                    },
                    _ => {
                        valid = false;
                    }
                }
            },
        }

        if !valid {
            // We get the type references again because we have potentially mutated the type arena
            let expr_type = self.global_table.type_arena.get(expr_type_id);
            self.diagnostics.push(
                ErrMsg::IncompatibleUnaryType(&expr_type, node.operator)
                .make_diagnostic(*node.get_position())
            )
        }

        Some(*self.symbol_table.ast_types.get(&node.get_id()).unwrap())
    }

    fn visit_literal(&mut self, node: &'ctx LiteralExpr) -> Option<TypeId> {
        

        match &node.value {
            Literal::Int(_) => {
                let int_type_id = self.global_table.type_arena.int();
                self.symbol_table.ast_types.insert(node.get_id(), int_type_id);
            },
            Literal::Double(_) => {
                let double_type_id = self.global_table.type_arena.double();
                self.symbol_table.ast_types.insert(node.get_id(), double_type_id);
            },
            Literal::Bool(_) => {
                let bool_type_id = self.global_table.type_arena.bool();
                self.symbol_table.ast_types.insert(node.get_id(), bool_type_id);
            },
            Literal::Char(_) => {
                let char_type_id = self.global_table.type_arena.char();
                self.symbol_table.ast_types.insert(node.get_id(), char_type_id);
            },
            Literal::String(_) => {
                let char_type_id = self.global_table.type_arena.char();
                let string_type_id = self.global_table.type_arena.make_ptr(char_type_id);
                self.symbol_table.ast_types.insert(node.get_id(), string_type_id);
            },
        }

        Some(*self.symbol_table.ast_types.get(&node.get_id()).unwrap())
    }

    fn visit_member_access(&mut self, node: &'ctx MemberAccess) -> Option<TypeId> {
        self.log_debug(self.log_target, format!("Visiting member access for id {:?}", node.get_id()));
        self.log_debug(self.log_target, format!("Node expr is {}", serde_json::to_string(&node.expr).unwrap()));
        let expr_type_id = node.expr.accept_visitor(self)?;
        self.log_debug(self.log_target, format!("Expression type id for member access: {:?}", expr_type_id));

        for member_access in &node.member_accesses {
            match member_access {
                AccessType::Direct(field_name) => {
                    match &*self.global_table.type_arena.get(expr_type_id) {
                        ResolvedType::Struct(struct_type) => {
                            if let Some(field_type) = (struct_type).members.get(field_name.as_ref()) {
                                self.symbol_table.ast_types.insert(node.get_id(), *field_type);
                            } else {
                                self.diagnostics.push(
                                    ErrMsg::FieldNotFound(field_name.to_string())
                                    .make_diagnostic(*field_name.get_position())
                                );
                                return None;
                            }
                        },
                        expr_type => {
                            self.diagnostics.push(
                                ErrMsg::IncompatibleMemberAccessType(&expr_type)
                                .make_diagnostic(*node.get_position())
                            );
                            return None;
                        }
                    }
                },
                AccessType::Indirect(field_name) => {
                    let expr_type = self.global_table.type_arena.get(expr_type_id);
                    match &*expr_type {
                        ResolvedType::Pointer(inner_type) => {
                            match &*self.global_table.type_arena.get(*inner_type) {
                                ResolvedType::Struct(struct_type) => {
                                    if let Some(field_type) = struct_type.members.get(field_name.as_ref()) {
                                        self.symbol_table.ast_types.insert(node.get_id(), *field_type);
                                    } else {
                                        self.diagnostics.push(
                                            ErrMsg::FieldNotFound(field_name.to_string())
                                            .make_diagnostic(*field_name.get_position())
                                        );

                                        return None;
                                    }
                                },
                                _ => {
                                    self.diagnostics.push(
                                        ErrMsg::IncompatibleMemberAccessType(&expr_type)
                                        .make_diagnostic(*node.get_position())
                                    );

                                    return None;
                                }
                            }
                        },
                        _ => {
                            self.diagnostics.push(
                                ErrMsg::IncompatibleMemberAccessType(&expr_type)
                                .make_diagnostic(*node.get_position())
                            );

                            return None;
                        }
                    }
                },
                AccessType::Array(index_expr) => {
                    match *self.global_table.type_arena.get(expr_type_id) {
                        ResolvedType::Array(inner_type) => {
                            self.symbol_table.ast_types.insert(node.get_id(), inner_type);
                        },
                        ResolvedType::Pointer(inner_type) => {
                            self.symbol_table.ast_types.insert(node.get_id(), inner_type);
                        }
                        _ => {
                            let expr_type = self.global_table.type_arena.get(expr_type_id);

                            self.diagnostics.push(
                                ErrMsg::IncompatibleMemberAccessType(&expr_type)
                                .make_diagnostic(*node.get_position())
                            );
                            return None;
                        }
                    }

                    let index_type_id = index_expr.accept_visitor(self)?;
                    let index_type = self.global_table.type_arena.get(index_type_id);

                    if *index_type != ResolvedType::Integer {
                        self.diagnostics.push(
                            ErrMsg::ArrayIndexNotInteger(&index_type)
                            .make_diagnostic(*index_expr.get_position())
                        );
                        return None;
                    }
                },
                AccessType::Function(arguments) => {
                    match &*self.global_table.type_arena.get(expr_type_id) {
                        ResolvedType::Function(func_sig) => {
                            if func_sig.param_types.len() != arguments.len() {
                                self.diagnostics.push(
                                    ErrMsg::FunctionArgumentCountMismatch(func_sig.param_types.len(), arguments.len())
                                    .make_diagnostic(*node.get_position())
                                );
                                return None;
                            }
                            
                           let func_sig = func_sig.clone();
            
                            for (i, arg) in arguments.iter().enumerate() {
                                let param_type_id = func_sig.param_types[i];
                                let arg_type_id = arg.accept_visitor(self)?;
            
                                if arg_type_id != param_type_id {
                                    let arg_type = self.global_table.type_arena.get(arg_type_id);
                                    let param_type = self.global_table.type_arena.get(param_type_id);
            
                                    self.diagnostics.push(
                                        ErrMsg::FunctionArgumentTypeMismatch(i, &param_type, &arg_type)
                                        .make_diagnostic(*arg.get_position())
                                    );
                                    return None;
                                }
                            }
            
                            self.symbol_table.ast_types.insert(node.get_id(), func_sig.return_type);
                        },
                        expr_type => {
                            self.diagnostics.push(
                                ErrMsg::CallOnNonFunctionType(&expr_type)
                                .make_diagnostic(*node.get_position())
                            );
                        }
                    }
                }
            }
        }

        self.log_debug(self.log_target, format!("Member access type id for id {:?} is {:?}", node.get_id(), self.symbol_table.ast_types.get(&node.get_id())));

        Some(*self.symbol_table.ast_types.get(&node.get_id()).unwrap())
    }

    fn visit_var(&mut self, node: &'ctx VarExpr) -> Option<TypeId> {
        self.log_debug(self.log_target, format!("Visiting variable expression {}", serde_json::to_string(&node).unwrap()));

        if let Some(decl_id) = self.symbol_table.variables.get(&node.get_id()) {
            let decl_type_id = *self.symbol_table.declaration_types.get(decl_id.value()).unwrap();
            self.symbol_table.ast_types.insert(node.get_id(), decl_type_id);

            return Some(decl_type_id);
        } else if let Some(fn_type_id) = self.symbol_table.functions.get(node.path.segments.first().unwrap().as_str()) {
            self.symbol_table.ast_types.insert(node.get_id(), *fn_type_id);
            self.log_debug(self.log_target, format!("Variable '{}' is a function with type id {:?}", node.path.segments.first().unwrap(), fn_type_id));
            return Some(*fn_type_id);
        } else {
            let path_vec = node.path.segments[0..node.path.segments.len() - 1]
                .iter()
                .map(|s| s.clone())
                .collect::<Vec<_>>();

            self.log_debug(self.log_target, format!("Looking up symbol table for path {:?}", path_vec));

            let symbol_table = self.global_table.scopes.get(&path_vec)?;

            self.log_debug(self.log_target, format!("Looking up function '{}' in symbol table", node.path.segments.last().unwrap()));

            // Print list of functions in symbol table
            let function_names: Vec<String> = symbol_table.functions.iter().map(|entry| entry.key().clone()).collect();
            self.log_debug(self.log_target, format!("Available functions in symbol table: {:?}", function_names));

            self.symbol_table.ast_types.insert(node.get_id(), *symbol_table.functions.get(node.path.segments.last().unwrap())?);

            self.log_debug(self.log_target, format!("Variable '{}' has type id {:?}", node.path.segments.last().unwrap(), self.symbol_table.ast_types.get(&node.get_id())));

            return symbol_table.functions.get(node.path.segments.last().unwrap()).map(|x| *x.value());
        }
    }

    fn visit_if(&mut self, node: &'ctx IfExpr) -> Option<TypeId> {
        let condition_type_id = node.condition.accept_visitor(self)?;
        
        let condition_type = self.global_table.type_arena.get(condition_type_id);

        if *condition_type != ResolvedType::Boolean {
            self.diagnostics.push(
                ErrMsg::IncompatibleUnaryType(&condition_type, UnaryOperator::Not)
                .make_diagnostic(*node.condition.get_position())
            );
            return None;
        }

        let then_type_id = node.success.accept_visitor(self).unwrap_or(self.global_table.type_arena.void());

        let else_type_id = if let Some(fail) = &node.fail {
            fail.accept_visitor(self).unwrap_or(self.global_table.type_arena.void())
        } else {
            self.global_table.type_arena.void()
        };   

        if then_type_id != else_type_id {
            let then_type = self.global_table.type_arena.get(then_type_id);
            let else_type = self.global_table.type_arena.get(else_type_id);

            self.diagnostics.push(
                ErrMsg::MismatchedIfBranches(&then_type, &else_type)
                .make_diagnostic(*node.get_position())
            );
            return None;
        }

        self.symbol_table.ast_types.insert(node.get_id(), then_type_id);
        Some(then_type_id)
    }

    fn visit_assignment(&mut self, node: &'ctx AssignmentExpr) -> Option<TypeId> {
        let assignee_type_id = node.assignee.accept_visitor(self)?;
        let expr_type_id = node.expr.accept_visitor(self)?;

        

        if assignee_type_id != expr_type_id {
            let assignee_type = self.global_table.type_arena.get(assignee_type_id);
            let expr_type = self.global_table.type_arena.get(expr_type_id);

            self.diagnostics.push(
                ErrMsg::IncompatibleAssignment(&assignee_type, &expr_type)
                .make_diagnostic(*node.get_position())
            );
            return None;
        }

        self.symbol_table.ast_types.insert(node.get_id(), assignee_type_id);
        Some(assignee_type_id)
    }

    fn visit_delete(&mut self, _node: &'ctx DeleteExpr) -> Option<TypeId> { 
        unimplemented!();
    }

    fn visit_declaration(&mut self, node: &'ctx DeclarationExpr) -> Option<TypeId> {

        if let Some(expr) = &node.expr {
            let init_type_id = expr.accept_visitor(self)?;
            
            let declaration_type_id = *self.symbol_table.declaration_types.get(&node.get_id()).unwrap();
    
            if init_type_id != declaration_type_id {
                let init_type = self.global_table.type_arena.get(init_type_id);
                let declaration_type = self.global_table.type_arena.get(declaration_type_id);
    
                self.diagnostics.push(
                    ErrMsg::IncompatibleAssignment(&declaration_type, &init_type)
                    .make_diagnostic(*node.get_position())
                );
            }
        }

        None
    }

    fn visit_block(&mut self, node: &'ctx BlockExpr) -> Option<TypeId> { 
        for expr in &node.exprs {
            expr.accept_visitor(self);
        }

        None
    }

    fn visit_loop(&mut self, node: &'ctx LoopExpr) -> Option<TypeId> {
        node.initial.as_ref().map(|init_expr| init_expr.accept_visitor(self));
        node.condition.as_ref().map(|cond_expr| cond_expr.accept_visitor(self));
        node.increment.as_ref().map(|inc_expr| inc_expr.accept_visitor(self));

        node.body.accept_visitor(self);

        None
    }

    fn visit_exit(&mut self, node: &'ctx ExitExpr) -> Option<TypeId> {
        if let Some(expr) = &node.expr {
            expr.accept_visitor(self)
        } else {
            Some(self.global_table.type_arena.void())
        }
    }

    fn visit_constructor_call(&mut self, node: &'ctx ConstructorCallExpr) -> Option<TypeId> {
        let mut arg_types = Vec::new();
        
        for arg in &node.arguments {
            arg_types.push(arg.accept_visitor(self)?);
        }

        let resolved_type = *self.symbol_table.types.get(node.type_name.as_ref())?;

        let fn_type = FunctionType {
            param_types: arg_types,
            return_type: resolved_type
        };

        
        let fn_type_id = self.global_table.type_arena.make_function(fn_type);

        if let ResolvedType::Struct(struct_type) = &*self.global_table.type_arena.get(resolved_type) {
            if struct_type.constructors.contains(&fn_type_id) {
                self.symbol_table.ast_types.insert(node.get_id(), resolved_type);
                return Some(resolved_type);
            } else {
                let resolved_type = self.global_table.type_arena.get(resolved_type);

                self.diagnostics.push(
                    ErrMsg::ConstructorNotFound(&resolved_type)
                    .make_diagnostic(*node.get_position())
                );
                return None;
            }
        } else {
            let resolved_type = self.global_table.type_arena.get(resolved_type);

            self.diagnostics.push(
                ErrMsg::ConstructorNotFound(&resolved_type)
                .make_diagnostic(*node.get_position())
            );

            return None;
        }
    }

    fn visit_new_array(&mut self, node: &'ctx NewArrayExpr) -> Option<TypeId> {
        
        let mut resolved_type_id = self.symbol_table.resolve_type(&self.global_table.type_arena, &node.array_type)?;

        for _ in 0..node.dimension {
            resolved_type_id = self.global_table.type_arena.make_ptr(resolved_type_id);
        }

        self.symbol_table.ast_types.insert(node.get_id(), resolved_type_id);
        Some(resolved_type_id)
    }

    fn visit_impl(&mut self, node: &'ctx ImplItem) -> Option<TypeId> { 
        for function in &node.functions {
            function.accept_visitor(self);
        }

        None
    }

    fn visit_function(&mut self, node: &'ctx FunctionItem) -> Option<TypeId> { 
        node.body.as_ref()?.accept_visitor(self)
    }

    fn visit_struct(&mut self, node: &'ctx StructItem) -> Option<TypeId> { 
        for constructor in &node.constructors {
            constructor.accept_visitor(self);
        }

        None
    }

    fn visit_constructor(&mut self, node: &'ctx ConstructorItem) -> Option<TypeId> { 
        node.body.accept_visitor(self)
    }

    fn visit_scope(&mut self, node: &'ctx Scope) -> Option<TypeId> {
        for item in &node.items {
            item.accept_visitor(self);
        }

        None
    }
}