use std::{io::Read, thread::sleep, time::Duration};

use crate::{instruction::Instruction, opcodes};

const N_REGISTERS: usize = 16;
const N_MEM_LOCATIONS: usize = 65536;
const BYTES_ZERO: Bytes = Bytes::zero();

const RET_ADDR_REG: usize = 15;
const STACK_PTR_REG: usize = 14;
const FRAME_PTR_REG: usize = 13;
const ARG_PTR_REG: usize = 12;

pub const REG_0: u8 = 0;
pub const REG_1: u8 = 1;
pub const REG_2: u8 = 2;
pub const REG_3: u8 = 3;
pub const REG_4: u8 = 4;
pub const REG_5: u8 = 5;
pub const REG_6: u8 = 6;
pub const REG_7: u8 = 7;
pub const REG_8: u8 = 8;
pub const REG_9: u8 = 9;
pub const REG_10: u8 = 10;
pub const REG_11: u8 = 11;


pub const VM_PC_INITIAL: usize = 0;

#[derive(PartialEq, Eq)]
pub struct ConditionCode {
    pub n: bool,
    pub z: bool,
    pub p: bool,
}

impl ConditionCode {
    pub fn new(n: bool, z: bool, p: bool) -> ConditionCode {
        ConditionCode { n, z, p }
    }

    pub fn from_bytes_i64(bytes: &Bytes) -> ConditionCode {
        ConditionCode::from_cmp_i64(bytes.as_i64(), 0)
    }

    pub fn from_bytes_f64(bytes: &Bytes) -> ConditionCode {
        ConditionCode::from_cmp_f64(bytes.as_f64(), 0.0)
    }

    pub fn from_cmp_i64(a: i64, b: i64) -> ConditionCode {
        ConditionCode {
            n: a < b,
            z: a == b,
            p: a > b
        }
    }

    pub fn from_cmp_f64(a: f64, b: f64) -> ConditionCode {
        ConditionCode {
            n: a < b,
            z: a == b,
            p: a > b
        }
    }

    pub fn has_match(matches: &ConditionCode, actual: &ConditionCode) -> bool {
        return (matches.n && actual.n) || (matches.z && actual.z) || (matches.p && actual.p)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Bytes {
    bytes: [u8; 8]
}

impl Bytes {
    pub fn new(bytes: [u8; 8]) -> Bytes {
        Bytes { bytes }
    }
    
    pub const fn zero() -> Bytes {
        Bytes {
            bytes: [0; 8]
        }
    }

    pub fn as_u64(&self) -> u64 {
        u64::from_le_bytes(self.bytes)
    }

    pub fn as_i64(&self) -> i64 {
        i64::from_le_bytes(self.bytes)
    }

    pub fn as_f64(&self) -> f64 {
        f64::from_le_bytes(self.bytes)
    }

    pub fn as_bool(&self) -> bool {
        self.as_u64() != 0
    }

    pub fn from_u64(val: u64) -> Bytes {
        Bytes {
            bytes: val.to_le_bytes()
        }
    }

    pub fn from_bool(val: bool) -> Bytes {
        let mut bytes = [0; 8];
        bytes[0] = val as u8;

        Bytes {
            bytes
        }
    }

    pub fn from_i64(val: i64) -> Bytes {
        Bytes {
            bytes: val.to_le_bytes()
        }
    }

    pub fn from_f64(val: f64) -> Bytes {
        Bytes {
            bytes: val.to_le_bytes()
        }
    }
}

pub struct VM {
    pub registers: [Bytes; N_REGISTERS],
    pub memory: [Bytes; N_MEM_LOCATIONS],
    pub cc: ConditionCode,
    pub pc: usize,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [BYTES_ZERO; N_REGISTERS],
            memory: [BYTES_ZERO; N_MEM_LOCATIONS],
            cc: ConditionCode::new(false, false, false),
            pc: VM_PC_INITIAL
        }
    }

    pub fn write_log(&self, message: &str) {
        //println!("{}", message);
    }

    pub fn load_memory(&mut self, origin: usize, values: &[u64], n_values: usize) {
        for i in 0..n_values {
            self.memory[origin + i] = Bytes::from_u64(values[i]);
        }
    }

