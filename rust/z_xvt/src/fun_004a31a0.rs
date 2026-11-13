// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00645ae0: u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004a31a0() -> u16 {
    // SAFETY: the global is the game's own memory, read by value.
    unsafe { DAT_00645ae0 }
}
