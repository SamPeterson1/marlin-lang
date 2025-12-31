use core::str;
use std::{collections::{HashMap, VecDeque}, f32::consts::E, hash::Hash, path::Path, process::Command, sync::MutexGuard};

use inkwell::{AddressSpace, basic_block::BasicBlock, builder::Builder, context::Context, llvm_sys::prelude::LLVMValueRef, module::Module, types::BasicTypeEnum, values::{AnyValueEnum, AsValueRef, BasicValue, BasicValueEnum, FunctionValue, IntValue, PointerValue}};

use crate::{ast::*, logger::Log, resolver::{GlobalSymbolTable, ResolvedType, SymbolTable, TypeArena}};

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    value: LLVMValueRef,
    local_vars: HashMap<AstId, PointerValue<'ctx>>,
    break_block: HashMap<String, BasicBlock<'ctx>>,
    result_block: Option<BasicBlock<'ctx>>,
    global_table: &'ctx GlobalSymbolTable,
    symbol_table: &'ctx SymbolTable,
    type_arena: Option<MutexGuard<'ctx, TypeArena>>,
    break_values: HashMap<String, Vec<(BasicBlock<'ctx>, LLVMValueRef)>>,
    functions: HashMap<String, FunctionValue<'ctx>>,
    struct_types: HashMap<String, inkwell::types::StructType<'ctx>>,
    struct_field_maps: HashMap<String, HashMap<String, u32>>,
    loop_labels: VecDeque<String>,
    lvalue_mode: bool,
}

impl Log for CodeGen<'_> {
    fn get_source(&self) -> String {
        "CodeGen".to_string()
    }
}

impl<'ctx> CodeGen<'ctx> {
    unsafe fn get_basic_value(&self, resolved_type: &ResolvedType, value: LLVMValueRef) -> BasicValueEnum<'_> {
        match resolved_type {
            ResolvedType::Boolean | ResolvedType::Integer | ResolvedType::Char => {
                unsafe { IntValue::new(value).into() }
            },
            ResolvedType::Pointer(_) => {
                unsafe { IntValue::new(value).into() }
            },
            _ => unimplemented!("Type not implemented"),
        }
    }

    fn get_int_type(&self, resolved_type: &ResolvedType) -> inkwell::types::IntType<'_> {
        match resolved_type {
            ResolvedType::Boolean => self.context.bool_type(),
            ResolvedType::Integer => self.context.i32_type(),
            ResolvedType::Char => self.context.i8_type(),
            _ => unimplemented!("Type not implemented"),
        }
    }

    fn get_type(&self, resolved_type: &ResolvedType) -> BasicTypeEnum<'ctx> {
        match resolved_type {
            ResolvedType::Boolean => self.context.bool_type().into(),
            ResolvedType::Integer => self.context.i32_type().into(),
            ResolvedType::Char => self.context.i8_type().into(),
            ResolvedType::Struct(struct_type) => {
                self.struct_types.get(&struct_type.name).unwrap().clone().into()
            }
            ResolvedType::Pointer(_) => self.context.ptr_type(AddressSpace::default()).into(),
            _ => unimplemented!("Type not implemented"),
        }
    }
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, global_table: &'ctx GlobalSymbolTable, symbol_table: &'ctx SymbolTable) -> Self {
        let module = context.create_module("main_module");
        let builder = context.create_builder();
        
        let mut functions = HashMap::new();

        let i32_type = context.i32_type();

        let putchar_type = i32_type.fn_type(&[i32_type.into()], false);
        let putchar = module.add_function("putchar", putchar_type, None);
        functions.insert("putchar".to_string(), putchar);

        let getchar_type = i32_type.fn_type(&[], false);
        let getchar = module.add_function("getchar", getchar_type, None);
        functions.insert("getchar".to_string(), getchar);

        Self {
            context,
            module,
            builder,
            value: std::ptr::null_mut(),
            local_vars: HashMap::new(),
            break_block: HashMap::new(),
            result_block: None,
            global_table,
            symbol_table,
            type_arena: None,
            break_values: HashMap::new(),
            functions,
            struct_types: HashMap::new(),
            struct_field_maps: HashMap::new(),
            loop_labels: VecDeque::new(),
            lvalue_mode: false,
        }
    }

    pub fn compile_with_clang(&self, output_file: &str) -> Result<(), String> {
        // First, write the LLVM IR to a file
        self.module.print_to_file(Path::new("output.ll"))
            .map_err(|e| format!("Failed to write LLVM IR: {}", e))?;

        // Compile the LLVM IR to an executable using clang
        let output = Command::new("clang")
            .arg("output.ll")
            .arg("-o")
            .arg(output_file)
            .output()
            .map_err(|e| format!("Failed to execute clang: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Clang compilation failed:\n{}", stderr));
        }

        println!("Successfully compiled to: {}", output_file);
        Ok(())
    }
}

