use std::collections::HashMap;

use chrono::format::Parsed;

use crate::{expr::{assignment_expr::AssignmentExpr, binary_expr::BinaryExpr, block_expr::BlockExpr, break_expr::BreakExpr, call_expr::CallExpr, declaration_expr::DeclarationExpr, get_address_expr::GetAddressExpr, get_char_expr::GetCharExpr, if_expr::IfExpr, literal_expr::{Literal, LiteralExpr}, loop_expr::LoopExpr, put_char_expr::PutCharExpr, static_array_expr::StaticArrayExpr, struct_initializer_expr::StructInitializerExpr, unary_expr::UnaryExpr, var_expr::{MemberAccess, VarExpr}, ExprVisitor}, instruction::InstructionBuilder, item::{FunctionItem, Item, ItemVisitor, StructItem}, resolver::SymbolTable, token::{Position, PositionRange}, type_checker::TypeChecker, types::{parsed_type::{ParsedType, ParsedTypeName}, resolved_type::ResolvedType}};

const RET_ADDR_REG: u8 = 15;
const STACK_PTR_REG: u8 = 14;
const FRAME_PTR_REG: u8 = 13;
const CONSTANT_POOL_REG: u8 = 12;

pub struct CompilerFunction {
    body_instructions: Vec<u64>,
    local_vars_size: usize,
    local_vars: HashMap<i32, usize>,
    args: Vec<usize>,
    args_size: usize,
}

impl CompilerFunction {
    pub fn new() -> CompilerFunction {
        CompilerFunction {
            body_instructions: Vec::new(),
            local_vars_size: 0,
            local_vars: HashMap::new(),
            args: Vec::new(),
            args_size: 0,
        }
    }

    pub fn get_args_size(&self) -> usize {
        self.args_size
    }

    pub fn add_argument(&mut self, size: usize) -> usize {
        let arg_index = self.args_size;
        self.args.push(self.args_size);
        self.args_size += size;
        arg_index
    }

    pub fn get_arg(&mut self, id: usize) -> usize {
        self.args[id]
    }

    pub fn get_local_var(&mut self, id: i32) -> usize {
        *self.local_vars.get(&id).unwrap()
    }

    pub fn define_intermediate_var(&mut self, size: usize) -> usize {
        let stack_index = self.local_vars_size;
        self.local_vars_size += size;
        stack_index
    }

    pub fn allocate_array(&mut self, size: usize) -> usize {
        let array_addr = self.local_vars_size;
        self.local_vars_size += size;

        array_addr
    }

    pub fn define_local_var(&mut self, id: i32, size: usize) -> usize {
        let stack_index = self.local_vars_size;
        self.local_vars.insert(id, self.local_vars_size);
        self.local_vars_size += size;
        stack_index
    }

    pub fn local_vars_size(&self) -> usize {
        self.local_vars_size
    }

    pub fn n_instructions(&self) -> usize {
        self.body_instructions.len()
    }

    pub fn set_instruction(&mut self, index: usize, instruction: u64) {
        self.body_instructions[index] = instruction;
    }
    
    pub fn push_instructions(&mut self, instructions: Vec<u64>) {
        self.body_instructions.extend(instructions);
    }

    pub fn push_instruction(&mut self, instruction: u64) {
        self.body_instructions.push(instruction);
    }

    pub fn to_instructions(mut self) -> Vec<u64> {
        let mut instructions = Vec::new();
        
        instructions.push(InstructionBuilder::push(RET_ADDR_REG));
        instructions.push(InstructionBuilder::push(FRAME_PTR_REG));
        instructions.push(InstructionBuilder::addi_imm(FRAME_PTR_REG, STACK_PTR_REG, 0));

        instructions.push(InstructionBuilder::addi_imm(STACK_PTR_REG, STACK_PTR_REG, self.local_vars_size() as i32));

        for i in 0..2 {
            instructions.push(InstructionBuilder::push(i));
        }

        instructions.append(&mut self.body_instructions);

        instructions.push(InstructionBuilder::stfp(0, -2));

        for i in (0..2).rev() {
            instructions.push(InstructionBuilder::pop(i));
        }

        instructions.push(InstructionBuilder::addi_imm(STACK_PTR_REG, STACK_PTR_REG, -(self.local_vars_size() as i32)));

        instructions.push(InstructionBuilder::pop(FRAME_PTR_REG));
        instructions.push(InstructionBuilder::pop(RET_ADDR_REG));

        
        instructions.push(InstructionBuilder::ret());

        instructions
    }
}

