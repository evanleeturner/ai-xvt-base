// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758ee2: u32;
    static mut DAT_00758edf: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004db550() -> u8 {
    // SAFETY: the globals are the game's own memory, read by value.
    unsafe {
        if DAT_00758ee2 != 0 {
            return 0;
        }
        DAT_00758edf
    }
}
