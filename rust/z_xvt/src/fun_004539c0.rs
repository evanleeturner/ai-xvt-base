// SPDX-License-Identifier: MIT

use core::arch::asm;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00703c6c: i32;
    static mut DAT_00703c70: i32;
    static mut DAT_00703c84: i32;
}

/// Truncating conversion of x87 st0 to i64, popping it (the `__ftol` behavior).
#[inline(always)]
unsafe fn ftol() -> i64 {
    let mut cw: u16 = 0;
    let mut cw2: u16 = 0;
    let mut out: i64 = 0;
    // SAFETY: operates on the x87 stack the caller left st0 on; the control
    // word is saved and restored, and all memory operands are local.
    unsafe {
        asm!(
            "fnstcw word ptr [{cw}]",
            "movzx {t:e}, word ptr [{cw}]",
            "or {t:e}, 0xc00",
            "mov word ptr [{cw2}], {t:x}",
            "fldcw word ptr [{cw2}]",
            "fistp qword ptr [{out}]",
            "fldcw word ptr [{cw}]",
            cw = in(reg) &raw mut cw,
            cw2 = in(reg) &raw mut cw2,
            out = in(reg) &raw mut out,
            t = out(reg) _,
            options(nostack),
        );
    }
    out
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004539c0() {
    // SAFETY: the globals are the game's own memory; the conversions consume
    // the floating values the original leaves on the x87 stack.
    unsafe {
        let l_var1 = ftol();
        DAT_00703c6c = (l_var1 as i16) as i32;
        let l_var1 = ftol();
        DAT_00703c70 = (l_var1 as i16) as i32;
        let l_var1 = ftol();
        DAT_00703c84 = (l_var1 as i16) as i32;
    }
}