impl ASTVisitor<'_, ()> for CodeGen<'_> {
    fn visit_binary(&mut self, node: &BinaryExpr) {
        node.left.accept_visitor(self);
        let left_value = self.value;
        
        node.right.accept_visitor(self);
        let right_value = self.value;
        
        println!("{left_value:?} {right_value:?}");

        match node.operator {
            BinaryOperator::Plus => {
                let left = unsafe { IntValue::new(left_value) };
                let right = unsafe { IntValue::new(right_value) };
                let result = self.builder.build_int_add(left, right, "addtmp").unwrap();
                self.value = result.as_value_ref();
            },
            BinaryOperator::Minus => {
                let left = unsafe { IntValue::new(left_value) };
                let right = unsafe { IntValue::new(right_value) };
                let result = self.builder.build_int_sub(left, right, "subtmp").unwrap();
                self.value = result.as_value_ref();
            },
            BinaryOperator::Times => {
                let left = unsafe { IntValue::new(left_value) };
                let right = unsafe { IntValue::new(right_value) };
                let result = self.builder.build_int_mul(left, right, "multmp").unwrap();
                self.value = result.as_value_ref();
            },
            BinaryOperator::Divide => {
                let left = unsafe { IntValue::new(left_value) };
                let right = unsafe { IntValue::new(right_value) };
                let result = self.builder.build_int_signed_div(left, right, "divtmp").unwrap();
                self.value = result.as_value_ref();
            },
            BinaryOperator::Equal => {
                let left = unsafe { IntValue::new(left_value) };
                let right = unsafe { IntValue::new(right_value) };
                let result = self.builder.build_int_compare(
                    inkwell::IntPredicate::EQ,
                    left,
                    right,
                    "eqtmp",
                ).unwrap();
                self.value = result.as_value_ref();
            },
            BinaryOperator::NotEqual => {
                let left = unsafe { IntValue::new(left_value) };
                let right = unsafe { IntValue::new(right_value) };
                let result = self.builder.build_int_compare(
                    inkwell::IntPredicate::NE,
                    left,
                    right,
                    "netmp",
                ).unwrap();
                self.value = result.as_value_ref();
            },
            BinaryOperator::Less => { 
                let left = unsafe { IntValue::new(left_value) };
                let right = unsafe { IntValue::new(right_value) };
                let result = self.builder.build_int_compare(
                    inkwell::IntPredicate::SLT,
                    left,
                    right,
                    "lttmp",
                ).unwrap();
                self.value = result.as_value_ref();
            },
            BinaryOperator::LessEqual => {
                let left = unsafe { IntValue::new(left_value) };
                let right = unsafe { IntValue::new(right_value) };
                let result = self.builder.build_int_compare(
                    inkwell::IntPredicate::SLE,
                    left,
                    right,
                    "letmp",
                ).unwrap();
                self.value = result.as_value_ref();
            },
            BinaryOperator::Greater => {
                let left = unsafe { IntValue::new(left_value) };
                let right = unsafe { IntValue::new(right_value) };
                let result = self.builder.build_int_compare(
                    inkwell::IntPredicate::SGT,
                    left,
                    right,
                    "gttmp",
                ).unwrap();
                self.value = result.as_value_ref();
            },
            BinaryOperator::GreaterEqual => {
                let left = unsafe { IntValue::new(left_value) };
                let right = unsafe { IntValue::new(right_value) };
                let result = self.builder.build_int_compare(
                    inkwell::IntPredicate::SGE,
                    left,
                    right,
                    "getmp",
                ).unwrap();
                self.value = result.as_value_ref();
            },
            BinaryOperator::BitwiseAnd => {
                let left = unsafe { IntValue::new(left_value) };
                let right = unsafe { IntValue::new(right_value) };
                let result = self.builder.build_and(left, right, "andtmp").unwrap();
                self.value = result.as_value_ref();
            },
            BinaryOperator::BitwiseOr => {
                let left = unsafe { IntValue::new(left_value) };
                let right = unsafe { IntValue::new(right_value) };
                let result = self.builder.build_or(left, right, "ortmp").unwrap();
                self.value = result.as_value_ref();
            },
            BinaryOperator::Modulo => {
                let left = unsafe { IntValue::new(left_value) };
                let right = unsafe { IntValue::new(right_value) };
                let result = self.builder.build_int_signed_rem(left, right, "modtmp").unwrap();
                self.value = result.as_value_ref();
            },
            BinaryOperator::Or => {
                let left = unsafe { IntValue::new(left_value) };
                let right = unsafe { IntValue::new(right_value) };
                let result = self.builder.build_or(left, right, "ortmp").unwrap();
                self.value = result.as_value_ref();
            },
            BinaryOperator::And => {
                let left = unsafe { IntValue::new(left_value) };
                let right = unsafe { IntValue::new(right_value) };
                let result = self.builder.build_and(left, right, "andtmp").unwrap();
                self.value = result.as_value_ref();
            },
            _ => {
                println!("Operator {} not implemented", node.operator);
                unimplemented!("Binary operator not implemented")
            }
        }
    }

