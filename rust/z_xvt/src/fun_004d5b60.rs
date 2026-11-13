// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759663: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d5b60() -> u32 {
    // SAFETY: the global is the game's own memory; it is read by value
    // as the original does, without alignment assumptions beyond the C's.
    unsafe { DAT_00759663 }
}
