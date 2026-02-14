//! Memory subsystem for Alya VM.
//!
//! Provides:
//! - Main memory manager
//! - Stack operations
//! - Address validation

mod manager;
pub mod stack;
mod address;

pub use manager::{Memory, MemoryError};
pub use stack::{Stack, StackError};
pub use address::{Address, AddressError};

/// Trait for memory operations (allows mocking in tests)
pub trait MemoryAccess {
    fn read_byte(&self, addr: usize) -> Result<u8, MemoryError>;
    fn write_byte(&mut self, addr: usize, value: u8) -> Result<(), MemoryError>;
    fn read_qword(&self, addr: usize) -> Result<u64, MemoryError>;
    fn write_qword(&mut self, addr: usize, value: u64) -> Result<(), MemoryError>;
    fn size(&self) -> usize;
}

/// Trait for stack operations
pub trait StackAccess {
    fn push(&mut self, value: u64) -> Result<(), StackError>;
    fn pop(&mut self) -> Result<u64, StackError>;
    fn peek(&self) -> Result<u64, StackError>;
    fn pointer(&self) -> usize;
    fn set_pointer(&mut self, addr: usize);
}
