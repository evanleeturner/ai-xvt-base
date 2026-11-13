// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0051815c: f32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0046a6d0(param_1: i16) -> i64 {
    // SAFETY: the global is the game's own memory; the asm uses x87 fcos on a
    // local f64 through a valid pointer and clobbers only x87 registers.
    unsafe {
        let mut x: f64 = (param_1 as i32 as f64) * (DAT_0051815c as f64);
        core::arch::asm!(
            "fld qword ptr [{p}]",
            "fcos",
            "fstp qword ptr [{p}]",
            p = in(reg) &raw mut x,
            out("st(0)") _,
            out("st(1)") _,
            out("st(2)") _,
            out("st(3)") _,
            out("st(4)") _,
            out("st(5)") _,
            out("st(6)") _,
            out("st(7)") _,
            options(nostack)
        );
        x as i64
    }
}
