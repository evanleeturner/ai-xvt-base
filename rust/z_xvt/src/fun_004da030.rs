// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0064f680: i32;
    static mut DAT_0064f78c: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004da030() {
    // SAFETY: the globals are the game's own memory, written where the original does.
    unsafe {
        DAT_0064f680 = 1;
        DAT_0064f78c = 0;
    }
}
