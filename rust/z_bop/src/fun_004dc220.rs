// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa73a0: i8;
    static mut DAT_00aa73a2: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004dc220() -> i8 {
    // SAFETY: the globals are the game's own memory, read by value as the original does.
    unsafe {
        if DAT_00aa73a2 != 0 {
            return 0;
        }
        DAT_00aa73a0
    }
}
