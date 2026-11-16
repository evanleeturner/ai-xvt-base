// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: i32;
    static mut DAT_0072f44c: i32;
    static mut DAT_0072f450: i32;
    static mut DAT_0072f448: i32;
    static mut DAT_006db7b4: i32;
    static mut DAT_0071bb1b: i32;
    static mut DAT_0071bb1f: i32;
    static mut DAT_0071bb23: i32;
    static mut DAT_006db7ac: i32;
    static mut DAT_006db7b0: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040ca60(a: i32, b: i32, c: i32) -> i32;
    fn FUN_0040ca00(a: i32, b: i32, c: i32) -> i32;
    fn FUN_0040ca30(a: i32, b: i32, c: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004340e0(param_1: i32, param_2: u32, param_3: i32) -> u8 {
    let mut param_2 = param_2;
    // SAFETY: all globals and callees are the game's own memory; the pointers
    // are read at the byte offsets the original uses.
    unsafe {
        let i_var2 = param_1.wrapping_mul(0x23).wrapping_add(DAT_006d4988);
        let off = param_3.wrapping_mul(0x5bd) as isize;
        let local_1b = (&raw mut DAT_0071bb1b).byte_offset(off).read_unaligned();
        let local_1f = (&raw mut DAT_0071bb1f).byte_offset(off).read_unaligned();
        let local_23 = (&raw mut DAT_0071bb23).byte_offset(off).read_unaligned();
        DAT_0072f44c = ((i_var2.wrapping_add(4)) as usize as *const i32)
            .read_unaligned()
            .wrapping_sub(local_1b);
        DAT_0072f450 = ((i_var2.wrapping_add(8)) as usize as *const i32)
            .read_unaligned()
            .wrapping_sub(local_1f);
        DAT_0072f448 = ((i_var2.wrapping_add(0xc)) as usize as *const i32)
            .read_unaligned()
            .wrapping_sub(local_23);
        DAT_006db7b4 = FUN_0040ca60(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
        let i_var2 = DAT_006db7b4.wrapping_add(param_2 as i32);
        if i_var2 < 0 {
            return 0;
        }
        if param_2 < (i_var2 >> 4) as u32 {
            param_2 = (i_var2 >> 4) as u32;
        }
        DAT_006db7ac = FUN_0040ca00(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
        let mut i_var1 = DAT_006db7ac;
        if DAT_006db7ac < 0 {
            i_var1 = DAT_006db7ac.wrapping_neg();
        }
        if i_var2 < i_var1.wrapping_sub(param_2 as i32) {
            return 0;
        }
        i_var1 = FUN_0040ca30(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
        DAT_006db7b0 = i_var1;
        if i_var1 < 0 {
            i_var1 = i_var1.wrapping_neg();
        }
        (i_var1.wrapping_sub(param_2 as i32) <= i_var2) as u8
    }
}
