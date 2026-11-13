// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759b08: i32;
    static mut DAT_00759b0c: i32;
    static mut DAT_00759b14: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004cfe60() -> u32 {
    // SAFETY: the globals are the game's own memory, read by value.
    unsafe {
        if DAT_00759b08 == 0 {
            return 0;
        }
        if DAT_00759b0c == 0 {
            return 0;
        }
        DAT_00759b14
    }
}
