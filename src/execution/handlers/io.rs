//! I/O instruction handlers.

use crate::core::Register;
use crate::execution::context::ExecutionContext;

/// Execute Print: print register value as integer
pub fn handle_print(ctx: &ExecutionContext, src: Register, output: &mut Vec<String>) {
    let value = ctx.get_reg(src);
    let line = format!("{}", value);
    output.push(line);
}

/// Execute PrintChar: print register value as ASCII character
pub fn handle_print_char(ctx: &ExecutionContext, src: Register, output: &mut Vec<String>) {
    let value = ctx.get_reg(src) as u8 as char;
    let line = format!("{}", value);
    output.push(line);
}

/// Execute Debug: print register name and value
pub fn handle_debug(ctx: &ExecutionContext, src: Register, output: &mut Vec<String>) {
    let value = ctx.get_reg(src);
    let line = format!("DEBUG {} = {} (0x{:x})", src, value, value);
    output.push(line);
}
