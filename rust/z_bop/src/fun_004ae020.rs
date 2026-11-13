// SPDX-License-Identifier: MIT
use core::arch::asm;

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
pub unsafe extern "C" fn FUN_004ae020(param_1: i32) -> i64 {
    let mut cw: [u16; 2] = [0; 2];
    let mut out: i64 = 0;
    // SAFETY: the global array and the callee are the game's own. The asm
    // truncates the x87 st(0) value left by the C's preceding code (__ftol
    // takes no argument), using local scratch memory and restoring the
    // control word afterwards.
    unsafe {
        let p = (&raw mut DAT_00663658)
            .cast::<i32>()
            .offset(param_1 as isize);
        if p.read() == 0 {
            FUN_004add60(param_1);
        }
        asm!(
            "fnstcw word ptr [{cw}]",
            "movzx {tmp}, word ptr [{cw}]",
            "or {tmp}, 0xC00",
            "mov word ptr [{cw} + 2], {tmp:x}",
            "fldcw word ptr [{cw} + 2]",
            "fistp qword ptr [{out}]",
            "fldcw word ptr [{cw}]",
            cw = in(reg) cw.as_mut_ptr(),
            out = in(reg) &raw mut out,
            tmp = out(reg) _,
        );
    }
    out
}
