use super::instruction::Instruction;
use crate::trap::{Exception, Trap};

/// Decodes a 32-bit RISC-V instruction and returns an `Instruction` enum.
///
/// # Errors
///
/// - Returns `IllegalInstruction` if `inst` is not supported.
pub fn decode(inst: u32) -> Result<Instruction, Trap> {
    let opcode = inst & 0x7f;
    let rd = ((inst >> 7) & 0x1f) as u8;
    let funct3 = (inst >> 12) & 0x7;
    let rs1 = ((inst >> 15) & 0x1f) as u8;

    match opcode {
        0b001_0011 => {
            let imm = ((inst as i32) >> 20) as u32; // Sign-extended 12-bit immediate
            match funct3 {
                0b000 => Ok(Instruction::Addi { rd, rs1, imm }),
                _ => Err(Trap::Exception(Exception::IllegalInstruction)),
            }
        }
        _ => Err(Trap::Exception(Exception::IllegalInstruction)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_addi_handles_positive_immediate() {
        assert_eq!(
            decode(0x0010_0093), // addi x1, x0, 1
            Ok(Instruction::Addi {
                rd: 1,
                rs1: 0,
                imm: 1,
            })
        );
    }

    #[test]
    fn decode_addi_handles_negative_immediate() {
        assert_eq!(
            decode(0xfff0_0093), // addi x1, x0, -1
            Ok(Instruction::Addi {
                rd: 1,
                rs1: 0,
                imm: u32::MAX, // -1 as a 32-bit unsigned integer
            })
        );
    }

    #[test]
    fn decode_traps_illegal_instruction() {
        assert_eq!(
            decode(0x0000_0000), // 0 is not a valid RV32I instruction
            Err(Trap::Exception(Exception::IllegalInstruction))
        );
    }
}
