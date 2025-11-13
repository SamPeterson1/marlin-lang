use std::rc::Rc;

use crate::{error::Diagnostic, expr::{ExprVisitor, assignment_expr::AssignmentExpr, binary_expr::BinaryExpr, block_expr::BlockExpr, break_expr::BreakExpr, call_expr::CallExpr, declaration_expr::DeclarationExpr, get_address_expr::GetAddressExpr, get_char_expr::GetCharExpr, if_expr::IfExpr, literal_expr::LiteralExpr, loop_expr::LoopExpr, put_char_expr::PutCharExpr, static_array_expr::StaticArrayExpr, struct_initializer_expr::StructInitializerExpr, unary_expr::UnaryExpr, var_expr::{MemberAccess, VarExpr}}, item::{FunctionItem, Item, ItemVisitor, StructItem}, logger::{LogSource, Logger}, resolver::SymbolTable, types::resolved_type::{PointerType, ResolvedType, StructType}};

pub struct TypeChecker<'a> {
    symbol_table: &'a SymbolTable,
    loop_types: Vec<Option<ResolvedType>>,
    current_loop_idx: Option<usize>,
    diagnostics: Vec<Diagnostic>
}

impl LogSource for TypeChecker<'_> {
    fn get_source(&self) -> String {
        "TypeChecker".to_string()
    }
}

impl<'a> TypeChecker<'a> {
    pub fn new(symbol_table: &'a SymbolTable) -> TypeChecker {
        TypeChecker {
            symbol_table,
            loop_types: Vec::new(),
            current_loop_idx: None,
            diagnostics: Vec::new()
        }
    }

    pub fn check_types(mut self, items: &[Box<dyn Item>]) -> Vec<Diagnostic> {        
        for item in items {
            item.accept_visitor(&mut self);
        }

        self.diagnostics
    }

    fn push_diagnostic(&mut self, diagnostic: Diagnostic) {
        Logger::log_error(self, &format!("Pushing diagnostic: {}", diagnostic));
        self.diagnostics.push(diagnostic);
    }
}

impl ItemVisitor<()> for TypeChecker<'_> {
    fn visit_struct(&mut self, item: &StructItem) {  }

    fn visit_function(&mut self, item: &FunctionItem) {
        let return_type = item.expr.accept_visitor(self);

        if return_type.is_none() {
            return;
        }

        let function_ret_type = self.symbol_table.get_resolved_type(&item.ret_type);

        match function_ret_type {
            Ok(function_ret_type) => {
                if &function_ret_type != return_type.as_ref().unwrap() {
                    panic!("Mismatched types {:?}, {:?} for function return", function_ret_type, return_type.unwrap());
                }
            },
            Err(diagnostic) => {
                self.diagnostics.push(diagnostic);
            }
         }
    }
}

