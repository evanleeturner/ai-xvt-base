// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071e6b4: u16;
    static mut DAT_00732a3c: u16;
    static mut DAT_006d4980: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_00732e54: i32;
    static mut DAT_0070ad49: u8;
    static mut DAT_0052b083: u8;
    static mut DAT_0052b0b4: u8;
    static mut DAT_0071f2dd: u8;
    static mut DAT_0071b758: u8;
    static mut DAT_0071b757: u8;
    static mut DAT_0071b756: u8;
    static mut DAT_0071b741: u8;
    static mut DAT_0071b74a: u16;
    static mut DAT_0071bce1: u8;
    static mut DAT_006db580: u32;
    static mut DAT_006d499c: u32;
    static mut DAT_0071e6be: u16;
    static mut DAT_0071ecf4: i32;
    static mut DAT_0070a1d0: u16;
    static mut DAT_007092d4: i8;
    static mut DAT_00703c88: i32;
    static mut DAT_00732a38: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a7aa0(a: u32) -> u32;
    fn FUN_0042a140(a: u32, b: u32) -> u64;
    fn FUN_00441fc0(a: u32, b: i32, c: u32, d: i32, e: i32) -> u64;
    fn FUN_004a1d70(a: u32, b: i32);
    fn FUN_004a31a0() -> i16;
    fn FUN_0040a980(a: i32, b: i32);
    fn FUN_00444040(a: u32, b: u32) -> u8;
    fn FUN_0041b920(a: u32, b: u32);
    fn FUN_0042b150(a: i32) -> i32;
    fn FUN_004490f0(a: u32) -> i32;
    fn FUN_00449790(a: u32, b: i32) -> i32;
    fn FUN_0040a1a0(a: i32, b: i32, c: i32);
    fn FUN_00447240(a: u32, b: u32);
    fn FUN_0042b1e0(a: i32, b: u32) -> *mut u16;
    fn FUN_0041aba0(a: u16, b: u32, c: u16);
    fn FUN_00413580(a: u16);
}

unsafe fn g8(a: i32) -> u8 {
    // SAFETY: the address is game memory the original reads.
    unsafe { (a as u32 as usize as *const u8).read() }
}
unsafe fn gs8(a: i32) -> i8 {
    // SAFETY: the address is game memory the original reads.
    unsafe { (a as u32 as usize as *const i8).read() }
}
unsafe fn g16(a: i32) -> u16 {
    // SAFETY: the address is game memory the original reads.
    unsafe { (a as u32 as usize as *const u16).read_unaligned() }
}
unsafe fn g32(a: i32) -> u32 {
    // SAFETY: the address is game memory the original reads.
    unsafe { (a as u32 as usize as *const u32).read_unaligned() }
}
unsafe fn p8(a: i32, v: u8) {
    // SAFETY: the address is game memory the original writes.
    unsafe { (a as u32 as usize as *mut u8).write(v) }
}
unsafe fn p16(a: i32, v: u16) {
    // SAFETY: the address is game memory the original writes.
    unsafe { (a as u32 as usize as *mut u16).write_unaligned(v) }
}
unsafe fn p32(a: i32, v: u32) {
    // SAFETY: the address is game memory the original writes.
    unsafe { (a as u32 as usize as *mut u32).write_unaligned(v) }
}
unsafe fn dd(off: i32) -> i32 {
    // SAFETY: reads the game's global pointer value.
    unsafe { DAT_00732e54.wrapping_add(off) }
}
fn ng(c: bool) -> u16 {
    0u16.wrapping_sub(c as u16)
}
fn ad(a: i32, b: i32) -> i32 {
    a.wrapping_add(b)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_assignments, unused_variables, unused_mut)]
