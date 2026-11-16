// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1fd0: i32;
    static mut DAT_009a1fd4: i32;
    static mut DAT_009a1fd8: i32;
    static mut DAT_009d80c0: i32;
    static mut DAT_00a07bdc: i32;
    static mut DAT_009d8c18: i32;
    static mut DAT_00a07c54: i32;
    static mut DAT_009d1150: i32;
    static mut DAT_00a07c58: i32;
    static mut DAT_00a07cdc: i32;
    static mut DAT_00a07cd8: i32;
    static mut DAT_00a07cd4: i32;
    static mut DAT_009ec604: i32;
    static mut DAT_00a081e8: i32;
    static mut DAT_00a08298: i32;
    static mut DAT_00521f1c: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00458e20(a: u32, b: i32) -> u32;
    fn FUN_0041f3d0(a: u32, b: u32, c: u32) -> u32;
    fn FUN_0041c570(a: i32) -> u32;
    fn FUN_004a5490(a: u16, b: u16) -> u32;
}

fn abs_diff_wrap(a: i32, b: i32) -> i32 {
    let d = a.wrapping_sub(b);
    if d < 0 { d.wrapping_neg() } else { d }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00446700(param_1: u16, param_2: u16) -> u32 {
    // SAFETY: the globals and the pointed-to memory are the game's own;
    // addresses are read exactly where the original reads them.
    unsafe {
        let base = DAT_009a1fe8 as u32;
        let mut tmp_int: u32 = ((u32::from(param_1))
            .wrapping_mul(0x23)
            .wrapping_add(0x1f)
            .wrapping_add(base) as *const u32)
            .read_unaligned();
        let mut tmp_short: u16 = (tmp_int.wrapping_add(0x85) as *const u16).read_unaligned();
        let mut u_var4: u32 = u32::from(tmp_short);
        let mut u_var5: u32 = u32::from(param_2);
        if u_var5 <= u_var4 {
            return (DAT_009a1fe8 as u32) & 0xffff0000;
        }
        let mut i_var7: u32 = (DAT_009a1fe8 as u32).wrapping_add(u_var5.wrapping_mul(0x23));
        let c_var1: i8 = (i_var7.wrapping_add(2) as *const i8).read();
        if c_var1 == 0x0e {
            return 0;
        }
        if c_var1 == 0x0b {
            return 0;
        }
        if c_var1 != 0x0a && (u_var4 as i32) < DAT_009a1fe0 {
            let i_var3: u32 = u_var4.wrapping_mul(0x23).wrapping_add(DAT_009a1fe8 as u32);
            tmp_int = (i_var3.wrapping_add(0x1b) as *const u32).read_unaligned();
            if tmp_int as i32 == -1 {
                u_var4 = (i_var3.wrapping_add(0x1f) as *const u32).read_unaligned();
                tmp_int = (u_var4.wrapping_add(0xa9) as *const u32).read_unaligned();
                tmp_short = (tmp_int.wrapping_add(0x42) as *const u16).read_unaligned();
                if tmp_short != param_2 {
                    return u_var4 & 0xffff0000;
                }
            }
        }
        let b_var2: u8 = (i_var7.wrapping_add(3) as *const u8).read();
        FUN_00458e20(u_var5, 0);
        DAT_009d80c0 = DAT_00a07cdc;
        DAT_00a07bdc = DAT_00a07cdc;
        DAT_009d8c18 = DAT_00a07cd8;
        DAT_00a07c54 = DAT_00a07cd8;
        DAT_009d1150 = DAT_00a07cd4;
        DAT_00a07c58 = DAT_00a07cd4;
        u_var4 = (DAT_009a1fd4 as u32).wrapping_sub(DAT_00a07cdc as u32);
        let arr = (&raw mut DAT_00521f1c).cast::<u8>();
        let i_var7_v: i32 = arr
            .byte_add((b_var2 as usize).wrapping_mul(0x1a))
            .cast::<i32>()
            .read_unaligned();
        i_var7 = i_var7_v as u32;
        if (u_var4 as i32) < 0 {
            u_var4 = 0u32.wrapping_sub(u_var4);
        }
        if 0x20000 < (u_var4 as i32) {
            return u_var4 & 0xffff0000;
        }
        let mut u_var8: u32 = (DAT_009a1fd8 as u32).wrapping_sub(DAT_00a07cd8 as u32);
        if (u_var8 as i32) < 0 {
            u_var8 = 0u32.wrapping_sub(u_var8);
        }
        let u_var6: u16 = (u_var4 >> 0x10) as u16;
        if 0x20000 < (u_var8 as i32) {
            return u32::from(u_var6) << 0x10;
        }
        let mut u_var10: u32 = (DAT_009a1fd0 as u32).wrapping_sub(DAT_00a07cd4 as u32);
        if (u_var10 as i32) < 0 {
            u_var10 = 0u32.wrapping_sub(u_var10);
        }
        if 0x20000 < (u_var10 as i32) {
            return u32::from(u_var6) << 0x10;
        }
        u_var4 = FUN_0041f3d0(u_var4, u_var8, u_var10);
        if 0x20000 < (u_var4 as i32) {
            return u_var4 & 0xffff0000;
        }
        let i_var3 = abs_diff_wrap(DAT_009a1fd4, DAT_009ec604);
        let i_var9 = abs_diff_wrap(DAT_009a1fd8, DAT_00a081e8);
        let i_var11 = abs_diff_wrap(DAT_009a1fd0, DAT_00a08298);
        let i_var12 = abs_diff_wrap(DAT_00a07bdc, DAT_009d80c0);
        let i_var13 = abs_diff_wrap(DAT_00a07c54, DAT_009d8c18);
        let i_var14 = abs_diff_wrap(DAT_00a07c58, DAT_009d1150);
        u_var8 = FUN_0041f3d0(
            (i_var3 as u32)
                .wrapping_add(i_var12 as u32)
                .wrapping_add(i_var7),
            (i_var9 as u32)
                .wrapping_add(i_var13 as u32)
                .wrapping_add(i_var7),
            (i_var11 as u32)
                .wrapping_add(i_var14 as u32)
                .wrapping_add(i_var7),
        );
        if u_var8 < u_var4 {
            return u_var8 & 0xffff0000;
        }
        if i_var7_v < 0x447
            && ((DAT_009a1fe8 as u32)
                .wrapping_add(3)
                .wrapping_add(u_var5.wrapping_mul(0x23)) as *const i8)
                .read()
                != 0x3a
        {
            u_var5 = FUN_0041c570((i_var7_v >> 3).wrapping_add(i_var7_v >> 2));
            return u_var5;
        }
        u_var5 = FUN_004a5490(param_1, param_2);
        u_var5
    }
}
