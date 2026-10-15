use std::io;
use std::path::Path;

use crate::Address;
use crate::cpu::Cpu;
use crate::cpu::decoder;
use crate::loader::BinaryLoader;
use crate::memory;
use crate::memory::Memory;
use crate::trap::{Exception, Trap};

/// The outcome of running a program in the simulator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunOutcome {
    /// The program executed successfully and reached an `ecall` instruction.
    Exited(u32),
    /// The program executed successfully but did not reach an `ecall`
    /// instruction within the given number of steps.
    MaxStepsExceeded,
    /// The program encountered a trap during execution.
    Trapped(Trap),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Simulator {
    cpu: Cpu,
    memory: Memory,
}

impl Simulator {
    /// Creates a new `Simulator` configured with the given memory base
    /// address and size.
    ///
    /// # Notes
    ///
    /// The entry point of the `Cpu` is currently set to `memory_base`.
    /// In the future, this will be updated to automatically load and set the
    /// entry point from an ELF file.
    pub fn new(memory_base: Address, memory_size: usize) -> Self {
        Self {
            cpu: Cpu::new(memory_base),
            memory: Memory::new(memory_base, memory_size),
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

    /// Runs the simulator for a maximum number of steps.
    ///
    /// # Returns
    ///
    /// - `RunOutcome::Exited(a0)` - If the program executes an `ecall`
    ///   instruction. Returns with the value of register `a0`.
    /// - `RunOutcome::MaxStepsExceeded` - If the program exceeds the maximum
    ///   number of steps.
    /// - `RunOutcome::Trapped(trap)` - If a trap occurs during execution.
    ///   Returns with the corresponding `Trap`.
    pub fn run(&mut self, max_steps: usize) -> RunOutcome {
        for _ in 0..max_steps {
            match self.step() {
                Ok(()) => {}
                Err(Trap::Exception(Exception::EnvironmentCall)) => {
                    return RunOutcome::Exited(self.cpu.read_reg(10));
                }
                Err(trap) => return RunOutcome::Trapped(trap),
            }
        }
        RunOutcome::MaxStepsExceeded
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_binary_into_memory() {
        let base = 0x8000_0000;
        let mut simulator = Simulator::new(base, 4);
        simulator
            .load_binary(&Path::new("tests/fixtures/bin/test.bin"))
            .unwrap();

        assert_eq!(simulator.memory.read_u32(base), Ok(0x1234_5678));
    }

    #[test]
    fn step_executes_addi_instruction() {
        let base = 0x8000_0000;
        let mut sim = Simulator::new(base, 4);
        sim.memory.write_u32(base, 0x00a0_0093).unwrap(); // addi x1, x0, 10

        assert_eq!(sim.cpu.pc(), base);
        sim.step().unwrap();

        assert_eq!(sim.cpu.read_reg(1), 10);
        assert_eq!(sim.cpu.pc(), base + 4);
    }

    #[test]
    fn step_traps_misaligned_fetch() {
        let mut sim = Simulator::new(0, 8);
        sim.cpu.set_pc(2); // Misaligned address

        assert_eq!(
            sim.step(),
            Err(Trap::Exception(Exception::InstructionAddressMisaligned))
        );
        assert_eq!(sim.cpu.pc(), 2);
    }

    #[test]
    fn step_traps_out_of_bounds_fetch() {
        let mut sim = Simulator::new(0, 4);
        sim.cpu.set_pc(4); // Out of bounds address

        assert_eq!(
            sim.step(),
            Err(Trap::Exception(Exception::InstructionAccessFault))
        );
        assert_eq!(sim.cpu.pc(), 4);
    }

    #[test]
    fn step_traps_illegal_instruction() {
        let mut sim = Simulator::new(0, 4);
        sim.memory.write_u32(0, 0).unwrap();

        assert_eq!(
            sim.step(),
            Err(Trap::Exception(Exception::IllegalInstruction))
        );
        assert_eq!(sim.cpu.pc(), 0);
    }

    #[test]
    fn run_exits_on_ecall() {
        let base = 0x8000_0000;
        let mut sim = Simulator::new(base, 4);
        sim.memory.write_u32(base, 0x0000_0073).unwrap(); // ecall
        sim.cpu.write_reg(10, 42);

        assert_eq!(sim.run(1), RunOutcome::Exited(42));
    }

    #[test]
    fn run_stops_when_max_steps_exceeded() {
        let base = 0x8000_0000;
        let mut sim = Simulator::new(base, 4);
        sim.memory.write_u32(base, 0x0000_0013).unwrap(); // nop

        assert_eq!(sim.run(1), RunOutcome::MaxStepsExceeded);
    }
}
