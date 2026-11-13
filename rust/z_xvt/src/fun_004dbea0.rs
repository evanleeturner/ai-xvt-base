// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_007595d4: u32;
    static mut DAT_007595d0: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004dbea0() -> u32 {
    // SAFETY: the globals are the game's own memory, written by value.
    unsafe {
        DAT_007595d4 = 0;
        DAT_007595d0 = 0;
    }
    1
}
