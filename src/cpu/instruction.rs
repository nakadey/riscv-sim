/// A decoded RISC-V instruction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    Addi { rd: u8, rs1: u8, imm: u32 },
    Bne { rs1: u8, rs2: u8, imm: u32 },
    Lui { rd: u8, imm: u32 },
    Ecall,
}