    fn visit_cast(&mut self, node: &CastExpr) {
        node.expr.accept_visitor(self);

        let target_type = self.type_arena.as_ref().unwrap().get(self.symbol_table.ast_types.get(&node.get_id()).unwrap());
        let int_val = unsafe { IntValue::new(self.value) };

        self.value = self.builder.build_int_cast(int_val, self.get_int_type(target_type), "casttmp").unwrap().as_value_ref();
    }

    fn visit_unary(&mut self, node: &UnaryExpr) {
        match node.operator {
            UnaryOperator::Negative => {
                node.expr.accept_visitor(self);
                let expr = unsafe { IntValue::new(self.value) };
                let zero = self.context.i32_type().const_int(0, false);
                let result = self.builder.build_int_sub(zero, expr, "negtmp").unwrap();
                self.value = result.as_value_ref();
            },
            UnaryOperator::Not => {
                node.expr.accept_visitor(self);
                let expr = unsafe { IntValue::new(self.value) };
                let one = self.context.i32_type().const_int(1, false);
                let result = self.builder.build_xor(expr, one, "nottmp").unwrap();
                self.value = result.as_value_ref();
            },
            UnaryOperator::BitwiseNot => {
                node.expr.accept_visitor(self);
                let expr = unsafe { IntValue::new(self.value) };
                let all_ones = self.context.i32_type().const_all_ones();
                let result = self.builder.build_xor(expr, all_ones, "bitwisenottmp").unwrap();
                self.value = result.as_value_ref();
            },
            UnaryOperator::Deref => {
                if self.lvalue_mode {
                    self.lvalue_mode = false;
                    node.expr.accept_visitor(self);
                } else {
                    node.expr.accept_visitor(self);
                    let ptr = unsafe { PointerValue::new(self.value) };
                    let loaded = self.builder.build_load(
                        self.context.i32_type(),
                        ptr,
                        "derefload",
                    ).unwrap();
                    self.value = loaded.as_value_ref();
                }
            },
            UnaryOperator::AddressOf => {
                self.lvalue_mode = true;
                node.expr.accept_visitor(self);
                self.lvalue_mode = false;
            },
            _ => unimplemented!("Unary operator not implemented"),
        }
    }

