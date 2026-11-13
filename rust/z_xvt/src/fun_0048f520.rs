// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052a564: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0048f520() {
    // SAFETY: the global is the game's own memory; written by value.
    unsafe {
        DAT_0052a564 = 0;
    }
}
