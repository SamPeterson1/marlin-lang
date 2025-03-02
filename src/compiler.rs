use std::collections::HashMap;

use crate::{environment::Literal, expr::{item::{FunctionItem, Item, ItemVisitor, StructItem}, Expr, ExprVisitor}, instruction::InstructionBuilder, resolver::SymbolTable, type_checker::TypeChecker};

/*
R0 stores result of previous operation
*/

/*
int foo = a * (b + c / d)
*/

const RET_ADDR_REG: u8 = 15;
const STACK_PTR_REG: u8 = 14;
const FRAME_PTR_REG: u8 = 13;
const CONSTANT_POOL_REG: u8 = 12;

pub struct CompilerFunction {
    body_instructions: Vec<u64>,
    local_vars: HashMap<i32, u32>,
}

impl CompilerFunction {
    pub fn new() -> CompilerFunction {
        CompilerFunction {
            body_instructions: Vec::new(),
            local_vars: HashMap::new(),
        }
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

        instructions.push(InstructionBuilder::addi_imm(STACK_PTR_REG, STACK_PTR_REG, self.local_vars.len() as i32));

        for i in 0..12 {
            instructions.push(InstructionBuilder::push(i));
        }

        instructions.append(&mut self.body_instructions);

        instructions.push(InstructionBuilder::stfp(0, -2));

        for i in (0..12).rev() {
            instructions.push(InstructionBuilder::pop(i));
        }

        instructions.push(InstructionBuilder::addi_imm(STACK_PTR_REG, STACK_PTR_REG, -(self.local_vars.len() as i32)));

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

    fn push_constant(&mut self, bytes: u64) -> usize {
        let index = self.constant_pool_size;
        self.constant_pool_size += 1;
        self.constant_pool.push(bytes);

        index
    }

    pub fn compile(mut self, items: &[Box<dyn Item>]) -> CompilerResult {
        self.instructions.push(InstructionBuilder::and_imm(STACK_PTR_REG, STACK_PTR_REG, 0));
        self.instructions.push(InstructionBuilder::addi_imm(STACK_PTR_REG, STACK_PTR_REG, 1000));

        self.instructions.push(InstructionBuilder::and(CONSTANT_POOL_REG, CONSTANT_POOL_REG, 0));
        self.instructions.push(InstructionBuilder::addi_imm(CONSTANT_POOL_REG, CONSTANT_POOL_REG, 3000));
        
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
    fn visit_empty(&mut self, expr: &crate::expr::EmptyExpr) -> () {
        todo!()
    }

    fn visit_binary(&mut self, expr: &crate::expr::BinaryExpr) -> () {
        expr.left.accept_visitor(self);
        self.current_function.push_instruction(InstructionBuilder::addi_imm(1, 0, 0));
        expr.right.accept_visitor(self);

        let operation_type = expr.left.accept_visitor(&mut self.type_checker);
        self.current_function.push_instructions(expr.operator.compile(0, 1, 0, operation_type));
    }

    fn visit_unary(&mut self, expr: &crate::expr::UnaryExpr) -> () {
        expr.expr.accept_visitor(self);
        let operation_type = expr.expr.accept_visitor(&mut self.type_checker);
        self.current_function.push_instructions(expr.operator.compile(0, 0, operation_type));
    }

    fn visit_literal(&mut self, expr: &crate::expr::LiteralExpr) -> () {
        println!("Literal");
        self.current_function.push_instructions(match &expr.value {
            Literal::Int(x) => [
                InstructionBuilder::and_imm(0, 0, 0),
                InstructionBuilder::addi_imm(0, 0,*x as i32),
            ],
            Literal::Double(_) => todo!(),
            Literal::Bool(_) => todo!(),
            Literal::String(_) => todo!(),
        }.into_iter().collect());
    }

    fn visit_var(&mut self, expr: &crate::expr::VarExpr) -> () {
        let resolved_var = self.symbol_table.get_variable(expr);
        
        if resolved_var.is_argument {
            let arg_index = resolved_var.id as i32;
            self.current_function.push_instruction(InstructionBuilder::ldfp(0, -arg_index - 3));
        } else {
            let stack_index = *self.current_function.local_vars.get(&resolved_var.id).unwrap() as i32;
            self.current_function.push_instruction(InstructionBuilder::ldfp(0, stack_index + 1));
        }
    }

    /*
    JMP to ELSE if condition is false
    <true block>
    ELSE
    <false block> or nothing
    END
     */
    fn visit_if(&mut self, expr: &crate::expr::IfExpr) -> () {
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

    fn visit_assignment(&mut self, expr: &crate::expr::AssignmentExpr) -> () {
        let resolved_var = self.symbol_table.get_variable(&expr.asignee);
        
        expr.expr.accept_visitor(self);

        if resolved_var.is_argument {
            let arg_index = resolved_var.id as i32;
            self.current_function.push_instruction(InstructionBuilder::stfp(0, -arg_index - 3));
        } else {
            let stack_index = *self.current_function.local_vars.get(&resolved_var.id).unwrap() as i32;
            self.current_function.push_instruction(InstructionBuilder::stfp(0, stack_index + 1));
        }
    }

    fn visit_declaration(&mut self, expr: &crate::expr::DeclarationExpr) -> () {
        let stack_index = self.current_function.local_vars.len() as i32;
        self.current_function.local_vars.insert(expr.id, stack_index as u32);

        expr.expr.accept_visitor(self);

        self.current_function.push_instruction(InstructionBuilder::stfp(0, stack_index + 1))
    }

    fn visit_block(&mut self, expr: &crate::expr::BlockExpr) -> () {
        expr.exprs.iter().for_each(|expr| expr.accept_visitor(self));
    }

    fn visit_print(&mut self, expr: &crate::expr::PrintExpr) -> () {
        todo!()
    }

    fn visit_rand(&mut self, expr: &crate::expr::RandExpr) -> () {
        todo!()
    }

    /*
    START
    JMP to END if condition is false
    <loop block>
    JMP to START
    END
     */
    fn visit_loop(&mut self, expr: &crate::expr::LoopExpr) -> () {
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

    fn visit_break(&mut self, expr: &crate::expr::BreakExpr) -> () {
        todo!()
    }

    fn visit_input(&mut self, expr: &crate::expr::InputExpr) -> () {
        todo!()
    }

    fn visit_call(&mut self, expr: &crate::expr::CallExpr) -> () {
        let function_name = &*expr.function.identifier;
        let function_address_location = if self.function_address_locations.contains_key(function_name) {
            *self.function_address_locations.get(function_name).unwrap()
        } else {
            let function_address_location = self.push_constant(0) as u32; // Placeholder for function address
            self.function_address_locations.insert(function_name.clone(), function_address_location);
            function_address_location
        };

        for arg in &expr.args {
            arg.accept_visitor(self);
            self.current_function.push_instruction(InstructionBuilder::push(0));
        }
        
        self.current_function.push_instruction(InstructionBuilder::addi_imm(STACK_PTR_REG, STACK_PTR_REG, 1));

        self.current_function.push_instruction(InstructionBuilder::ldr_imm(0, CONSTANT_POOL_REG, function_address_location as i32));
        self.current_function.push_instruction(InstructionBuilder::call(0));

        self.current_function.push_instruction(InstructionBuilder::pop(0));
        self.current_function.push_instruction(InstructionBuilder::addi_imm(STACK_PTR_REG, STACK_PTR_REG, -(expr.args.len() as i32)));
    }

    fn visit_struct_initializer(&mut self, expr: &crate::expr::StructInitializerExpr) -> () {
        todo!()
    }
}