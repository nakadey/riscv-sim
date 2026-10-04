/// All RISC-V traps (exceptions and interrupts).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Trap {
    Exception(Exception),
    Interrupt(Interrupt),
}

/// RISC-V synchronous exceptions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Exception {
    LoadAddressMisaligned,
    LoadAccessFault,
    StoreAddressMisaligned,
    StoreAccessFault,
}

/// RISC-V asynchronous interrupts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Interrupt {
    // Reserved for future extensions
}
