// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa73b4: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d6300(param_1: i32) -> u8 {
    if param_1 < 2 {
        return 0;
    }
    // SAFETY: DAT_00aa73b4 is the game's own table; the index is read where the original reads it.
    unsafe {
        (&raw mut DAT_00aa73b4)
            .cast::<u8>()
            .offset(param_1 as isize)
            .read()
    }
}
