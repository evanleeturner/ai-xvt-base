// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d9d50: u16;
    static mut DAT_006d4988: i32;
    static mut DAT_006d9d72: u16;
    static mut DAT_006d9d56: i32;
    static mut DAT_00732e54: i32;
    static mut DAT_0071b704: u16;
    static mut DAT_006d9d70: u16;
    static mut DAT_00525e50: [i32; 1];
    static mut DAT_00525e40: [i32; 1];
    static mut DAT_00732904: i32;
    static mut DAT_006d4980: i32;
    static mut DAT_006da9f0: u16;
    static mut DAT_00525e60: [i8; 1];
    static mut DAT_006d9d60: u16;
    static mut DAT_0070ad44: [i8; 1];
    static mut DAT_0070b215: [i8; 1];
    static mut DAT_00732a3c: u16;
    static mut DAT_00732e40: u16;
    static mut DAT_0052b08d: [u8; 1];
    static mut DAT_006db580: u32;
    static mut DAT_00525e68: [u8; 1];
    static mut DAT_00525e70: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00429fc0(a: u32, b: i32) -> u8;
    fn FUN_00444040(a: u32, b: u32) -> u8;
    fn FUN_0042a140(a: u32, b: u32);
    fn FUN_00491ad0() -> u32;
    fn FUN_00448840(a: u32, b: i32) -> u32;
    fn FUN_00478470() -> u32;
}