    fn visit_literal(&mut self, node: &LiteralExpr) {
        match &node.value {
            Literal::Int(i) => {
                let int_type = self.context.i32_type();
                let int_value = int_type.const_int(*i as u64, false);
                self.value = int_value.as_value_ref();
            },
            Literal::Char(c) => {
                let int_type = self.context.i8_type();
                let int_value = int_type.const_int(*c as u64, false);
                self.value = int_value.as_value_ref();
            },
            Literal::String(s) => {
                let string_value = self.builder.build_global_string_ptr(s, "strtmp").unwrap();
                self.value = string_value.as_value_ref();
            }
            _ => unimplemented!("Literal type not implemented"),
        }
    }

    fn visit_member_access(&mut self, node: &MemberAccess) {
        let mut current_type_id = *self.symbol_table.ast_types.get(&node.expr.get_id()).unwrap();

        for access in &node.member_accesses {
            match &access {
                AccessType::Function(args) => {
                    node.expr.accept_visitor(self);

                    let function = unsafe { FunctionValue::new(self.value).unwrap() };
                    let mut arg_values = Vec::new();

                    for arg in &args.args {
                        let arg_type = self.symbol_table.ast_types.get(&arg.get_id()).unwrap();
                        arg.accept_visitor(self);

                        match self.type_arena.as_ref().unwrap().get(arg_type) {
                            ResolvedType::Boolean | ResolvedType::Integer | ResolvedType::Char => {
                                let int_value = unsafe { IntValue::new(self.value) };
                                arg_values.push(int_value.into());
                            },
                            ResolvedType::Pointer(_) => {
                                let ptr_value = unsafe { PointerValue::new(self.value) };
                                arg_values.push(ptr_value.into());
                            },
                            _ => unimplemented!("Argument type not implemented"),
                        }
                    }

                    let call_site = self.builder.build_call(
                        function,
                        &arg_values,
                        "calltmp",
                    ).unwrap();

                    self.value = call_site.as_value_ref();
                },
                AccessType::Array(index_expr) => {
                    let lvalue_mode_backup = self.lvalue_mode;


                    self.lvalue_mode = false;
                    node.expr.accept_visitor(self);
                    let ptr = unsafe { PointerValue::new(self.value) };

                    index_expr.accept_visitor(self);
                    self.lvalue_mode = lvalue_mode_backup;

                    let index_value = unsafe { IntValue::new(self.value) };

                    current_type_id = *match self.type_arena.as_ref().unwrap().get(&current_type_id) {
                        ResolvedType::Pointer(pointee_type_id) => pointee_type_id,
                        ResolvedType::Array(element_type_id) => element_type_id,
                        _ => panic!("Expected pointer or array type for array access"),
                    };

                    let pointee_ty = self.get_type(&self.type_arena.as_ref().unwrap().get(&current_type_id));
                    println!("GEP pointee type: {:?}, index value {:?}", pointee_ty, index_value);


                    let gep = unsafe {
                        self.builder.build_in_bounds_gep(
                            pointee_ty,
                            ptr,
                            &[index_value],
                            "arrayidx",
                        ).unwrap()
                    };

                    if self.lvalue_mode {
                        self.value = gep.as_value_ref();
                        return;
                    }

                    let loaded = self.builder.build_load(
                        pointee_ty,
                        gep,
                        "arrayload",
                    ).unwrap();

                    self.value = loaded.as_value_ref();
                },
                AccessType::Direct(member_name) => {
                    let lvalue_mode_backup = self.lvalue_mode;
                    self.lvalue_mode = true;
                    node.expr.accept_visitor(self);
                    let struct_ptr = unsafe { PointerValue::new(self.value) };
                    self.lvalue_mode = lvalue_mode_backup;

                    let struct_type = self.type_arena.as_ref().unwrap().get(&current_type_id);
                    let struct_field_map = match struct_type {
                        ResolvedType::Struct(struct_type) => {
                            self.struct_field_maps.get(&struct_type.name).unwrap()
                        },
                        _ => panic!("Expected struct type for member access"),
                    };

                    let member_index = *struct_field_map.get(&member_name.data).unwrap();

                    let gep = unsafe {
                        self.builder.build_in_bounds_gep(
                            self.get_type(struct_type),
                            struct_ptr,
                            &[
                                self.context.i32_type().const_int(0, false),
                                self.context.i32_type().const_int(member_index as u64, false),
                            ],
                            "memberidx",
                        ).unwrap()
                    };

                    if self.lvalue_mode {
                        self.value = gep.as_value_ref();
                        return;
                    }

                    let member_type_id = match struct_type {
                        ResolvedType::Struct(struct_type) => {
                            struct_type.members.get(&member_name.data).unwrap()
                        },
                        _ => panic!("Expected struct type for member access"),
                    };

                    let pointee_ty = self.get_type(&self.type_arena.as_ref().unwrap().get(member_type_id));

                    let loaded = self.builder.build_load(
                        pointee_ty,
                        gep,
                        "memberload",
                    ).unwrap();

                    self.value = loaded.as_value_ref();

                    current_type_id = *member_type_id;
                },
                AccessType::Indirect(member_name) => {
                    let lvalue_mode_backup = self.lvalue_mode;
                    self.lvalue_mode = true;
                    node.expr.accept_visitor(self);
                    let struct_ptr_ptr = unsafe { PointerValue::new(self.value) };
                    self.lvalue_mode = lvalue_mode_backup;

                    let struct_ptr = unsafe { PointerValue::new(self.builder.build_load(
                        self.context.ptr_type(AddressSpace::default()),
                        struct_ptr_ptr,
                        "indirectstructptrload",
                    ).unwrap().as_value_ref()) };

                    let struct_type = self.type_arena.as_ref().unwrap().get(&current_type_id);
                    let struct_field_map = match struct_type {
                        ResolvedType::Pointer(struct_type) => {
                            match self.type_arena.as_ref().unwrap().get(struct_type) {
                                ResolvedType::Struct(struct_type) => {
                                    self.struct_field_maps.get(&struct_type.name).unwrap()
                                },
                                _ => panic!("Expected pointer to struct type for indirect member access"),
                            }
                        },
                        _ => panic!("Expected pointer to struct type for indirect member access"),
                    };

                    let member_index = *struct_field_map.get(&member_name.data).unwrap();

                    let gep = unsafe {
                        if let ResolvedType::Pointer(struct_type) = struct_type {
                            self.builder.build_in_bounds_gep(
                                self.get_type(self.type_arena.as_ref().unwrap().get(struct_type)),
                                struct_ptr,
                                &[
                                    self.context.i32_type().const_int(0, false),
                                    self.context.i32_type().const_int(member_index as u64, false),
                                ],
                                "memberidx",
                            ).unwrap()
                        } else {
                            panic!("Expected pointer to struct type for indirect member access");
                        }
                        
                    };

                    if self.lvalue_mode {
                        self.value = gep.as_value_ref();
                        return;
                    }

                    let member_type_id = match struct_type {
                        ResolvedType::Pointer(struct_type) => {
                            match self.type_arena.as_ref().unwrap().get(struct_type) {
                                ResolvedType::Struct(struct_type) => {
                                    struct_type.members.get(&member_name.data).unwrap()
                                },
                                _ => panic!("Expected pointer to struct type for indirect member access"),
                            }
                        },
                        _ => panic!("Expected pointer to struct type for indirect member access"),
                    };

                    let pointee_ty = self.get_type(&self.type_arena.as_ref().unwrap().get(member_type_id));

                    let loaded = self.builder.build_load(
                        pointee_ty,
                        gep,
                        "memberload",
                    ).unwrap();

                    self.value = loaded.as_value_ref();

                    current_type_id = *member_type_id;
                },
                _ => unimplemented!("Member access type not implemented"),
            }
        }
    }

