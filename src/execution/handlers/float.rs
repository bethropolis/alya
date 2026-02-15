//! Floating-point instruction handlers.

use crate::core::Register;
use crate::execution::context::ExecutionContext;

/// Execute FAdd: dest = left + right
pub fn handle_fadd(ctx: &mut ExecutionContext, dest: Register, left: Register, right: Register) {
    let a = f64::from_bits(ctx.get_reg(left));
    let b = f64::from_bits(ctx.get_reg(right));
    ctx.set_reg(dest, (a + b).to_bits());
}

/// Execute FSub: dest = left - right
pub fn handle_fsub(ctx: &mut ExecutionContext, dest: Register, left: Register, right: Register) {
    let a = f64::from_bits(ctx.get_reg(left));
    let b = f64::from_bits(ctx.get_reg(right));
    ctx.set_reg(dest, (a - b).to_bits());
}

/// Execute FMul: dest = left * right
pub fn handle_fmul(ctx: &mut ExecutionContext, dest: Register, left: Register, right: Register) {
    let a = f64::from_bits(ctx.get_reg(left));
    let b = f64::from_bits(ctx.get_reg(right));
    ctx.set_reg(dest, (a * b).to_bits());
}

/// Execute FDiv: dest = left / right
pub fn handle_fdiv(ctx: &mut ExecutionContext, dest: Register, left: Register, right: Register) {
    let a = f64::from_bits(ctx.get_reg(left));
    let b = f64::from_bits(ctx.get_reg(right));
    ctx.set_reg(dest, (a / b).to_bits());
}

/// Execute FSqrt: dest = sqrt(src)
pub fn handle_fsqrt(ctx: &mut ExecutionContext, dest: Register, src: Register) {
    let a = f64::from_bits(ctx.get_reg(src));
    ctx.set_reg(dest, a.sqrt().to_bits());
}

/// Execute FAbs: dest = abs(src)
pub fn handle_fabs(ctx: &mut ExecutionContext, dest: Register, src: Register) {
    let a = f64::from_bits(ctx.get_reg(src));
    ctx.set_reg(dest, a.abs().to_bits());
}

/// Execute FNeg: dest = -src
pub fn handle_fneg(ctx: &mut ExecutionContext, dest: Register, src: Register) {
    let a = f64::from_bits(ctx.get_reg(src));
    ctx.set_reg(dest, (-a).to_bits());
}

/// Execute F2I: dest = (u64)src
pub fn handle_f2i(ctx: &mut ExecutionContext, dest: Register, src: Register) {
    let a = f64::from_bits(ctx.get_reg(src));
    ctx.set_reg(dest, a as u64);
}

/// Execute I2F: dest = (f64)src
pub fn handle_i2f(ctx: &mut ExecutionContext, dest: Register, src: Register) {
    let a = ctx.get_reg(src) as f64;
    ctx.set_reg(dest, a.to_bits());
}

/// Execute FCmp: set flags based on left vs right
pub fn handle_fcmp(ctx: &mut ExecutionContext, left: Register, right: Register) {
    let a = f64::from_bits(ctx.get_reg(left));
    let b = f64::from_bits(ctx.get_reg(right));

    // Reset flags
    ctx.flags.set_zero(false);
    ctx.flags.set_negative(false);
    ctx.flags.set_overflow(false);
    ctx.flags.set_carry(false);

    if a == b {
        ctx.flags.set_zero(true);
    } else if a < b {
        ctx.flags.set_negative(true);
    } else if a > b {
        // Just leave zero and negative as false
    } else {
        // NaN case: set carry as a marker for unordered
        ctx.flags.set_carry(true);
    }
}
