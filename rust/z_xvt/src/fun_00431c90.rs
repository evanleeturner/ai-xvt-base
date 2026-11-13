// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_007330ac: u32;
    static mut DAT_00733100: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00431c90(param_1: *mut u32) -> *mut u32 {
    // SAFETY: the globals are the game's own memory; param_1 points at a
    // caller-provided u32 that is written where the original does.
    unsafe {
        param_1.write_unaligned(DAT_007330ac);
        &raw mut DAT_00733100
    }
}
