// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00418500(param_1: i16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004184f0(param_1: i16) {
    // SAFETY: the callee is the game's own function; it is called with the
    // same argument the original passes.
    unsafe {
        FUN_00418500(param_1);
    }
}
