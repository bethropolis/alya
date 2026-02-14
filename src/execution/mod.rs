//! Execution engine for Alya VM.
//!
//! The VM facade owns memory, registers, flags, and a call stack.
//! It dispatches instructions to handler functions.

pub mod vm;
mod context;
mod handlers;

pub use vm::VM;
pub use context::ExecutionContext;
