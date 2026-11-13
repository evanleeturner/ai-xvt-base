// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b24: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d4f60() -> i32 {
    // SAFETY: the global is the game's own memory; written by value.
    unsafe {
        DAT_00aa7b24 = 1;
    }
    1
}
