// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7436: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d62c0(param_1: i32) -> u8 {
    if param_1 >= 2 {
        return 0;
    }
    // SAFETY: the table is the game's own memory; the index is read where the original reads it, with no lower bound check, as in the C.
    unsafe {
        (&raw mut DAT_00aa7436)
            .cast::<u8>()
            .offset(param_1 as isize)
            .read()
    }
}
