// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: i32;
    static mut DAT_006d9d64: i32;
    static mut DAT_006d9d68: i32;
    static mut DAT_006d9d6c: i32;
    static mut DAT_006d499c: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00429fc0(param_1: i32, param_2: i32) -> i32 {
    // SAFETY: the globals are the game's own memory; DAT_006d4988 points at a
    // table whose entries are 0x23 bytes wide, read at the original's offsets.
    unsafe {
        let base = param_1.wrapping_mul(0x23).wrapping_add(DAT_006d4988);
        let p = base as usize as *const u8;
        let mut i_var1 = DAT_006d9d64.wrapping_sub(p.byte_add(4).cast::<i32>().read_unaligned());
        let mut i_var2 = DAT_006d9d68.wrapping_sub(p.byte_add(8).cast::<i32>().read_unaligned());
        let mut i_var3 = DAT_006d9d6c.wrapping_sub(p.byte_add(0xc).cast::<i32>().read_unaligned());
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
            DAT_006d499c = (i_var2 >> 1).wrapping_add(i_var1);
        } else {
            DAT_006d499c = (i_var1 >> 1).wrapping_add(i_var2);
        }
        if i_var3 < DAT_006d499c {
            i_var3 >>= 1;
        } else {
            DAT_006d499c >>= 1;
        }
        DAT_006d499c = DAT_006d499c.wrapping_add(i_var3);
        (DAT_006d499c < param_2) as i32
    }
}
