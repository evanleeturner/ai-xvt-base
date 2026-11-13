// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00acebf1: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d11d0() -> u32 {
    // SAFETY: the global is the game's own memory, read by value as the original does.
    unsafe { DAT_00acebf1 }
}
