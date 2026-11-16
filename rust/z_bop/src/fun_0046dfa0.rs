// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009994fc: u32;
    static mut DAT_00999550: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0046dfa0(param_1: *mut u32) -> *mut u32 {
    // SAFETY: the globals are the game's own memory; param_1 points at a
    // caller-provided u32 slot, written where the original does.
    unsafe {
        param_1.write_unaligned(DAT_009994fc);
        &raw mut DAT_00999550
    }
}
