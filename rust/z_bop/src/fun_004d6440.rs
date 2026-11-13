// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa73a6: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d6440(param_1: i32) -> u32 {
    if 2 < param_1 {
        return 0;
    }
    // SAFETY: the global is the game's own memory; the index is used as the
    // original does, as a 4-byte element offset from the (unaligned) address.
    unsafe {
        (&raw mut DAT_00aa73a6)
            .cast::<u8>()
            .byte_offset((param_1 as isize).wrapping_mul(4))
            .cast::<u32>()
            .read_unaligned()
    }
}