pub struct Compiler<'a> {
    instructions: Vec<u64>,
    function_address_locations: HashMap<String, u32>,
    current_function: CompilerFunction,
    symbol_table: &'a SymbolTable,
    type_checker: TypeChecker<'a>,
    constant_pool: Vec<u64>,
    constant_pool_size: usize,
}

pub struct CompilerResult {
    pub instructions: Vec<u64>,
    pub constant_pool: Vec<u64>,
}

impl Compiler<'_> {
    pub fn new(symbol_table: &SymbolTable) -> Compiler<'_> {
        Compiler {
            instructions: Vec::new(),
            function_address_locations: HashMap::new(),
            current_function: CompilerFunction::new(),
            type_checker: TypeChecker::new(symbol_table),
            constant_pool: Vec::new(),
            constant_pool_size: 0,
            symbol_table
        }
    }

    fn push_constant(&mut self, value: u64) -> usize {
        let index = self.constant_pool_size;
        self.constant_pool_size += 1;
        self.constant_pool.push(value);

        index
    }

    pub fn compile(mut self, items: &[Box<dyn Item>]) -> CompilerResult {
        self.instructions.push(InstructionBuilder::and_imm(STACK_PTR_REG, STACK_PTR_REG, 0));
        self.instructions.push(InstructionBuilder::addi_imm(STACK_PTR_REG, STACK_PTR_REG, 3000));

        self.instructions.push(InstructionBuilder::and(CONSTANT_POOL_REG, CONSTANT_POOL_REG, 0));
        self.instructions.push(InstructionBuilder::addi_imm(CONSTANT_POOL_REG, CONSTANT_POOL_REG, 6000));
        
        let function_address_location = self.push_constant(0) as u32;
        self.function_address_locations.insert("main".to_string(), function_address_location);

        self.instructions.push(InstructionBuilder::ldr_imm(0, CONSTANT_POOL_REG, function_address_location as i32));

        self.instructions.push(InstructionBuilder::call(0));

        self.instructions.push(InstructionBuilder::pop(0));

        self.instructions.push(InstructionBuilder::halt());

        for item in items {
            item.accept_visitor(&mut self);
        }

        CompilerResult {
            instructions: self.instructions,
            constant_pool: self.constant_pool,
        }
    }
}

impl ItemVisitor<()> for Compiler<'_> {
    fn visit_struct(&mut self, item: &StructItem) -> () { }

    fn visit_function(&mut self, item: &FunctionItem) -> () {
        for arg in item.args.iter() {
            let resolved_type = self.symbol_table.get_resolved_type(&arg.1).unwrap();
            self.current_function.add_argument(resolved_type.n_bytes() / 8);
        }

        item.expr.accept_visitor(self);

        let mut current_function_swap = CompilerFunction::new();
        std::mem::swap(&mut self.current_function, &mut current_function_swap);
        let function_instructions = current_function_swap.to_instructions();

        let function_address = self.instructions.len() as u64;
        self.instructions.extend(function_instructions);

        let function_name = &*item.name;

        if !self.function_address_locations.contains_key(function_name) {
            let function_address_location = self.push_constant(function_address); // Placeholder for function address
            self.function_address_locations.insert(function_name.clone(), function_address_location as u32);
        } else {
            let function_address_location: &u32 = self.function_address_locations.get(function_name).unwrap();
            self.constant_pool[*function_address_location as usize] = function_address;
        }
    }
}

