// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b0e: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d4bf0() {
    // SAFETY: the global is the game's own memory; written as the original does.
    unsafe {
        DAT_00aa7b0e = 0;
    }
}
