// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005569f8: u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00451930() -> u16 {
    // SAFETY: the global is the game's own memory, read by value.
    unsafe { DAT_005569f8 }
}
