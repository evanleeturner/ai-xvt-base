// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a7b5c: i32;
    static mut DAT_009a8c1c: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00436310() {
    // SAFETY: the globals are the game's own memory, written where the original does.
    unsafe {
        DAT_009a7b5c = 0;
        DAT_009a8c1c = 0;
    }
}
