use std::{collections::HashMap, fmt::Pointer, rc::Rc};

use crate::{environment::{FunctionType, PointerType, ResolvedType, StructType}, error::Diagnostic, expr::{item::{FunctionItem, Item, ItemVisitor, StructItem}, AssignmentExpr, BinaryExpr, BlockExpr, BreakExpr, CallExpr, DeclarationExpr, EmptyExpr, Expr, ExprVisitor, GetAddressExpr, GetCharExpr, IfExpr, InputExpr, LiteralExpr, LoopExpr, MemberAccess, PrintExpr, PutCharExpr, RandExpr, StaticArrayExpr, StructInitializerExpr, UnaryExpr, VarExpr}, resolver::SymbolTable};

pub struct TypeChecker<'a> {
    symbol_table: &'a SymbolTable,
    loop_types: Vec<Option<ResolvedType>>,
    current_loop_idx: Option<usize>
}

impl<'a> TypeChecker<'a> {
    pub fn new(symbol_table: &'a SymbolTable) -> TypeChecker {
        TypeChecker {
            symbol_table,
            loop_types: Vec::new(),
            current_loop_idx: None
        }
    }

    pub fn check_types(&mut self, items: &[Box<dyn Item>]) -> Vec<Diagnostic> {        
        for item in items {
            println!("Check types");
            item.accept_visitor(self);
        }

        Vec::new()
    }
}

impl ItemVisitor<()> for TypeChecker<'_> {
    fn visit_struct(&mut self, item: &StructItem) {  }

    fn visit_function(&mut self, item: &FunctionItem) {
        let return_type = item.expr.accept_visitor(self);

        if return_type != self.symbol_table.get_resolved_type(&item.ret_type) {
            panic!("Mismatched types {:?}, {:?} for function return", return_type, item.ret_type);
        }
    }
}

