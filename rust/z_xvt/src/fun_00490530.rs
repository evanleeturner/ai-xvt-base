// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00490550(param_1: *mut i8, param_2: *mut i8, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00490530(param_1: *mut i8, param_2: *mut i8) {
    // SAFETY: the callee is the game's own function; the pointers are passed
    // through unchanged.
    unsafe {
        FUN_00490550(param_1, param_2, 0);
    }
}
