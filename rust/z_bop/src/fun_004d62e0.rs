// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa73b2: [u8; 2];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d62e0(param_1: i32) -> u8 {
    if 1 < param_1 {
        return 0;
    }
    // SAFETY: the table is the game's own memory; the index is read where
    // the original reads it, including negative values.
    unsafe {
        (&raw mut DAT_00aa73b2)
            .cast::<u8>()
            .offset(param_1 as isize)
            .read()
    }
}
