// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00425d10() -> u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004305d0(param_1: u16) -> u16 {
    // SAFETY: the callee is the game's own; the division uses the machine's
    // `div`, which faults on a zero divisor as the original does.
    unsafe {
        let u_var1 = FUN_00425d10() as u32;
        let q: u32;
        core::arch::asm!(
            "div {d:e}",
            d = in(reg) param_1 as u32,
            inout("eax") u_var1 => q,
            inout("edx") 0u32 => _,
            options(nomem, nostack),
        );
        let q16 = q as u16;
        if q16 == 0 {
            return 0;
        }
        (u_var1 % (q16 as u32)) as u16
    }
}
