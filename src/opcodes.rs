pub const OP_NOP: u8 = 0b000000;
pub const OP_ADDI: u8 = 0b000001;
pub const OP_ADDD: u8 = 0b000010;
pub const OP_SUBI: u8 = 0b000011;
pub const OP_SUBD: u8 = 0b000100;
pub const OP_MULTI: u8 = 0b000101;
pub const OP_MULTD: u8 = 0b000110;
pub const OP_DIVI: u8 = 0b000111;
pub const OP_DIVD: u8 = 0b001000;
pub const OP_AND: u8 = 0b001001;
pub const OP_OR: u8 = 0b001010;
pub const OP_XOR: u8 = 0b001011;
pub const OP_NOT: u8 = 0b001100;
pub const OP_NEGI: u8 = 0b001101;
pub const OP_NEGD: u8 = 0b001110;
pub const OP_LSHIFT: u8 = 0b001111;
pub const OP_RSHIFT: u8 = 0b010000;
pub const OP_CMPI: u8 = 0b010010;
pub const OP_CMPD: u8 = 0b010011;
pub const OP_ITOD: u8 = 0b010100;
pub const OP_DTOI: u8 = 0b010101;
pub const OP_NOTB: u8 = 0b010110;
pub const OP_LDR: u8 = 0b100000;
pub const OP_STR: u8 = 0b100001;
pub const OP_PUSH: u8 = 0b100010;
pub const OP_POP: u8 = 0b100011;
pub const OP_LDSP: u8 = 0b100100;
pub const OP_LDFP: u8 = 0b100101;
pub const OP_LDAP: u8 = 0b100110;
pub const OP_STSP: u8 = 0b101110;
pub const OP_STFP: u8 = 0b101111;
pub const OP_STAP: u8 = 0b110000;
pub const OP_JMP: u8 = 0b100111;
pub const OP_LD: u8 = 0b101000;
pub const OP_LDI: u8 = 0b101001;
pub const OP_ST: u8 = 0b101010;
pub const OP_STI: u8 = 0b101011;
pub const OP_CALL: u8 = 0b110001;
pub const OP_PUTC: u8 = 0b110010;
pub const OP_GETC: u8 = 0b110011;
pub const OP_RET: u8 = 0b101100;
pub const OP_HALT: u8 = 0b101101;