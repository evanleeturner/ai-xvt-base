// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa739e: u8;
    static mut DAT_00aa73a2: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004dc1e0() -> u8 {
    // SAFETY: the globals are the game's own memory, read by value.
    unsafe {
        if DAT_00aa73a2 != 0 {
            return 0;
        }
        DAT_00aa739e
    }
}
