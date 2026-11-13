// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00524778: [u16; 0];
    static mut DAT_006d9d70: u32;
    static mut DAT_006d4988: i32;
    static mut DAT_006d9d64: i32;
    static mut DAT_006d9d68: i32;
    static mut DAT_006d9d6c: i32;
    static mut DAT_006d499c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00429a20(param_2: u32) -> i32;
    fn FUN_00478320(param_1: u32, param_2: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00429920(param_1: u32, param_2: u32, param_3: i32) -> i32 {
    let _ = param_1;
    // SAFETY: globals and callees are the game's own memory; the pointer
    // computed from the table at param_2 * 0x23 + DAT_006d4988 is read where
    // the original reads it.
    unsafe {
        let ret = FUN_00429a20(param_2);
        if ret != 0 {
            let idx = DAT_006d9d70 as usize;
            let entry = (&raw mut DAT_00524778).cast::<u16>().add(idx).read();
            let mut u_var2 = FUN_00478320(0x500, entry as u32);
            u_var2 = (u_var2 & 0xFFFF).wrapping_add(0xA00);
            if param_3 != 0 {
                let u_var3 = FUN_00478320(u_var2, 0x5555);
                u_var2 = u_var2.wrapping_add(u_var3 & 0xFFFF);
            }
            let base =
                param_2.wrapping_mul(0x23).wrapping_add(DAT_006d4988 as u32) as usize as *const u8;
            let mut i_var4 =
                DAT_006d9d64.wrapping_sub(base.byte_add(4).cast::<i32>().read_unaligned());
            let mut i_var5 =
                DAT_006d9d68.wrapping_sub(base.byte_add(8).cast::<i32>().read_unaligned());
            let mut i_var6 =
                DAT_006d9d6c.wrapping_sub(base.byte_add(0xC).cast::<i32>().read_unaligned());
            if i_var4 < 0 {
                i_var4 = i_var4.wrapping_neg();
            }
            if i_var5 < 0 {
                i_var5 = i_var5.wrapping_neg();
            }
            if i_var6 < 0 {
                i_var6 = i_var6.wrapping_neg();
            }
            if i_var5 < i_var4 {
                DAT_006d499c = (i_var5 >> 1).wrapping_add(i_var4);
            } else {
                DAT_006d499c = (i_var4 >> 1).wrapping_add(i_var5);
            }
            if i_var6 < DAT_006d499c {
                i_var6 >>= 1;
            } else {
                DAT_006d499c >>= 1;
            }
            DAT_006d499c = DAT_006d499c.wrapping_add(i_var6);
            if DAT_006d499c < (u_var2.wrapping_mul(0x100) as i32) {
                return 1;
            }
        }
        0
    }
}
