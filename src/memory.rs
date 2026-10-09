use crate::Address;

/// An error type for memory operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Misaligned,
    OutOfBounds,
}

/// A simple, byte-addressable memory model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
    }

    /// Reads a 32-bit little-endian value from the given memory address.
    ///
    /// # Errors
    ///
    /// - Returns `Error::Misaligned` if `addr` is not 4-byte aligned.
    /// - Returns `Error::OutOfBounds` if the access is outside the memory region.
    pub fn read_u32(&self, addr: Address) -> Result<u32, Error> {
        let start = addr as usize;

        if !start.is_multiple_of(4) {
            return Err(Error::Misaligned);
        }
        if start.checked_add(4).is_none_or(|end| end > self.data.len()) {
            return Err(Error::OutOfBounds);
        }

        Ok(u32::from_le_bytes(
            self.data[start..start + 4].try_into().unwrap(),
        ))
    }

    /// Writes a 32-bit little-endian value to the given memory address.
    ///
    /// # Errors
    ///
    /// - Returns `Error::Misaligned` if `addr` is not 4-byte aligned.
    /// - Returns `Error::OutOfBounds` if the access is outside the memory region.
    pub fn write_u32(&mut self, addr: Address, value: u32) -> Result<(), Error> {
        let start = addr as usize;

        if !start.is_multiple_of(4) {
            return Err(Error::Misaligned);
        }
        if start.checked_add(4).is_none_or(|end| end > self.data.len()) {
            return Err(Error::OutOfBounds);
        }

        self.data[start..start + 4].copy_from_slice(&value.to_le_bytes());
        Ok(())
    }

    /// Reads a sequence of bytes from memory without requiring alignment.
    ///
    /// # Errors
    ///
    /// - Returns `Error::OutOfBounds` if the access is outside the memory region.
    pub fn read_bytes(&self, addr: Address, len: usize) -> Result<&[u8], Error> {
        let start = addr as usize;

        if start
            .checked_add(len)
            .is_none_or(|end| end > self.data.len())
        {
            return Err(Error::OutOfBounds);
        }

        Ok(&self.data[start..start + len])
    }

    /// Writes a sequence of bytes to memory without requiring alignment.
    ///
    /// # Errors
    ///
    /// - Returns `Error::OutOfBounds` if the access is outside the memory region.
    pub fn write_bytes(&mut self, addr: Address, value: &[u8]) -> Result<(), Error> {
        let start = addr as usize;

        if start
            .checked_add(value.len())
            .is_none_or(|end| end > self.data.len())
        {
            return Err(Error::OutOfBounds);
        }

        self.data[start..start + value.len()].copy_from_slice(value);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_u32_returns_written_value() {
        let mut mem = Memory::new(4);
        mem.write_u32(0, 0x1234_5678).unwrap();
        assert_eq!(mem.read_u32(0), Ok(0x1234_5678));
    }

    #[test]
    fn read_u32_traps_misaligned_access() {
        let mem = Memory::new(8);
        assert_eq!(mem.read_u32(1), Err(Error::Misaligned));
    }

    #[test]
    fn write_u32_traps_misaligned_access() {
        let mut mem = Memory::new(8);
        assert_eq!(mem.write_u32(1, 0x1234_5678), Err(Error::Misaligned));
    }

    #[test]
    fn read_u32_traps_out_of_bounds_access() {
        let mem = Memory::new(4);
        assert_eq!(mem.read_u32(4), Err(Error::OutOfBounds));
    }

    #[test]
    fn write_u32_traps_out_of_bounds_access() {
        let mut mem = Memory::new(4);
        assert_eq!(mem.write_u32(4, 0x1234_5678), Err(Error::OutOfBounds));
    }

    #[test]
    fn read_bytes_returns_written_bytes() {
        let mut mem = Memory::new(4);
        mem.write_bytes(0, &[0x78, 0x56, 0x34, 0x12]).unwrap();
        assert_eq!(mem.read_bytes(0, 4), Ok(&[0x78, 0x56, 0x34, 0x12][..]));
    }

    #[test]
    fn read_bytes_traps_out_of_bounds_access() {
        let mem = Memory::new(4);
        assert_eq!(mem.read_bytes(2, 4), Err(Error::OutOfBounds));
    }

    #[test]
    fn write_bytes_traps_out_of_bounds_access() {
        let mut mem = Memory::new(4);
        assert_eq!(
            mem.write_bytes(2, &[0x78, 0x56, 0x34, 0x12]),
            Err(Error::OutOfBounds)
        );
    }
}
