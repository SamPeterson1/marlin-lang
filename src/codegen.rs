use std::{collections::{HashMap, VecDeque}, path::Path, process::Command};

use inkwell::{AddressSpace, basic_block::BasicBlock, builder::Builder, context::Context, llvm_sys::prelude::LLVMValueRef, module::Module, values::{AsValueRef, BasicValue, IntValue, PointerValue}};

use crate::{ast::*, resolver::SymbolTable};

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    value: LLVMValueRef,
    local_vars: HashMap<DeclarationId, PointerValue<'ctx>>,
    break_block: HashMap<String, BasicBlock<'ctx>>,
    result_block: Option<BasicBlock<'ctx>>,
    symbol_table: &'ctx SymbolTable,
    break_values: HashMap<String, Vec<(BasicBlock<'ctx>, LLVMValueRef)>>,
    loop_labels: VecDeque<String>,
    lvalue_mode: bool,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, symbol_table: &'ctx SymbolTable) -> Self {
        let module = context.create_module("main_module");
        let builder = context.create_builder();
        
        let i32_type = context.i32_type();

        let putchar_type = i32_type.fn_type(&[i32_type.into()], false);
        module.add_function("putchar", putchar_type, None);

        let getchar_type = i32_type.fn_type(&[], false);
        module.add_function("getchar", getchar_type, None);

        Self {
            context,
            module,
            builder,
            value: std::ptr::null_mut(),
            local_vars: HashMap::new(),
            break_block: HashMap::new(),
            result_block: None,
            symbol_table,
            break_values: HashMap::new(),
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
            _ => unimplemented!("Operator not implemented"),
        }
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
            }
            Literal::Char(c) => {
                let int_type = self.context.i32_type();
                let int_value = int_type.const_int(*c as u64, false);
                self.value = int_value.as_value_ref();
            }
            _ => unimplemented!("Literal type not implemented"),
        }
    }

    fn visit_member_access(&mut self, node: &MemberAccess) {
        if let Some(AccessType::FunctionCall(arguments)) = node.member_accesses.get(0) {
            if let Some(var_expr) = node.expr.as_any().downcast_ref::<VarExpr>() {
                let function_name = &var_expr.identifier.data;
                if function_name == "putchar" && arguments.args.len() == 1 {
                    arguments.args[0].accept_visitor(self);
                    let arg_value = unsafe {IntValue::new(self.value) };

                    // call print with arg_value
                    self.builder.build_call(
                        self.module.get_function("putchar").unwrap(),
                        &[arg_value.into()],
                        "putchar",
                    ).unwrap();
                    return;
                } else if function_name == "getchar" && arguments.args.is_empty() {
                    // call getchar
                    let call_site = self.builder.build_call(
                        self.module.get_function("getchar").unwrap(),
                        &[],
                        "getchar",
                    ).unwrap();
                    let ret_value = call_site.try_as_basic_value().basic().unwrap();
                    self.value = ret_value.as_value_ref();
                    return;
                }
            }
        }

        node.expr.accept_visitor(self);
    }

    fn visit_var(&mut self, node: &VarExpr) {
        let declaration_id = self.symbol_table.get_variable(&node.id).unwrap();
        let ptr = self.local_vars.get(declaration_id).unwrap();
        if self.lvalue_mode {
            self.value = ptr.as_value_ref();
            return;
        } else {
            let declaration_type = self.symbol_table.get_declaration_type(declaration_id).unwrap();
            let loaded = if let ParsedTypeEnum::Pointer(_) = declaration_type.parsed_type {
                self.builder.build_load(
                    self.context.ptr_type(AddressSpace::default()),
                    *ptr,
                    &node.identifier.data,
                ).unwrap()
            } else {
                self.builder.build_load(
                    self.context.i32_type(),
                    *ptr,
                    &node.identifier.data,
                ).unwrap()
            };

            self.value = loaded.as_value_ref();
            return;
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
        node.expr.accept_visitor(self);

        let ptr_value = if let ParsedTypeEnum::Pointer(_) = node.declaration_type.parsed_type {
            self.builder.build_alloca(
                self.context.ptr_type(AddressSpace::default()),
                &node.id.to_string(),
            ).unwrap()
        } else {
            self.builder.build_alloca(
                self.context.i32_type(),
                &node.id.to_string(),
            ).unwrap()
        };

        self.builder.build_store(
            ptr_value,
            unsafe { IntValue::new(self.value) },
        ).unwrap();

        self.local_vars.insert(node.id, ptr_value);
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
            value = Some(unsafe {IntValue::new(self.value)});
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

    fn visit_new_array(&mut self, _node: &NewArrayExpr) {
        
    }

    fn visit_impl(&mut self, _node: &ImplItem) {
        
    }

    fn visit_function(&mut self, _node: &FunctionItem) {
        
    }

    fn visit_struct(&mut self, _node: &StructItem) {
        
    }

    fn visit_constructor(&mut self, _node: &ConstructorItem) {
        
    }

    fn visit_main(&mut self, node: &MainItem) {
        let void_type = self.context.void_type();
        let fn_type = void_type.fn_type(&[], false);
        let function = self.module.add_function("main", fn_type, None);
        let entry = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(entry);

        node.body.accept_visitor(self);

        self.builder.build_return(None).unwrap();
    }

    fn visit_program(&mut self, node: &Program) {
        for item in &node.items {
            item.accept_visitor(self);
        }
    }
}