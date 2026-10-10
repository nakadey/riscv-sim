pub mod decoder;
pub mod instruction;

use self::instruction::Instruction;
use crate::trap::Trap;
use crate::{Address, Xlen};

/// RISC-V general-purpose registers (x0 - x31).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Registers {
    regs: [Xlen; 32],
}

impl Registers {
    pub fn new() -> Self {
        Self { regs: [0; 32] }
    }

    /// Reads a value from register `index` (0-31).
    ///
    /// Always returns 0 when reading x0.
    ///
    /// # Panics
    ///
    /// None, as the instruction decoder guarantees the register index is within `0..=31`.
    pub fn read(&self, index: u8) -> Xlen {
        self.regs[index as usize]
    }

    /// Writes a value to register `index` (0-31).
    ///
    /// Writes to x0 are ignored because x0 is hardwired to zero.
    ///
    /// # Panics
    ///
    /// None, as the instruction decoder guarantees the register index is within `0..=31`.
    pub fn write(&mut self, index: u8, value: Xlen) {
        if index != 0 {
            self.regs[index as usize] = value;
        }
    }
}

/// RISC-V CPU core state.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Cpu {
    pc: Address,
    regs: Registers,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            pc: 0,
            regs: Registers::new(),
        }
    }

    pub fn pc(&self) -> Address {
        self.pc
    }

    pub fn set_pc(&mut self, pc: Address) {
        self.pc = pc;
    }

    pub fn read_reg(&self, index: u8) -> Xlen {
        self.regs.read(index)
    }

    pub fn write_reg(&mut self, index: u8, value: Xlen) {
        self.regs.write(index, value);
    }

    /// Executes a single instruction and updates the CPU state accordingly.
    ///
    /// This function is responsible for updating the program counter (PC).
    /// Arithmetic operations follow RISC-V wrapping semantics.
    ///
    /// # Errors
    ///
    /// - Returns a `Trap` if the instruction causes an exception.
    pub fn execute(&mut self, inst: Instruction) -> Result<(), Trap> {
        match inst {
            Instruction::Addi { rd, rs1, imm } => {
                let result = self.regs.read(rs1).wrapping_add(imm);
                self.regs.write(rd, result);
                self.pc = self.pc.wrapping_add(4);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_read_returns_written_value() {
        let mut regs = Registers::new();
        regs.write(1, 42);
        assert_eq!(regs.read(1), 42);
    }

    #[test]
    fn register_x0_always_returns_zero() {
        let mut regs = Registers::new();
        assert_eq!(regs.read(0), 0);
        regs.write(0, 42);
        assert_eq!(regs.read(0), 0);
    }

    #[test]
    fn execute_addi_updates_register_and_pc() {
        let mut cpu = Cpu::new();
        cpu.regs.write(1, 5);

        cpu.execute(Instruction::Addi {
            rd: 2,
            rs1: 1,
            imm: 10,
        })
        .unwrap();

        assert_eq!(cpu.regs.read(2), 15);
        assert_eq!(cpu.pc(), 4);
    }

    #[test]
    fn execute_addi_does_not_change_x0() {
        let mut cpu = Cpu::new();

        cpu.execute(Instruction::Addi {
            rd: 0,
            rs1: 0,
            imm: 10,
        })
        .unwrap();

        assert_eq!(cpu.regs.read(0), 0);
    }

    #[test]
    fn execute_addi_wraps_on_overflow() {
        let mut cpu = Cpu::new();
        cpu.regs.write(1, u32::MAX);

        cpu.execute(Instruction::Addi {
            rd: 2,
            rs1: 1,
            imm: 2,
        })
        .unwrap();

        assert_eq!(cpu.regs.read(2), 1); // u32::MAX + 2 wraps to 1
    }

    #[test]
    fn execute_addi_handles_negative_immediate() {
        let mut cpu = Cpu::new();
        cpu.regs.write(1, 10);

        cpu.execute(Instruction::Addi {
            rd: 2,
            rs1: 1,
            imm: -5i32 as u32,
        })
        .unwrap();

        assert_eq!(cpu.regs.read(2), 5);
    }
}
