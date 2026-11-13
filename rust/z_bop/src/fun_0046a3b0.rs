// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0046a3f0(param_1: u16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0046a3b0(param_1: u16) {
    // SAFETY: the callee is the game's own function; it is called with the
    // same argument the original passes.
    unsafe {
        FUN_0046a3f0(param_1);
    }
}