impl ExprVisitor<Option<ResolvedType>> for TypeChecker<'_> {
    fn visit_binary(&mut self, expr: &BinaryExpr) -> Option<ResolvedType> {
        let left_type = expr.left.accept_visitor(self)?;
        let right_type = expr.right.accept_visitor(self)?;

        match expr.operator.interpret_type(left_type, right_type) {
            Ok(t) => Some(t),
            Err(diagnostic) => {
                self.push_diagnostic(diagnostic);
                None
            }
        }
    }

    fn visit_unary(&mut self, expr: &UnaryExpr) -> Option<ResolvedType> {
        let operand_type = expr.expr.accept_visitor(self)?;

        match expr.operator.interpret_type(operand_type) {
            Ok(t) => Some(t),
            Err(diagnostic) => {
                self.push_diagnostic(diagnostic);
                None
            }
        }
    }

    fn visit_literal(&mut self, expr: &LiteralExpr) -> Option<ResolvedType> {
        Some(self.symbol_table.get_resolved_type(&expr.parsed_type).unwrap())
    }

    fn visit_var(&mut self, expr: &VarExpr) -> Option<ResolvedType> {
        let mut base_type = self.symbol_table.get_variable(expr).value_type.clone();

        for member_access in expr.member_accesses.iter() {
            match member_access {
                MemberAccess::Indirect(member_name) => {
                    if let ResolvedType::Pointer(pointer_type) = base_type {
                        if let ResolvedType::Struct(struct_type) = &*pointer_type.pointee {
                            base_type = struct_type.get_member_type(member_name).clone();
                        } else {
                            panic!("Cannot access member of non-struct type");
                        }
                    } else {
                        panic!("Cannot dereference non-pointer type");
                    }
                },
                MemberAccess::Direct(member_name) => {
                    if let ResolvedType::Struct(struct_type) = base_type {
                        base_type = struct_type.get_member_type(member_name).clone();
                    } else {
                        panic!("Cannot access member of non-struct type");
                    }
                }
            }
        }

        for array_access in expr.array_accesses.iter() {
            println!("array access {}", array_access);
            let index_type = array_access.accept_visitor(self)?;

            if index_type != ResolvedType::Integer {
                panic!("Cannot access array with non-integer index");
            }

            if let ResolvedType::Pointer(pointer_type) = base_type {
                base_type = (*pointer_type.pointee).clone();
            } else {
                panic!("Cannot perform array access on non-pointer type");
            }
        }

        for _ in 0..expr.n_derefs {
            if let ResolvedType::Pointer(pointer_type) = base_type {
                base_type = (*pointer_type.pointee).clone();
            } else {
                panic!("Cannot dereference non-pointer type");
            }
        }

        Some(base_type)
    }

    fn visit_if(&mut self, expr: &IfExpr) -> Option<ResolvedType> {
        let condition_type = expr.condition.accept_visitor(self)?;

        if condition_type != ResolvedType::Boolean {
            panic!("Invalid type {:?} for if condition", condition_type);
        }

        let success_type = expr.success.accept_visitor(self)?;

        if let Some(fail) = &expr.fail {
            let fail_type = fail.accept_visitor(self)?;

            println!("{:?}, {:?}", success_type, fail_type);
            
            if success_type != fail_type {
                panic!("Mismatched types {:?}, {:?} for if branches", success_type, fail_type);
            }
        }

        Some(success_type)
    }

    fn visit_assignment(&mut self, expr: &AssignmentExpr) -> Option<ResolvedType> {
        let mut var_type = self.symbol_table.get_variable(&expr.asignee).value_type.clone();
        
        for member_access in expr.asignee.member_accesses.iter() {
            match member_access {
                MemberAccess::Indirect(member_name) => {
                    if let ResolvedType::Pointer(pointer_type) = var_type {
                        if let ResolvedType::Struct(struct_type) = &*pointer_type.pointee {
                            var_type = struct_type.get_member_type(member_name).clone();
                        } else {
                            panic!("Cannot access member of non-struct type");
                        }
                    } else {
                        panic!("Cannot dereference non-pointer type");
                    }
                },
                MemberAccess::Direct(member_name) => {
                    if let ResolvedType::Struct(struct_type) = var_type {
                        println!("accessing member {:?}", member_name);
                        var_type = struct_type.get_member_type(member_name).clone();
                    } else {
                        panic!("Cannot access member of non-struct type");
                    }
                }
            }
        }

        for array_access in expr.asignee.array_accesses.iter() {
            let index_type = array_access.accept_visitor(self)?;

            if index_type != ResolvedType::Integer {
                panic!("Cannot access array with non-integer index");
            }

            if let ResolvedType::Pointer(pointer_type) = var_type {
                var_type = (*pointer_type.pointee).clone();
            } else {
                panic!("Cannot perform array access on non-pointer type");
            }
        }

        for _ in 0..expr.asignee.n_derefs {
            if let ResolvedType::Pointer(pointer_type) = var_type {
                var_type = (*pointer_type.pointee).clone();
            } else {
                panic!("Cannot dereference non-pointer type");
            }
        }

        let value_type = expr.expr.accept_visitor(self)?;

        //println!("assigning {:?} to {:?}", var_type, value_type);

        if var_type != value_type {
            //panic!("Mismatched types {:?}, {:?} for assignment", var_type, value_type);
        }

        Some(ResolvedType::Empty)
    }

    fn visit_declaration(&mut self, expr: &DeclarationExpr) -> Option<ResolvedType> {
        let value_type = expr.expr.accept_visitor(self)?;

        println!("declaring {:?} as {:?}", expr.identifier, value_type);

        let declaration_type = match self.symbol_table.get_resolved_type(&expr.declaration_type) {
            Ok(t) => t,
            Err(diagnostic) => {
                self.push_diagnostic(diagnostic);
                return None;
            }
        };

        if value_type != declaration_type {
            //panic!("Mismatched types {:?}, {:?} for assignment", value_type, expr.declaration_type);
        }

        Some(ResolvedType::Empty)
    }

    fn visit_block(&mut self, expr: &BlockExpr) -> Option<ResolvedType> {
        let mut block_type = Some(ResolvedType::Empty);

        for expr in &expr.exprs {
            block_type = expr.accept_visitor(self);
        }

        block_type
    }

    fn visit_loop(&mut self, expr: &LoopExpr) -> Option<ResolvedType> {
        let loop_idx = self.loop_types.len();
        self.current_loop_idx = Some(loop_idx);
        self.loop_types.push(None);

        expr.body.accept_visitor(self);

        Some(match &self.loop_types[loop_idx] {
            Some(t) => t.clone(),
            None => ResolvedType::Empty
        })
    }

    fn visit_break(&mut self, expr: &BreakExpr) -> Option<ResolvedType> {
        let expr_type = expr.expr.accept_visitor(self)?;

        if self.current_loop_idx.is_none() {
            panic!("Break expression outside of loop");
        }

        let current_loop_idx = *self.current_loop_idx.as_ref().unwrap();

        Some(match &self.loop_types[current_loop_idx] {
            Some(t) => {
                if *t != expr_type {
                    panic!("Mismatched types {:?}, {:?} for break expression", t, expr_type);
                } else {
                    t.clone()
                }
            },
            None => {
                self.loop_types[current_loop_idx] = Some(expr_type.clone());
                expr_type
            }
        })
    }

    fn visit_call(&mut self, expr: &CallExpr) -> Option<ResolvedType> {
        let expr_type = self.symbol_table.get_function(&expr.function);

        println!("calling {:?}", expr_type);

        if let ResolvedType::Function(function_type) = &expr_type {
            if function_type.arg_types.len() != expr.args.len() {
                panic!("Mismatched number of arguments {:?}, {:?} for function call", function_type.arg_types.len(), expr.args.len());
            }


            for (i, arg) in expr.args.iter().enumerate() {
                let arg_type = arg.accept_visitor(self)?;
                println!("Function arg {:?} given {:?}", function_type.arg_types[i], arg_type);

                if arg_type != function_type.arg_types[i] {
                    panic!("Mismatched types {:?}, {:?} for function call", arg_type, function_type.arg_types[i]);
                }
            }
        }

        Some(match expr_type {
            ResolvedType::Function(function_type) => (*function_type.ret_type).clone(),
            _ => panic!("Invalid type {:?} for function call", &expr_type)
        })
    }

    fn visit_struct_initializer(&mut self, expr: &StructInitializerExpr) -> Option<ResolvedType> {
        let mut member_types = Vec::new();

        for entry in expr.member_inits.iter() {
            member_types.push((entry.0.clone(), entry.1.accept_visitor(self)?));
        }

        Some(ResolvedType::Struct(StructType::new(member_types)))
    }

    fn visit_get_address(&mut self, expr: &GetAddressExpr) -> Option<ResolvedType> {
        Some(ResolvedType::Pointer(PointerType {
            pointee: Rc::new(self.visit_var(&expr.var_expr)?)
        }))
    }

    fn visit_static_array(&mut self, expr: &StaticArrayExpr) -> Option<ResolvedType> {
        Some(ResolvedType::Pointer(PointerType {
            pointee: Rc::new(self.symbol_table.get_resolved_type(&expr.declaration_type).unwrap())
        }))
    }

    fn visit_get_char(&mut self, expr: &GetCharExpr) -> Option<ResolvedType> {
        Some(ResolvedType::Integer)
    }

    fn visit_put_char(&mut self, expr: &PutCharExpr) -> Option<ResolvedType> {
        Some(ResolvedType::Empty)
    }
}