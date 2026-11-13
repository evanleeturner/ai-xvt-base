// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525e14: i32;
    static mut DAT_00525e10: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_00465d20();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00452d00() {
    // SAFETY: the globals and the callee are the game's own memory and code,
    // accessed in the same order as the original.
    unsafe {
        if DAT_00525e14 != 0 {
            FUN_00465d20();
            DAT_00525e14 = 0;
        }
        DAT_00525e10 = 0;
    }
}
