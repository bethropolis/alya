//! Advanced bitwise instruction handlers.

use crate::core::Register;
use crate::execution::context::ExecutionContext;

/// Execute PopCnt: dest = count_ones(src)
pub fn handle_popcnt(ctx: &mut ExecutionContext, dest: Register, src: Register) {
    let val = ctx.get_reg(src);
    ctx.set_reg(dest, val.count_ones() as u64);
}

/// Execute Clz: dest = leading_zeros(src)
pub fn handle_clz(ctx: &mut ExecutionContext, dest: Register, src: Register) {
    let val = ctx.get_reg(src);
    ctx.set_reg(dest, val.leading_zeros() as u64);
}

/// Execute Ctz: dest = trailing_zeros(src)
pub fn handle_ctz(ctx: &mut ExecutionContext, dest: Register, src: Register) {
    let val = ctx.get_reg(src);
    ctx.set_reg(dest, val.trailing_zeros() as u64);
}

/// Execute BSwap: dest = reverse_bytes(src)
pub fn handle_bswap(ctx: &mut ExecutionContext, dest: Register, src: Register) {
    let val = ctx.get_reg(src);
    ctx.set_reg(dest, val.swap_bytes());
}

/// Execute RotL: dest = left_rotate(left, right)
pub fn handle_rotl(ctx: &mut ExecutionContext, dest: Register, left: Register, right: Register) {
    let val = ctx.get_reg(left);
    let shift = ctx.get_reg(right) as u32;
    ctx.set_reg(dest, val.rotate_left(shift));
}

/// Execute RotR: dest = right_rotate(left, right)
pub fn handle_rotr(ctx: &mut ExecutionContext, dest: Register, left: Register, right: Register) {
    let val = ctx.get_reg(left);
    let shift = ctx.get_reg(right) as u32;
    ctx.set_reg(dest, val.rotate_right(shift));
}
