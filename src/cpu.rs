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
}