    fn visit_var(&mut self, node: &VarExpr) {
        let identifier = &node.path.segments.first().unwrap().data;
        println!("Visiting variable: {}", identifier);
        if let Some(declaration_id) = self.symbol_table.variables.get(&node.get_id()) {
            let ptr = self.local_vars.get(declaration_id).unwrap();
            if self.lvalue_mode {
                self.value = ptr.as_value_ref();
                return;
            } else {
                let declaration_type = self.symbol_table.declaration_types.get(declaration_id).unwrap();
                let pointee_ty = self.get_type(&self.type_arena.as_ref().unwrap().get(declaration_type));

                println!("Loading type: {:?}", pointee_ty);

                let loaded = self.builder.build_load(
                    pointee_ty,
                    *ptr,
                    "varload",
                ).unwrap();

                self.value = loaded.as_value_ref();
                return;
            }
        } else {
            println!("Looking up function: {}", identifier);
            self.value = self.functions.get(identifier).unwrap().as_value_ref();
        }
    }

    fn visit_if(&mut self, node: &IfExpr) {
        let current_block = self.builder.get_insert_block().unwrap();
        let then_block = self.context.insert_basic_block_after(current_block, "then");
        let else_block = self.context.insert_basic_block_after(then_block, "else");
        let merge_block = self.context.insert_basic_block_after(else_block, "ifcont");

        node.condition.accept_visitor(self);
        let cond_value = unsafe { IntValue::new(self.value) };

        self.builder.build_conditional_branch(cond_value, then_block, else_block).unwrap();
        self.builder.position_at_end(then_block);
        node.success.accept_visitor(self);
        let success_value = self.value;
        let success_block = self.builder.get_insert_block().unwrap();
        self.builder.build_unconditional_branch(merge_block).unwrap();

        self.builder.position_at_end(else_block);
        let mut fail_value = None;
        let mut fail_block = None;
        if let Some(fail) = &node.fail {
            fail.accept_visitor(self);
            fail_value = Some(self.value);
            fail_block = self.builder.get_insert_block();
        }
        self.builder.build_unconditional_branch(merge_block).unwrap();

        self.builder.position_at_end(merge_block);

        if let Some(fail_value) = fail_value {
            let success_value = unsafe { IntValue::new(success_value) };
            let fail_value = unsafe { IntValue::new(fail_value) };
            
            if success_value.get_type() == fail_value.get_type() {
                let phi = self.builder.build_phi(self.context.i32_type(), "iftmp").unwrap();

                phi.add_incoming(&[
                    (&success_value, success_block),
                    (&fail_value, fail_block.unwrap()),
                ]);
                self.value = phi.as_value_ref();
            }
        } else {
            self.value = success_value;
        }
    }

