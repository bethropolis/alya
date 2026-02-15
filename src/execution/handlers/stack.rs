//! Stack instruction handlers.

use crate::core::Register;
use crate::execution::context::ExecutionContext;
use crate::memory::Memory;
use crate::memory::stack::Stack;
use crate::error::VmError;

/// Execute Push: push register value onto stack
pub fn handle_push(ctx: &mut ExecutionContext, stack: &mut Stack, memory: &mut Memory, src: Register) -> Result<(), VmError> {
    let value = ctx.get_reg(src);
    Ok(stack.push(memory, value)?)
}

/// Execute Pop: pop top of stack into register
pub fn handle_pop(ctx: &mut ExecutionContext, stack: &mut Stack, memory: &Memory, dest: Register) -> Result<(), VmError> {
    let value = stack.pop(memory)?;
    ctx.set_reg(dest, value);
    Ok(())
}

/// Execute Peek: read top of stack without removing
pub fn handle_peek(ctx: &mut ExecutionContext, stack: &Stack, memory: &Memory, dest: Register) -> Result<(), VmError> {
    let value = stack.peek(memory)?;
    ctx.set_reg(dest, value);
    Ok(())
}
