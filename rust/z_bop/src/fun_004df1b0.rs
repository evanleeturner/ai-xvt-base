// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7fbf: i32;
    static mut DAT_00aa7fc7: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004df1d0(param_1: i32, param_2: i32, param_3: *mut i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004df1b0(param_1: *mut i8) {
    // SAFETY: the globals and the callee are the game's own; param_1 is
    // passed through untouched.
    unsafe {
        FUN_004df1d0(DAT_00aa7fbf, DAT_00aa7fc7.wrapping_sub(1), param_1);
    }
}
