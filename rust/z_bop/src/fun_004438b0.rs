// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0049eb60(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004438b0() {
    // SAFETY: the callee is the game's own function, called with the original's argument.
    unsafe {
        FUN_0049eb60(0);
    }
}
