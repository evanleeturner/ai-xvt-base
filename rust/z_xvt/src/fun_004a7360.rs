// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052ae28: i32;
    static mut DAT_0052ae4c: i32;
    static mut DAT_0052ae18: i32;
    static mut DAT_0052ae3c: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d5500() -> u8;
    fn FUN_00461da0() -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004a7360() -> u8 {
    // SAFETY: the globals and callees are the game's own memory and code.
    unsafe {
        if DAT_0052ae28 != 0 {
            return FUN_004d5500();
        }
        if DAT_0052ae4c != 0 && DAT_0052ae18 != 0 {
            return FUN_00461da0();
        }
        (DAT_0052ae3c == 0x22b) as u8
    }
}
