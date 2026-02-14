//! Arithmetic instruction handlers.

use crate::core::Register;
use crate::execution::context::ExecutionContext;
use crate::error::VmError;

/// Execute Add: dest = left + right
pub fn handle_add(ctx: &mut ExecutionContext, dest: Register, left: Register, right: Register) {
    let a = ctx.get_reg(left);
    let b = ctx.get_reg(right);
    let (result, overflow) = a.overflowing_add(b);
    ctx.set_reg(dest, result);
    ctx.flags.update_from_result(result, overflow);
}

/// Execute Sub: dest = left - right
pub fn handle_sub(ctx: &mut ExecutionContext, dest: Register, left: Register, right: Register) {
    let a = ctx.get_reg(left);
    let b = ctx.get_reg(right);
    let (result, overflow) = a.overflowing_sub(b);
    ctx.set_reg(dest, result);
    ctx.flags.update_from_result(result, overflow);
}

/// Execute Mul: dest = left * right
pub fn handle_mul(ctx: &mut ExecutionContext, dest: Register, left: Register, right: Register) {
    let a = ctx.get_reg(left);
    let b = ctx.get_reg(right);
    let (result, overflow) = a.overflowing_mul(b);
    ctx.set_reg(dest, result);
    ctx.flags.update_from_result(result, overflow);
}

/// Execute Div: dest = left / right
pub fn handle_div(ctx: &mut ExecutionContext, dest: Register, left: Register, right: Register) -> Result<(), VmError> {
    let a = ctx.get_reg(left);
    let b = ctx.get_reg(right);
    if b == 0 {
        return Err(VmError::DivisionByZero);
    }
    let result = a / b;
    ctx.set_reg(dest, result);
    ctx.flags.update_from_result(result, false);
    Ok(())
}

/// Execute Mod: dest = left % right
pub fn handle_mod(ctx: &mut ExecutionContext, dest: Register, left: Register, right: Register) -> Result<(), VmError> {
    let a = ctx.get_reg(left);
    let b = ctx.get_reg(right);
    if b == 0 {
        return Err(VmError::DivisionByZero);
    }
    let result = a % b;
    ctx.set_reg(dest, result);
    ctx.flags.update_from_result(result, false);
    Ok(())
}

/// Execute AddAssign: dest += src
pub fn handle_add_assign(ctx: &mut ExecutionContext, dest: Register, src: Register) {
    let a = ctx.get_reg(dest);
    let b = ctx.get_reg(src);
    let (result, overflow) = a.overflowing_add(b);
    ctx.set_reg(dest, result);
    ctx.flags.update_from_result(result, overflow);
}

/// Execute SubAssign: dest -= src
pub fn handle_sub_assign(ctx: &mut ExecutionContext, dest: Register, src: Register) {
    let a = ctx.get_reg(dest);
    let b = ctx.get_reg(src);
    let (result, overflow) = a.overflowing_sub(b);
    ctx.set_reg(dest, result);
    ctx.flags.update_from_result(result, overflow);
}

/// Execute MulAssign: dest *= src
pub fn handle_mul_assign(ctx: &mut ExecutionContext, dest: Register, src: Register) {
    let a = ctx.get_reg(dest);
    let b = ctx.get_reg(src);
    let (result, overflow) = a.overflowing_mul(b);
    ctx.set_reg(dest, result);
    ctx.flags.update_from_result(result, overflow);
}

/// Execute DivAssign: dest /= src
pub fn handle_div_assign(ctx: &mut ExecutionContext, dest: Register, src: Register) -> Result<(), VmError> {
    let a = ctx.get_reg(dest);
    let b = ctx.get_reg(src);
    if b == 0 {
        return Err(VmError::DivisionByZero);
    }
    let result = a / b;
    ctx.set_reg(dest, result);
    ctx.flags.update_from_result(result, false);
    Ok(())
}
