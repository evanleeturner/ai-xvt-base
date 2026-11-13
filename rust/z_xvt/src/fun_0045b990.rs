// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071bb1b: i32;
    static mut DAT_0071bb1f: i32;
    static mut DAT_0071bb23: i32;
    static mut DAT_0071ecf4: i32;
    static mut DAT_0072f448: i32;
    static mut DAT_0072f44c: i32;
    static mut DAT_0072f450: i32;
    static mut DAT_006db7ac: i32;
    static mut DAT_006db7b0: i32;
    static mut DAT_006db7b4: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040ca60(a: i32, b: i32, c: i32) -> i32;
    fn FUN_0040ca00(a: i32, b: i32, c: i32) -> i32;
    fn FUN_0040ca30(a: i32, b: i32, c: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0045b990(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: u32,
) -> u32 {
    // SAFETY: the globals and callees are the game's own; the record table
    // is read at the byte offsets the original uses.
    unsafe {
        let off = DAT_0071ecf4.wrapping_mul(0x5bd) as isize;
        let v1 = (&raw mut DAT_0071bb1b)
            .cast::<u8>()
            .offset(off)
            .cast::<i32>()
            .read_unaligned();
        let v2 = (&raw mut DAT_0071bb1f)
            .cast::<u8>()
            .offset(off)
            .cast::<i32>()
            .read_unaligned();
        let v3 = (&raw mut DAT_0071bb23)
            .cast::<u8>()
            .offset(off)
            .cast::<i32>()
            .read_unaligned();
        DAT_0072f44c = param_1.wrapping_sub(v1);
        DAT_0072f450 = param_2.wrapping_sub(v2);
        DAT_0072f448 = param_3.wrapping_sub(v3);
        DAT_006db7b4 = FUN_0040ca60(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
        let u_var3: u32 = param_4 & 0xffff;
        let i_var1: i32 = (u_var3 as i32).wrapping_add(DAT_006db7b4);
        if i_var1 < 0 {
            return (DAT_006db7b4 as u32) & 0xffff0000;
        }
        if (u_var3 as i32) < (i_var1 >> 8) {
            return ((i_var1 >> 8) as u32) & 0xffff0000;
        }
        DAT_006db7ac = FUN_0040ca00(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
        let mut i_var2 = DAT_006db7ac;
        if DAT_006db7ac < 0 {
            i_var2 = DAT_006db7ac.wrapping_neg();
        }
        if i_var1 < i_var2.wrapping_sub(u_var3 as i32) {
            return (i_var2.wrapping_sub(u_var3 as i32) as u32) & 0xffff0000;
        }
        i_var2 = FUN_0040ca30(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
        DAT_006db7b0 = i_var2;
        if i_var2 < 0 {
            i_var2 = i_var2.wrapping_neg();
        }
        (i_var2.wrapping_sub(u_var3 as i32) <= i_var1) as u32
    }
}
