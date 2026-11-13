// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758ee0: u8;
    static mut DAT_00758ee2: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004db5f0(param_1: i32) -> u8 {
    // SAFETY: the globals are the game's own memory, read by value.
    unsafe {
        if param_1 != DAT_00758ee2 && DAT_00758ee2 != 0 {
            return 0;
        }
        DAT_00758ee0
    }
}
