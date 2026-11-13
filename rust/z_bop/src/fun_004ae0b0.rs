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
pub unsafe extern "C" fn FUN_004ae0b0(param_1: i32) -> i64 {
    let mut r: i64 = 0;
    let mut old: u16 = 0;
    let mut new: u16 = 0;
    // SAFETY: the global and callee are the game's own; the table is indexed
    // as the original does. The asm converts the x87 st(0) left by the
    // caller/callee with truncation (as __ftol does), restoring the control word.
    unsafe {
        let p = (&raw mut DAT_00663658)
            .cast::<i32>()
            .offset(param_1 as isize);
        if p.read() == 0 {
            FUN_004add60(param_1);
        }
        core::arch::asm!(
            "fnstcw word ptr [{old}]",
            "mov {t:x}, word ptr [{old}]",
            "or {t:x}, 0x0C00",
            "mov word ptr [{new}], {t:x}",
            "fldcw word ptr [{new}]",
            "fistp qword ptr [{r}]",
            "fldcw word ptr [{old}]",
            old = in(reg) &raw mut old,
            new = in(reg) &raw mut new,
            r = in(reg) &raw mut r,
            t = out(reg) _,
            out("st(0)") _,
            options(nostack)
        );
    }
    r
}
