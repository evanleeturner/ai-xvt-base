// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00663658: [i32; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004add60(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ae080(param_1: i32) -> i64 {
    let mut old: u16 = 0;
    let mut new: u16 = 0;
    let mut out: i64 = 0;
    // SAFETY: the global table and the callee are the game's own; the asm
    // converts the x87 st(0) left by the original code, truncating toward
    // zero, using only the local scratch variables passed by pointer.
    unsafe {
        if (&raw mut DAT_00663658)
            .cast::<i32>()
            .offset(param_1 as isize)
            .read()
            == 0
        {
            FUN_004add60(param_1);
        }
        core::arch::asm!(
            "fnstcw word ptr [{o}]",
            "movzx {t:e}, word ptr [{o}]",
            "or {t:e}, 0xc00",
            "mov word ptr [{n}], {t:x}",
            "fldcw word ptr [{n}]",
            "fistp qword ptr [{r}]",
            "fldcw word ptr [{o}]",
            o = in(reg) &raw mut old,
            n = in(reg) &raw mut new,
            r = in(reg) &raw mut out,
            t = out(reg) _,
        );
    }
    out
}
