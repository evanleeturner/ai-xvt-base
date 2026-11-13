// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa73a2: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004dc2e0(param_1: i32) -> u8 {
    // SAFETY: the global is the game's own memory; it is read by value
    // (unaligned address, so read through a raw pointer).
    unsafe { ((&raw const DAT_00aa73a2).read_unaligned() == param_1) as u8 }
}
