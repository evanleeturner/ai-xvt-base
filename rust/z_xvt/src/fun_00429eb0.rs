// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00429ed0(param_1: u32) -> i16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00429eb0(param_1: u32) -> u8 {
    // SAFETY: the callee is the game's own function; it is called with the
    // caller's argument unchanged.
    unsafe { (FUN_00429ed0(param_1) != -1) as u8 }
}
