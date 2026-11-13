// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa73a2: u32;
    static mut DAT_00aa73a1: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004dc240() -> u8 {
    // SAFETY: the globals are the game's own memory, read where the original reads them.
    unsafe {
        if DAT_00aa73a2 != 0 {
            return 0;
        }
        DAT_00aa73a1
    }
}
