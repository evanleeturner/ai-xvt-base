// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009d77c4: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00426c50(param_1: u32) {
    // SAFETY: the global is the game's own memory; written by value as the original does.
    unsafe {
        DAT_009d77c4 = param_1;
    }
}
