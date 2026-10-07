use std::io;
use std::path::Path;

use crate::cpu::Cpu;
use crate::loader::BinaryLoader;
use crate::memory::Memory;

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
}
