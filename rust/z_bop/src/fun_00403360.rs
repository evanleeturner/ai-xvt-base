// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a73c4: i32;
    static mut DAT_009a73c8: i32;
    static mut DAT_009a73cc: i32;
    static mut DAT_009a1ff8: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00403360(param_1: i32, param_2: i32) -> u8 {
    // SAFETY: the globals are the game's own; the table base in DAT_009a1fe8
    // points at the game's entries, read where the original reads.
    unsafe {
        let base = param_1.wrapping_mul(0x23).wrapping_add(DAT_009a1fe8);
        let mut i_var1 = DAT_009a73c4
            .wrapping_sub(((base.wrapping_add(4)) as usize as *const i32).read_unaligned());
        let mut i_var2 = DAT_009a73c8
            .wrapping_sub(((base.wrapping_add(8)) as usize as *const i32).read_unaligned());
        let mut i_var3 = DAT_009a73cc
            .wrapping_sub(((base.wrapping_add(0xc)) as usize as *const i32).read_unaligned());
        if i_var1 < 0 {
            i_var1 = i_var1.wrapping_neg();
        }
        if i_var2 < 0 {
            i_var2 = i_var2.wrapping_neg();
        }
        if i_var3 < 0 {
            i_var3 = i_var3.wrapping_neg();
        }
        if i_var2 < i_var1 {
            DAT_009a1ff8 = (i_var2 >> 1).wrapping_add(i_var1);
        } else {
            DAT_009a1ff8 = (i_var1 >> 1).wrapping_add(i_var2);
        }
        if i_var3 < DAT_009a1ff8 {
            i_var3 >>= 1;
        } else {
            DAT_009a1ff8 >>= 1;
        }
        DAT_009a1ff8 = DAT_009a1ff8.wrapping_add(i_var3);
        (DAT_009a1ff8 < param_2) as u8
    }
}
