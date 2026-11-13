// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004b9990(param_1: *mut u8, param_2: *mut i32, param_3: i32, param_4: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b9970(param_1: *mut u8, param_2: *mut i32, param_3: i32) {
    // SAFETY: the callee is the game's own function; the arguments are passed through unchanged.
    unsafe {
        FUN_004b9990(param_1, param_2, param_3, 0);
    }
}