impl ExprVisitor<ResolvedType> for TypeChecker<'_> {
    fn visit_empty(&mut self, _expr: &EmptyExpr) -> ResolvedType {
        ResolvedType::Empty
    }


    fn visit_binary(&mut self, expr: &BinaryExpr) -> ResolvedType {
        let left_type = expr.left.accept_visitor(self);
        let right_type = expr.right.accept_visitor(self);

        println!("{:?} {:?} {:?}", left_type, expr.operator, right_type);

        expr.operator.interpret_type(left_type, right_type).unwrap()
    }

    fn visit_unary(&mut self, expr: &UnaryExpr) -> ResolvedType {
        let operand_type = expr.expr.accept_visitor(self);

        expr.operator.interpret_type(operand_type).unwrap()
    }

    fn visit_literal(&mut self, expr: &LiteralExpr) -> ResolvedType {
        self.symbol_table.get_resolved_type(&expr.parsed_type)
    }

    fn visit_var(&mut self, expr: &VarExpr) -> ResolvedType {
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
            println!("array access {:?}", array_access);
            let index_type = array_access.accept_visitor(self);

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

        base_type
    }

    fn visit_if(&mut self, expr: &IfExpr) -> ResolvedType {
        let condition_type = expr.condition.accept_visitor(self);

        if condition_type != ResolvedType::Boolean {
            panic!("Invalid type {:?} for if condition", condition_type);
        }

        let success_type = expr.success.accept_visitor(self);

        if let Some(fail) = &expr.fail {
            let fail_type = fail.accept_visitor(self);

            println!("{:?}, {:?}", success_type, fail_type);
            
            if success_type != fail_type {
                panic!("Mismatched types {:?}, {:?} for if branches", success_type, fail_type);
            }
        }

        success_type
    }

    fn visit_assignment(&mut self, expr: &AssignmentExpr) -> ResolvedType {
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
                        var_type = struct_type.get_member_type(member_name).clone();
                    } else {
                        panic!("Cannot access member of non-struct type");
                    }
                }
            }
        }

        for array_access in expr.asignee.array_accesses.iter() {
            let index_type = array_access.accept_visitor(self);

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

        let value_type = expr.expr.accept_visitor(self);

        //println!("assigning {:?} to {:?}", var_type, value_type);

        if var_type != value_type {
            panic!("Mismatched types {:?}, {:?} for assignment", var_type, value_type);
        }

        var_type.clone()
    }

    fn visit_declaration(&mut self, expr: &DeclarationExpr) -> ResolvedType {
        let value_type = expr.expr.accept_visitor(self);

        println!("declaring {:?} as {:?}", expr.identifier, value_type);

        if value_type != self.symbol_table.get_resolved_type(&expr.declaration_type) {
            //panic!("Mismatched types {:?}, {:?} for assignment", value_type, expr.declaration_type);
        }

        value_type
    }

    fn visit_block(&mut self, expr: &BlockExpr) -> ResolvedType {
        let mut block_type = ResolvedType::Empty;

        for expr in &expr.exprs {
            block_type = expr.accept_visitor(self);
        }

        block_type
    }

    fn visit_print(&mut self, expr: &PrintExpr) -> ResolvedType {
        expr.expr.accept_visitor(self)
    }

    fn visit_rand(&mut self, expr: &RandExpr) -> ResolvedType {
        let min_type = expr.min.accept_visitor(self);
        let max_type = expr.max.accept_visitor(self);

        if min_type != ResolvedType::Integer || max_type != ResolvedType::Integer {
            panic!("Invalid types {:?}, {:?} for rand expression", min_type, max_type);
        }

        ResolvedType::Integer
    }

    fn visit_loop(&mut self, expr: &LoopExpr) -> ResolvedType {
        let loop_idx = self.loop_types.len();
        self.current_loop_idx = Some(loop_idx);
        self.loop_types.push(None);

        expr.body.accept_visitor(self);

        return match &self.loop_types[loop_idx] {
            Some(t) => t.clone(),
            None => ResolvedType::Empty
        }
    }

    fn visit_break(&mut self, expr: &BreakExpr) -> ResolvedType {
        let expr_type = expr.expr.accept_visitor(self);

        if self.current_loop_idx.is_none() {
            panic!("Break expression outside of loop");
        }

        let current_loop_idx = *self.current_loop_idx.as_ref().unwrap();

        match &self.loop_types[current_loop_idx] {
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
        }
    }

    fn visit_input(&mut self, expr: &InputExpr) -> ResolvedType {
        self.symbol_table.get_resolved_type(&expr.return_type).clone()
    }

    fn visit_call(&mut self, expr: &CallExpr) -> ResolvedType {
        let expr_type = self.symbol_table.get_function(&expr.function.identifier);

        println!("calling {:?}", expr_type);

        if let ResolvedType::Function(function_type) = &expr_type {
            if function_type.arg_types.len() != expr.args.len() {
                panic!("Mismatched number of arguments {:?}, {:?} for function call", function_type.arg_types.len(), expr.args.len());
            }


            expr.args.iter().enumerate().for_each(|(i, arg)| {

                let arg_type = arg.accept_visitor(self);
                println!("Function arg {:?} given {:?}", function_type.arg_types[i], arg_type);

                if arg_type != function_type.arg_types[i] {
                    panic!("Mismatched types {:?}, {:?} for function call", arg_type, function_type.arg_types[i]);
                }
            });
        }

        match expr_type {
            ResolvedType::Function(function_type) => (*function_type.ret_type).clone(),
            _ => panic!("Invalid type {:?} for function call", &expr_type)
        }
    }

    fn visit_struct_initializer(&mut self, expr: &StructInitializerExpr) -> ResolvedType {
        let mut member_types = Vec::new();

        for entry in expr.member_inits.iter() {
            member_types.push((entry.0.clone(), entry.1.accept_visitor(self)));
        }

        ResolvedType::Struct(StructType::new(member_types))
    }

    fn visit_get_address(&mut self, expr: &GetAddressExpr) -> ResolvedType {
        ResolvedType::Pointer(crate::environment::PointerType {
            pointee: Rc::new(self.visit_var(&expr.var_expr))
        })
    }

    fn visit_static_array(&mut self, expr: &StaticArrayExpr) -> ResolvedType {
        ResolvedType::Pointer(PointerType {
            pointee: Rc::new(self.symbol_table.get_resolved_type(&expr.declaration_type))
        })
    }

    fn visit_get_char(&mut self, expr: &GetCharExpr) -> ResolvedType {
        ResolvedType::Integer
    }

    fn visit_put_char(&mut self, expr: &PutCharExpr) -> ResolvedType {
        ResolvedType::Empty
    }
}