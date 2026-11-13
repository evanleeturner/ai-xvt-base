// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00528128: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004af090(param_1: *mut u32, param_2: i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004af150(param_1: *mut u32, param_2: i32, param_3: i32) {
    // SAFETY: the global and the callee are the game's own; the arguments
    // are passed through unchanged.
    unsafe {
        DAT_00528128 = 0;
        FUN_004af090(param_1, param_2, param_3);
    }
}