pub unsafe extern "stdcall" fn FUN_00445e10() {
    // SAFETY: all globals, callees and addresses are the game's own memory,
    // accessed exactly where and how the original does.
    unsafe {
        let a_0070ad49 = (&raw mut DAT_0070ad49) as usize as i32;
        let a_0052b083 = (&raw mut DAT_0052b083) as usize as i32;
        let a_0052b0b4 = (&raw mut DAT_0052b0b4) as usize as i32;
        let a_0071b758 = (&raw mut DAT_0071b758) as usize as i32;
        let a_0071b757 = (&raw mut DAT_0071b757) as usize as i32;
        let a_0071b756 = (&raw mut DAT_0071b756) as usize as i32;
        let a_0071b741 = (&raw mut DAT_0071b741) as usize as i32;
        let a_0071b74a = (&raw mut DAT_0071b74a) as usize as i32;
        let a_0071bce1 = (&raw mut DAT_0071bce1) as usize as i32;

        let mut pc4: i32;
        let mut iv5: i32;
        let mut iv8: i32;
        let mut iv10: i32;
        let mut iv16: i32;
        let mut u7: u16;
        let mut u9: u32;
        let mut u12: u32;
        let mut u17: u32;
        let mut s14: i16;
        let mut s15: i16;
        let mut u18: u16;
        let mut u19: u16;
        let mut u20: u16;
        let mut u21: u16;
        let mut unaff_edi: u32 = 0;
        let mut local_a: u16;
        let mut ustack8: u16 = 0;
        let mut c1: i8;
        let mut c3: i8;
        let mut b2: u8;
        let mut ps11: i32;

        let b22 = DAT_0071e6b4 == 0;
        if b22 {
            DAT_0071e6b4 = 0xec;
        }
        u12 = DAT_00732a3c as u32;
        if (u12 as i32) < DAT_006d4980 {
            loop {
                iv8 = (u12.wrapping_mul(0x23) as i32).wrapping_add(DAT_006d4988);
                if g8(ad(iv8, 3)) != 0 {
                    pc4 = g32(ad(iv8, 0x1f)) as i32;
                    if g8(pc4) == 0 {
                        iv8 = g32(ad(pc4, 0xa7)) as i32;
                        u18 = 0;
                        loop {
                            u7 = u18.wrapping_add(1);
                            unaff_edi = u18 as u32;
                            p32(ad(ad(iv8, 0x15), (unaff_edi as i32).wrapping_mul(4)), 0);
                            u18 = u7;
                            if !(u7 < 5) {
                                break;
                            }
                        }
                    }
                }
                u12 = (u12 as u16).wrapping_add(1) as u32;
                if !((u12 as i32) < DAT_006d4980) {
                    break;
                }
            }
        }
        local_a = DAT_00732a3c;
        if (local_a as i32) < DAT_006d4980 {
            loop {
                'next: {
                    u12 = local_a as u32;
                    iv8 = u12.wrapping_mul(0x23) as i32;
                    iv16 = DAT_006d4988.wrapping_add(iv8);
                    ustack8 = iv8 as u16;
                    if g8(ad(iv16, 3)) == 0 {
                        break 'next;
                    }
                    pc4 = g32(ad(iv16, 0x1f)) as i32;
                    if g8(pc4) != 0 {
                        break 'next;
                    }
                    u18 = (unaff_edi >> 16) as u16;
                    'l467: {
                        if g32(ad(iv16, 0x1b)) as i32 == -1 {
                            if !b22 {
                                break 'next;
                            }
                            DAT_00732e54 = g32(ad(pc4, 0xa7)) as i32;
                            if g8(ad(iv16, 2)) == 0 {
                                iv16 = FUN_0042b150(DAT_00732e54);
                                p8(dd(0x20f), 2);
                                p8(dd(0x212), 2);
                                if (g8(dd(0x1dd)) & 1) != 0 {
                                    b2 = g8(ad(
                                        a_0070ad49,
                                        (g8(ad(ad(DAT_006d4988, 0x16), iv8)) as u32)
                                            .wrapping_mul(0x566)
                                            as i32,
                                    ));
                                    if gs8(ad(iv16, 0x34)) == 0x20 {
                                        if b2 == 5 {
                                            p8(dd(0x20f), 0);
                                        } else if b2 == 4 || b2 == 3 {
                                            p8(dd(0x20f), 1);
                                        } else {
                                            p8(dd(0x20f), 2);
                                        }
                                        p8(dd(0x212), 3);
                                    }
                                    s14 = g16(ad(
                                        a_0052b083,
                                        (g8(ad(
                                            g32(ad(
                                                g32(ad(ad(DAT_006d4988, 0x1f), iv8)) as i32,
                                                0xa7,
                                            )) as i32,
                                            4,
                                        )) as u32)
                                            .wrapping_mul(0xef)
                                            as i32,
                                    )) as i16;
                                    ps11 = dd(0x20b);
                                    if (g16(ps11) as i16) < s14.wrapping_mul(2) {
                                        if g8(dd(0x212)) == 2 {
                                            p8(dd(0x212), 4);
                                        }
                                        if (g16(ps11) as i16) < 1 {
                                            if g8(dd(0x20f)) == 2 {
                                                p8(dd(0x20f), 4);
                                            }
                                            if b2 < 2 {
                                                u7 = 0xfa;
                                            } else {
                                                u7 = (ng(b2 < 3) & 0xdae4).wrapping_add(10000);
                                            }
                                        } else if b2 < 2 {
                                            u7 = 0;
                                        } else {
                                            if b2 == 5 {
                                                u7 = 1;
                                            } else {
                                                u7 = 3;
                                                if b2 != 4 {
                                                    u7 = (ng(b2 == 3) & 0xfff8).wrapping_add(0xf);
                                                }
                                            }
                                            u7 = ng((DAT_0071f2dd as u16 & u7) == u7) & 100;
                                        }
                                        s15 = 0;
                                        u12 = (u18 as u32) << 16;
                                        if g8(dd(0x213)) != 0 {
                                            loop {
                                                c1 = gs8(dd(ad(
                                                    0x345,
                                                    ((u12 & 0xffff).wrapping_mul(4)) as i32,
                                                )));
                                                if 0 < c1 {
                                                    s15 = s15.wrapping_add(c1 as i16);
                                                }
                                                u18 = (u12 as u16).wrapping_add(1);
                                                u12 = ((u12 >> 16) << 16) | (u18 as u32);
                                                if !((u18 as u32) < g8(dd(0x213)) as u32) {
                                                    break;
                                                }
                                            }
                                        }
                                        c1 = gs8(ad(ad(DAT_006d4988, 3), iv8));
                                        unaff_edi = u12 & 0xffff0000;
                                        u18 = 0;
                                        while s15 != 0 && u18 < u7 {
                                            pc4 = dd(ad(
                                                0x345,
                                                ((unaff_edi & 0xffff).wrapping_mul(4)) as i32,
                                            ));
                                            c3 = gs8(pc4);
                                            if 0 < c3 {
                                                s15 = s15.wrapping_sub(1);
                                                p8(pc4, c3.wrapping_sub(1) as u8);
                                                p16(
                                                    dd(0x20b),
                                                    g16(dd(0x20b)).wrapping_add(
                                                        (ng(c1 == 0xc) & 0x1c).wrapping_add(4),
                                                    ),
                                                );
                                                if s14.wrapping_mul(2) <= (g16(dd(0x20b)) as i16) {
                                                    s15 = 0;
                                                }
                                            }
                                            u21 = (unaff_edi >> 16) as u16;
                                            u19 = (unaff_edi as u16).wrapping_add(1);
                                            unaff_edi = ((u21 as u32) << 16) | (u19 as u32);
                                            if (g8(dd(0x213)) as u32) <= (u19 as u32) {
                                                unaff_edi = (u21 as u32) << 16;
                                            }
                                            u18 = u18.wrapping_add(1);
                                        }
                                    }
                                }
                                if g8(dd(0x212)) == 2 {
                                    s14 = 0;
                                    u18 = 0;
                                    unaff_edi &= 0xffff0000;
                                    if g8(dd(0x213)) != 0 {
                                        loop {
                                            if gs8(dd(ad(
                                                0x344,
                                                ((unaff_edi & 0xffff).wrapping_mul(4)) as i32,
                                            ))) != 0
                                            {
                                                s14 = s14.wrapping_add(gs8(dd(ad(
                                                    ((unaff_edi & 0xffff).wrapping_mul(4)) as i32,
                                                    0x345,
                                                )))
                                                    as i16);
                                                u18 = u18.wrapping_add(1);
                                            }
                                            u7 = (unaff_edi as u16).wrapping_add(1);
                                            unaff_edi =
                                                (((unaff_edi >> 16) & 0xffff) << 16) | (u7 as u32);
                                            if !((u7 as u32) < g8(dd(0x213)) as u32) {
                                                break;
                                            }
                                        }
                                    }
                                    if u18 != 0 {
                                        s14 = ((s14 as i32) / (u18 as i32)) as i16;
                                        if s14 < 0x20 {
                                            s14 = 0x14;
                                            p8(dd(0x212), 4);
                                            break 'l467;
                                        }
                                        p8(dd(0x212), ((s14 < 0x60) as u8).wrapping_add(2));
                                    }
                                }
                                s14 = 0x14;
                            } else {
                                p8(dd(0x20f), 2);
                                s14 = 0;
                                p8(dd(0x212), 2);
                                if DAT_007092d4 == 2 && gs8(ad(ad(DAT_006d4988, iv8), 2)) == 4 {
                                    if g16(dd(0xc3)) != 0 {
                                        b2 = g8(ad(ad(DAT_006d4988, iv8), 3));
                                        if b2 == 0x33 || b2 == 0x34 || b2 == 0x35 || b2 == 0x36 {
                                            s14 = 0;
                                            unaff_edi = 0;
                                            iv16 = FUN_004490f0(b2 as u32);
                                            if 0 < iv16 {
                                                loop {
                                                    iv10 = FUN_00449790(
                                                        g8(ad(ad(DAT_006d4988, 3), iv8)) as u32,
                                                        unaff_edi as i32,
                                                    );
                                                    if iv10 == 8
                                                        && g8(dd(ad(0x31a, unaff_edi as i32))) != 0
                                                    {
                                                        s14 = s14.wrapping_add(1);
                                                    }
                                                    unaff_edi = unaff_edi.wrapping_add(1);
                                                    if !((unaff_edi as i32) < iv16) {
                                                        break;
                                                    }
                                                }
                                            }
                                        } else {
                                            s14 = 1;
                                        }
                                    }
                                    p8(dd(0x20f), 3);
                                }
                                s14 = s14.wrapping_mul(5);
                            }
                        } else {
                            DAT_00732e54 = g32(ad(pc4, 0xa7)) as i32;
                            iv10 = g32(ad(iv16, 0x1b)) as i32;
                            iv5 = (iv10 as u32).wrapping_mul(0x5bd) as i32;
                            if g8(ad(a_0071b758, iv5)) != 0 {
                                u9 = FUN_004a7aa0(
                                    (((((((iv10 as u32).wrapping_mul(0x16f)) >> 8) & 0xffffff)
                                        << 8)
                                        | (g8(ad(iv16, 3)) as u32))
                                        & 0xffff00ff),
                                );
                                u7 = g16(ad(a_0071b74a, iv5));
                                unaff_edi = ((u18 as u32) << 16) | (u7 as u32);
                                let idx = (g8(ad(a_0071b757, iv5)) as u32)
                                    .wrapping_add((u9 & 0xffff).wrapping_mul(0xef));
                                let tv = g8(ad(a_0052b0b4, idx as i32)) as u32;
                                let toff = tv.wrapping_mul(4) as i32;
                                if u7 == 0xffff
                                    || (g8(dd(ad(toff, 0x346))) as u16)
                                        .wrapping_add(g8(dd(ad(0x34a, toff))) as u16)
                                        == 0
                                {
                                    p8(ad(a_0071b756, iv5), 0);
                                    p16(dd(0x231), 0);
                                } else {
                                    u9 = u7 as u32;
                                    let u24 = FUN_0042a140(u12, u9);
                                    let extraout_edx = (u24 >> 32) as u32 as i32;
                                    u17 = 0x18dad;
                                    iv16 = extraout_edx;
                                    if (u9 as i32) < DAT_006d4980 {
                                        iv16 = u9.wrapping_mul(0x23) as i32;
                                        c1 = gs8(ad(ad(iv16, 2), DAT_006d4988));
                                        if c1 == 4 || c1 == 5 || c1 == 3 {
                                            u17 = 0x3ba6c;
                                        }
                                    }
                                    let mut cond = false;
                                    if DAT_006db580 < u17 {
                                        let u23 = FUN_00441fc0(u17, iv16, unaff_edi, 0, iv10);
                                        if (u23 as i16) != 0 {
                                            cond = true;
                                        }
                                    }
                                    if cond {
                                        p16(dd(0x231), g16(dd(0x231)).wrapping_add(DAT_0071e6be));
                                        if (u9 as i32) < DAT_006d4980 {
                                            iv16 = g32(ad(
                                                g32(ad(
                                                    ad(u9.wrapping_mul(0x23) as i32, 0x1f),
                                                    DAT_006d4988,
                                                ))
                                                    as i32,
                                                0xa7,
                                            ))
                                                as i32;
                                            if gs8(ad(iv16, 0x23c)) == 1
                                                && g16(ad(iv16, 0x23e)) != 0
                                            {
                                                p16(
                                                    dd(0x231),
                                                    g16(dd(0x231)).wrapping_sub(DAT_0071e6be >> 1),
                                                );
                                            }
                                        }
                                        u9 = FUN_004a7aa0(0xc);
                                        u17 = FUN_004a7aa0(
                                            ((((((DAT_006d4988 as u32) >> 8) & 0xffffff) << 8)
                                                | (g8(ad(ad(DAT_006d4988, 3), iv8)) as u32))
                                                & 0xffff00ff),
                                        );
                                        let lim = (ng((u17 as i16) == (u9 as i16)) & 0xfe9e)
                                            .wrapping_add(0x2c4);
                                        if ((g16(dd(0x231)) as i16) as i32) < (lim as i32) {
                                            p8(ad(a_0071b756, iv5), 1);
                                        } else {
                                            p8(ad(a_0071b756, iv5), 2);
                                        }
                                    } else {
                                        if 0 < (g16(dd(0x231)) as i16) {
                                            p16(
                                                dd(0x231),
                                                g16(dd(0x231))
                                                    .wrapping_sub(DAT_0071e6be >> 1)
                                                    .wrapping_sub(DAT_0071e6be),
                                            );
                                            if (g16(dd(0x231)) as i16) < 0 {
                                                p16(dd(0x231), 0);
                                            }
                                        }
                                        p8(ad(a_0071b756, iv5), 0);
                                    }
                                }
                            }
                            u20 = (unaff_edi >> 16) as u16;
                            u18 = 0xffff;
                            if (g8(dd(0x1e0)) & 1) == 0
                                || g8(dd(0x237)) == 0
                                || g8(dd(0x233)) == 0
                                || g8(ad(a_0071b741, iv5)) != 0
                            {
                                if DAT_0071ecf4 == iv10 {
                                    DAT_0070a1d0 = 0xffff;
                                    if (g8(dd(0x1de)) & 1) != 0 {
                                        FUN_0040a980(0, iv10);
                                    }
                                }
                            } else {
                                if g32(ad(a_0071bce1, iv5)) == 0 {
                                    p32(ad(a_0071bce1, iv5), 0x3b);
                                    s14 = (g16(dd(0x235)) as i16).wrapping_add(-0x7d);
                                    if s14 < 0 {
                                        s14 = 0;
                                    }
                                    p16(dd(0x235), s14 as u16);
                                    if s14 == 0 && g8(dd(0x237)) != 0 {
                                        p8(dd(0x237), 0);
                                        p16(dd(0x238), 0);
                                        if DAT_0071ecf4 == iv10 {
                                            FUN_004a1d70(
                                                (g8(dd(0x233)) as u32).wrapping_add(0xf1),
                                                DAT_0071ecf4,
                                            );
                                        }
                                    }
                                }
                                u7 = g16(ad(a_0071b74a, iv5));
                                u9 = ((u20 as u32) << 16) | (u7 as u32);
                                if u7 != 0xffff && ((u7 as i32) < DAT_006d4980) {
                                    iv16 = g32(ad(
                                        ad((u7 as u32).wrapping_mul(0x23) as i32, 0x1f),
                                        DAT_006d4988,
                                    )) as i32;
                                    iv5 = g32(ad(iv16, 0xa7)) as i32;
                                    if g8(ad(iv5, 7)) == 0 {
                                        let u23 = FUN_00441fc0(iv16 as u32, iv5, u9, 0, iv10);
                                        if (u23 as i16) != 0 && DAT_006d499c < 0x20000 {
                                            u18 = u7;
                                        }
                                    }
                                }
                                if u18 != 0xffff {
                                    iv16 = g32(ad(
                                        g32(ad(
                                            ad((u18 as u32).wrapping_mul(0x23) as i32, 0x1f),
                                            DAT_006d4988,
                                        )) as i32,
                                        0xa7,
                                    )) as i32;
                                    b2 = g8(dd(0x233));
                                    if b2 == 1 || b2 == 2 {
                                        if gs8(ad(iv16, 0x23c)) == 1 && g16(ad(iv16, 0x23e)) != 0 {
                                            if DAT_0071ecf4 == iv10 {
                                                s14 = FUN_004a31a0();
                                                if s14 != 0xff {
                                                    FUN_004a1d70(0xff, iv10);
                                                }
                                            }
                                            u18 = 0xffff;
                                        } else {
                                            u9 = g16(dd(0x238)) as u32;
                                            let ea = ad(
                                                ad(iv16, 0x15),
                                                (b2 as u32).wrapping_mul(4) as i32,
                                            );
                                            p32(ea, g32(ea).wrapping_add(u9));
                                        }
                                    }
                                }
                                u20 = (u9 >> 16) as u16;
                                if DAT_0071ecf4 == iv10 {
                                    DAT_0070a1d0 = u18;
                                    FUN_0040a980(1, iv10);
                                }
                            }
                            s14 = 0x14;
                            unaff_edi = ((u20 as u32) << 16) | (DAT_00732a3c as u32);
                            if (DAT_00732a3c as i32) < DAT_006d4980 {
                                loop {
                                    iv16 = ((unaff_edi & 0xffff).wrapping_mul(0x23) as i32)
                                        .wrapping_add(DAT_006d4988);
                                    if g8(ad(iv16, 3)) != 0 && gs8(ad(iv16, 2)) == 4 {
                                        let b6 = FUN_00444040(
                                            ((ustack8 as u32) << 16) | (local_a as u32),
                                            g8(ad(g32(ad(iv16, 0x1f)) as i32, 0x87)) as u32,
                                        );
                                        if b6 != 0 {
                                            FUN_0041b920(u12, unaff_edi & 0xffff);
                                        }
                                    }
                                    u18 = (unaff_edi as u16).wrapping_add(1);
                                    unaff_edi = (((unaff_edi >> 16) & 0xffff) << 16) | (u18 as u32);
                                    if !((u18 as i32) < DAT_006d4980) {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    if b22 {
                        DAT_00732e54 =
                            g32(ad(g32(ad(ad(DAT_006d4988, iv8), 0x1f)) as i32, 0xa7)) as i32;
                        if gs8(ad(ad(DAT_006d4988, iv8), 3)) == 2 {
                            s14 = s14.wrapping_mul(2);
                        }
                        let mut go = false;
                        if (g8(dd(0x1df)) & 1) != 0 && s14 != 0 {
                            s14 = ((g8(dd(0x20f)) as i32)
                                .wrapping_sub(2)
                                .wrapping_mul(s14 as i32)) as i16;
                            if s14 != 0 {
                                go = true;
                            }
                        }
                        if go {
                            let base = (g8(dd(4)) as u32).wrapping_mul(0xef) as i32;
                            if g8(dd(0x210)) == 0 {
                                s15 = (g16(ad(a_0052b083, base)) as i16).wrapping_mul(2);
                                p16(dd(0x20b), (g16(dd(0x20b)) as i16).wrapping_add(s14) as u16);
                                if (g16(dd(0x20b)) as i16) < 0 {
                                    p16(dd(0x20b), 0);
                                }
                                ps11 = dd(0x20b);
                                if s15 < (g16(ps11) as i16) {
                                    p16(ps11, s15 as u16);
                                }
                            } else if g8(dd(0x210)) == 2 {
                                s15 = (g16(ad(a_0052b083, base)) as i16).wrapping_mul(2);
                                p16(dd(0x20d), (g16(dd(0x20d)) as i16).wrapping_add(s14) as u16);
                                if (g16(dd(0x20d)) as i16) < 0 {
                                    p16(dd(0x20d), 0);
                                }
                                ps11 = dd(0x20d);
                                if s15 < (g16(ps11) as i16) {
                                    p16(ps11, s15 as u16);
                                }
                            } else {
                                s15 = g16(ad(a_0052b083, base)) as i16;
                                let half = ((s14 as i32) / 2) as i16;
                                p16(dd(0x20b), (g16(dd(0x20b)) as i16).wrapping_add(half) as u16);
                                if (g16(dd(0x20b)) as i16) < 0 {
                                    p16(dd(0x20b), 0);
                                }
                                if s15.wrapping_mul(2) < (g16(dd(0x20b)) as i16) {
                                    p16(dd(0x20b), s15.wrapping_mul(2) as u16);
                                }
                                s15 = (g16(ad(
                                    a_0052b083,
                                    (g8(dd(4)) as u32).wrapping_mul(0xef) as i32,
                                )) as i16)
                                    .wrapping_mul(2);
                                p16(dd(0x20d), (g16(dd(0x20d)) as i16).wrapping_add(half) as u16);
                                if (g16(dd(0x20d)) as i16) < 0 {
                                    p16(dd(0x20d), 0);
                                }
                                ps11 = dd(0x20d);
                                if s15 < (g16(ps11) as i16) {
                                    p16(ps11, s15 as u16);
                                }
                            }
                        }
                        if (g8(dd(0x1df)) & 0x10) != 0 {
                            u12 = 0;
                            if g8(dd(0x213)) != 0 {
                                loop {
                                    iv16 = u12.wrapping_mul(4) as i32;
                                    c1 = gs8(dd(ad(0x344, iv16)));
                                    if c1 != 0 && c1 != 2 {
                                        s14 = (g8(dd(0x212)) as i32).wrapping_sub(2) as i16;
                                        if g16(dd(0xbf)) == 0 {
                                            s14 = (g8(dd(0x212)) as i32).wrapping_sub(6) as i16;
                                        }
                                        c1 = gs8(ad(ad(DAT_006d4988, 3), iv8));
                                        if c1 == 5 || c1 == 7 {
                                            s14 = s14.wrapping_mul(3);
                                        } else {
                                            s14 = s14.wrapping_mul(2);
                                        }
                                        p8(
                                            dd(ad(iv16, 0x345)),
                                            gs8(dd(ad(iv16, 0x345))).wrapping_add(s14 as i8) as u8,
                                        );
                                        if s14 < 0 {
                                            pc4 = dd(ad(0x345, iv16));
                                            if gs8(pc4) < 0 {
                                                p8(pc4, 0);
                                            }
                                        }
                                        if 0 < s14 && gs8(dd(ad(0x345, iv16))) < 0 {
                                            p8(dd(ad(0x345, iv16)), 0x7f);
                                        }
                                    }
                                    u12 = (u12 as u16).wrapping_add(1) as u32;
                                    if !(u12 < g8(dd(0x213)) as u32) {
                                        break;
                                    }
                                }
                            }
                        }
                        unaff_edi = dd(0xbf) as u32;
                        if g16(unaff_edi as i32) == 0 {
                            let mut b6 = false;
                            u12 = 0;
                            if g8(dd(0x213)) != 0 {
                                loop {
                                    if 0 < gs8(dd(ad(0x345, u12.wrapping_mul(4) as i32))) {
                                        b6 = true;
                                    }
                                    u12 = (u12 as u16).wrapping_add(1) as u32;
                                    if !(u12 < g8(dd(0x213)) as u32) {
                                        break;
                                    }
                                }
                            }
                            if !b6 {
                                p16(unaff_edi as i32, 0xffff);
                                FUN_004a1d70(0x113, DAT_0071ecf4);
                                FUN_0040a1a0(0x5b, -1, DAT_0071ecf4);
                            }
                        }
                        if (g8(dd(0x1e0)) & 1) != 0 {
                            s14 = ((g8(dd(0x234)) as i32)
                                .wrapping_sub(2)
                                .wrapping_mul(0x7d)
                                .wrapping_add((g16(dd(0x235)) as i16) as i32))
                                as i16;
                            if s14 < 0 {
                                s14 = 0;
                            }
                            if 9999 < s14 {
                                s14 = 9999;
                            }
                            p16(dd(0x235), s14 as u16);
                            if s14 == 0 && g8(dd(0x237)) != 0 {
                                p8(dd(0x237), 0);
                                p16(dd(0x238), 0);
                                if g32(ad(ad(DAT_006d4988, 0x1b), iv8)) as i32 == DAT_0071ecf4 {
                                    FUN_004a1d70(
                                        (g8(dd(0x233)) as u32).wrapping_add(0xf1),
                                        DAT_0071ecf4,
                                    );
                                }
                            }
                        }
                        if g16(dd(0x23e)) != 0 {
                            p16(dd(0x23e), g16(dd(0x23e)).wrapping_sub(1));
                            if gs8(dd(0x23c)) == 1 && g16(dd(0x23e)) == 0 {
                                iv8 = g32(ad(ad(DAT_006d4988, 0x1b), iv8)) as i32;
                                if iv8 != -1 {
                                    FUN_004a1d70(0x166, iv8);
                                }
                            }
                        }
                    }
                }
                local_a = local_a.wrapping_add(1);
                if !((local_a as i32) < DAT_006d4980) {
                    break;
                }
            }
        }
        local_a = DAT_00732a3c;
        if (local_a as i32) < DAT_006d4980 {
            loop {
                iv8 = (local_a as u32).wrapping_mul(0x23) as i32;
                iv16 = DAT_006d4988.wrapping_add(iv8);
                let mut ok = false;
                if g8(ad(iv16, 3)) != 0 {
                    pc4 = g32(ad(iv16, 0x1f)) as i32;
                    if g8(pc4) == 0 {
                        DAT_00732e54 = g32(ad(pc4, 0xa7)) as i32;
                        if g16(dd(0x1e1)) == 0 {
                            ok = true;
                        }
                    }
                }
                if ok {
                    if g32(dd(0x1d)) == 0 {
                        u12 = 0;
                        if g8(dd(0x211)) != 0 {
                            loop {
                                let o2 = u12.wrapping_mul(2) as i32;
                                let o4 = u12.wrapping_mul(4) as i32;
                                s14 = g16(dd(ad(0x21c, o2))) as i16;
                                if s14 != 0 {
                                    s14 = s14.wrapping_sub(DAT_0071e6be as i16);
                                    if s14 < 0 {
                                        s14 = 0;
                                    }
                                    p16(dd(ad(0x21c, o2)), s14 as u16);
                                }
                                if (g32(ad(ad(DAT_006d4988, 0x1b), iv8)) as i32 == -1)
                                    && s14 < (DAT_0071e6be as i16)
                                    && g8(dd(ad(0x216, u12 as i32))) != 0
                                {
                                    if (g8(dd(0x1df)) & 0x10) != 0 && g8(dd(7)) == 0 {
                                        FUN_00447240(local_a as u32, u12);
                                    }
                                    pc4 = dd(ad(0x218, u12 as i32));
                                    p8(pc4, gs8(pc4).wrapping_sub(1) as u8);
                                    ps11 = dd(ad(0x21c, o2));
                                    p16(ps11, g16(ps11).wrapping_add(DAT_0071e6be.wrapping_mul(2)));
                                    p32(
                                        dd(ad(0x220, o4)),
                                        g32(dd(ad(0x220, o4)))
                                            .wrapping_add((DAT_0071e6be as u32).wrapping_mul(2)),
                                    );
                                    if g8(dd(ad(u12 as i32, 0x218))) == 0 {
                                        p8(dd(ad(u12 as i32, 0x216)), 0);
                                    }
                                }
                                u12 = (u12 as u16).wrapping_add(1) as u32;
                                if !(u12 < g8(dd(0x211)) as u32) {
                                    break;
                                }
                            }
                        }
                    }
                    u12 = 0;
                    if g8(dd(0x213)) != 0 {
                        loop {
                            if gs8(dd(ad(0x344, u12.wrapping_mul(4) as i32))) == 2 {
                                let pv = FUN_0042b1e0(DAT_00732e54, u12) as usize as i32;
                                if g16(pv) != 0xffff {
                                    FUN_0041aba0(local_a, u12, g16(pv));
                                }
                            }
                            u12 = (u12 as u16).wrapping_add(1) as u32;
                            if !(u12 < g8(dd(0x213)) as u32) {
                                break;
                            }
                        }
                    }
                    u12 = 0;
                    if g8(dd(0x228)) != 0 {
                        loop {
                            ps11 = dd(ad(0x22d, u12.wrapping_mul(2) as i32));
                            s14 = g16(ps11) as i16;
                            if s14 != 0 {
                                s14 = s14.wrapping_sub(DAT_0071e6be as i16);
                                if s14 < 0 {
                                    s14 = 0;
                                }
                                p16(ps11, s14 as u16);
                            }
                            u12 = (u12 as u16).wrapping_add(1) as u32;
                            if !(u12 < g8(dd(0x228)) as u32) {
                                break;
                            }
                        }
                    }
                }
                local_a = local_a.wrapping_add(1);
                if !((local_a as i32) < DAT_006d4980) {
                    break;
                }
            }
        }
        u12 = DAT_00732a38 & 0xffff;
        if (u12 as i32) < ((DAT_00703c88 as u32).wrapping_add(DAT_00732a38) as i32) {
            loop {
                iv8 = (u12.wrapping_mul(0x23) as i32).wrapping_add(DAT_006d4988);
                if g8(ad(iv8, 3)) != 0 && gs8(ad(iv8, 2)) == 8 {
                    FUN_00413580(u12 as u16);
                }
                u18 = (u12 as u16).wrapping_add(1);
                u12 = u18 as u32;
                if !((u18 as i32) < ((DAT_00703c88 as u32).wrapping_add(DAT_00732a38) as i32)) {
                    break;
                }
            }
        }
    }
}
