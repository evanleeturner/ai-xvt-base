// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0051c00c: i32;
    static mut DAT_0051c014: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004206a0() {
    // SAFETY: the globals are the game's own memory, written where the original does.
    unsafe {
        DAT_0051c00c = 0;
        DAT_0051c014 = 0;
    }
}
