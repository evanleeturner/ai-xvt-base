// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00730894: u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0048ee10(param_1: u16) {
    // SAFETY: the global is the game's own memory; written by value.
    unsafe {
        DAT_00730894 = param_1;
    }
}
