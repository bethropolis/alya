//! Control flow instruction handlers.

use crate::core::Register;
use crate::execution::context::ExecutionContext;
use crate::error::VmError;

/// Execute Compare: set flags based on left - right
pub fn handle_compare(ctx: &mut ExecutionContext, left: Register, right: Register) {
    let a = ctx.get_reg(left) as i64;
    let b = ctx.get_reg(right) as i64;
    let diff = a.wrapping_sub(b);
    ctx.flags.set_zero(diff == 0);
    ctx.flags.set_negative(diff < 0);
    ctx.flags.set_carry(a < b); // "borrow" flag for unsigned comparison
}

/// Execute Jump: unconditional jump
pub fn handle_jump(ctx: &mut ExecutionContext, target: usize) {
    ctx.pc = target;
}

/// Execute JumpIfZero
pub fn handle_jump_if_zero(ctx: &mut ExecutionContext, target: usize) {
    if ctx.flags.zero() {
        ctx.pc = target;
    }
}

/// Execute JumpIfNotZero
pub fn handle_jump_if_not_zero(ctx: &mut ExecutionContext, target: usize) {
    if !ctx.flags.zero() {
        ctx.pc = target;
    }
}

/// Execute JumpIfGt (greater than: not zero and not negative)
pub fn handle_jump_if_gt(ctx: &mut ExecutionContext, target: usize) {
    if !ctx.flags.zero() && !ctx.flags.negative() {
        ctx.pc = target;
    }
}

/// Execute JumpIfLt (less than: negative)
pub fn handle_jump_if_lt(ctx: &mut ExecutionContext, target: usize) {
    if ctx.flags.negative() {
        ctx.pc = target;
    }
}

/// Execute JumpIfGe (greater or equal: not negative)
pub fn handle_jump_if_ge(ctx: &mut ExecutionContext, target: usize) {
    if !ctx.flags.negative() {
        ctx.pc = target;
    }
}

/// Execute JumpIfLe (less or equal: zero or negative)
pub fn handle_jump_if_le(ctx: &mut ExecutionContext, target: usize) {
    if ctx.flags.zero() || ctx.flags.negative() {
        ctx.pc = target;
    }
}

/// Execute JumpIfEq (equal: zero flag set)
pub fn handle_jump_if_eq(ctx: &mut ExecutionContext, target: usize) {
    if ctx.flags.zero() {
        ctx.pc = target;
    }
}

/// Execute JumpIfNe (not equal: zero flag not set)
pub fn handle_jump_if_ne(ctx: &mut ExecutionContext, target: usize) {
    if !ctx.flags.zero() {
        ctx.pc = target;
    }
}

/// Execute Call: push return address, jump to target
pub fn handle_call(ctx: &mut ExecutionContext, target: usize) {
    ctx.call_stack.push(ctx.pc);
    ctx.pc = target;
}

/// Execute Return: pop return address, jump back
pub fn handle_return(ctx: &mut ExecutionContext) -> Result<(), VmError> {
    let return_addr = ctx.call_stack.pop()
        .ok_or_else(|| VmError::Execution("Return without matching call".to_string()))?;
    ctx.pc = return_addr;
    Ok(())
}
