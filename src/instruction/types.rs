//! Instruction type definitions.
//!
//! Each variant holds just the data needed for execution.
//! The VM's executor dispatches on these variants.

use crate::core::Register;

/// A single VM instruction with its operands.
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    // === Control ===
    Halt,
    Nop,

    // === Data Movement ===
    /// Load an immediate value into a register
    LoadImm { dest: Register, value: u64 },
    /// Copy value from src to dest
    Move { dest: Register, src: Register },
    /// Swap values of two registers
    Swap { r1: Register, r2: Register },

    // === Arithmetic ===
    /// dest = left op right
    Add { dest: Register, left: Register, right: Register },
    Sub { dest: Register, left: Register, right: Register },
    Mul { dest: Register, left: Register, right: Register },
    Div { dest: Register, left: Register, right: Register },
    Mod { dest: Register, left: Register, right: Register },

    // === Compound Assignment ===
    /// dest += src (or immediate)
    AddAssign { dest: Register, src: Register },
    SubAssign { dest: Register, src: Register },
    MulAssign { dest: Register, src: Register },
    DivAssign { dest: Register, src: Register },

    // === Bitwise ===
    And { dest: Register, left: Register, right: Register },
    Or  { dest: Register, left: Register, right: Register },
    Xor { dest: Register, left: Register, right: Register },
    Not { dest: Register, src: Register },
    Shl { dest: Register, left: Register, right: Register },
    Shr { dest: Register, left: Register, right: Register },

    // === Stack ===
    Push { src: Register },
    Pop { dest: Register },
    Peek { dest: Register },

    // === Memory ===
    /// Load from memory address in src register into dest
    Load { dest: Register, addr_reg: Register },
    /// Store value from src register to memory address in addr register
    Store { src: Register, addr_reg: Register },
    /// Load from base[index] — address = base_reg + index_reg * 8
    LoadIndexed { dest: Register, base_reg: Register, index_reg: Register },
    /// Store to base[index] — address = base_reg + index_reg * 8
    StoreIndexed { src: Register, base_reg: Register, index_reg: Register },

    // === Control Flow ===
    /// Unconditional jump to instruction index
    Jump { target: usize },
    /// Compare two registers, set flags
    Compare { left: Register, right: Register },
    /// Conditional jumps (use flags set by Compare)
    JumpIfZero { target: usize },
    JumpIfNotZero { target: usize },
    JumpIfGt { target: usize },
    JumpIfLt { target: usize },
    JumpIfGe { target: usize },
    JumpIfLe { target: usize },
    JumpIfEq { target: usize },
    JumpIfNe { target: usize },

    // === Functions ===
    /// Call: push return address, jump to target
    Call { target: usize },
    /// Return: pop return address, jump back
    Return,

    // === I/O ===
    /// Print register value as integer
    Print { src: Register },
    /// Print register value as ASCII character
    PrintChar { src: Register },

    // === Debug ===
    Debug { src: Register },
}
