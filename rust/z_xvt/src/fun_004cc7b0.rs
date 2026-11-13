// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00780732: u32;
    static mut DAT_0078075a: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cc7b0(param_1: *mut u32) -> *mut u32 {
    // SAFETY: the globals are the game's own memory; param_1 points at the
    // caller's buffer, written where the original does. The read at an odd
    // address is done by value as the C does.
    unsafe {
        param_1.write_unaligned(DAT_00780732);
        &raw mut DAT_0078075a
    }
}
