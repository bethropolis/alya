//! Execution context — register file and flags state.

use crate::core::{Register, Flags};

/// Holds the mutable state of the VM during execution.
pub struct ExecutionContext {
    /// Register values (indexed by Register::to_u8())
    pub registers: [u64; Register::COUNT],
    /// CPU flags
    pub flags: Flags,
    /// Program counter (index into instruction list)
    pub pc: usize,
    /// Whether the VM is halted
    pub halted: bool,
    /// Call stack for return addresses
    pub call_stack: Vec<usize>,
    /// Whether tracing is enabled
    pub trace: bool,
}

impl ExecutionContext {
    /// Create a new execution context
    pub fn new() -> Self {
        Self {
            registers: [0; Register::COUNT],
            flags: Flags::new(),
            pc: 0,
            halted: false,
            call_stack: Vec::new(),
            trace: false,
        }
    }

    /// Get a register value
    pub fn get_reg(&self, reg: Register) -> u64 {
        self.registers[reg.to_u8() as usize]
    }

    /// Set a register value
    pub fn set_reg(&mut self, reg: Register, value: u64) {
        self.registers[reg.to_u8() as usize] = value;
    }

    /// Reset the context
    pub fn reset(&mut self) {
        self.registers = [0; Register::COUNT];
        self.flags = Flags::new();
        self.pc = 0;
        self.halted = false;
        self.call_stack.clear();
        self.trace = false;
    }
}

impl Default for ExecutionContext {
    fn default() -> Self {
        Self::new()
    }
}
