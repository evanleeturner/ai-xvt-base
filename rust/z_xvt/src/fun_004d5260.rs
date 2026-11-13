// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_007595d8: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d5260() {
    // SAFETY: the global is the game's own memory; written by value as the original does.
    unsafe {
        DAT_007595d8 = 0;
    }
}
