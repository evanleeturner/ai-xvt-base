// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009cd270: u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a9620(param_1: u16) {
    // SAFETY: the global is the game's own memory; written by value.
    unsafe {
        DAT_009cd270 = param_1;
    }
}
