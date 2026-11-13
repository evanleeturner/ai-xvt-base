// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00acebfd: u32;
    static mut DAT_00acec25: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cfe80(param_1: *mut u32) -> *mut u32 {
    // SAFETY: the globals are the game's own memory; param_1 points at the
    // caller's buffer, written where the original does.
    unsafe {
        param_1.write_unaligned(DAT_00acebfd);
        &raw mut DAT_00acec25
    }
}
