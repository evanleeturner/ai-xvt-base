// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: u32;
    static mut DAT_0070ad48: u8;
    static mut DAT_0070ad42: u8;
    static mut DAT_005247e8: u8;
    static mut DAT_005247e9: u8;
    static mut DAT_0052686b: u8;
    static mut DAT_00524860: u8;
    static mut DAT_0052686a: u8;
    static mut DAT_0052486c: u8;
    static mut DAT_0070ad47: u8;
    static mut DAT_0070ad92: u8;
    static mut DAT_0070ad40: i8;
    static mut DAT_0052b0b4: u8;
    static mut DAT_0052b0b6: u8;
    static mut DAT_0070ad4f: u8;
    static mut DAT_0070ad49: u8;
    static mut DAT_0070ad44: u8;
    static mut DAT_0071b73e: u8;
    static mut DAT_0070ad54: u8;
    static mut DAT_0071f2dc: u8;
    static mut DAT_0071f2dd: u8;
    static mut DAT_0070b216: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00478320(param_1: u32, param_2: u32) -> u32;
}

#[inline(always)]
unsafe fn rb(p: *const u8, off: u32) -> u8 {
    // SAFETY: caller guarantees the address is valid game memory.
    unsafe { p.wrapping_add(off as usize).read() }
}

#[inline(always)]
unsafe fn rw(p: *const u8, off: u32) -> u16 {
    // SAFETY: caller guarantees the address is valid game memory.
    unsafe { p.wrapping_add(off as usize).cast::<u16>().read_unaligned() }
}

