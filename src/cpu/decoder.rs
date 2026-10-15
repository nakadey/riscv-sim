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
    let rs2 = ((inst >> 20) & 0x1f) as u8;

    match opcode {
        0b001_0011 => {
            let imm = sign_extend(inst >> 20, 12);
            match funct3 {
                0b000 => Ok(Instruction::Addi { rd, rs1, imm }),
                _ => Err(Trap::Exception(Exception::IllegalInstruction)),
            }
        }
        0b011_0111 => {
            let imm = inst & 0xffff_f000;
            Ok(Instruction::Lui { rd, imm })
        }
        0b110_0011 => {
            let imm = ((inst >> 31) & 0x1) << 12
                | ((inst >> 7) & 0x1) << 11
                | ((inst >> 25) & 0x3f) << 5
                | ((inst >> 8) & 0xf) << 1;
            let imm = sign_extend(imm, 13);

            match funct3 {
                0b001 => Ok(Instruction::Bne { rs1, rs2, imm }),
                _ => Err(Trap::Exception(Exception::IllegalInstruction)),
            }
        }
        0b111_0011 if inst == 0x0000_0073 => Ok(Instruction::Ecall),
        _ => Err(Trap::Exception(Exception::IllegalInstruction)),
    }
}

fn sign_extend(value: u32, bits: u32) -> u32 {
    let shift = u32::BITS - bits;
    ((value << shift) as i32 >> shift) as u32
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
    fn decode_bne_handles_positive_immediate() {
        assert_eq!(
            decode(0x0020_9263), // bne x1, x2, 4
            Ok(Instruction::Bne {
                rs1: 1,
                rs2: 2,
                imm: 4,
            })
        );
    }

    #[test]
    fn decode_bne_handles_negative_immediate() {
        assert_eq!(
            decode(0xfe20_9ee3), // bne x1, x2, -4
            Ok(Instruction::Bne {
                rs1: 1,
                rs2: 2,
                imm: -4i32 as u32,
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
