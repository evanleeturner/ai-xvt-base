// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7fd3: u32;
    static mut DAT_00aa7fd7: u32;
    static mut DAT_00aa7fdf: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d32a0() -> u32 {
    // SAFETY: the globals are the game's own memory, read by value as the original does.
    unsafe {
        if DAT_00aa7fd3 == 0 {
            return 0;
        }
        if DAT_00aa7fd7 == 0 {
            return 0;
        }
        DAT_00aa7fdf
    }
}
