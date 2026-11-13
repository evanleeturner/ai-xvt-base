// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d3570(param_1: *mut i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d04e0(param_1: *mut i8) -> i32 {
    // SAFETY: param_1 points at the caller's NUL-terminated string; the callee is the game's own.
    unsafe {
        if param_1.read() == 0 {
            return 0;
        }
        (FUN_004d3570(param_1) != -1) as i32
    }
}
