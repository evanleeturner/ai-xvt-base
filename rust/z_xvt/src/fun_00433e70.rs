// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006da4f8: i32;
    static mut DAT_006db59c: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00433e70() {
    // SAFETY: the globals are the game's own memory, written by value.
    unsafe {
        DAT_006da4f8 = 0;
        DAT_006db59c = 0;
    }
}
