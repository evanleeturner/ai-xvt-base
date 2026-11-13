// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758ede: u8;
    static mut DAT_00758ee2: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004db530() -> u8 {
    // SAFETY: the globals are the game's own memory, read by value.
    unsafe {
        if DAT_00758ee2 != 0 {
            return 0;
        }
        DAT_00758ede
    }
}
