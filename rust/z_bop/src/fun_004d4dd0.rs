// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7aa0: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d4dd0() {
    // SAFETY: the global is the game's own memory; written by value as the original does.
    unsafe {
        DAT_00aa7aa0 = 0;
    }
}
