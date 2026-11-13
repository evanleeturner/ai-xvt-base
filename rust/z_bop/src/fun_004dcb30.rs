// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7a98: i32;
    static mut DAT_00aa7a9c: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004dcb30() -> i32 {
    // SAFETY: the globals are the game's own memory, written where the original does.
    unsafe {
        DAT_00aa7a9c = 0;
        DAT_00aa7a98 = 0;
    }
    1
}
