use crate::opcodes;

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
}
pub struct InstructionBuilder {
    pub instruction: u64,
}

impl InstructionBuilder {
    pub fn new() -> InstructionBuilder {
        InstructionBuilder {
            instruction: 0
        }
    }

    pub fn with_opcode(mut self, opcode: u8) -> InstructionBuilder {
        self.instruction |= (opcode as u64) << 57;
        self
    }

    pub fn with_dr(mut self, dr: u8) -> InstructionBuilder {
        self.instruction |= ((dr as u64) & 0xF) << 53;
        self
    }

    pub fn with_sr1(mut self, sr1: u8) -> InstructionBuilder {
        self.instruction |= (sr1 as u64) & 0xF;
        self
    }

    pub fn with_sr2(mut self, sr2: u8) -> InstructionBuilder {
        self.instruction |= ((sr2 as u64) & 0xF) << 4;
        self
    }

    pub fn with_sr3(mut self, sr3: u8) -> InstructionBuilder {
        self.instruction |= ((sr3 as u64) & 0xF) << 53;
        self
    }

    pub fn with_nzp(mut self, n: bool, z: bool, p: bool) -> InstructionBuilder {
        self.instruction |= (n as u64) << 39;
        self.instruction |= (z as u64) << 38;
        self.instruction |= (p as u64) << 37;

        self
    }

    pub fn with_imm32i(mut self, imm32: i32) -> InstructionBuilder {
        self.instruction |= 1u64 << 52;
        self.instruction |= ((u32::from_le_bytes(imm32.to_le_bytes())) as u64) << 4;

        self
    }

    pub fn with_imm32u(mut self, imm32: u32) -> InstructionBuilder {
        self.instruction |= 1u64 << 52;
        self.instruction |= (imm32 as u64) << 4;

        self
    }

    pub fn with_pcOffset32(mut self, pcOffset32: i32) -> InstructionBuilder {
        self.instruction |= ((u32::from_le_bytes(pcOffset32.to_le_bytes())) as u64) << 4;
        self
    }
}

