//! # Alya VM
//!
//! A modular, educational virtual machine with a clean instruction set.
//!
//! ## Modules
//!
//! - `core` — Foundation types (registers, opcodes, flags)
//! - `error` — Unified error handling
//! - `memory` — Memory manager + stack
//! - `instruction` — Instruction types + program container
//! - `execution` — VM execution engine
//! - `assembler` — Source-to-instruction assembler pipeline

pub mod core;
pub mod error;
pub mod memory;
pub mod instruction;
pub mod execution;
pub mod assembler;

// Re-export commonly used types
pub use core::{Register, Opcode, Flags};
pub use error::{VmError, VmResult};
pub use execution::VM;
