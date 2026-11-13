// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005f0e30: i16;
    static mut DAT_005f0e32: i16;
    static mut DAT_005f0e34: i16;
}

/// Truncating conversion of the x87 top of stack, popping it, as `__ftol` does.
#[inline(always)]
unsafe fn ftol() -> i64 {
    let mut v: i64 = 0;
    // SAFETY: stores st(0) truncated toward zero into a local and pops it;
    // the value on the x87 stack comes from the caller, as in the original.
    unsafe {
        core::arch::asm!(
            "fisttp qword ptr [{p}]",
            p = in(reg) &raw mut v,
            options(nostack)
        );
    }
    v
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00453a50() {
    // SAFETY: the globals are the game's own memory; the conversions read the
    // x87 stack in the original's order.
    unsafe {
        let l_var1 = ftol();
        DAT_005f0e32 = l_var1 as i16;
        let l_var1 = ftol();
        DAT_005f0e30 = l_var1 as i16;
        let l_var1 = ftol();
        DAT_005f0e34 = l_var1 as i16;
    }
}