impl InstructionBuilder {
    pub fn nop() -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_NOP).instruction
    }

    pub fn addi(dr: u8, sr1: u8, sr2: u8) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_ADDI)
            .with_dr(dr)
            .with_sr1(sr1)
            .with_sr2(sr2).instruction
    }

    pub fn addi_imm(dr: u8, sr1: u8, imm32: i32) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_ADDI)
            .with_dr(dr)
            .with_sr1(sr1)
            .with_imm32i(imm32).instruction
    }

    pub fn addd(dr: u8, sr1: u8, sr2: u8) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_ADDD)
            .with_dr(dr)
            .with_sr1(sr1)
            .with_sr2(sr2).instruction
    }

    pub fn subi(dr: u8, sr1: u8, sr2: u8) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_SUBI)
            .with_dr(dr)
            .with_sr1(sr1)
            .with_sr2(sr2).instruction
    }

    pub fn subi_imm(dr: u8, sr1: u8, imm32: i32) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_SUBI)
            .with_dr(dr)
            .with_sr1(sr1)
            .with_imm32i(imm32).instruction
    }

    pub fn subd(dr: u8, sr1: u8, sr2: u8) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_SUBD)
            .with_dr(dr)
            .with_sr1(sr1)
            .with_sr2(sr2).instruction
    }

    pub fn multi(dr: u8, sr1: u8, sr2: u8) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_MULTI)
            .with_dr(dr)
            .with_sr1(sr1)
            .with_sr2(sr2).instruction
    }

    pub fn multi_imm(dr: u8, sr1: u8, imm32: i32) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_MULTI)
            .with_dr(dr)
            .with_sr1(sr1)
            .with_imm32i(imm32).instruction
    }

    pub fn multd(dr: u8, sr1: u8, sr2: u8) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_MULTD)
            .with_dr(dr)
            .with_sr1(sr1)
            .with_sr2(sr2).instruction
    }

    pub fn divi(dr: u8, sr1: u8, sr2: u8) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_DIVI)
            .with_dr(dr)
            .with_sr1(sr1)
            .with_sr2(sr2).instruction
    }

    pub fn divi_imm(dr: u8, sr1: u8, imm32: i32) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_DIVI)
            .with_dr(dr)
            .with_sr1(sr1)
            .with_imm32i(imm32).instruction
    }

    pub fn divd(dr: u8, sr1: u8, sr2: u8) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_DIVD)
            .with_dr(dr)
            .with_sr1(sr1)
            .with_sr2(sr2).instruction
    }

    pub fn and(dr: u8, sr1: u8, sr2: u8) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_AND)
            .with_dr(dr)
            .with_sr1(sr1)
            .with_sr2(sr2).instruction
    }

    pub fn and_imm(dr: u8, sr1: u8, imm32: i32) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_AND)
            .with_dr(dr)
            .with_sr1(sr1)
            .with_imm32i(imm32).instruction
    }

    pub fn or(dr: u8, sr1: u8, sr2: u8) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_OR)
            .with_dr(dr)
            .with_sr1(sr1)
            .with_sr2(sr2).instruction
    }

    pub fn or_imm(dr: u8, sr1: u8, imm32: i32) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_OR)
            .with_dr(dr)
            .with_sr1(sr1)
            .with_imm32i(imm32).instruction
    }

    pub fn xor(dr: u8, sr1: u8, sr2: u8) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_XOR)
            .with_dr(dr)
            .with_sr1(sr1)
            .with_sr2(sr2).instruction
    }

    pub fn xor_imm(dr: u8, sr1: u8, imm32: i32) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_XOR)
            .with_dr(dr)
            .with_sr1(sr1)
            .with_imm32i(imm32).instruction
    }

    pub fn not(dr: u8, sr1: u8) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_NOT)
            .with_dr(dr)
            .with_sr1(sr1).instruction
    }

    pub fn notb(dr: u8, sr1: u8) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_NOTB)
            .with_dr(dr)
            .with_sr1(sr1).instruction
    }

    pub fn negi(dr: u8, sr1: u8) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_NEGI)
            .with_dr(dr)
            .with_sr1(sr1).instruction
    }

    pub fn negd(dr: u8, sr1: u8) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_NEGD)
            .with_dr(dr)
            .with_sr1(sr1).instruction
    }

    pub fn lshift(dr: u8, sr1: u8, sr2: u8) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_LSHIFT)
            .with_dr(dr)
            .with_sr1(sr1)
            .with_sr2(sr2).instruction
    }

    pub fn lshift_imm(dr: u8, sr1: u8, imm32: i32) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_LSHIFT)
            .with_dr(dr)
            .with_sr1(sr1)
            .with_imm32i(imm32).instruction
    }

    pub fn rshift(dr: u8, sr1: u8, sr2: u8) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_RSHIFT)
            .with_dr(dr)
            .with_sr1(sr1)
            .with_sr2(sr2).instruction
    }

    pub fn rshift_imm(dr: u8, sr1: u8, imm32: i32) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_RSHIFT)
            .with_dr(dr)
            .with_sr1(sr1)
            .with_imm32i(imm32).instruction
    }

    pub fn cmpi(dr: u8, n: bool, z: bool, p: bool, sr1: u8, sr2: u8) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_CMPI)
            .with_dr(dr)
            .with_nzp(n, z, p)
            .with_sr1(sr1)
            .with_sr2(sr2).instruction
    }

    pub fn cmpi_imm(dr: u8, n: bool, z: bool, p: bool, sr1: u8, imm32: i32) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_CMPI)
            .with_dr(dr)
            .with_nzp(n, z, p)
            .with_sr1(sr1)
            .with_imm32i(imm32).instruction
    }

    pub fn cmpd(dr: u8, n: bool, z: bool, p: bool, sr1: u8, sr2: u8) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_CMPD)
            .with_dr(dr)
            .with_nzp(n, z, p)
            .with_sr1(sr1)
            .with_sr2(sr2).instruction
    }

    pub fn itod(dr: u8, sr1: u8) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_ITOD)
            .with_dr(dr)
            .with_sr1(sr1).instruction
    }

    pub fn dtoi(dr: u8, sr1: u8) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_DTOI)
            .with_dr(dr)
            .with_sr1(sr1).instruction
    }

    pub fn ldr(dr: u8, sr1: u8, sr2: u8) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_LDR)
            .with_dr(dr)
            .with_sr1(sr1)
            .with_sr2(sr2).instruction
    }

    pub fn ldr_imm(dr: u8, sr1: u8, imm32: i32) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_LDR)
            .with_sr1(sr1)
            .with_dr(dr)
            .with_imm32i(imm32).instruction
    }

    pub fn str(sr3: u8, sr1: u8, sr2: u8) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_STR)
            .with_sr3(sr3)
            .with_sr1(sr1)
            .with_sr2(sr2).instruction
    }

    pub fn str_imm(sr3: u8, sr1: u8, imm32: i32) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_STR)
            .with_sr3(sr3)
            .with_sr1(sr1)
            .with_imm32i(imm32).instruction
    }

    pub fn push(sr2: u8) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_PUSH)
            .with_sr2(sr2).instruction
    }

    pub fn push_imm(imm32: i32) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_PUSH)
            .with_imm32i(imm32).instruction
    }

    pub fn pop(dr: u8) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_POP)
            .with_dr(dr).instruction
    }

    pub fn ldsp(dr: u8, imm32: i32) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_LDSP)
            .with_dr(dr)
            .with_imm32i(imm32).instruction
    }

    pub fn ldfp(dr: u8, imm32: i32) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_LDFP)
            .with_dr(dr)
            .with_imm32i(imm32).instruction
    }

    pub fn ldap(dr: u8, imm32: i32) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_LDAP)
            .with_dr(dr)
            .with_imm32i(imm32).instruction
    }

    pub fn stsp(sr1: u8, imm32: i32) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_STSP)
            .with_sr1(sr1)
            .with_imm32i(imm32).instruction
    }

    pub fn stfp(sr1: u8, imm32: i32) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_STFP)
            .with_sr1(sr1)
            .with_imm32i(imm32).instruction
    }

    pub fn stap(sr1: u8, imm32: i32) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_STAP)
            .with_sr1(sr1)
            .with_imm32i(imm32).instruction
    }

    pub fn jmp(n: bool, z: bool, p: bool, pcOffset32: i32) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_JMP)
            .with_nzp(n, z, p)
            .with_pcOffset32(pcOffset32).instruction
    }

    pub fn ld(dr: u8, pcOffset32: i32) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_LD)
            .with_dr(dr)
            .with_pcOffset32(pcOffset32).instruction
    }

    pub fn ldi(dr: u8, pcOffset32: i32) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_LDI)
            .with_dr(dr)
            .with_pcOffset32(pcOffset32).instruction
    }

    pub fn st(pcOffset32: i32, sr1: u8) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_ST)
            .with_sr1(sr1)
            .with_pcOffset32(pcOffset32).instruction
    }

    pub fn sti(pcOffset32: i32, sr1: u8) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_STI)
            .with_sr1(sr1)
            .with_pcOffset32(pcOffset32).instruction
    }

    pub fn call(sr1: u8) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_CALL)
            .with_sr1(sr1).instruction
    }

    pub fn putc(sr1: u8) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_PUTC)
            .with_sr1(sr1).instruction
    }

    pub fn getc(dr: u8) -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_GETC)
            .with_dr(dr).instruction
    }

    pub fn ret() -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_RET).instruction
    }

    pub fn halt() -> u64 {
        InstructionBuilder::new()
            .with_opcode(opcodes::OP_HALT).instruction
    }
}