    pub fn step(&mut self) -> bool {
        let instruction = self.fetch();

        if instruction.opcode == opcodes::OP_HALT {
            return false;
        }

        self.execute(instruction);
        
        true
    }

    pub fn run(&mut self) {
        let mut instruction: Instruction = self.fetch();

        while instruction.opcode != opcodes::OP_HALT {
            self.execute(instruction);
            instruction = self.fetch();
        }

        self.write_log(&format!("HALT"));
    }

    fn fetch(&mut self) -> Instruction {
        let instruction = Instruction::new(self.memory[self.pc].as_u64());
        self.pc += 1;

        instruction
    }

    fn execute(&mut self, instruction: Instruction) {        
        match instruction.opcode {
            opcodes::OP_ADDI => self.execute_addi(instruction),
            opcodes::OP_ADDD => self.execute_addd(instruction),
            opcodes::OP_SUBI => self.execute_subi(instruction),
            opcodes::OP_SUBD => self.execute_subd(instruction),
            opcodes::OP_MULTI => self.execute_multi(instruction),
            opcodes::OP_MULTD => self.execute_multd(instruction),
            opcodes::OP_DIVI => self.execute_divi(instruction),
            opcodes::OP_DIVD => self.execute_divd(instruction),
            opcodes::OP_AND => self.execute_and(instruction),
            opcodes::OP_OR => self.execute_or(instruction),
            opcodes::OP_XOR => self.execute_xor(instruction),
            opcodes::OP_NOT => self.execute_not(instruction),
            opcodes::OP_NOTB => self.execute_notb(instruction),
            opcodes::OP_NEGI => self.execute_negi(instruction),
            opcodes::OP_NEGD => self.execute_negd(instruction),
            opcodes::OP_LSHIFT => self.execute_lshift(instruction),
            opcodes::OP_RSHIFT => self.execute_rshift(instruction),
            opcodes::OP_CMPI => self.execute_cmpi(instruction),
            opcodes::OP_CMPD => self.execute_cmpd(instruction),
            opcodes::OP_ITOD => self.execute_itod(instruction),
            opcodes::OP_DTOI => self.execute_dtoi(instruction),
            opcodes::OP_LDR => self.execute_ldr(instruction),
            opcodes::OP_STR => self.execute_str(instruction),
            opcodes::OP_PUSH => self.execute_push(instruction),
            opcodes::OP_POP => self.execute_pop(instruction),
            opcodes::OP_LDSP => self.execute_ldsp(instruction),
            opcodes::OP_LDFP => self.execute_ldfp(instruction),
            opcodes::OP_LDAP => self.execute_ldap(instruction),
            opcodes::OP_STSP => self.execute_stsp(instruction),
            opcodes::OP_STFP => self.execute_stfp(instruction),
            opcodes::OP_STAP => self.execute_stap(instruction),
            opcodes::OP_JMP => self.execute_jmp(instruction),
            opcodes::OP_LD => self.execute_ld(instruction),
            opcodes::OP_LDI => self.execute_ldi(instruction),
            opcodes::OP_ST => self.execute_st(instruction),
            opcodes::OP_STI => self.execute_sti(instruction),
            opcodes::OP_NOP => self.execute_nop(instruction),
            opcodes::OP_CALL => self.execute_call(instruction),
            opcodes::OP_PUTC => self.execute_putc(instruction),
            opcodes::OP_GETC => self.execute_getc(instruction),
            opcodes::OP_RET => self.execute_ret(instruction),
            _ => self.write_log(&format!("Unknown opcode: {:#010b}", instruction.opcode)),
        }
    }
}



impl VM {
    fn execute_addi(&mut self, instruction: Instruction) {

        let a = self.registers[instruction.sr1].as_i64();
        let b: i64 = if instruction.use_imm32 {
            self.write_log(&format!("ADDI DR={} SR1={} imm32={}", instruction.dr, instruction.sr1, i32::from_le_bytes(instruction.imm32)));
            i32::from_le_bytes(instruction.imm32) as i64
        } else {
            self.write_log(&format!("ADDI DR={} SR1={} SR2={}", instruction.dr, instruction.sr1, instruction.sr2));
            self.registers[instruction.sr2].as_i64()
        };

        let result = Bytes::from_i64(a + b);

        self.cc = ConditionCode::from_bytes_i64(&result);
        self.registers[instruction.dr] = result;
    }
    
