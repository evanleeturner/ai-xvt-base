// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a7b70: [i8; 1];
    static mut DAT_009a7b74: [i8; 1];
    static mut DAT_00a00498: i16;
    static mut DAT_00a004a0: i16;
    static mut DAT_009a8c12: i16;
    static mut DAT_00a08244: i16;
    static mut DAT_00a08242: i16;
    static mut DAT_009e95f0: i16;
    static mut DAT_005505ec: u32;
    static mut DAT_005505f0: i32;
    static mut DAT_00bb7380: [u8; 1];
    static mut DAT_0051a888: [u8; 1];
    static mut DAT_009a73e8: i16;
    static mut DAT_009a73f2: i16;
    static mut DAT_009d8c06: i16;
    static mut DAT_009a739c: i16;
    static mut DAT_009a739e: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004acb90(a: *mut u32, b: *mut u32, c: *mut u32) -> u32;
    fn FUN_004a4e70() -> i32;
    fn FUN_004a4ea0(a: *mut i16, b: *mut i16);
    fn FUN_004aa7f0() -> i32;
    fn FUN_004aa870() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00411810(param_1: i32) -> u32 {
    // SAFETY: all globals and callees are the game's own memory and code;
    // table reads are at the offsets the original uses.
    unsafe {
        let mut u_var8: u32 = 0;
        if -1 < param_1 {
            let p = param_1 as u32;
            let b70 = (&raw mut DAT_009a7b70).cast::<u8>();
            let b74 = (&raw mut DAT_009a7b74).cast::<u8>();
            DAT_00a00498 =
                (b70.add(p.wrapping_mul(6).wrapping_add(3) as usize).read() as i8) as i16;
            let i6 = p.wrapping_mul(6) as i32;
            DAT_00a004a0 = (b74.offset(i6 as isize).read() as i8) as i16;
            let hi = (p.wrapping_mul(3) >> 8) << 8;
            let lo = b70.offset(i6 as isize + 2).read() as u32;
            u_var8 = (hi | lo) & 0xffff00ff;
            DAT_009a8c12 = u_var8 as i16;
            DAT_00a08244 = (b74.offset(i6 as isize + 1).read() & 3) as i16;
            return u_var8;
        }
        let mut local_16: i16 = 0;
        let mut local_1a: i16 = 0;
        let mut local_18: i16 = 0;
        let mut local_10: u32 = 0;
        let mut local_4: u32 = 0;
        let mut local_8: u32 = 0;
        if DAT_00a08242 != 0 {
            u_var8 = FUN_004acb90(&raw mut local_8, &raw mut local_4, &raw mut local_10);
        }
        if DAT_009e95f0 != 0 {
            local_16 = FUN_004a4e70() as i16;
            FUN_004a4ea0(&raw mut local_1a, &raw mut local_18);
        }
        let mut u_var4: u32 = 0;
        if FUN_004aa7f0() != 0 {
            u_var4 = FUN_004aa870() & 0xffff;
        }
        let mut s_var9: i16 = 0;
        let mut u_var7: u32 = 1;
        let mut i_var3: i32 = 0;
        let mut local_c: i32 = 0;
        let tbl = (&raw mut DAT_00bb7380).cast::<u8>();
        loop {
            let b_var1 = tbl.offset(i_var3 as isize).read();
            let mut u_var6 = u_var4;
            if b_var1 != 0 {
                if (u_var8 & u_var7) == 0 {
                    if (u_var7 & DAT_005505ec) != 0 {
                        let u_var5: u32 = if b_var1 == 0xb2 {
                            0xb2
                        } else if b_var1 < 0xb3 || b_var1 > 0xbb {
                            0
                        } else {
                            0xba
                        };
                        if u_var5 != 0 {
                            u_var6 = u_var5;
                            if (u_var4 as u16) != 0 {
                                u_var8 |= u_var7;
                                u_var6 = u_var4;
                            }
                        }
                    }
                } else {
                    if b_var1 == 0x9c {
                        local_c = 1;
                    } else if b_var1 == 0x9d {
                        s_var9 = 1;
                    }
                    if (u_var7 & DAT_005505ec) == 0 {
                        if (u_var4 as u16) == 0 {
                            u_var6 = b_var1 as u32;
                        } else {
                            u_var8 &= !u_var7;
                        }
                    }
                }
            }
            DAT_009a8c12 = u_var6 as u16 as i16;
            u_var7 = u_var7.wrapping_mul(2);
            i_var3 += 1;
            u_var4 = u_var6;
            if i_var3 >= 0x14 {
                break;
            }
        }
        DAT_005505ec = u_var8;
        let s_var2 = local_c as i16;
        let mut i_var3 = DAT_005505f0;
        if DAT_009a8c12 == 0 {
            local_c = ((local_10 as i8) as i32 as u32).wrapping_add(0x80) as i32;
            i_var3 = local_c;
            if DAT_005505f0 != -1 {
                let d = (local_c as u32).wrapping_sub(DAT_005505f0 as u32) as i32;
                let adj = d.wrapping_add(((d >> 31) as u32 & 3) as i32);
                i_var3 = DAT_005505f0.wrapping_add(adj >> 2);
                let t = i_var3.wrapping_add(8);
                local_c = t.wrapping_add(((t >> 31) as u32 & 0xf) as i32) >> 4;
                let t2 = DAT_005505f0.wrapping_add(8);
                let r2 = t2.wrapping_add(((t2 >> 31) as u32 & 0xf) as i32) >> 4;
                if local_c != r2 {
                    if local_c < 0 {
                        local_c = 0;
                    }
                    if 0x10 < local_c {
                        local_c = 0x10;
                    }
                    let v = (&raw mut DAT_0051a888)
                        .cast::<u8>()
                        .offset(-(local_c as isize))
                        .read();
                    DAT_009a8c12 = v as u16 as i16;
                }
            }
        }
        DAT_005505f0 = i_var3;
        DAT_009a73e8 = 0;
        DAT_009a73f2 = 0;
        DAT_00a00498 = local_8 as i16;
        DAT_00a004a0 = local_4 as i16;
        DAT_00a08244 =
            ((s_var2 as i32 as u32).wrapping_add(((s_var9 as i32).wrapping_mul(2)) as u32)) as i16;
        DAT_009d8c06 = local_16;
        DAT_009a739c = local_1a;
        DAT_009a739e = local_18;
        (((local_c as u32) >> 16) << 16) | (DAT_009a8c12 as u16 as u32)
    }
}
