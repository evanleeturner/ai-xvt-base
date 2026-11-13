// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0064f78c: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004da070() {
    // SAFETY: the global is the game's own memory; written by value.
    unsafe {
        DAT_0064f78c = 1;
    }
}