    fn execute_addd(&mut self, instruction: Instruction) {
        self.write_log(&format!("ADDD DR={} SR1={} SR2={}", instruction.dr, instruction.sr1, instruction.sr2));

        let a = self.registers[instruction.sr1].as_f64();
        let b = self.registers[instruction.sr2].as_f64();

        let result = Bytes::from_f64(a + b);
        
        self.cc = ConditionCode::from_bytes_f64(&result);
        self.registers[instruction.dr] = result;
    }
    
    fn execute_subi(&mut self, instruction: Instruction) {
        let a = self.registers[instruction.sr1].as_i64();
        let b = if instruction.use_imm32 {
            self.write_log(&format!("SUBI DR={} SR1={} imm32={}", instruction.dr, instruction.sr1, i32::from_le_bytes(instruction.imm32)));
            i32::from_le_bytes(instruction.imm32) as i64
        } else {
            self.write_log(&format!("SUBI DR={} SR1={} SR2={}", instruction.dr, instruction.sr1, instruction.sr2));
            self.registers[instruction.sr2].as_i64()
        };

        let result = Bytes::from_i64(a - b);
        
        self.cc = ConditionCode::from_bytes_i64(&result);
        self.registers[instruction.dr] = result;
    }
    
    fn execute_subd(&mut self, instruction: Instruction) {
        self.write_log(&format!("SUBD DR={} SR1={} SR2={}", instruction.dr, instruction.sr1, instruction.sr2));

        let a = self.registers[instruction.sr1].as_f64();
        let b = self.registers[instruction.sr2].as_f64();

        let result = Bytes::from_f64(a - b);
        
        self.cc = ConditionCode::from_bytes_f64(&result);
        self.registers[instruction.dr] = result;
    }
    
    fn execute_multi(&mut self, instruction: Instruction) {
        let a = self.registers[instruction.sr1].as_i64();
        let b = if instruction.use_imm32 {
            self.write_log(&format!("MULTI DR={} SR1={} imm32={}", instruction.dr, instruction.sr1, i32::from_le_bytes(instruction.imm32)));
            i32::from_le_bytes(instruction.imm32) as i64
        } else {
            self.write_log(&format!("MULTI DR={} SR1={} SR2={}", instruction.dr, instruction.sr1, instruction.sr2));
            self.registers[instruction.sr2].as_i64()
        };

        self.registers[instruction.dr] = Bytes::from_i64(a * b);
    }
    
    fn execute_multd(&mut self, instruction: Instruction) {
        self.write_log(&format!("MULTD DR={} SR1={} SR2={}", instruction.dr, instruction.sr1, instruction.sr2));

        let a = self.registers[instruction.sr1].as_f64();
        let b = self.registers[instruction.sr2].as_f64();

        self.registers[instruction.dr] = Bytes::from_f64(a * b);
    }
    
    fn execute_divi(&mut self, instruction: Instruction) {
        let a = self.registers[instruction.sr1].as_i64();
        let b = if instruction.use_imm32 {
            self.write_log(&format!("DIVI DR={} SR1={} imm32={}", instruction.dr, instruction.sr1, i32::from_le_bytes(instruction.imm32)));
            i32::from_le_bytes(instruction.imm32) as i64
        } else {
            self.write_log(&format!("DIVI DR={} SR1={} SR2={}", instruction.dr, instruction.sr1, instruction.sr2));
            self.registers[instruction.sr2].as_i64()
        };

        self.registers[instruction.dr] = Bytes::from_i64(a / b);
    }

    fn execute_divd(&mut self, instruction: Instruction) {
        self.write_log(&format!("DIVD DR={} SR1={} SR2={}", instruction.dr, instruction.sr1, instruction.sr2));

        let a = self.registers[instruction.sr1].as_f64();
        let b = self.registers[instruction.sr2].as_f64();

        self.registers[instruction.dr] = Bytes::from_f64(a / b);
    }
    
