// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00669204: i16;
    static mut DAT_00669206: i16;
    static mut DAT_0066920c: i16;
    static mut DAT_0066920e: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6e30(a: i32, b: i32, c: i32) -> i32;
    fn FUN_004d6480(rect: *mut i32, param_1: *mut u32);
    fn FUN_00506550(a: i32, b: i32, c: i32, d: i16);
    fn FUN_005063b0(a: i32, b: i32, c: i32, d: i16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004f7fd0(param_1: *mut u32) {
    // SAFETY: the callees and globals are the game's own; rc is a local
    // four-int rectangle filled by the callee.
    unsafe {
        let mut rc: [i32; 4] = [0; 4];
        let i_var2 = FUN_004d6e30(0x96, 0, 0);
        let i_var3 = FUN_004d6e30(0x50, 0, 0);
        FUN_004d6480(rc.as_mut_ptr(), param_1);
        let l10 = rc[0];
        let lc = rc[1];
        let l8 = rc[2];
        let l4 = rc[3];

        let d4 = DAT_00669204 as i32;
        let mut local_18: u16 = ((d4 + ((d4 >> 31) & 0xff)) >> 8) as u16;
        if 0 < DAT_00669204 && (DAT_00669204 as i8) != 0 {
            local_18 = local_18.wrapping_add(1);
        }
        let mut s_var4: i16 = (((d4.wrapping_neg() & 0xff).wrapping_mul(DAT_0066920c as i32) >> 8)
            as i16)
            .wrapping_add(((l8 as u32).wrapping_sub(l10 as u32) as i32 >> 1) as i16)
            .wrapping_add(l10 as i16);
        while l10 < s_var4 as i32 {
            local_18 = local_18.wrapping_sub(1);
            s_var4 = s_var4.wrapping_sub(DAT_0066920c);
        }

        let d6 = DAT_00669206 as i32;
        let mut local_16: u16 = ((d6 + ((d6 >> 31) & 0xff)) >> 8) as u16;
        if 0 < DAT_00669206 && (DAT_00669206 as i8) != 0 {
            local_16 = local_16.wrapping_add(1);
        }
        let mut s_var5: i16 = (((d6.wrapping_neg() & 0xff).wrapping_mul(DAT_0066920e as i32) >> 8)
            as i16)
            .wrapping_add(((l4 as u32).wrapping_sub(lc as u32) as i32 >> 1) as i16)
            .wrapping_add(lc as i16);
        while lc < s_var5 as i32 {
            local_16 = local_16.wrapping_sub(1);
            s_var5 = s_var5.wrapping_sub(DAT_0066920e);
        }

        if 0xf < DAT_0066920c {
            let b_var1 = 0x1f < DAT_0066920c;
            let mut s_var6 = s_var4;
            let mut u_var7 = local_18;
            if (s_var4 as i32) < l8 {
                loop {
                    if (u_var7 & 3) != 0 && ((u_var7 & 3) == 2 || b_var1) {
                        FUN_00506550(lc, l4, s_var6 as i32, i_var3 as i16);
                    }
                    s_var6 = s_var6.wrapping_add(DAT_0066920c);
                    u_var7 = u_var7.wrapping_add(1);
                    if !((s_var6 as i32) < l8) {
                        break;
                    }
                }
            }
            s_var6 = s_var5;
            u_var7 = local_16;
            if (s_var5 as i32) < l4 {
                loop {
                    if (u_var7 & 3) != 0 && ((u_var7 & 3) == 2 || b_var1) {
                        FUN_005063b0(l10, l8, s_var6 as i32, i_var3 as i16);
                    }
                    s_var6 = s_var6.wrapping_add(DAT_0066920e);
                    u_var7 = u_var7.wrapping_add(1);
                    if !((s_var6 as i32) < l4) {
                        break;
                    }
                }
            }
        }
        if (s_var4 as i32) < l8 {
            loop {
                if (local_18 & 3) == 0 {
                    FUN_00506550(lc, l4, s_var4 as i32, i_var2 as i16);
                }
                s_var4 = s_var4.wrapping_add(DAT_0066920c);
                local_18 = local_18.wrapping_add(1);
                if !((s_var4 as i32) < l8) {
                    break;
                }
            }
        }
        if (s_var5 as i32) < l4 {
            loop {
                if (local_16 & 3) == 0 {
                    FUN_005063b0(l10, l8, s_var5 as i32, i_var2 as i16);
                }
                s_var5 = s_var5.wrapping_add(DAT_0066920e);
                local_16 = local_16.wrapping_add(1);
                if !((s_var5 as i32) < l4) {
                    break;
                }
            }
        }
    }
}