impl ExprVisitor<()> for Compiler<'_> {
    fn visit_binary(&mut self, expr: &BinaryExpr) -> () {
        expr.left.accept_visitor(self);
        let intermmediate_var_index = self.current_function.define_intermediate_var(1);
        self.current_function.push_instruction(InstructionBuilder::stfp(0, intermmediate_var_index as i32 + 1));
        expr.right.accept_visitor(self);

        let operation_type = expr.left.accept_visitor(&mut self.type_checker).unwrap();
        self.current_function.push_instruction(InstructionBuilder::ldfp(1, intermmediate_var_index as i32 + 1));
        self.current_function.push_instructions(expr.operator.compile(0, 1, 0, operation_type));
    }

    fn visit_unary(&mut self, expr: &UnaryExpr) -> () {
        expr.expr.accept_visitor(self);
        let operation_type = expr.expr.accept_visitor(&mut self.type_checker).unwrap();
        self.current_function.push_instructions(expr.operator.compile(0, 0, operation_type));
    }

    fn visit_literal(&mut self, expr: &LiteralExpr) -> () {
        println!("Literal");
        let instructions = match &expr.value {
            Literal::Int(x) => vec![
                InstructionBuilder::and_imm(0, 0, 0),
                InstructionBuilder::addi_imm(0, 0,*x as i32),
            ],
            Literal::Double(_) => todo!(),
            Literal::Bool(bool) => vec![
                InstructionBuilder::and_imm(0, 0, 0),
                InstructionBuilder::addi_imm(0, 0, *bool as i32),
            ],
            Literal::String(str) => {
                let str_index = self.constant_pool_size;

                for c in str.chars() {
                    println!("Pushing constant {}", c as u64);
                    self.push_constant(c as u64);
                }
                
                self.push_constant(0);

                vec![
                    InstructionBuilder::addi_imm(0, CONSTANT_POOL_REG, str_index as i32)
                ]
            }
        };

        self.current_function.push_instructions(instructions);
    }

    fn visit_var(&mut self, expr: &VarExpr) -> () {
        let resolved_var = self.symbol_table.get_variable(expr);
        
        if resolved_var.is_argument {
            let arg_index = self.current_function.get_arg(resolved_var.id as usize) as i32;
            println!("Getting argument index {} for var {}", arg_index, expr.identifier);
            self.current_function.push_instruction(InstructionBuilder::ldfp(1, -arg_index - 3));
        } else {
            let stack_index = self.current_function.get_local_var(resolved_var.id) as i32;
            self.current_function.push_instruction(InstructionBuilder::ldfp(1, stack_index + 1));
        }

        let mut var_type = resolved_var.value_type.clone();

        for member_access in expr.member_accesses.iter() {
            match member_access {
                MemberAccess::Direct(member_name) => {
                    if let ResolvedType::Struct(struct_type) = &var_type {
                        let member_offset = struct_type.get_member_offset(member_name) / 8;
                        self.current_function.push_instruction(InstructionBuilder::ldr_imm(1, 1, member_offset as i32));
                        var_type = struct_type.get_member_type(member_name).clone();
                    } else {
                        panic!("Member access on non-struct type");
                    }
                },
                MemberAccess::Indirect(member_name) => {
                    if let ResolvedType::Pointer(pointer_type) = &var_type {
                        if let ResolvedType::Struct(struct_type) = &*pointer_type.pointee {
                            let member_offset = struct_type.get_member_offset(member_name) / 8;
                            self.current_function.push_instruction(InstructionBuilder::ldr_imm(1, 1, 0));
                            self.current_function.push_instruction(InstructionBuilder::ldr_imm(1, 1, member_offset as i32));
                            var_type = struct_type.get_member_type(member_name).clone();
                        } else {
                            panic!("Member access on non-struct type");
                        }
                    } else {
                        panic!("Member access on non-pointer type");
                    }
                }
            }
        }

        let addr_var_index = self.current_function.define_intermediate_var(1);

        for array_access in expr.array_accesses.iter() {
            self.current_function.push_instruction(InstructionBuilder::stfp(1, addr_var_index as i32 + 1));
            array_access.accept_visitor(self);
            self.current_function.push_instruction(InstructionBuilder::ldfp(1, addr_var_index as i32 + 1));
            self.current_function.push_instruction(InstructionBuilder::ldr(1, 1, 0));
        }

        for _ in 0..expr.n_derefs {
            println!("Dereferencing {}", expr.identifier);
            self.current_function.push_instruction(InstructionBuilder::ldr_imm(1, 1, 0));
        }

        self.current_function.push_instruction(InstructionBuilder::addi_imm(0, 1, 0));
    }

    /*
    JMP to ELSE if condition is false
    <true block>
    ELSE
    <false block> or nothing
    END
     */
    fn visit_if(&mut self, expr: &IfExpr) -> () {
        expr.condition.accept_visitor(self);

        let jmp_else_instruction_index = self.current_function.n_instructions() as i32;
        self.current_function.push_instruction(InstructionBuilder::nop()); // Placeholder for JMP instruction 
        expr.success.accept_visitor(self);

        let jmp_end_instruction_index = self.current_function.n_instructions() as i32;
        self.current_function.push_instruction(InstructionBuilder::nop()); // Placeholder for JMP instruction

        let jmp_else_pc_offset = self.current_function.n_instructions() as i32 - jmp_else_instruction_index - 1;
        self.current_function.set_instruction(jmp_else_instruction_index as usize, InstructionBuilder::jmp(false, true, false, jmp_else_pc_offset));

        if let Some(fail) = &expr.fail {
            println!("Fail");
            fail.accept_visitor(self);
        }

        let jmp_end_pc_offset = self.current_function.n_instructions() as i32 - jmp_end_instruction_index - 1;
        self.current_function.set_instruction(jmp_end_instruction_index as usize, InstructionBuilder::jmp(true, true, true, jmp_end_pc_offset));
    }

    fn visit_assignment(&mut self, expr: &AssignmentExpr) -> () {
        let resolved_var = self.symbol_table.get_variable(&expr.asignee);
        
        expr.expr.accept_visitor(self);
        let value_var_index = self.current_function.define_intermediate_var(1);
        self.current_function.push_instruction(InstructionBuilder::stfp(0, value_var_index as i32 + 1));

        if resolved_var.is_argument {
            let arg_index = self.current_function.get_arg(resolved_var.id as usize) as i32;
            self.current_function.push_instruction(InstructionBuilder::addi_imm(1, FRAME_PTR_REG, -arg_index - 3));
        } else {
            let stack_index = self.current_function.get_local_var(resolved_var.id) as i32;
            self.current_function.push_instruction(InstructionBuilder::addi_imm(1, FRAME_PTR_REG, stack_index + 1));
        }

        let mut var_type = resolved_var.value_type.clone();

        for member_access in expr.asignee.member_accesses.iter() {
            match member_access {
                MemberAccess::Direct(member_name) => {
                    if let ResolvedType::Struct(struct_type) = &var_type {
                        let member_offset = struct_type.get_member_offset(member_name) / 8;
                        
                        self.current_function.push_instruction(InstructionBuilder::ldr_imm(1, 1, 0));
                        self.current_function.push_instruction(InstructionBuilder::addi_imm(1, 1, member_offset as i32));
    
                        var_type = struct_type.get_member_type(member_name).clone();
                    } else {
                        panic!("Member access on non-struct type");
                    }
                },
                MemberAccess::Indirect(member_name) => {
                    if let ResolvedType::Pointer(pointer_type) = &var_type {
                        if let ResolvedType::Struct(struct_type) = &*pointer_type.pointee {
                            let member_offset = struct_type.get_member_offset(member_name) / 8;
                            
                            self.current_function.push_instruction(InstructionBuilder::ldr_imm(1, 1, 0));
                            self.current_function.push_instruction(InstructionBuilder::ldr_imm(1, 1, 0));
                            self.current_function.push_instruction(InstructionBuilder::addi_imm(1, 1, member_offset as i32));

                            var_type = struct_type.get_member_type(member_name).clone();
                        } else {
                            panic!("Member access on non-struct type");
                        }
                    } else {
                        panic!("Member access on non-pointer type");
                    }
                }
            }
        }

        let addr_var_index = self.current_function.define_intermediate_var(1);

        for array_access in expr.asignee.array_accesses.iter() {
            self.current_function.push_instruction(InstructionBuilder::ldr_imm(1, 1, 0));
            self.current_function.push_instruction(InstructionBuilder::stfp(1, addr_var_index as i32 + 1));
            array_access.accept_visitor(self);
            self.current_function.push_instruction(InstructionBuilder::ldfp(1, addr_var_index as i32 + 1));
            self.current_function.push_instruction(InstructionBuilder::addi(1, 1, 0));
        }

        for _ in 0..expr.asignee.n_derefs {
            self.current_function.push_instruction(InstructionBuilder::ldr_imm(1, 1, 0));
        }

        self.current_function.push_instruction(InstructionBuilder::ldfp(0, value_var_index as i32 + 1));

        self.current_function.push_instruction(InstructionBuilder::str_imm(0, 1, 0));
    }

    fn visit_declaration(&mut self, expr: &DeclarationExpr) -> () {
        let stack_index = self.current_function.define_local_var(expr.id, 1);

        expr.expr.accept_visitor(self);

        self.current_function.push_instruction(InstructionBuilder::stfp(0, stack_index as i32 + 1))
    }

    fn visit_block(&mut self, expr: &BlockExpr) -> () {
        expr.exprs.iter().for_each(|expr| expr.accept_visitor(self));
    }

    /*
    START
    JMP to END if condition is false
    <loop block>
    JMP to START
    END
     */
    fn visit_loop(&mut self, expr: &LoopExpr) -> () {
        if let Some(initial) = &expr.initial {
            initial.accept_visitor(self);
        }

        let start_loop_instruction_index = self.current_function.n_instructions() as i32;

        let mut jmp_end_instruction_index = None;

        if let Some(condition) = &expr.condition {
            condition.accept_visitor(self);
            jmp_end_instruction_index = Some(self.current_function.n_instructions() as i32);
            self.current_function.push_instruction(InstructionBuilder::nop()); // Placeholder for JMP instruction
        }

        expr.body.accept_visitor(self);

        if let Some(increment) = &expr.increment {
            increment.accept_visitor(self);
        }

        let jmp_start_instruction_index = self.current_function.n_instructions() as i32;
        let jmp_start_pc_offset = start_loop_instruction_index - jmp_start_instruction_index - 1;
        self.current_function.push_instruction(InstructionBuilder::jmp(true, true, true, jmp_start_pc_offset));

        if let Some(jmp_end_instruction_index) = jmp_end_instruction_index {
            let jmp_end_pc_offset = self.current_function.n_instructions() as i32 - jmp_end_instruction_index - 1;
            self.current_function.set_instruction(jmp_end_instruction_index as usize, InstructionBuilder::jmp(false, true, false, jmp_end_pc_offset));
        }
    }

    fn visit_break(&mut self, expr: &BreakExpr) -> () {
        todo!()
    }

    fn visit_call(&mut self, expr: &CallExpr) -> () {
        let function_name = &expr.function;
        let function_address_location = if self.function_address_locations.contains_key(function_name) {
            *self.function_address_locations.get(function_name).unwrap()
        } else {
            let function_address_location = self.push_constant(0) as u32; // Placeholder for function address
            self.function_address_locations.insert(function_name.clone(), function_address_location);
            function_address_location
        };

        for arg in expr.args.iter().rev() {
            arg.accept_visitor(self);
            self.current_function.push_instruction(InstructionBuilder::push(0));
        }
        
        self.current_function.push_instruction(InstructionBuilder::addi_imm(STACK_PTR_REG, STACK_PTR_REG, 1));

        self.current_function.push_instruction(InstructionBuilder::ldr_imm(0, CONSTANT_POOL_REG, function_address_location as i32));
        self.current_function.push_instruction(InstructionBuilder::call(0));

        self.current_function.push_instruction(InstructionBuilder::pop(0));
        self.current_function.push_instruction(InstructionBuilder::addi_imm(STACK_PTR_REG, STACK_PTR_REG, -(expr.args.len() as i32)));
    }

    fn visit_struct_initializer(&mut self, expr: &StructInitializerExpr) -> () {
        if let ResolvedType::Struct(struct_type) = self.symbol_table.get_resolved_type(&ParsedType::TypeName(ParsedTypeName {
                name: (*expr.type_name).clone().into(), 
                position: PositionRange::new(Position::new(0, 0))
            })).unwrap() {
            let stack_offset = self.current_function.define_intermediate_var(struct_type.n_bytes() / 8);
            for (member_name, member_expr) in expr.member_inits.iter() {
                member_expr.accept_visitor(self);
                let member_offset = struct_type.get_member_offset(member_name) / 8;

                self.current_function.push_instruction(InstructionBuilder::stfp(0, (stack_offset + member_offset) as i32 + 1));
            }

            self.current_function.push_instruction(InstructionBuilder::addi_imm(0, FRAME_PTR_REG, stack_offset as i32 + 1));
        } else {
            panic!("Struct initializer on non-struct type");
        }


    }

    fn visit_get_address(&mut self, expr: &GetAddressExpr) -> () {
        let resolved_var = self.symbol_table.get_variable(&expr.var_expr);

        if resolved_var.is_argument {
            let arg_index = self.current_function.get_arg(resolved_var.id as usize) as i32;
            self.current_function.push_instruction(InstructionBuilder::addi_imm(0, FRAME_PTR_REG, -arg_index - 3));
        } else {
            let stack_index = self.current_function.get_local_var(resolved_var.id) as i32;
            self.current_function.push_instruction(InstructionBuilder::addi_imm(0, FRAME_PTR_REG, stack_index + 1));
        }
    }

    fn visit_static_array(&mut self, expr: &StaticArrayExpr) -> () {
        let type_size = self.symbol_table.get_resolved_type(&expr.declaration_type).unwrap().n_bytes() / 8;
        let array_addr = self.current_function.allocate_array(type_size * expr.len);

        self.current_function.push_instruction(InstructionBuilder::and_imm(0, 0, 0));
        self.current_function.push_instruction(InstructionBuilder::addi_imm(0, FRAME_PTR_REG, array_addr as i32 + 1));
    }

    fn visit_get_char(&mut self, expr: &GetCharExpr) -> () {
        self.current_function.push_instruction(InstructionBuilder::getc(0));
    }

    fn visit_put_char(&mut self, expr: &PutCharExpr) -> () {
        expr.expr.accept_visitor(self);
        self.current_function.push_instruction(InstructionBuilder::putc(0));
    }
}