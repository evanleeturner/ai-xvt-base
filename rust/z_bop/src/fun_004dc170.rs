// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aac111: u32;
    static mut DAT_00aac115: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004dc170() -> u32 {
    // SAFETY: the globals are the game's own memory; they are written
    // by value where the original does, unaligned addresses included.
    unsafe {
        (&raw mut DAT_00aac111).write_unaligned(0);
        (&raw mut DAT_00aac115).write_unaligned(0);
    }
    1
}
