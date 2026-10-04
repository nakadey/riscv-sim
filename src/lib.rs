pub mod cpu;
pub mod memory;
pub mod trap;

/// The base integer type for the RISC-V architecture.
///
/// Currently `u32` for RV32I.
pub type Xlen = u32;

/// The memory address type for the RISC-V architecture.
///
/// Currently `u32` for RV32I.
pub type Address = u32;
