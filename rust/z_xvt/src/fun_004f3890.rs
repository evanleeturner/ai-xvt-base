// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00650424: i16;
    static mut DAT_00650426: i16;
    static mut DAT_0065042c: i16;
    static mut DAT_0065042e: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004df760(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_004dd000(param_1: *mut i32, param_2: *mut u32);
    fn FUN_004f82f0(param_1: i32, param_2: i32, param_3: i32, param_4: i16);
    fn FUN_004f8160(param_1: i32, param_2: i32, param_3: i32, param_4: i16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004f3890(param_1: *mut u32) {
    // SAFETY: the callees and globals are the game's own; param_1 is passed
    // through unchanged to the callee that fills the 4-int block.
    unsafe {
        let mut blk: [i32; 4] = [0; 4];
        let i_var2 = FUN_004df760(0x96, 0, 0);
        let i_var3 = FUN_004df760(0x50, 0, 0);
        FUN_004dd000(&raw mut blk as *mut i32, param_1);
        let local_10 = blk[0];
        let local_c = blk[1];
        let local_8 = blk[2];
        let local_4 = blk[3];

        let d = DAT_00650424 as i32;
        let mut local_18: u16 = ((d as u32).wrapping_add(((d >> 31) as u32) & 0xff) >> 8) as u16;
        if 0 < d && (d as i8) != 0 {
            local_18 = local_18.wrapping_add(1);
        }
        let mut s_var4: i16 = (((0u32.wrapping_sub(DAT_00650424 as i32 as u32) & 0xff)
            .wrapping_mul(DAT_0065042c as i32 as u32))
            >> 8) as i16;
        s_var4 = s_var4
            .wrapping_add((((local_8 as u32).wrapping_sub(local_10 as u32)) as i32 >> 1) as i16)
            .wrapping_add(local_10 as i16);
        while local_10 < s_var4 as i32 {
            local_18 = local_18.wrapping_sub(1);
            s_var4 = s_var4.wrapping_sub(DAT_0065042c);
        }

        let d = DAT_00650426 as i32;
        let mut local_16: u16 = ((d as u32).wrapping_add(((d >> 31) as u32) & 0xff) >> 8) as u16;
        if 0 < d && (d as i8) != 0 {
            local_16 = local_16.wrapping_add(1);
        }
        let mut s_var5: i16 = (((0u32.wrapping_sub(DAT_00650426 as i32 as u32) & 0xff)
            .wrapping_mul(DAT_0065042e as i32 as u32))
            >> 8) as i16;
        s_var5 = s_var5
            .wrapping_add((((local_4 as u32).wrapping_sub(local_c as u32)) as i32 >> 1) as i16)
            .wrapping_add(local_c as i16);
        while local_c < s_var5 as i32 {
            local_16 = local_16.wrapping_sub(1);
            s_var5 = s_var5.wrapping_sub(DAT_0065042e);
        }

        if 0xf < DAT_0065042c {
            let b_var1 = 0x1f < DAT_0065042c;
            let mut s_var7 = s_var4;
            let mut u_var6 = local_18;
            if (s_var4 as i32) < local_8 {
                loop {
                    if (u_var6 & 3) != 0 && ((u_var6 & 3) == 2 || b_var1) {
                        FUN_004f82f0(local_c, local_4, s_var7 as i32, i_var3 as i16);
                    }
                    s_var7 = s_var7.wrapping_add(DAT_0065042c);
                    u_var6 = u_var6.wrapping_add(1);
                    if !((s_var7 as i32) < local_8) {
                        break;
                    }
                }
            }
            s_var7 = s_var5;
            u_var6 = local_16;
            if (s_var5 as i32) < local_4 {
                loop {
                    if (u_var6 & 3) != 0 && ((u_var6 & 3) == 2 || b_var1) {
                        FUN_004f8160(local_10, local_8, s_var7 as i32, i_var3 as i16);
                    }
                    s_var7 = s_var7.wrapping_add(DAT_0065042e);
                    u_var6 = u_var6.wrapping_add(1);
                    if !((s_var7 as i32) < local_4) {
                        break;
                    }
                }
            }
        }

        if (s_var4 as i32) < local_8 {
            loop {
                if (local_18 & 3) == 0 {
                    FUN_004f82f0(local_c, local_4, s_var4 as i32, i_var2 as i16);
                }
                s_var4 = s_var4.wrapping_add(DAT_0065042c);
                local_18 = local_18.wrapping_add(1);
                if !((s_var4 as i32) < local_8) {
                    break;
                }
            }
        }
        if (s_var5 as i32) < local_4 {
            loop {
                if (local_16 & 3) == 0 {
                    FUN_004f8160(local_10, local_8, s_var5 as i32, i_var2 as i16);
                }
                s_var5 = s_var5.wrapping_add(DAT_0065042e);
                local_16 = local_16.wrapping_add(1);
                if !((s_var5 as i32) < local_4) {
                    break;
                }
            }
        }
    }
}
