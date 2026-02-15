//! Assembler pipeline for Alya VM.
//!
//! Converts `.alya` source text into a `Program` of instructions.
//!
//! Pipeline: Source → Lexer → Parser → CodeGen → Program

pub mod lexer;
pub mod parser;
pub mod codegen;

use crate::instruction::Program;
use crate::error::VmError;

/// Assemble source code into a program.
pub fn assemble(source: &str, name: &str) -> Result<Program, VmError> {
    // Parse the source into AST statements
    let statements = parser::parse(source)?;

    // Generate instructions and line table from AST
    let (instructions, data, line_table) = codegen::generate(statements)?;

    let mut program = Program::with_data(name, instructions, data);
    program.line_table = line_table;
    Ok(program)
}