#[inline(always)]
unsafe fn rd8(a: u32) -> u8 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as *const u8).read_unaligned() }
}
#[inline(always)]
unsafe fn rd16(a: u32) -> u16 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as *const u16).read_unaligned() }
}
#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as *const u32).read_unaligned() }
}
#[inline(always)]
unsafe fn wr8(a: u32, v: u8) {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as *mut u8).write_unaligned(v) }
}
#[inline(always)]
unsafe fn wr16(a: u32, v: u16) {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as *mut u16).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00453c90() -> u32 {
    // SAFETY: all globals, callees and pointed-to records belong to the game;
    // reads and writes happen in the original's order.
    unsafe {
        let mut u_var5: u32 = DAT_006d9d50 as u32;
        let c_var1 = rd8((DAT_006d4988 as u32)
            .wrapping_add(2)
            .wrapping_add(u_var5.wrapping_mul(0x23))) as i8;
        if c_var1 != 0 && c_var1 != 1 {
            return u_var5.wrapping_mul(7) & 0xffff0000;
        }
        let mut u_var6: u32 = DAT_006d9d72 as u32;
        if rd8((DAT_006d9d56 as u32).wrapping_add(0x34)) as u32 == u_var6 {
            if rd16((DAT_00732e54 as u32).wrapping_add(0x6a)) as i16 == -1 {
                u_var6 = DAT_0071b704 as u32;
                let i_var2: i32 = (&raw const DAT_00525e50)
                    .cast::<i32>()
                    .add(DAT_006d9d70 as usize)
                    .read();
                if (DAT_0071b704 as i32) < DAT_00732904 {
                    loop {
                        let i_var9 = (u_var6 & 0xffff)
                            .wrapping_mul(0x23)
                            .wrapping_add(DAT_006d4988 as u32);
                        let c = rd8(i_var9.wrapping_add(3)) as i8;
                        if c != 0 {
                            let i_tmp = rd32(i_var9.wrapping_add(0x1f));
                            let i9 = rd32(i_tmp.wrapping_add(0xa3));
                            if rd8(i9.wrapping_add(1)) != 0
                                && rd16(i9.wrapping_add(4)) == DAT_006d9d50
                            {
                                let arg = if c != -0x70 {
                                    i_var2
                                } else {
                                    i_var2.wrapping_mul(3)
                                };
                                let b = FUN_00429fc0(u_var6 & 0xffff, arg);
                                if b == 1 {
                                    wr16((DAT_00732e54 as u32).wrapping_add(0x6a), u_var6 as u16);
                                    FUN_0042a140(DAT_006d9d50 as u32, u_var6 & 0xffff);
                                    let d = DAT_006da9f0.wrapping_sub(rd16(
                                        (DAT_006d9d50 as u32)
                                            .wrapping_mul(0x23)
                                            .wrapping_add(0x10)
                                            .wrapping_add(DAT_006d4988 as u32),
                                    ));
                                    let t = (&raw const DAT_00525e60)
                                        .cast::<i8>()
                                        .add((d >> 0xd) as usize)
                                        .read();
                                    if t == 0 {
                                        wr8((DAT_006d9d56 as u32).wrapping_add(0x34), 0x18);
                                    } else {
                                        wr8((DAT_006d9d56 as u32).wrapping_add(0x34), 1);
                                    }
                                    FUN_00491ad0();
                                    u_var5 = DAT_00732e54 as u32;
                                    let m = DAT_00732e54 as u32;
                                    if rd8(m.wrapping_add(0x23c)) != 0
                                        && rd8(m.wrapping_add(0x23d)) != 0
                                    {
                                        if rd8(m.wrapping_add(0x23c)) == 1
                                            && (rd8(m.wrapping_add(0x1df)) & 2) != 0
                                        {
                                            wr16(
                                                m.wrapping_add(0x23e),
                                                rd16(m.wrapping_add(0x23e)).wrapping_add(10),
                                            );
                                            u_var6 = DAT_00732e54 as u32;
                                            u_var5 = (DAT_006d9d60 as u32).wrapping_mul(0x566);
                                            if (&raw const DAT_0070ad44)
                                                .cast::<i8>()
                                                .add(u_var5 as usize)
                                                .read()
                                                != 0x15
                                                && (&raw const DAT_0070b215)
                                                    .cast::<i8>()
                                                    .add(u_var5 as usize)
                                                    .read()
                                                    != 0x15
                                            {
                                                let v = (rd8(m.wrapping_add(0x23d)) as i8 as i32)
                                                    .wrapping_sub(1);
                                                wr8(m.wrapping_add(0x23d), v as u8);
                                                return u_var6 & 0xffff0000;
                                            }
                                        } else if rd16(m.wrapping_add(0x240)) == 0 {
                                            u_var5 = FUN_00448840(DAT_006d9d50 as u32, 0x9b);
                                        }
                                    }
                                    return u_var5 & 0xffff0000;
                                }
                            }
                        }
                        let u11 = (u_var6 as u16).wrapping_add(1);
                        u_var6 = (u_var6 & 0xffff0000) | u11 as u32;
                        if !((u11 as i32) < DAT_00732904) {
                            break;
                        }
                    }
                }
                let i_var2: i32 = (&raw const DAT_00525e40)
                    .cast::<i32>()
                    .add(DAT_006d9d70 as usize)
                    .read();
                if rd16((DAT_00732e54 as u32).wrapping_add(0x6a)) as i16 == -1 {
                    u_var6 = (u_var6 & 0xffff0000) | DAT_00732a3c as u32;
                    if (DAT_00732a3c as i32) < DAT_006d4980 {
                        loop {
                            let u_var10 = u_var6 & 0xffff;
                            let i_var9 = u_var10.wrapping_mul(0x23);
                            let base = DAT_006d4988 as u32;
                            let u_tmp = rd32(base.wrapping_add(i_var9).wrapping_add(0x1b));
                            if u_tmp == 0xffffffff {
                                let i_tmp2 = rd32(
                                    (DAT_006d9d50 as u32)
                                        .wrapping_mul(0x23)
                                        .wrapping_add(0x1f)
                                        .wrapping_add(base),
                                );
                                if rd8(base.wrapping_add(i_var9).wrapping_add(3)) != 0 {
                                    let b4 =
                                        FUN_00444040(u_var6, rd8(i_tmp2.wrapping_add(0x87)) as u32);
                                    if b4 != 0
                                        && rd8((DAT_00732e54 as u32).wrapping_add(7)) == 0
                                        && rd8(base.wrapping_add(2).wrapping_add(i_var9)) == 0
                                    {
                                        let b4 = FUN_00429fc0(u_var10, i_var2);
                                        if b4 == 1 {
                                            FUN_0042a140(u_var10, u_var5);
                                            let mut u11 = DAT_006da9f0.wrapping_sub(rd16(
                                                base.wrapping_add(i_var9).wrapping_add(0x10),
                                            ));
                                            if 0x7fff < u11 {
                                                u11 = 0u16.wrapping_sub(u11);
                                            }
                                            let mut u8v = DAT_00732e40.wrapping_sub(rd16(
                                                base.wrapping_add(i_var9).wrapping_add(0x12),
                                            ));
                                            if 0x7fff < u8v {
                                                u8v = 0u16.wrapping_sub(u8v);
                                            }
                                            if u11 < 0x2000 && u8v < 0x2000 {
                                                wr16(
                                                    (DAT_00732e54 as u32).wrapping_add(0x6a),
                                                    u_var6 as u16,
                                                );
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                            let u11 = (u_var6 as u16).wrapping_add(1);
                            u_var6 = (u_var6 & 0xffff0000) | u11 as u32;
                            if !((u11 as i32) < DAT_006d4980) {
                                break;
                            }
                        }
                    }
                }
            }
            let mut u_var11 = rd16((DAT_00732e54 as u32).wrapping_add(0x6a));
            u_var6 = (((DAT_00732e54 as u32) >> 0x10) << 0x10) | u_var11 as u32;
            if u_var11 != 0xffff {
                FUN_0042a140(u_var5, u_var11 as u32);
                let d = DAT_006da9f0.wrapping_sub(rd16(
                    (DAT_006d4988 as u32)
                        .wrapping_add(0x10)
                        .wrapping_add(u_var5.wrapping_mul(0x23)),
                ));
                let c_var1 = (&raw const DAT_00525e60)
                    .cast::<i8>()
                    .add((d >> 0xd) as usize)
                    .read();
                let tbl = (&raw const DAT_0052b08d) as u32;
                u_var11 = rd16(tbl.wrapping_add(
                    (rd8((DAT_00732e54 as u32).wrapping_add(4)) as u32).wrapping_mul(0xef),
                ));
                let pc = rd32(
                    (rd16((DAT_00732e54 as u32).wrapping_add(0x6a)) as u32)
                        .wrapping_mul(0x23)
                        .wrapping_add(0x1f)
                        .wrapping_add(DAT_006d4988 as u32),
                );
                let u_var8: u16;
                if rd8(pc) == 0 {
                    let i_tmp = rd32(pc.wrapping_add(0xa7));
                    u_var8 = rd16(
                        tbl.wrapping_add((rd8(i_tmp.wrapping_add(4)) as u32).wrapping_mul(0xef)),
                    );
                } else {
                    u_var8 = 900;
                }
                u_var5 = FUN_00478470();
                let u_var7: u8;
                if c_var1 == 1 {
                    if DAT_006db580 < 0x2000 && (u_var5 as u16) < 0x4000 {
                        u_var7 = 1;
                    } else {
                        u_var7 = (&raw const DAT_00525e68)
                            .cast::<u8>()
                            .add((u_var5 & 3) as usize)
                            .read();
                    }
                } else if c_var1 == 0 {
                    if (u_var5 as u16) < 0x8001 {
                        u_var7 = (&raw const DAT_00525e68)
                            .cast::<u8>()
                            .add((u_var5 & 3) as usize)
                            .read();
                    } else {
                        u_var7 = 9;
                    }
                } else {
                    let mut v = 0x10u8;
                    if u_var11 < u_var8 || DAT_006db580 < 0x8001 {
                        v = (&raw const DAT_00525e70)
                            .cast::<u8>()
                            .add((u_var5 & 7) as usize)
                            .read();
                    }
                    u_var7 = v;
                }
                wr8((DAT_006d9d56 as u32).wrapping_add(0x34), u_var7);
                u_var6 = FUN_00491ad0();
            }
        }
        u_var6 & 0xffff0000
    }
}
