// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009fe7e4: u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a9630(param_1: u16) {
    // SAFETY: the global is the game's own memory; written by value as the original does.
    unsafe {
        DAT_009fe7e4 = param_1;
    }
}
