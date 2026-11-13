// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525e08: i16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00453c10() {
    let mut old: u16 = 0;
    let mut new: u16 = 0;
    let mut r: i64 = 0;
    // SAFETY: the value to convert is already in st(0) on entry, as the
    // original's `__ftol()` expects; the asm only touches local memory and
    // restores the x87 control word after a truncating store.
    unsafe {
        core::arch::asm!(
            "fnstcw word ptr [{o}]",
            "movzx {t:e}, word ptr [{o}]",
            "or {t:e}, 0xC00",
            "mov word ptr [{n}], {t:x}",
            "fldcw word ptr [{n}]",
            "fistp qword ptr [{r}]",
            "fldcw word ptr [{o}]",
            o = in(reg) &raw mut old,
            n = in(reg) &raw mut new,
            r = in(reg) &raw mut r,
            t = out(reg) _,
            options(nostack),
        );
        DAT_00525e08 = r as i16;
    }
}
