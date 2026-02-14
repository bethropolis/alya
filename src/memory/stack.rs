//! Stack operations for the VM.

use super::{MemoryAccess};
use std::fmt;

/// Stack manager that operates on memory.
/// The stack grows downward from the top of memory.
pub struct Stack {
    pointer: usize,
    base: usize,
}

impl Stack {
    /// Create a new stack with the given base (top of stack region).
    /// The stack grows downward.
    pub fn new(base: usize) -> Self {
        Self {
            pointer: base,
            base,
        }
    }

    /// Create a stack with a custom initial pointer
    pub fn with_pointer(pointer: usize, base: usize) -> Self {
        Self { pointer, base }
    }

    /// Push a value onto the stack using external memory
    pub fn push(&mut self, memory: &mut dyn MemoryAccess, value: u64) -> Result<(), StackError> {
        if self.pointer < 8 {
            return Err(StackError::Overflow);
        }

        self.pointer -= 8;
        memory
            .write_qword(self.pointer, value)
            .map_err(|e| StackError::MemoryError(format!("{}", e)))?;

        Ok(())
    }

    /// Pop a value from the stack using external memory
    pub fn pop(&mut self, memory: &dyn MemoryAccess) -> Result<u64, StackError> {
        if self.pointer >= self.base {
            return Err(StackError::Underflow);
        }

        let value = memory
            .read_qword(self.pointer)
            .map_err(|e| StackError::MemoryError(format!("{}", e)))?;

        self.pointer += 8;
        Ok(value)
    }

    /// Peek at the top of the stack without removing
    pub fn peek(&self, memory: &dyn MemoryAccess) -> Result<u64, StackError> {
        if self.pointer >= self.base {
            return Err(StackError::Empty);
        }

        memory
            .read_qword(self.pointer)
            .map_err(|e| StackError::MemoryError(format!("{}", e)))
    }

    /// Get the current stack pointer
    pub fn pointer(&self) -> usize {
        self.pointer
    }

    /// Set the stack pointer
    pub fn set_pointer(&mut self, addr: usize) {
        self.pointer = addr;
    }

    /// Get the base (bottom) of the stack
    pub fn base(&self) -> usize {
        self.base
    }
}

/// Stack-related errors
#[derive(Debug, Clone, PartialEq)]
pub enum StackError {
    Overflow,
    Underflow,
    Empty,
    MemoryError(String),
}

impl fmt::Display for StackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StackError::Overflow => write!(f, "Stack overflow"),
            StackError::Underflow => write!(f, "Stack underflow"),
            StackError::Empty => write!(f, "Stack is empty"),
            StackError::MemoryError(msg) => write!(f, "Stack memory error: {}", msg),
        }
    }
}

impl std::error::Error for StackError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::Memory;

    #[test]
    fn test_stack_push_pop() {
        let mut mem = Memory::new(1024);
        let mut stack = Stack::new(1024);

        stack.push(&mut mem, 42).unwrap();
        stack.push(&mut mem, 99).unwrap();

        assert_eq!(stack.pop(&mem).unwrap(), 99);
        assert_eq!(stack.pop(&mem).unwrap(), 42);
    }

    #[test]
    fn test_stack_peek() {
        let mut mem = Memory::new(1024);
        let mut stack = Stack::new(1024);

        stack.push(&mut mem, 42).unwrap();
        assert_eq!(stack.peek(&mem).unwrap(), 42);
        assert_eq!(stack.peek(&mem).unwrap(), 42); // Still there
        assert_eq!(stack.pop(&mem).unwrap(), 42);
    }

    #[test]
    fn test_stack_underflow() {
        let mem = Memory::new(1024);
        let mut stack = Stack::new(1024);

        assert!(stack.pop(&mem).is_err());
    }
}
