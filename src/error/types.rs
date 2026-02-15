use std::fmt;
use crate::core::{RegisterError, OpcodeError};
use crate::memory::{MemoryError, StackError};

/// Unified error type for the entire VM.
#[derive(Debug, Clone, PartialEq)]
pub enum VmError {
    /// Register-related errors
    Register(RegisterError),
    /// Opcode-related errors
    Opcode(OpcodeError),
    /// Memory access errors
    Memory(MemoryError),
    /// Stack errors
    Stack(StackError),
    /// Execution errors
    Execution(String),
    /// Assembler errors
    Assembler(String),
    /// I/O errors
    Io(String),
    /// Division by zero
    DivisionByZero,
    /// Halt instruction encountered
    Halted,
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmError::Register(e) => write!(f, "Register error: {}", e),
            VmError::Opcode(e) => write!(f, "Opcode error: {}", e),
            VmError::Memory(e) => write!(f, "Memory error: {}", e),
            VmError::Stack(e) => write!(f, "Stack error: {}", e),
            VmError::Execution(msg) => write!(f, "Execution error: {}", msg),
            VmError::Assembler(msg) => write!(f, "Assembler error: {}", msg),
            VmError::Io(msg) => write!(f, "I/O error: {}", msg),
            VmError::DivisionByZero => write!(f, "Division by zero"),
            VmError::Halted => write!(f, "VM halted"),
        }
    }
}

impl std::error::Error for VmError {}

impl From<RegisterError> for VmError {
    fn from(e: RegisterError) -> Self {
        VmError::Register(e)
    }
}

impl From<OpcodeError> for VmError {
    fn from(e: OpcodeError) -> Self {
        VmError::Opcode(e)
    }
}

impl From<MemoryError> for VmError {
    fn from(e: MemoryError) -> Self {
        VmError::Memory(e)
    }
}

impl From<StackError> for VmError {
    fn from(e: StackError) -> Self {
        VmError::Stack(e)
    }
}
