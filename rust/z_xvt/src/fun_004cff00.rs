// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759b1c: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004cff00() -> u32 {
    // SAFETY: the global is the game's own memory, written where the original does.
    unsafe {
        DAT_00759b1c = 0;
    }
    1
}
