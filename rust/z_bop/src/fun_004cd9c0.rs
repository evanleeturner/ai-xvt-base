// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cd9f0(param_1: i32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004cd9c0() {
    // SAFETY: the callee is the game's own function; it takes two plain integers.
    unsafe {
        FUN_004cd9f0(1, 1);
    }
}
