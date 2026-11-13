// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004287d0(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00428db0(param_1: u32, param_2: i32) {
    let _ = param_1;
    // SAFETY: the callee is the game's own function; it receives the
    // caller's second argument unchanged.
    unsafe {
        FUN_004287d0(param_2);
    }
}
