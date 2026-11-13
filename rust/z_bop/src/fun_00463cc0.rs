// SPDX-License-Identifier: MIT
#![no_std]

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b6d5de: [i32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00463c80(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00463cc0(param_1: i32) {
    // SAFETY: the callee and the global array are the game's own memory;
    // the write lands at the same element the original indexes.
    unsafe {
        let i_var1 = FUN_00463c80(param_1);
        (&raw mut DAT_00b6d5de)
            .cast::<i32>()
            .wrapping_offset(i_var1.wrapping_mul(0x16) as isize)
            .write_unaligned(1);
    }
}
