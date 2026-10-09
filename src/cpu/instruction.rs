/// A decoded RISC-V instruction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    Addi { rd: u8, rs1: u8, imm: u32 },
}
