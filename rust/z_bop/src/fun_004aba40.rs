// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00527f78: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004aba40() -> u32 {
    // SAFETY: the global is the game's own memory, read by value.
    unsafe { DAT_00527f78 }
}
