// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759647: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d5090(param_1: u32) {
    // SAFETY: DAT_00759647 is the game's own global; it is written once, as in the original.
    unsafe {
        DAT_00759647 = param_1;
    }
}
