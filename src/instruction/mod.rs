//! Instruction layer for Alya VM.
//!
//! Provides:
//! - Instruction enum (data-only representation)
//! - Program container

mod types;
mod program;

pub use types::Instruction;
pub use program::Program;

pub mod binary;
