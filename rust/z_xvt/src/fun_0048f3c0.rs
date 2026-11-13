// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048f3e0(param_1: u32, param_2: u32, param_3: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0048f3c0(param_1: u32, param_2: u32) {
    // SAFETY: the callee is the game's own function; arguments are passed through unchanged.
    unsafe {
        FUN_0048f3e0(param_1, param_2, 0);
    }
}
