// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759662: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d6030() -> u32 {
    // SAFETY: the global is the game's own memory; written by value.
    unsafe {
        DAT_00759662 = 1;
    }
    1
}