    fn execute_and(&mut self, instruction: Instruction) {        
        let a = self.registers[instruction.sr1].as_u64();
        let b = if instruction.use_imm32 {
            self.write_log(&format!("AND DR={} SR1={} imm32={}", instruction.dr, instruction.sr1, u32::from_le_bytes(instruction.imm32)));
            u32::from_le_bytes(instruction.imm32) as u64
        } else {
            self.write_log(&format!("AND DR={} SR1={} SR2={}", instruction.dr, instruction.sr1, instruction.sr2));
            self.registers[instruction.sr2].as_u64()
        };

        let result = Bytes::from_u64(a & b);
        
        self.cc = ConditionCode::from_bytes_i64(&result);
        self.registers[instruction.dr] = result;
    }
    
    fn execute_or(&mut self, instruction: Instruction) {
        let a = self.registers[instruction.sr1].as_u64();
        let b = if instruction.use_imm32 {
            self.write_log(&format!("OR DR={} SR1={} imm32={}", instruction.dr, instruction.sr1, u32::from_le_bytes(instruction.imm32)));
            u32::from_le_bytes(instruction.imm32) as u64
        } else {
            self.write_log(&format!("OR DR={} SR1={} SR2={}", instruction.dr, instruction.sr1, instruction.sr2));
            self.registers[instruction.sr2].as_u64()
        };

        let result = Bytes::from_u64(a | b);
        
        self.cc = ConditionCode::from_bytes_i64(&result);
        self.registers[instruction.dr] = result;
    }
    
    fn execute_xor(&mut self, instruction: Instruction) {
        let a = self.registers[instruction.sr1].as_u64();
        let b = if instruction.use_imm32 {
            self.write_log(&format!("XOR DR={} SR1={} imm32={}", instruction.dr, instruction.sr1, u32::from_le_bytes(instruction.imm32)));
            u32::from_le_bytes(instruction.imm32) as u64
        } else {
            self.write_log(&format!("XOR DR={} SR1={} SR2={}", instruction.dr, instruction.sr1, instruction.sr2));
            self.registers[instruction.sr2].as_u64()
        };

        let result = Bytes::from_u64(a ^ b);
        
        self.cc = ConditionCode::from_bytes_i64(&result);
        self.registers[instruction.dr] = result;
    }
    
    fn execute_not(&mut self, instruction: Instruction) {
        self.write_log(&format!("NOT DR={} SR1={}", instruction.dr, instruction.sr1));

        let a = self.registers[instruction.sr1].as_u64();

        let result: Bytes = Bytes::from_u64(!a);
        
        self.cc = ConditionCode::from_bytes_i64(&result);
        self.registers[instruction.dr] = result;
    }

    fn execute_notb(&mut self, instruction: Instruction) {
        self.write_log(&format!("NOTB DR={} SR1={}", instruction.dr, instruction.sr1));

        let a = self.registers[instruction.sr1].as_bool();

        let result: Bytes = Bytes::from_bool(!a);

        
        self.cc = ConditionCode::from_bytes_i64(&result);
        self.registers[instruction.dr] = result;
    }
    
    fn execute_negi(&mut self, instruction: Instruction) {
        self.write_log(&format!("NEGI DR={} SR1={}", instruction.dr, instruction.sr1));

        let a = self.registers[instruction.sr1].as_u64();

        self.registers[instruction.dr] = Bytes::from_u64(!a + 1);
    }
    
    fn execute_negd(&mut self, instruction: Instruction) {
        self.write_log(&format!("NEGD DR={} SR1={}", instruction.dr, instruction.sr1));

        let a = self.registers[instruction.sr1].as_f64();

        self.registers[instruction.dr] = Bytes::from_f64(-a);
    }
    
    fn execute_lshift(&mut self, instruction: Instruction) {
        let a = self.registers[instruction.sr1].as_u64();
        let b = if instruction.use_imm32 {
            self.write_log(&format!("LSHIFT DR={} SR1={} imm32={}", instruction.dr, instruction.sr1, u32::from_le_bytes(instruction.imm32)));
            u32::from_le_bytes(instruction.imm32) as u64
        } else {
            self.write_log(&format!("LSHIFT DR={} SR1={} SR2={}", instruction.dr, instruction.sr1, instruction.sr2));
            self.registers[instruction.sr2].as_u64()
        };

        self.registers[instruction.dr] = Bytes::from_u64(a << b);
    }
    
