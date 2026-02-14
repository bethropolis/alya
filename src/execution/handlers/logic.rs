//! Bitwise/logic instruction handlers.

use crate::core::Register;
use crate::execution::context::ExecutionContext;

/// Execute And: dest = left & right
pub fn handle_and(ctx: &mut ExecutionContext, dest: Register, left: Register, right: Register) {
    let result = ctx.get_reg(left) & ctx.get_reg(right);
    ctx.set_reg(dest, result);
    ctx.flags.update_from_result(result, false);
}

/// Execute Or: dest = left | right
pub fn handle_or(ctx: &mut ExecutionContext, dest: Register, left: Register, right: Register) {
    let result = ctx.get_reg(left) | ctx.get_reg(right);
    ctx.set_reg(dest, result);
    ctx.flags.update_from_result(result, false);
}

/// Execute Xor: dest = left ^ right
pub fn handle_xor(ctx: &mut ExecutionContext, dest: Register, left: Register, right: Register) {
    let result = ctx.get_reg(left) ^ ctx.get_reg(right);
    ctx.set_reg(dest, result);
    ctx.flags.update_from_result(result, false);
}

/// Execute Not: dest = ~src
pub fn handle_not(ctx: &mut ExecutionContext, dest: Register, src: Register) {
    let result = !ctx.get_reg(src);
    ctx.set_reg(dest, result);
    ctx.flags.update_from_result(result, false);
}

/// Execute Shl: dest = left << right
pub fn handle_shl(ctx: &mut ExecutionContext, dest: Register, left: Register, right: Register) {
    let shift = ctx.get_reg(right) as u32;
    let result = ctx.get_reg(left).wrapping_shl(shift);
    ctx.set_reg(dest, result);
    ctx.flags.update_from_result(result, false);
}

/// Execute Shr: dest = left >> right
pub fn handle_shr(ctx: &mut ExecutionContext, dest: Register, left: Register, right: Register) {
    let shift = ctx.get_reg(right) as u32;
    let result = ctx.get_reg(left).wrapping_shr(shift);
    ctx.set_reg(dest, result);
    ctx.flags.update_from_result(result, false);
}
