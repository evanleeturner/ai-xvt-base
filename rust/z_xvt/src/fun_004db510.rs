// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758ee2: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004db510(param_1: u32) -> u32 {
    // SAFETY: the global is the game's own memory; it is written by value.
    unsafe {
        DAT_00758ee2 = param_1;
    }
    1
}
