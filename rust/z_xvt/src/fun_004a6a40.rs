// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00657c0c: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004a6a40() -> u32 {
    // SAFETY: DAT_00657c0c is the game's own global, read by value.
    unsafe { DAT_00657c0c }
}
