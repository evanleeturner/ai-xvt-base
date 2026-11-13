// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758ee0: u8;
    static mut DAT_00758ee1: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004db650() -> u32 {
    // SAFETY: the globals are the game's own memory, written where the original does.
    unsafe {
        DAT_00758ee0 = 0;
        DAT_00758ee1 = 0;
    }
    1
}
