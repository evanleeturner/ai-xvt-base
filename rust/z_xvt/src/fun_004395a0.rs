// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d0070(param_1: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004395a0(param_1: u32) {
    // SAFETY: the callee is the game's own function; it is called with the
    // same argument the original passes.
    unsafe {
        FUN_004d0070(param_1);
    }
}
