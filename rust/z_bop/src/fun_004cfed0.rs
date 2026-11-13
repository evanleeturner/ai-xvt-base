// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00acec01: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004cfed0() -> u32 {
    // SAFETY: the global is the game's own memory; it is read by value
    // unaligned-safe as a plain load at the original address.
    unsafe { (&raw const DAT_00acec01).read_unaligned() }
}