    fn execute_rshift(&mut self, instruction: Instruction) {
        let a = self.registers[instruction.sr1].as_i64();
        let b = if instruction.use_imm32 {
            self.write_log(&format!("RSHIFT DR={} SR1={} imm32={}", instruction.dr, instruction.sr1, i32::from_le_bytes(instruction.imm32)));
            i32::from_le_bytes(instruction.imm32) as i64
        } else {
            self.write_log(&format!("RSHIFT DR={} SR1={} SR2={}", instruction.dr, instruction.sr1, instruction.sr2));
            self.registers[instruction.sr2].as_i64()
        };

        self.registers[instruction.dr] = Bytes::from_i64(a >> b);
    }
    
    fn execute_cmpi(&mut self, instruction: Instruction) {
        self.write_log(&format!("SR1 Value: {}, SR2 Value: {}", self.registers[instruction.sr1].as_i64(), self.registers[instruction.sr2].as_i64()));

        let a = self.registers[instruction.sr1].as_i64();
        let b = if instruction.use_imm32 {
            self.write_log(&format!("CMPI DR={} SR1={} imm32={} N={} Z={} P={}", instruction.dr, instruction.sr1, i32::from_le_bytes(instruction.imm32), instruction.cc.n, instruction.cc.z, instruction.cc.p));
            i32::from_le_bytes(instruction.imm32) as i64
        } else {
            self.write_log(&format!("CMPI DR={} SR1={} SR2={} N={} Z={} P={}", instruction.dr, instruction.sr1, instruction.sr2, instruction.cc.n, instruction.cc.z, instruction.cc.p));
            self.registers[instruction.sr2].as_i64()
        };
        
        let cmp_cc = ConditionCode::from_cmp_i64(a, b);
        let cc_match = ConditionCode::has_match(&instruction.cc, &cmp_cc);
        let result = Bytes::from_bool(cc_match);
        self.cc = ConditionCode::from_bytes_i64(&result);
        self.write_log(&format!("CMP result={}", result.as_bool()));
        self.write_log(&format!("CMP N={} Z={} P={}", self.cc.n, self.cc.z, self.cc.p));
        self.registers[instruction.dr] = result;
    }
    
    fn execute_cmpd(&mut self, instruction: Instruction) {
        let a = self.registers[instruction.sr1].as_f64();
        let b = if instruction.use_imm32 {
            self.write_log(&format!("CMPD DR={} SR1={} imm32={} N={} Z={} P={}", instruction.dr, instruction.sr1, i32::from_le_bytes(instruction.imm32), instruction.cc.n, instruction.cc.z, instruction.cc.p));
            f32::from_le_bytes(instruction.imm32) as f64
        } else {
            self.write_log(&format!("CMPD DR={} SR1={} SR2={} N={} Z={} P={}", instruction.dr, instruction.sr1, instruction.sr2, instruction.cc.n, instruction.cc.z, instruction.cc.p));
            self.registers[instruction.sr2].as_f64()
        };
        
        self.cc = ConditionCode::from_cmp_f64(a, b);
        let cc_match = ConditionCode::has_match(&instruction.cc, &self.cc);
        self.registers[instruction.dr] = Bytes::from_bool(cc_match);
    }

    fn execute_dtoi(&mut self, instruction: Instruction) {
        self.write_log(&format!("DTOI DR={} SR1={}", instruction.dr, instruction.sr1));

        let a = self.registers[instruction.sr1].as_f64();
        self.registers[instruction.dr] = Bytes::from_i64(a as i64);
    }
    
    fn execute_itod(&mut self, instruction: Instruction) {
        self.write_log(&format!("ITOD DR={} SR1={}", instruction.dr, instruction.sr1));

        let a = self.registers[instruction.sr1].as_i64();
        self.registers[instruction.dr] = Bytes::from_f64(a as f64);
    }

    fn execute_ldr(&mut self, instruction: Instruction) {
        let base = self.registers[instruction.sr1].as_u64();
        let offset = if instruction.use_imm32 {
            self.write_log(&format!("LDR DR={} SR1={} imm32={}", instruction.dr, instruction.sr1, u32::from_le_bytes(instruction.imm32)));
            u32::from_le_bytes(instruction.imm32) as u64
        } else {
            self.write_log(&format!("LDR DR={} SR1={} SR2={}", instruction.dr, instruction.sr1, instruction.sr2));
            self.registers[instruction.sr2].as_u64()
        };

        self.write_log(&format!("Address: {}, value: {}", base + offset, self.memory[(base + offset) as usize].as_u64()));

        self.registers[instruction.dr] = self.memory[(base + offset) as usize];
    }
    