    fn visit_assignment(&mut self, node: &AssignmentExpr) {
        self.lvalue_mode = true;
        node.assignee.accept_visitor(self);
        let ptr = unsafe { PointerValue::new(self.value) };
        self.lvalue_mode = false;

        node.expr.accept_visitor(self);
        let expr_value = self.value;

        self.builder.build_store(
            ptr,
            unsafe { IntValue::new(expr_value) },
        ).unwrap();
    }

    fn visit_delete(&mut self, _node: &DeleteExpr) {
        
    }

    fn visit_declaration(&mut self, node: &DeclarationExpr) {
        let declaration_type_id = self.symbol_table.declaration_types.get(&node.get_id()).unwrap();
        let declaration_type = self.type_arena.as_ref().unwrap().get(declaration_type_id);
        let llvm_type = self.get_type(declaration_type); 

        let ptr_value = self.builder.build_alloca(
            llvm_type,
            &node.get_id().to_string(),
        ).unwrap();

        if let Some(expr) = &node.expr {
            expr.accept_visitor(self);

            self.builder.build_store(
                ptr_value,
                unsafe { IntValue::new(self.value) },
            ).unwrap();
        }

        self.local_vars.insert(node.get_id(), ptr_value);
    }

    fn visit_block(&mut self, node: &BlockExpr) {
        let current_block = self.builder.get_insert_block().unwrap();
        let block_entry = self.context.insert_basic_block_after(current_block, "blockentry");
        let after_block = self.context.insert_basic_block_after(block_entry, "afterblock");

        self.result_block = Some(after_block);

        self.builder.build_unconditional_branch(block_entry).unwrap();
        self.builder.position_at_end(block_entry);

        for expr in &node.exprs {
            expr.accept_visitor(self);
        }

        self.builder.build_unconditional_branch(after_block).unwrap();
        self.builder.position_at_end(after_block);

        self.result_block = None;
    }

