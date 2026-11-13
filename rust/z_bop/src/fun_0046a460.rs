// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0046a470(param_1: i16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0046a460(param_1: i16) {
    // SAFETY: the callee is the game's own function; the argument is passed through unchanged.
    unsafe {
        FUN_0046a470(param_1);
    }
}
