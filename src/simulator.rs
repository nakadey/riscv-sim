use std::io;
use std::path::Path;

use crate::cpu::Cpu;
use crate::cpu::decoder;
use crate::loader::BinaryLoader;
use crate::memory;
use crate::memory::Memory;
use crate::trap::{Exception, Trap};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Simulator {
    cpu: Cpu,
    memory: Memory,
}

impl Simulator {
    pub fn new(memory_size: usize) -> Self {
        Self {
            cpu: Cpu::new(),
            memory: Memory::new(memory_size),
        }
    }

    /// Loads a binary file into the simulator's memory.
    pub fn load_binary(&mut self, path: &Path) -> io::Result<()> {
        BinaryLoader::load(path, &mut self.memory)
    }

    /// Fetches, decodes, and executes a single instruction.
    ///
    /// If fetching or decoding fails, the CPU state is left unchanged.
    ///
    /// # Errors
    ///
    /// - Returns a `Trap` if the instruction causes an exception.
    pub fn step(&mut self) -> Result<(), Trap> {
        // Fetch
        let pc = self.cpu.pc();
        let inst = self.memory.read_u32(pc).map_err(|err| match err {
            memory::Error::Misaligned => Trap::Exception(Exception::InstructionAddressMisaligned),
            memory::Error::OutOfBounds => Trap::Exception(Exception::InstructionAccessFault),
        })?;

        // Decode
        let inst = decoder::decode(inst)?;

        // Execute
        self.cpu.execute(inst)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_binary_into_memory() {
        let mut simulator = Simulator::new(4);
        simulator
            .load_binary(&Path::new("tests/fixtures/bin/test.bin"))
            .unwrap();

        assert_eq!(simulator.memory.read_u32(0), Ok(0x1234_5678));
    }

    #[test]
    fn step_executes_addi_instruction() {
        let mut sim = Simulator::new(4);
        sim.memory.write_u32(0, 0x00a0_0093).unwrap(); // addi x1, x0, 10
        sim.step().unwrap();

        assert_eq!(sim.cpu.read_reg(1), 10);
        assert_eq!(sim.cpu.pc(), 4);
    }

    #[test]
    fn step_traps_misaligned_fetch() {
        let mut sim = Simulator::new(8);
        sim.cpu.set_pc(2); // Misaligned address

        assert_eq!(
            sim.step(),
            Err(Trap::Exception(Exception::InstructionAddressMisaligned))
        );
        assert_eq!(sim.cpu.pc(), 2);
    }

    #[test]
    fn step_traps_out_of_bounds_fetch() {
        let mut sim = Simulator::new(4);
        sim.cpu.set_pc(4); // Out of bounds address

        assert_eq!(
            sim.step(),
            Err(Trap::Exception(Exception::InstructionAccessFault))
        );
        assert_eq!(sim.cpu.pc(), 4);
    }

    #[test]
    fn step_traps_illegal_instruction() {
        let mut sim = Simulator::new(4);
        sim.memory.write_u32(0, 0).unwrap();

        assert_eq!(
            sim.step(),
            Err(Trap::Exception(Exception::IllegalInstruction))
        );
        assert_eq!(sim.cpu.pc(), 0);
    }
}
