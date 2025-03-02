use std::collections::HashMap;

use crate::{environment::Literal, expr::{Expr, ExprVisitor}, instruction::InstructionBuilder, resolver::SymbolTable, type_checker::TypeChecker};

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

        for i in (0..12).rev() {
            instructions.push(InstructionBuilder::pop(i));
        }

        instructions.push(InstructionBuilder::addi_imm(STACK_PTR_REG, STACK_PTR_REG, -(self.local_vars.len() as i32)));

        instructions.push(InstructionBuilder::pop(FRAME_PTR_REG));
        instructions.push(InstructionBuilder::pop(RET_ADDR_REG));

        instructions
    }
}

pub struct Compiler<'a> {
    instructions: Vec<u64>,
    function_address_locations: HashMap<i32, u32>,
    current_function: CompilerFunction,
    symbol_table: &'a SymbolTable,
    type_checker: TypeChecker<'a>,
    local_vars: HashMap<i32, u32>,
    constant_pool: Vec<Vec<u64>>,
    constant_pool_size: usize,
}

impl Compiler<'_> {
    pub fn new(symbol_table: &SymbolTable) -> Compiler<'_> {
        Compiler {
            instructions: Vec::new(),
            function_address_locations: HashMap::new(),
            current_function: CompilerFunction::new(),
            type_checker: TypeChecker::new(symbol_table),
            local_vars: HashMap::new(),
            constant_pool: Vec::new(),
            constant_pool_size: 0,
            symbol_table
        }
    }

    fn push_constant(&mut self, bytes: Vec<u64>) -> usize {
        let index = self.constant_pool_size;
        self.constant_pool_size += bytes.len();
        self.constant_pool.push(bytes);

        index
    }

    pub fn compile(mut self, exprs: &[Box<dyn Expr>]) -> Vec<u64> {
        self.instructions.push(InstructionBuilder::and_imm(12, 12, 0));
        self.instructions.push(InstructionBuilder::addi_imm(12, 12, 1000));


        for expr in exprs {
            expr.accept_visitor(&mut self);
        }

        self.instructions.push(InstructionBuilder::halt());

        self.instructions    
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
        let id = self.symbol_table.get_variable_id(expr);
        let stack_index = *self.local_vars.get(&id).unwrap() as i32;

        self.current_function.push_instruction(InstructionBuilder::ldap(0, -stack_index));
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

        let jmp_end_instruction_index = self.instructions.len() as i32;
        self.current_function.push_instruction(InstructionBuilder::nop()); // Placeholder for JMP instruction

        let jmp_else_pc_offset = self.current_function.n_instructions() as i32 - jmp_else_instruction_index - 1;
        self.current_function.set_instruction(jmp_else_instruction_index as usize, InstructionBuilder::jmp(false, true, false, jmp_else_pc_offset));

        if let Some(fail) = &expr.fail {
            println!("Fail");
            fail.accept_visitor(self);
        }

        let jmp_end_pc_offset = self.instructions.len() as i32 - jmp_end_instruction_index - 1;
        self.current_function.set_instruction(jmp_end_instruction_index as usize, InstructionBuilder::jmp(true, true, true, jmp_end_pc_offset));
    }

    fn visit_assignment(&mut self, expr: &crate::expr::AssignmentExpr) -> () {
        let id = self.symbol_table.get_variable_id(&expr.asignee);
        let stack_index = *self.local_vars.get(&id).unwrap() as i32;

        expr.expr.accept_visitor(self);

        self.current_function.push_instruction(InstructionBuilder::stap(0, -stack_index));
    }

    fn visit_declaration(&mut self, expr: &crate::expr::DeclarationExpr) -> () {
        let stack_index = self.local_vars.len() as u32;
        self.local_vars.insert(expr.id, stack_index);

        expr.expr.accept_visitor(self);

        self.current_function.push_instruction(InstructionBuilder::stap(0, -(stack_index as i32)))
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

        let jmp_start_instruction_index = self.instructions.len() as i32;
        let jmp_start_pc_offset = start_loop_instruction_index - jmp_start_instruction_index - 1;
        self.current_function.push_instruction(InstructionBuilder::jmp(true, true, true, jmp_start_pc_offset));

        if let Some(jmp_end_instruction_index) = jmp_end_instruction_index {
            let jmp_end_pc_offset = self.instructions.len() as i32 - jmp_end_instruction_index - 1;
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
        todo!()
    }

    fn visit_struct(&mut self, expr: &crate::expr::StructExpr) -> () {
        todo!()
    }

    fn visit_struct_initializer(&mut self, expr: &crate::expr::StructInitializerExpr) -> () {
        todo!()
    }
}