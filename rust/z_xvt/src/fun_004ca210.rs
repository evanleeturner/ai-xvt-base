// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ca220(param_1: i32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004ca210() {
    // SAFETY: the callee is the game's own function.
    unsafe {
        FUN_004ca220(0, 0);
    }
}
