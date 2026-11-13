// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0051c4a8: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004244d0() {
    // SAFETY: the global is the game's own memory; it is written by value.
    unsafe {
        DAT_0051c4a8 = 1;
    }
}
