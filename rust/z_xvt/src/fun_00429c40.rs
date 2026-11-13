// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d9d70: u32;
    static mut DAT_00524778: u16;
    static mut DAT_006d4988: i32;
    static mut DAT_006d9d64: i32;
    static mut DAT_006d9d68: i32;
    static mut DAT_006d9d6c: i32;
    static mut DAT_006d499c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00478320(param_1: u32, param_2: u16) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00429c40(param_1: u32) -> u8 {
    // SAFETY: the globals and the callee are the game's own memory; the table
    // and the record pointer are read where the original reads them.
    unsafe {
        let idx = DAT_006d9d70 as usize;
        let w = (&raw const DAT_00524778).add(idx).read_unaligned();
        let u_var1 = FUN_00478320(0x500, w);
        let i_var4 = ((param_1 & 0xffff) as i32)
            .wrapping_mul(0x23)
            .wrapping_add(DAT_006d4988);
        let base = i_var4 as u32 as usize as *const u8;
        let mut i_var3 = DAT_006d9d64.wrapping_sub(base.byte_add(4).cast::<i32>().read_unaligned());
        let mut i_var2 = DAT_006d9d68.wrapping_sub(base.byte_add(8).cast::<i32>().read_unaligned());
        let mut i_var4 =
            DAT_006d9d6c.wrapping_sub(base.byte_add(0xc).cast::<i32>().read_unaligned());
        if i_var3 < 0 {
            i_var3 = i_var3.wrapping_neg();
        }
        if i_var2 < 0 {
            i_var2 = i_var2.wrapping_neg();
        }
        if i_var4 < 0 {
            i_var4 = i_var4.wrapping_neg();
        }
        if i_var2 < i_var3 {
            DAT_006d499c = (i_var2 >> 1).wrapping_add(i_var3);
        } else {
            DAT_006d499c = (i_var3 >> 1).wrapping_add(i_var2);
        }
        if i_var4 < DAT_006d499c {
            i_var4 >>= 1;
        } else {
            DAT_006d499c >>= 1;
        }
        DAT_006d499c = DAT_006d499c.wrapping_add(i_var4);
        let limit = (((u_var1 & 0xffff).wrapping_add(0xa00)).wrapping_mul(0x100)) as i32;
        (DAT_006d499c < limit) as u8
    }
}