#[inline(always)]
unsafe fn rd(p: *const u8, off: u32) -> u32 {
    // SAFETY: caller guarantees the address is valid game memory.
    unsafe { p.wrapping_add(off as usize).cast::<u32>().read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0049f010(param_1: u32, param_2: u16, param_3: u16) -> u8 {
    // SAFETY: all globals and the record pointers are the game's own memory,
    // read exactly where the original reads them.
    unsafe {
        let a_ad48 = &raw mut DAT_0070ad48 as *const u8;
        let a_ad42 = &raw mut DAT_0070ad42 as *const u8;
        let a_5247e8 = &raw mut DAT_005247e8 as *const u8;
        let a_5247e9 = &raw mut DAT_005247e9 as *const u8;
        let a_52686b = &raw mut DAT_0052686b as *const u8;
        let a_524860 = &raw mut DAT_00524860 as *const u8;
        let a_52686a = &raw mut DAT_0052686a as *const u8;
        let a_52486c = &raw mut DAT_0052486c as *const u8;
        let a_ad47 = &raw mut DAT_0070ad47 as *const u8;
        let a_ad92 = &raw mut DAT_0070ad92 as *const u8;
        let a_ad40 = &raw mut DAT_0070ad40 as *const u8;
        let a_b4 = &raw mut DAT_0052b0b4 as *const u8;
        let a_b6 = &raw mut DAT_0052b0b6 as *const u8;
        let a_ad4f = &raw mut DAT_0070ad4f as *const u8;
        let a_ad49 = &raw mut DAT_0070ad49 as *const u8;
        let a_ad44 = &raw mut DAT_0070ad44 as *const u8;
        let a_b73e = &raw mut DAT_0071b73e as *const u8;
        let a_ad54 = &raw mut DAT_0070ad54 as *const u8;
        let a_b216 = &raw mut DAT_0070b216 as *const u8;

        let i_var8 =
            ((param_1 & 0xffff).wrapping_mul(0x23)).wrapping_add(DAT_006d4988) as *const u8;
        let i_var6 = rd(i_var8, 0x1f);
        let p6 = i_var6 as *const u8;
        let i_var10: *const u8;
        let b_var2: u8;
        if i_var6 == 0 {
            i_var10 = core::ptr::null();
            b_var2 = rb(a_ad48, (rb(i_var8, 0x16) as u32).wrapping_mul(0x566));
        } else {
            i_var10 = rd(p6, 0xa7) as *const u8;
            b_var2 = rb(p6, 0x87);
        }
        let local_a: u16 = rb(i_var8, 0x16) as u16;
        let mut b_var9: u8 = 0;
        let i_var1: u32 = (local_a as u32).wrapping_mul(0x566);
        let b_var3: u8 = rb(a_5247e8, rb(a_ad42, i_var1) as u32);
        let p3 = param_3;

        match param_2 {
            0 => return 0,
            1 => {
                if local_a == p3 {
                    return 1;
                }
            }
            2 => {
                if rb(a_5247e9, p3 as u32) == b_var3 {
                    return 1;
                }
            }
            3 => {
                if rb(a_52686b, (b_var3 as u32) * 0x16) == rb(a_524860, p3 as u32) {
                    return 1;
                }
            }
            4 => {
                if rb(a_52686a, (b_var3 as u32) * 0x16) == rb(a_52486c, p3 as u32) {
                    return 1;
                }
            }
            5 => {
                if i_var6 == 0 {
                    if rb(a_ad47, i_var1) as u16 == p3 {
                        return 1;
                    }
                } else if rb(p6, 0x86) as u16 == p3 {
                    return 1;
                }
            }
            6 => {
                if rb(a_ad92, i_var1) as u16 == p3 {
                    return 1;
                }
            }
            7 => {
                if i_var6 != 0 {
                    match p3 {
                        0 => {
                            if rb(i_var10, 0x1e5) != 0 {
                                return 1;
                            }
                        }
                        1 => {
                            let mut u4: u16 = 0;
                            loop {
                                if (rb(p6, 0x87) as u32) != (u4 as u32)
                                    && rb(i_var10, (u4 as u32).wrapping_add(0x1f0)) != 0
                                {
                                    b_var9 = 1;
                                }
                                u4 = u4.wrapping_add(1);
                                if !(u4 < 10) {
                                    break;
                                }
                            }
                            return b_var9;
                        }
                        2 => {
                            if rb(i_var10, 0x7b) != 0 {
                                return 1;
                            }
                        }
                        3 => {
                            if rb(i_var10, 0x7d) != 0 {
                                return 1;
                            }
                        }
                        4 => {
                            if rw(i_var10, 0x1df) == 0 {
                                return 1;
                            }
                        }
                        5 => {
                            let mut u4: u16 = 0;
                            loop {
                                if (rb(p6, 0x87) as u32) != (u4 as u32)
                                    && rb(i_var10, (u4 as u32).wrapping_add(0x1e6)) != 0
                                {
                                    b_var9 = 1;
                                }
                                u4 = u4.wrapping_add(1);
                                if !(u4 < 10) {
                                    break;
                                }
                            }
                            return b_var9;
                        }
                        6 => {
                            if rw(i_var10, 0xc3) != 0 {
                                return 1;
                            }
                        }
                        7 => {
                            if rb(a_ad40, i_var1) == rb(i_var10, 0xb0) {
                                return 1;
                            }
                        }
                        8 => {
                            if rb(a_ad40, i_var1) != rb(i_var10, 0xb0) {
                                return 1;
                            }
                        }
                        9 => {
                            if rd(i_var8, 0x1b) as i32 != -1 {
                                return 1;
                            }
                        }
                        10 => {
                            if rd(i_var8, 0x1b) as i32 == -1 {
                                return 1;
                            }
                        }
                        0xb => {
                            let mut u5: u32 = 0;
                            loop {
                                if rb(i_var10, u5.wrapping_add(0x1e6)) == 0 {
                                    b_var9 = 1;
                                }
                                let u4 = (u5 as u16).wrapping_add(1);
                                u5 = u4 as u32;
                                if !(u4 < 10) {
                                    break;
                                }
                            }
                            return b_var9;
                        }
                        0xc => {
                            if rw(i_var10, 0x1df) != 0 {
                                return 1;
                            }
                        }
                        0xd => {
                            if rb(i_var10, 0x1e5) == 0 {
                                return 1;
                            }
                        }
                        0xe => {
                            let mut u5: u32 = 0;
                            loop {
                                if rb(i_var10, u5.wrapping_add(0x1f0)) == 0 {
                                    b_var9 = 1;
                                }
                                let u4 = (u5 as u16).wrapping_add(1);
                                u5 = u4 as u32;
                                if !(u4 < 10) {
                                    break;
                                }
                            }
                            return b_var9;
                        }
                        0xf => return 0,
                        0x10 => {
                            if rb(i_var10, 0x7b) == 0 {
                                return 1;
                            }
                        }
                        0x11 => return 0,
                        0x12 => {
                            if rb(i_var10, 0x7d) == 0 {
                                return 1;
                            }
                        }
                        0x16 => {
                            let a = rw(i_var10, 199);
                            let b = rw(i_var10, 0xc3);
                            if ((a >> 2) as u32) <= (b as u32) {
                                return 1;
                            }
                        }
                        0x17 => {
                            let a = rw(i_var10, 199);
                            let b = rw(i_var10, 0xc3);
                            if ((a >> 1) as u32) <= (b as u32) {
                                return 1;
                            }
                        }
                        0x18 => {
                            let a = rw(i_var10, 199);
                            let b = rw(i_var10, 0xc3);
                            let u5 = FUN_00478320(a as u32, 0xc000);
                            if (u5 as u16) <= b {
                                return 1;
                            }
                        }
                        0x19 => {
                            let mut s_var11: u16 = 0;
                            if rb(i_var10, 0x228) != 0 {
                                let mut u4: u16 = 0;
                                loop {
                                    let u7 = u4.wrapping_add(1);
                                    let idx = (rb(i_var10, 4) as u32)
                                        .wrapping_mul(0xef)
                                        .wrapping_add(u4 as u32);
                                    let x1 = rb(
                                        i_var10,
                                        0x346u32
                                            .wrapping_add((rb(a_b4, idx) as u32).wrapping_mul(4)),
                                    ) as u16;
                                    let x2 = rb(
                                        i_var10,
                                        0x346u32
                                            .wrapping_add((rb(a_b6, idx) as u32).wrapping_mul(4)),
                                    ) as u16;
                                    s_var11 = s_var11.wrapping_add(x1).wrapping_add(x2);
                                    u4 = u7;
                                    if !((u7 as u32) < (rb(i_var10, 0x228) as u32)) {
                                        break;
                                    }
                                }
                            }
                            if s_var11 == 0 {
                                return 1;
                            }
                        }
                        _ => {}
                    }
                }
            }
            8 => {
                if rb(a_ad4f, i_var1) as u16 == p3 {
                    return 1;
                }
            }
            9 => {
                if rb(a_ad49, i_var1) as u16 == p3 {
                    return 1;
                }
            }
            10 => {
                if rb(a_ad44, i_var1) as u16 == p3 {
                    return 1;
                }
            }
            0xb => return 1,
            0xc => {
                if b_var2 as u16 == p3 {
                    return 1;
                }
            }
            0xd => {
                let t = rd(i_var8, 0x1b);
                let w = rw(a_b73e, t.wrapping_mul(0x5bd));
                if (rb(a_ad54, (w as u32).wrapping_mul(0x566)) as u32).wrapping_sub(p3 as u32) == 1
                {
                    return 1;
                }
            }
            0xe => {
                let v: u16 = p3.wrapping_mul(5);
                let u5: u32 = (v as u32) / 0x3c;
                let d1 = DAT_0071f2dc as u32;
                if u5 < d1 {
                    return 0;
                }
                if d1 < u5 {
                    return 1;
                }
                let lt = (((v as u32) % 0x3c) < (DAT_0071f2dd as u32)) as u32;
                return 1u32.wrapping_sub(lt) as u8;
            }
            0xf => {
                if local_a != p3 {
                    return 1;
                }
            }
            0x10 => {
                if rb(a_5247e9, p3 as u32) != b_var3 {
                    return 1;
                }
            }
            0x11 => {
                if rb(a_52686b, (b_var3 as u32) * 0x16) != rb(a_524860, p3 as u32) {
                    return 1;
                }
            }
            0x12 => {
                if rb(a_52686a, (b_var3 as u32) * 0x16) != rb(a_52486c, p3 as u32) {
                    return 1;
                }
            }
            0x13 => {
                if i_var6 == 0 {
                    if rb(a_ad47, i_var1) as u16 != p3 {
                        return 1;
                    }
                } else if rb(p6, 0x86) as u16 != p3 {
                    return 1;
                }
            }
            0x14 => {
                if rb(a_ad4f, i_var1) as u16 != p3 {
                    return 1;
                }
            }
            0x15 => {
                if b_var2 as u16 != p3 {
                    return 1;
                }
            }
            0x16 => {
                if rd(i_var8, 0x1b) != p3 as u32 {
                    return 1;
                }
            }
            0x17 => {
                if rb(a_b216, i_var1) as u16 == p3 {
                    return 1;
                }
            }
            0x18 => {
                if rb(a_b216, i_var1) as u16 != p3 {
                    b_var9 = 1;
                }
            }
            _ => {}
        }
        b_var9
    }
}