    fn visit_loop(&mut self, node: &LoopExpr) {
        let label = match &node.label {
            Some(label) => label.data.clone(),
            None => format!("{}", self.loop_labels.len()),
        };

        self.loop_labels.push_back(label.clone());

        self.break_values.insert(label.clone(), Vec::new());

        if let Some(initial) = &node.initial {
            initial.accept_visitor(self);
        }
        
        let current_block = self.builder.get_insert_block().unwrap();
        let condition_block = self.context.insert_basic_block_after(current_block, "loopcondition");
        let loop_block = self.context.insert_basic_block_after(condition_block, "loop");
        let after_loop_block = self.context.insert_basic_block_after(loop_block, "afterloop");
        
        self.break_block.insert(label.clone(), after_loop_block);

        self.builder.build_unconditional_branch(condition_block).unwrap();
        self.builder.position_at_end(condition_block);

        if let Some(condition) = &node.condition {
            condition.accept_visitor(self);
            let cond_value = unsafe { IntValue::new(self.value) };
            self.builder.build_conditional_branch(cond_value, loop_block, after_loop_block).unwrap();
        } else {
            self.builder.build_unconditional_branch(loop_block).unwrap();
        }

        self.builder.position_at_end(loop_block);

        node.body.accept_visitor(self);
        if let Some(increment) = &node.increment {
            increment.accept_visitor(self);
        }
            
        self.builder.build_unconditional_branch(condition_block).unwrap();
            
        self.builder.position_at_end(after_loop_block);

        let breaks = self.break_values.get(&label).unwrap();
        let mut is_safe = true;

        for (_, val) in breaks {
            let int_value = unsafe {IntValue::new(*val)};

            if int_value.get_type() != self.context.i32_type() {
                is_safe = false;
                break;
            }
        }

        if is_safe {
            let phi = self.builder.build_phi(self.context.i32_type(), "looptmp").unwrap();

            for (bb, val) in breaks {
                let int_value = unsafe {IntValue::new(*val)};

                phi.add_incoming(&[
                    (&int_value, *bb),
                ]);
            }
            self.value = phi.as_value_ref();
        }

        self.break_values.remove(&label);
        self.break_block.remove(&label);
        self.loop_labels.pop_back();
    }

    fn visit_exit(&mut self, node: &ExitExpr) {
        let mut value = None;
        if let Some(expr) = &node.expr {
            expr.accept_visitor(self);

            let expr_type_id = self.symbol_table.ast_types.get(&expr.get_id()).unwrap();
            let expr_type = self.type_arena.as_ref().unwrap().get(expr_type_id);

            value = Some(unsafe { self.get_basic_value(expr_type, self.value) });
        }

        let label = match &node.label {
            Some(label) => Some(&label.data),
            None => self.loop_labels.back(),
        };

        match node.exit_type {
            ExitType::Break => {
                self.break_values.get_mut(label.map(|x| x.as_str()).unwrap()).unwrap().push((
                    self.builder.get_insert_block().unwrap(),
                    self.value,
                ));
                self.builder.build_unconditional_branch(*self.break_block.get(label.unwrap().as_str()).unwrap()).unwrap();
            },
            ExitType::Result => {
                self.builder.build_unconditional_branch(self.result_block.unwrap()).unwrap();
            },
            ExitType::Return => {
                self.builder.build_return(value.as_ref().map(|v| v as &dyn BasicValue)).unwrap();
            }
        }
    }

    fn visit_constructor_call(&mut self, _node: &ConstructorCallExpr) {
        
    }

    fn visit_new_array(&mut self, node: &NewArrayExpr) {
        if node.dimension != 1 {
            unimplemented!("Only single-dimensional arrays are currently supported");
        }

        node.sizes[0].accept_visitor(self);
        let size_value = unsafe { IntValue::new(self.value) };

        let array_type = self.type_arena.as_ref().unwrap().get(self.symbol_table.ast_types.get(&node.get_id()).unwrap());
        if let ResolvedType::Pointer(element_type_id) = array_type {
            let element_type = self.type_arena.as_ref().unwrap().get(element_type_id);
            let llvm_element_type = self.get_type(element_type);

            let array_ptr = self.builder.build_array_alloca(
                llvm_element_type,
                size_value,
                "newarraytmp",
            ).unwrap();

            self.value = array_ptr.as_value_ref();
        } else {
            panic!("Expected pointer type for new array expression");
        }
    }

