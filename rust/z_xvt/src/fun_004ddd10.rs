// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758e8e: u32;
    static mut DAT_00758e92: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ddd10(param_1: *mut u32, param_2: *mut u32) -> u32 {
    // SAFETY: the globals are the game's own memory; the caller's pointers
    // are written where the original writes them. The globals sit at
    // unaligned addresses, so they are read through raw pointers unaligned.
    unsafe {
        param_1.write_unaligned((&raw const DAT_00758e8e).read_unaligned());
        param_2.write_unaligned((&raw const DAT_00758e92).read_unaligned());
    }
    1
}
