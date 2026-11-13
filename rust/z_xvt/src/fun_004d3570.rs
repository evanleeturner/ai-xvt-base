// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758d41: i32;
    static mut DAT_00758d45: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d35a0(param_1: i32, param_2: i32, param_3: *mut i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d3570(param_1: *mut i8) -> i32 {
    if param_1.is_null() {
        return -1;
    }
    // SAFETY: the globals and the callee are the game's own; param_1 is
    // passed through unchanged.
    unsafe { FUN_004d35a0(DAT_00758d41, DAT_00758d45.wrapping_sub(1), param_1) }
}