    fn visit_impl(&mut self, _node: &ImplItem) {
        
    }

    fn visit_function(&mut self, node: &FunctionItem) {
        let fn_type_id = self.symbol_table.functions.get(&node.name.data).unwrap();
        let fn_type = self.type_arena.as_ref().unwrap().get(fn_type_id);

        if let ResolvedType::Function(fn_type) = fn_type {
            let mut param_types = Vec::new();

            for parameter in &fn_type.param_types {
                let param_type = self.type_arena.as_ref().unwrap().get(parameter);
                match param_type {
                    ResolvedType::Integer => param_types.push(self.context.i32_type().into()),
                    ResolvedType::Char => param_types.push(self.context.i32_type().into()),
                    ResolvedType::Pointer(_) => param_types.push(self.context.ptr_type(AddressSpace::default()).into()),
                    _ => unimplemented!("Parameter type not implemented"),
                }
            }

            let ret_type_id = fn_type.return_type;
            let ret_type = self.type_arena.as_ref().unwrap().get(&fn_type.return_type);
            let fn_type = match ret_type {
                ResolvedType::Integer => self.context.i32_type().fn_type(&param_types, false),
                ResolvedType::Void => self.context.void_type().fn_type(&param_types, false),
                ResolvedType::Char => self.context.i32_type().fn_type(&param_types, false),
                ResolvedType::Pointer(_) => self.context.ptr_type(AddressSpace::default()).fn_type(&param_types, false),
                _ => unimplemented!("Return type not implemented"),
            };

            let function = self.module.add_function(&node.name.data, fn_type, None);
            self.functions.insert(node.name.data.clone(), function);
            let entry = self.context.append_basic_block(function, "entry");
            self.builder.position_at_end(entry);

            for (i, parameter) in node.parameters.iter().enumerate() {
                let param = function.get_nth_param(i as u32).unwrap();

                let param_ptr = self.builder.build_alloca(
                    param.get_type(),
                    &parameter.identifier.data,
                ).unwrap();

                self.builder.build_store(param_ptr, param).unwrap();
                self.local_vars.insert(parameter.get_id(), param_ptr);
            }

            node.body.as_ref().unwrap().accept_visitor(self);

            let ret_type = self.type_arena.as_ref().unwrap().get(&ret_type_id);
            if ret_type == &ResolvedType::Void {
                self.builder.build_return(None).unwrap();
            } else {
                let value = unsafe { self.get_basic_value(ret_type, self.value) };
                self.builder.build_return(Some(&value)).unwrap();
            }
        } else {
            panic!("Function type expected");
        }
    }

    fn visit_struct(&mut self, node: &StructItem) {
        let mut field_types = Vec::new();

        let struct_type_id = self.symbol_table.types.get(&node.name.data).unwrap();
        let struct_type = self.type_arena.as_ref().unwrap().get(struct_type_id);

        let mut struct_field_map = HashMap::new();

        if let ResolvedType::Struct(struct_type) = struct_type {
            for (field_name, field_type_id) in &struct_type.members {
                struct_field_map.insert(field_name.clone(), field_types.len() as u32);

                let field_type = self.type_arena.as_ref().unwrap().get(field_type_id);
                match field_type {
                    ResolvedType::Integer => field_types.push(self.context.i32_type().into()),
                    ResolvedType::Char => field_types.push(self.context.i8_type().into()),
                    ResolvedType::Pointer(_) => field_types.push(self.context.ptr_type(AddressSpace::default()).into()),
                    _ => unimplemented!("Field type not implemented"),
                }
            }
        } else {
            panic!("Struct type expected");
        }

        self.struct_field_maps.insert(node.name.data.clone(), struct_field_map);

        let struct_type = self.context.opaque_struct_type(&node.name.data);
        struct_type.set_body(&field_types, false);

        self.struct_types.insert(node.name.data.clone(), struct_type);
    }

    fn visit_constructor(&mut self, _node: &ConstructorItem) {
        
    }

    fn visit_scope(&mut self, node: &Scope) -> () {
        self.type_arena = Some(self.global_table.type_arena.lock().unwrap());

        for item in &node.items {
            item.accept_visitor(self);
        }

        self.type_arena = None;
    }
}