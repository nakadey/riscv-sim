use std::fs;
use std::io;
use std::path::Path;

use crate::memory::Memory;

/// A loader for binary files into memory.
pub struct BinaryLoader;

/// Loads a binary file into `memory` starting at address 0.
///
/// The binary must fit entirely within the provided memory region.
///
/// # Errors
///
/// - Returns `io::Error` if file I/O fails or if loading exceeds memory bounds.
///
/// # Notes
///
/// The error type is currently `io::Result` for simplicity. A dedicated loader
/// error type may be introduced later to preserve the underlying errors.
impl BinaryLoader {
    pub fn load(path: &Path, memory: &mut Memory) -> io::Result<()> {
        let data = fs::read(path)?;

        // TODO: Preserve memory traps separately from file I/O errors when
        // a dedicated loader error type is introduced.
        memory
            .write_bytes(0, &data)
            .map_err(|_| io::Error::other("Memory access fault"))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    use crate::memory::Memory;

    #[test]
    fn load_binary_into_memory() {
        let mut memory = Memory::new(4);
        BinaryLoader::load(&Path::new("tests/fixtures/bin/test.bin"), &mut memory).unwrap();

        assert_eq!(memory.read_u32(0), Ok(0x1234_5678));
    }

    #[test]
    fn load_fails_when_binary_exceeds_memory() {
        let mut memory = Memory::new(2);
        let result = BinaryLoader::load(&Path::new("tests/fixtures/bin/test.bin"), &mut memory);

        assert!(result.is_err());
    }
}
