// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005233d8: i32;
    static mut DAT_005235f8: i32;
    static mut DAT_009a8d50: i32;
    static mut DAT_009d8c2c: i16;
    static mut DAT_009e95f2: i16;
    static mut DAT_009ecc44: u16;
    static mut DAT_009ed230: u16;
    static mut DAT_00a07cd0: i16;
    static mut DAT_00a080f2: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0046a3b0(a: u16) -> u16;
    fn FUN_00421980(a: u16, b: i16) -> u16;
}

unsafe extern "stdcall" {
    fn RaiseException(code: u32, flags: u32, nargs: u32, args: *const u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00421c50(param_1: u16, param_2: *mut u16) {
    let mut param_1 = param_1;
    let p = param_2;
    // SAFETY: the callees and globals are the game's own; param_2 points at
    // the caller's buffer, accessed at the same offsets as the original.
    unsafe {
        let rd = |i: usize| -> u16 { p.add(i).read_unaligned() };
        let wr = |i: usize, v: u16| {
            p.add(i).write_unaligned(v);
        };

        let mut u_var1: u16;
        let mut u_var2: u16;
        let mut u_var5: u16;
        let mut u_var6: u16;
        let mut u_var8: u16;
        let mut s_var7: i16;
        let s_var11: i16;
        let local_8: i16;

        wr(0, param_1);
        u_var1 = 0;
        wr(2, param_1 & 0x8000);
        wr(4, param_1.wrapping_add(0x4000) & 0x8000);
        u_var2 = 0;
        let skip;
        if param_1 < 0x8000 {
            u_var5 = 0;
            skip = param_1 < 0x4000;
        } else {
            param_1 &= 0x7fff;
            u_var1 = 1;
            u_var5 = 1;
            skip = param_1 > 0x3fff;
        }
        if !skip {
            u_var1 = u_var5;
            u_var2 = 2;
        }
        wr(8, u_var1);
        wr(9, u_var2);
        u_var2 = param_1;
        if param_1 > 0x3fff {
            u_var2 = 0x8000u16.wrapping_sub(param_1);
        }
        u_var1 = FUN_0046a3b0(u_var2);
        wr(1, u_var1);
        u_var1 = FUN_0046a3b0(u_var2.wrapping_add(0x4000));
        wr(3, u_var1);
        if u_var2 < DAT_00a080f2 {
            local_8 = 0;
            wr(0xc, 0);
            s_var7 = if DAT_005235f8 == 1 { 100 } else { 0x5b };
        } else {
            local_8 = 4;
            wr(0xc, 4);
            u_var2 = 0x4000u16.wrapping_sub(u_var2);
            s_var7 = if DAT_005235f8 == 1 { 100 } else { 0x6e };
        }
        u_var1 = FUN_00421980(u_var2, s_var7);
        u_var2 = FUN_0046a3b0(u_var2.wrapping_add(0x4000));
        wr(5, u_var2);
        if u_var2 == 0 {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
        let q: u64 = if u_var2 != 0 {
            0x8000_0000u64 / (u_var2 as u64)
        } else {
            0
        };
        u_var5 = q as u16;
        if local_8 != 0 && DAT_005235f8 == 0 {
            u_var5 = ((((q as u32) & 0xffff)
                .wrapping_mul(DAT_009ecc44 as u32)
                .wrapping_add(0x8000))
                >> 16) as u16;
        }
        wr(6, u_var5);
        u_var2 = 0;
        wr(0x18, 0);
        wr(0x19, 0);
        if rd(0xc) == 0 {
            u_var8 = 0;
            wr(7, DAT_009d8c2c as u16);
            u_var5 = 0x8000;
            s_var7 = DAT_009d8c2c.wrapping_sub(2);
            u_var6 = 2;
            if DAT_009d8c2c != 1 {
                loop {
                    u_var2 = u_var2.wrapping_add(1);
                    if u_var5.wrapping_add(u_var1) < u_var5 {
                        u_var8 = u_var8.wrapping_add(1);
                    }
                    wr(u_var6 as usize + 0x18, u_var2);
                    wr(u_var6 as usize + 0x19, u_var8);
                    let b = s_var7 != 0;
                    u_var5 = u_var5.wrapping_add(u_var1);
                    s_var7 = s_var7.wrapping_sub(1);
                    u_var6 = u_var6.wrapping_add(2);
                    if !b {
                        break;
                    }
                }
            }
        } else {
            u_var8 = 0;
            u_var6 = 0;
            wr(7, DAT_00a07cd0 as u16);
            u_var2 = 0x8000;
            s_var7 = DAT_00a07cd0.wrapping_sub(2);
            u_var5 = 2;
            if DAT_00a07cd0 != 1 {
                loop {
                    u_var6 = u_var6.wrapping_add(1);
                    if u_var2.wrapping_add(u_var1) < u_var2 {
                        u_var8 = u_var8.wrapping_add(1);
                    }
                    wr(u_var5 as usize + 0x18, u_var8);
                    wr(u_var5 as usize + 0x19, u_var6);
                    let b = s_var7 != 0;
                    u_var2 = u_var2.wrapping_add(u_var1);
                    s_var7 = s_var7.wrapping_sub(1);
                    u_var5 = u_var5.wrapping_add(2);
                    if !b {
                        break;
                    }
                }
            }
        }
        u_var2 = rd(7);
        u_var5 = (rd(0xc) == 0) as u16;
        u_var6 = 0;
        while u_var2 != 0 {
            u_var8 = 1;
            let u_var3 = u_var5 as usize;
            loop {
                u_var2 = u_var2.wrapping_sub(1);
                if !(u_var2 != 0 && rd(u_var5 as usize + 0x1a) == rd(u_var3 + 0x18)) {
                    break;
                }
                u_var8 = u_var8.wrapping_add(1);
                u_var5 = u_var5.wrapping_add(2);
            }
            u_var5 = u_var5.wrapping_add(2);
            wr(u_var6 as usize + 0xc9a, u_var8);
            u_var6 = u_var6.wrapping_add(1);
        }
        wr(0xc99, u_var6);
        u_var2 = param_1.wrapping_add(0x4000) & 0x7fff;
        if u_var2 > 0x3fff {
            u_var2 = 0x8000u16.wrapping_sub(u_var2);
        }
        if u_var2 < DAT_00a080f2 {
            s_var7 = 0;
            wr(0xd, 0);
            s_var11 = if DAT_005235f8 == 1 { 100 } else { 0x5b };
        } else {
            s_var7 = 4;
            u_var2 = 0x4000u16.wrapping_sub(u_var2);
            wr(0xd, 4);
            s_var11 = if DAT_005235f8 == 1 { 100 } else { 0x6e };
        }
        u_var2 = FUN_00421980(u_var2, s_var11);
        wr(0xe, u_var2);
        wr(0xf, 0);
        if s_var7 == local_8 && DAT_005235f8 == 0 {
            u_var2 = rd(0);
            if (u_var2 < 0xe000 && u_var2 > 0x9fff) || (u_var2 < 0x6000 && u_var2 > 0x1fff) {
                let v = rd(0xe) as i32;
                if v == 0 {
                    RaiseException(0xC0000094, 0, 0, core::ptr::null());
                }
                let div_lo: u32 = if v != 0 {
                    (0x1000000i64 / (v as i64)) as u32
                } else {
                    0
                };
                let u_var3 = div_lo & 0xffff;
                wr(0xe, (u_var3 << 8) as u16);
                wr(0xf, (u_var3 >> 8) as u16);
                wr(0xc98, u_var1 >> 8);
            } else {
                wr(0xe, 0x100);
                wr(0xf, 0x100);
                wr(
                    0xc98,
                    ((((u_var1 as u32).wrapping_mul(0xd800)) >> 0x18) as u8) as u16,
                );
            }
        } else {
            wr(
                0xc98,
                ((((rd(0xe) as u32)
                    .wrapping_mul(u_var1 as u32)
                    .wrapping_add(0x800000))
                    >> 0x18) as u8) as u16,
            );
        }
        wr(0xb, rd(0xc) | rd(8) | rd(9));
        wr(10, (rd(9) >> 1).wrapping_add(rd(8)));
        u_var2 = rd(7);
        wr(0x10, rd(0x18));
        wr(0x14, rd(0x19));
        u_var6 = DAT_009ed230.wrapping_sub(rd(0x19));
        wr(0x12, u_var6);
        u_var1 = rd((u_var2 as usize) * 2 + 0x16);
        wr(0x11, u_var1);
        u_var5 = rd((u_var2 as usize) * 2 + 0x17);
        wr(0x15, u_var5);
        u_var5 = DAT_009ed230.wrapping_sub(u_var5);
        u_var1 = u_var1.wrapping_sub(rd(0x18));
        wr(0x13, u_var5);
        wr(0x16, u_var1);
        if (u_var1 as i16) < 0 {
            wr(0x16, u_var1.wrapping_neg());
        }
        u_var5 = u_var5.wrapping_sub(u_var6);
        wr(0x17, u_var5);
        if (u_var5 as i16) < 0 {
            wr(0x17, u_var5.wrapping_neg());
        }
        let mode2 = DAT_005233d8 == 2;
        u_var1 = 0;
        if u_var2 != 0 {
            loop {
                let u_var3 = u_var1 as usize;
                u_var2 = rd(u_var3 * 2 + 0x18);
                if rd(9) != 0 {
                    u_var2 = u_var2.wrapping_neg();
                }
                u_var5 = rd(u_var3 * 2 + 0x19);
                if rd(8) != 0 {
                    u_var5 = u_var5.wrapping_neg();
                }
                let a: i32 = if mode2 {
                    ((u_var2 as i16 as i32).wrapping_mul(2)) as i32
                } else {
                    u_var2 as i16 as i32
                };
                let b: u32 = (u_var5 as i16 as i32 as u32).wrapping_mul(DAT_009a8d50 as u32);
                let iv: i32 = if DAT_009e95f2 < 1 {
                    (a as u32).wrapping_sub(b) as i32
                } else {
                    (a as u32).wrapping_add(b) as i32
                };
                u_var1 = u_var1.wrapping_add(1);
                p.add(u_var3 * 2 + 0x12dc).cast::<i32>().write_unaligned(iv);
                if !(u_var1 < rd(7)) {
                    break;
                }
            }
        }
    }
}
