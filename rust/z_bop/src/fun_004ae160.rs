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
pub unsafe extern "C" fn FUN_004ae160(param_1: i32) -> i64 {
    let mut r: i64 = 0;
    // SAFETY: the global table and callee are the game's own; the index is
    // used as the original does. The asm pops the x87 stack top left by the
    // original's computation and stores it truncated toward zero, as __ftol.
    unsafe {
        let p = (&raw mut DAT_00663658)
            .cast::<i32>()
            .offset(param_1 as isize);
        if p.read() == 0 {
            FUN_004add60(param_1);
        }
        asm!(
            "fisttp qword ptr [{0}]",
            in(reg) &raw mut r,
            options(nostack)
        );
    }
    r
}
