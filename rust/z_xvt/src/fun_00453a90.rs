// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005254e0: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00453a90(param_1: u32) {
    // SAFETY: the global is the game's own memory; written by value.
    unsafe {
        DAT_005254e0 = param_1;
    }
}
