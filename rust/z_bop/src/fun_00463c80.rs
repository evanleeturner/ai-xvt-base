// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b6d5aa: i32;
    static mut DAT_009e9bf8: i32;
    static mut DAT_00b6d86a: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00463c80(param_1: i32) -> i32 {
    let mut i_var1: i32 = 0;
    // SAFETY: the globals are the game's own memory; only their addresses
    // are taken and the table is read where the original reads it.
    unsafe {
        let mut pi_var2: *mut i32 = &raw mut DAT_00b6d5aa;
        let end = (&raw mut DAT_00b6d86a) as usize;
        loop {
            let off = param_1.wrapping_mul(0x5bd) as isize;
            let other = (&raw mut DAT_009e9bf8)
                .cast::<u8>()
                .byte_offset(off)
                .cast::<i32>()
                .read_unaligned();
            if pi_var2.read_unaligned() == other {
                return i_var1;
            }
            pi_var2 = pi_var2.byte_add(0x16 * 4);
            i_var1 = i_var1.wrapping_add(1);
            if (pi_var2 as usize) >= end {
                break;
            }
        }
    }
    0
}
