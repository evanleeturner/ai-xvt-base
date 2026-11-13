// SPDX-License-Identifier: MIT
#![no_std]

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa73a2: u32;
    static mut DAT_00aa73a1: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004dc2c0(param_1: i32) -> u8 {
    // SAFETY: the globals are the game's own memory, read by value as the original does.
    unsafe {
        if DAT_00aa73a2 != (param_1 as u32) && DAT_00aa73a2 != 0 {
            return 0;
        }
        DAT_00aa73a1
    }
}
