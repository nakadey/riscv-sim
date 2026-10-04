use crate::Address;
use crate::trap::{Exception, Trap};

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
    /// - Returns `LoadAddressMisaligned` if `addr` is not 4-byte aligned.
    /// - Returns `LoadAccessFault` if the access is outside the memory region.
    pub fn read_u32(&self, addr: Address) -> Result<u32, Trap> {
        let start = addr as usize;

        if !start.is_multiple_of(4) {
            return Err(Trap::Exception(Exception::LoadAddressMisaligned));
        }
        if start.checked_add(4).is_none_or(|end| end > self.data.len()) {
            return Err(Trap::Exception(Exception::LoadAccessFault));
        }

        Ok(u32::from_le_bytes(
            self.data[start..start + 4].try_into().unwrap(),
        ))
    }

    /// Writes a 32-bit little-endian value to the given memory address.
    ///
    /// # Errors
    ///
    /// - Returns `StoreAddressMisaligned` if `addr` is not 4-byte aligned.
    /// - Returns `StoreAccessFault` if the access is outside the memory region.
    pub fn write_u32(&mut self, addr: Address, value: u32) -> Result<(), Trap> {
        let start = addr as usize;

        if !start.is_multiple_of(4) {
            return Err(Trap::Exception(Exception::StoreAddressMisaligned));
        }
        if start.checked_add(4).is_none_or(|end| end > self.data.len()) {
            return Err(Trap::Exception(Exception::StoreAccessFault));
        }

        self.data[start..start + 4].copy_from_slice(&value.to_le_bytes());
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
        assert_eq!(
            mem.read_u32(1),
            Err(Trap::Exception(Exception::LoadAddressMisaligned))
        );
    }

    #[test]
    fn write_u32_traps_misaligned_access() {
        let mut mem = Memory::new(8);
        assert_eq!(
            mem.write_u32(1, 0x1234_5678),
            Err(Trap::Exception(Exception::StoreAddressMisaligned))
        );
    }

    #[test]
    fn read_u32_traps_out_of_bounds_access() {
        let mem = Memory::new(4);
        assert_eq!(
            mem.read_u32(4),
            Err(Trap::Exception(Exception::LoadAccessFault))
        );
    }

    #[test]
    fn write_u32_traps_out_of_bounds_access() {
        let mut mem = Memory::new(4);
        assert_eq!(
            mem.write_u32(4, 0x1234_5678),
            Err(Trap::Exception(Exception::StoreAccessFault))
        );
    }
}