    fn execute_str(&mut self, instruction: Instruction) {
        let base = self.registers[instruction.sr1].as_u64();
        let offset = if instruction.use_imm32 {
            self.write_log(&format!("STR SR3={} SR1={} imm32={}", instruction.sr3, instruction.sr1, u32::from_le_bytes(instruction.imm32)));
            u32::from_le_bytes(instruction.imm32) as u64
        } else {
            self.write_log(&format!("STR SR3={} SR1={} SR2={}", instruction.sr3, instruction.sr1, instruction.sr2));
            self.registers[instruction.sr2].as_u64()
        };

        self.memory[(base + offset) as usize] = self.registers[instruction.sr3]
    }
    
    fn execute_push(&mut self, instruction: Instruction) {
        let mut stack_ptr = self.registers[STACK_PTR_REG].as_u64();
        stack_ptr += 1;
        self.registers[STACK_PTR_REG] = Bytes::from_u64(stack_ptr);

        let value = if instruction.use_imm32 {
            self.write_log(&format!("PUSH imm32={} Value={}", u32::from_le_bytes(instruction.imm32), u32::from_le_bytes(instruction.imm32) as u64));
            u32::from_le_bytes(instruction.imm32) as u64
        } else {
            self.write_log(&format!("PUSH SR2={} Value={}", instruction.sr2, self.registers[instruction.sr2].as_u64()));
            self.registers[instruction.sr2].as_u64()
        };

        self.memory[stack_ptr as usize] = Bytes::from_u64(value);
    }
    
    fn execute_pop(&mut self, instruction: Instruction) {

        let mut stack_ptr = self.registers[STACK_PTR_REG].as_u64();

        self.registers[instruction.dr] = self.memory[stack_ptr as usize];
        stack_ptr -= 1;
        self.registers[STACK_PTR_REG] = Bytes::from_u64(stack_ptr);
        self.write_log(&format!("POP DR={} result={}", instruction.dr, self.registers[instruction.dr].as_u64()));

    }
    
    fn execute_ldsp(&mut self, instruction: Instruction) {
        self.write_log(&format!("LDSP DR={} imm32={}", instruction.dr, i32::from_le_bytes(instruction.imm32)));

        let stack_ptr = self.registers[STACK_PTR_REG].as_u64() as i64;
        let offset = i32::from_le_bytes(instruction.imm32) as i64;
        self.registers[instruction.dr] = self.memory[(stack_ptr + offset) as usize];
    }
    
    fn execute_ldfp(&mut self, instruction: Instruction) {

        let frame_ptr = self.registers[FRAME_PTR_REG].as_u64() as i64;
        let offset = i32::from_le_bytes(instruction.imm32) as i64;
        self.registers[instruction.dr] = self.memory[(frame_ptr + offset) as usize];
        self.write_log(&format!("LDFP DR={} imm32={}, Current FP={}, Value={}", instruction.dr, i32::from_le_bytes(instruction.imm32), self.registers[FRAME_PTR_REG].as_u64(), self.memory[(frame_ptr + offset) as usize].as_u64()));
    }

    fn execute_ldap(&mut self, instruction: Instruction) {
        self.write_log(&format!("LDAP DR={} imm32={}", instruction.dr, i32::from_le_bytes(instruction.imm32)));

        let arg_ptr = self.registers[ARG_PTR_REG].as_u64() as i64;
        let offset = i32::from_le_bytes(instruction.imm32) as i64;
        self.registers[instruction.dr] = self.memory[(arg_ptr + offset) as usize];
    }

    fn execute_stsp(&mut self, instruction: Instruction) {
        self.write_log(&format!("STSP SR1={} imm32={}", instruction.sr1, i32::from_le_bytes(instruction.imm32)));

        let stack_ptr = self.registers[STACK_PTR_REG].as_u64() as i64;
        let offset = i32::from_le_bytes(instruction.imm32) as i64;
        self.memory[(stack_ptr + offset) as usize] = self.registers[instruction.sr1]
    }
    
