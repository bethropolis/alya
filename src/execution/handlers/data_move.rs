//! Data movement instruction handlers.

use crate::core::Register;
use crate::execution::context::ExecutionContext;

/// Execute LoadImm: dest = immediate value
pub fn handle_load_imm(ctx: &mut ExecutionContext, dest: Register, value: u64) {
    ctx.set_reg(dest, value);
}

/// Execute Move: dest = src
pub fn handle_move(ctx: &mut ExecutionContext, dest: Register, src: Register) {
    let value = ctx.get_reg(src);
    ctx.set_reg(dest, value);
}

/// Execute Swap: swap(r1, r2)
pub fn handle_swap(ctx: &mut ExecutionContext, r1: Register, r2: Register) {
    let v1 = ctx.get_reg(r1);
    let v2 = ctx.get_reg(r2);
    ctx.set_reg(r1, v2);
    ctx.set_reg(r2, v1);
}
