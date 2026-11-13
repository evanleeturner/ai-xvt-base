// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00425d80(param_1: i16, param_2: u32) -> u32 {
    let u_var2: u32 = (param_1 as i32).wrapping_mul(0x1234).wrapping_add(0x80) as u32;
    let u_var3: u32 = u_var2 >> 8;
    let d: u32 = param_2 & 0xffff;
    let mut u_var1: u32;
    // SAFETY: unsigned divide by the unchecked divisor, using the machine's
    // own `div` so a zero divisor faults as in the original.
    unsafe {
        core::arch::asm!(
            "div {d}",
            d = in(reg) d,
            inout("eax") u_var3 => u_var1,
            inout("edx") 0u32 => _,
            options(nomem, nostack)
        );
    }
    if (u_var2 >> 9) < (u_var3 & param_2 & 0xffff) {
        u_var1 = u_var1.wrapping_add(1);
    }
    u_var1
}
