// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005280e4: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004ac750() {
    // SAFETY: the global is the game's own memory, written where the original does.
    unsafe {
        DAT_005280e4 = 0;
    }
}
