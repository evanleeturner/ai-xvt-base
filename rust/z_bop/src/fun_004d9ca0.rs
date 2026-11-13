// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052c004: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d9ca0() -> u32 {
    // SAFETY: the global is the game's own memory, written where the original does.
    unsafe {
        DAT_0052c004 = 0;
    }
    1
}