    fn execute_stfp(&mut self, instruction: Instruction) {
        self.write_log(&format!("STFP SR1={} imm32={}, Current FP={}, Value={}", instruction.sr1, i32::from_le_bytes(instruction.imm32), self.registers[FRAME_PTR_REG].as_u64(), self.registers[instruction.sr1].as_u64()));

        let frame_ptr = self.registers[FRAME_PTR_REG].as_u64() as i64;
        let offset = i32::from_le_bytes(instruction.imm32) as i64;
        self.memory[(frame_ptr + offset) as usize] = self.registers[instruction.sr1];
    }

    fn execute_stap(&mut self, instruction: Instruction) {
        self.write_log(&format!("STAP SR1={} imm32={}", instruction.sr1, i32::from_le_bytes(instruction.imm32)));

        let arg_ptr = self.registers[ARG_PTR_REG].as_u64() as i64;
        let offset = i32::from_le_bytes(instruction.imm32) as i64;
        self.memory[(arg_ptr + offset) as usize] = self.registers[instruction.sr1];
    }
    
    fn execute_jmp(&mut self, instruction: Instruction) {
        self.write_log(&format!("JMP pcOffset32={} N={}, Z={}, P={}", instruction.pc_offset32, instruction.cc.n, instruction.cc.z, instruction.cc.p));

        if ConditionCode::has_match(&instruction.cc, &self.cc) {
            self.pc = (self.pc as i64 + instruction.pc_offset32 as i64) as usize;
        }
    }
    
    fn execute_ld(&mut self, instruction: Instruction) {
        self.write_log(&format!("LD DR={} pcOffset32={}", instruction.dr, instruction.pc_offset32));

        let addr = self.pc as i64 + instruction.pc_offset32 as i64;
        self.registers[instruction.dr] = self.memory[addr as usize];
    }
    
    fn execute_ldi(&mut self, instruction: Instruction) {
        self.write_log(&format!("LDI DR={} pcOffset32={}", instruction.dr, instruction.pc_offset32));

        let addr = self.pc as i64 + instruction.pc_offset32 as i64;
        self.registers[instruction.dr] = self.memory[self.memory[addr as usize].as_u64() as usize];
    }
    
    fn execute_st(&mut self, instruction: Instruction) {
        self.write_log(&format!("ST SR1={} pcOffset32={}", instruction.sr1, instruction.pc_offset32));

        let addr = self.pc as i64 + instruction.pc_offset32 as i64;
        self.memory[addr as usize] = self.registers[instruction.sr1];
    }
    
    fn execute_sti(&mut self, instruction: Instruction) {

        let addr = self.pc as i64 + instruction.pc_offset32 as i64;
        self.write_log(&format!("STI SR1={} pcOffset32={}, value={}, address={}", instruction.sr1, instruction.pc_offset32, addr, self.registers[instruction.sr1].as_u64()));

        self.memory[self.memory[addr as usize].as_u64() as usize] = self.registers[instruction.sr1];
    }

    fn execute_nop(&mut self, _instruction: Instruction) {
        self.write_log(&format!("NOP"));
    }
    
    fn execute_call(&mut self, instruction: Instruction) {
        self.write_log(&format!("CALL sr1={}", instruction.sr1));

        self.registers[RET_ADDR_REG] = Bytes::from_u64(self.pc as u64);
        self.pc = self.registers[instruction.sr1].as_u64() as usize;
    }

    fn execute_ret(&mut self, _instruction: Instruction) {
        self.write_log(&format!("RET to address={}", self.registers[RET_ADDR_REG].as_u64()));

        self.pc = self.registers[RET_ADDR_REG].as_u64() as usize;
    }    

    fn execute_putc(&mut self, instruction: Instruction) {
        self.write_log(&format!("PUTC SR1={} value={}", instruction.sr1, self.registers[instruction.sr1].as_u64()));
        let value = self.registers[instruction.sr1].as_u64();
        print!("{}", value as u8 as char);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    }

    fn execute_getc(&mut self, instruction: Instruction) {
        let mut buffer = [0; 2];
        std::io::stdin().read_exact(&mut buffer).unwrap();
        self.registers[instruction.dr] = Bytes::from_u64(buffer[0] as u64);
    }
}