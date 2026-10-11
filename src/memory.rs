use std::ops::Range;

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
    base_addr: Address,
    data: Vec<u8>,
}

impl Memory {
    /// Creates a new `Memory` with the given base address and size in bytes.
    pub fn new(base_addr: Address, size: usize) -> Self {
        Self {
            base_addr,
            data: vec![0; size],
        }
    }

    pub fn base_addr(&self) -> Address {
        self.base_addr
    }

    fn checked_range(&self, addr: Address, len: usize) -> Result<Range<usize>, Error> {
        let start = addr.checked_sub(self.base_addr).ok_or(Error::OutOfBounds)? as usize;
        if start
            .checked_add(len)
            .is_none_or(|end| end > self.data.len())
        {
            return Err(Error::OutOfBounds);
        }
        Ok(start..start + len)
    }

    /// Reads a 32-bit little-endian value from the given memory address.
    ///
    /// # Errors
    ///
    /// - Returns `Error::Misaligned` if `addr` is not 4-byte aligned.
    /// - Returns `Error::OutOfBounds` if the access is outside the memory region.
    pub fn read_u32(&self, addr: Address) -> Result<u32, Error> {
        if !addr.is_multiple_of(4) {
            return Err(Error::Misaligned);
        }
        let range = self.checked_range(addr, 4)?;

        Ok(u32::from_le_bytes(self.data[range].try_into().unwrap()))
    }

    /// Writes a 32-bit little-endian value to the given memory address.
    ///
    /// # Errors
    ///
    /// - Returns `Error::Misaligned` if `addr` is not 4-byte aligned.
    /// - Returns `Error::OutOfBounds` if the access is outside the memory region.
    pub fn write_u32(&mut self, addr: Address, value: u32) -> Result<(), Error> {
        if !addr.is_multiple_of(4) {
            return Err(Error::Misaligned);
        }
        let range = self.checked_range(addr, 4)?;

        self.data[range].copy_from_slice(&value.to_le_bytes());
        Ok(())
    }

    /// Reads a sequence of bytes from memory without requiring alignment.
    ///
    /// # Errors
    ///
    /// - Returns `Error::OutOfBounds` if the access is outside the memory region.
    pub fn read_bytes(&self, addr: Address, len: usize) -> Result<&[u8], Error> {
        let range = self.checked_range(addr, len)?;

        Ok(&self.data[range])
    }

    /// Writes a sequence of bytes to memory without requiring alignment.
    ///
    /// # Errors
    ///
    /// - Returns `Error::OutOfBounds` if the access is outside the memory region.
    pub fn write_bytes(&mut self, addr: Address, value: &[u8]) -> Result<(), Error> {
        let range = self.checked_range(addr, value.len())?;

        self.data[range].copy_from_slice(value);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASE: Address = 0x8000_0000;

    #[test]
    fn read_u32_returns_written_value() {
        let mut mem = Memory::new(BASE, 4);
        mem.write_u32(BASE, 0x1234_5678).unwrap();
        assert_eq!(mem.read_u32(BASE), Ok(0x1234_5678));
    }

    #[test]
    fn read_u32_traps_misaligned_access() {
        let mem = Memory::new(BASE, 8);
        assert_eq!(mem.read_u32(BASE + 1), Err(Error::Misaligned));
    }

    #[test]
    fn write_u32_traps_misaligned_access() {
        let mut mem = Memory::new(BASE, 8);
        assert_eq!(mem.write_u32(BASE + 1, 0x1234_5678), Err(Error::Misaligned));
    }

    #[test]
    fn read_u32_traps_out_of_bounds_access() {
        let mem = Memory::new(BASE, 4);
        assert_eq!(mem.read_u32(BASE + 4), Err(Error::OutOfBounds));
        assert_eq!(mem.read_u32(BASE - 4), Err(Error::OutOfBounds));
    }

    #[test]
    fn write_u32_traps_out_of_bounds_access() {
        let mut mem = Memory::new(BASE, 4);
        assert_eq!(
            mem.write_u32(BASE + 4, 0x1234_5678),
            Err(Error::OutOfBounds)
        );
        assert_eq!(
            mem.write_u32(BASE - 4, 0x1234_5678),
            Err(Error::OutOfBounds)
        );
    }

    #[test]
    fn read_bytes_returns_written_bytes() {
        let mut mem = Memory::new(BASE, 4);
        mem.write_bytes(BASE, &[0x78, 0x56, 0x34, 0x12]).unwrap();
        assert_eq!(mem.read_bytes(BASE, 4), Ok(&[0x78, 0x56, 0x34, 0x12][..]));
    }

    #[test]
    fn read_bytes_traps_out_of_bounds_access() {
        let mem = Memory::new(BASE, 4);
        assert_eq!(mem.read_bytes(BASE + 2, 4), Err(Error::OutOfBounds));
        assert_eq!(mem.read_bytes(BASE - 2, 4), Err(Error::OutOfBounds));
    }

    #[test]
    fn write_bytes_traps_out_of_bounds_access() {
        let mut mem = Memory::new(BASE, 4);
        assert_eq!(
            mem.write_bytes(BASE + 2, &[0x78, 0x56, 0x34, 0x12]),
            Err(Error::OutOfBounds)
        );
        assert_eq!(
            mem.write_bytes(BASE - 2, &[0x78, 0x56, 0x34, 0x12]),
            Err(Error::OutOfBounds)
        );
    }
}
