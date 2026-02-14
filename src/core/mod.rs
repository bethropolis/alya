//! Core types and definitions for Alya VM.
//!
//! This module contains the fundamental building blocks:
//! - Registers (general-purpose and special)
//! - Opcodes (instruction identifiers)
//! - Flags (CPU status flags)
//!
//! These types have NO dependencies on other modules.

mod register;
mod opcode;
mod flags;

pub use register::{Register, RegisterError};
pub use opcode::{Opcode, OpcodeError};
pub use flags::{Flags, Flag};

/// Re-export commonly used items
pub mod prelude {
    pub use super::{Register, Opcode, Flags};
}
