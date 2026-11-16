// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: i32;
    static mut DAT_00525550: i32;
    static mut DAT_0071eae4: u16;
    static mut DAT_0071ecf0: u16;
    static mut DAT_0071e534: u16;
    static mut DAT_0071e6be: u16;
    static mut DAT_0070a1d2: u16;
    static mut _DAT_006d4860: u16;
    static mut DAT_00732568: i32;
    static mut DAT_00732530: i32;
    static mut DAT_00732548: i32;
    static mut DAT_0071b730: [u8; 0];
    static mut DAT_0052686c: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00405590();
    fn FUN_00478320(a: u32, b: u32) -> u32;
    fn FUN_004783e0(a: i32, b: i32) -> u32;
    fn FUN_00478290(a: i32, b: u32, c: i32) -> i32;
    fn FUN_00442e60(a: i32, b: i32, c: u32, d: i32);
    fn FUN_00436450(a: i32, b: i32, c: i32);
    fn FUN_00412810(a: i32, b: i32, c: i32) -> i32;
    fn FUN_00483180(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32);
    fn FUN_00413ca0(a: i32, b: i32) -> u64;
}

#[inline(always)]
unsafe fn r8(a: u32) -> u8 {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *const u8).read() }
}
#[inline(always)]
unsafe fn r16(a: u32) -> u16 {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *const u16).read_unaligned() }
}
#[inline(always)]
unsafe fn s16(a: u32) -> i16 {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { r16(a) as i16 }
}
#[inline(always)]
unsafe fn r32(a: u32) -> u32 {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *const u32).read_unaligned() }
}
#[inline(always)]
unsafe fn w8(a: u32, v: u8) {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *mut u8).write(v) }
}
#[inline(always)]
unsafe fn w16(a: u32, v: u16) {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *mut u16).write_unaligned(v) }
}
#[inline(always)]
unsafe fn w32(a: u32, v: u32) {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *mut u32).write_unaligned(v) }
}
#[inline(always)]
unsafe fn ent(i: u32, o: u32) -> u32 {
    // SAFETY: reads the game's entity table pointer.
    unsafe {
        i.wrapping_mul(0x23)
            .wrapping_add(o)
            .wrapping_add(DAT_006d4988 as u32)
    }
}
#[inline(always)]
unsafe fn chain(base: u32) {
    // SAFETY: the game's own entity memory, accessed as the original does.
    unsafe {
        let mut c = r32(ent(r32(base), 0x1f));
        w8(c.wrapping_add(0x90), 1);
        c = r32(ent(r32(base), 0x1f));
        w8(c.wrapping_add(0x89), r8(c.wrapping_add(0x90)));
    }
}
#[inline(always)]
unsafe fn clamp(a: u32) {
    // SAFETY: the address is inside the game's own memory.
    unsafe {
        if (r32(a) as i32) < -0x1000000 {
            w32(a, 0xff000000);
        }
        if 0x1000000 < (r32(a) as i32) {
            w32(a, 0x1000000);
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0043fe30(param_1: i32) {
    // SAFETY: the callees, globals and the game's object memory are the
    // game's own; accesses follow the original's order and offsets.
    unsafe {
        let mut local_4: u32 = 0;
        FUN_00405590();
        DAT_0071eae4 = DAT_0071eae4.wrapping_shl(1);
        let iv2 = (param_1 as u32).wrapping_mul(0x5bd);
        let base = ((&raw mut DAT_0071b730).cast::<u8>() as usize as u32).wrapping_add(iv2);
        let tbl = (&raw mut DAT_0052686c).cast::<u8>() as usize as u32;

        if s16(base + 0x410) == 0 && r8(base + 0x13) == 0 {
            if r8(base + 0x14) != 0 {
                return;
            }
            let iv6 = r32(ent(r32(base), 0x1f));
            let iv14 = r32(iv6.wrapping_add(0xa7));
            let mut u8_ = r16(iv14.wrapping_add(0xbd));
            if r8(iv14.wrapping_add(0x1df)) & 0x40 == 0 {
                u8_ = 0;
            }
            let mut u12: u32 = ((iv6 >> 16) << 16) | u8_ as u32;
            if u8_ < 0x5555 {
                u8_ = (u8_ as u32).wrapping_mul(2).wrapping_add(0x5555) as u16;
            } else {
                u12 = u8_ as u32;
                let q = (0x5555u32.wrapping_sub(u12) as i32) / 2;
                u8_ = ((q as i16 as i32).wrapping_sub(1)) as u16;
            }
            u12 = (((u12 >> 8) << 8) | r8(iv14.wrapping_add(0x212)) as u32) & 0xffff00ff;
            let mut s9 = u12 as i16;
            if r8(iv14.wrapping_add(0x1dd)) & 1 == 0 {
                s9 = (s9 as i32).wrapping_mul(2) as i16;
            } else {
                s9 = (s9 as i32).wrapping_add(r8(iv14.wrapping_add(0x20f)) as i32) as i16;
            }
            s9 = 4i32.wrapping_sub(s9 as i32) as i16;
            let u15: u16 = if s9 < 0 {
                (s9 as i32).wrapping_mul(-0xc00) as u16
            } else {
                (s9 as i32).wrapping_mul(0xc00) as u16
            };
            u12 = FUN_00478320(
                ((u12 >> 16) << 16) | r16(iv14.wrapping_add(0xa0)) as u32,
                u8_ as u32,
            );
            let mut u4: u32 = (((param_1 as u32) >> 16) << 16) | (u12 as u16 as u32);
            let mut s17: i16;
            if s9 < 1 {
                u4 = FUN_00478320(u4, u15 as u32);
                s17 = (u4 as i16).wrapping_neg();
            } else {
                u4 = FUN_00478320(u4, u15 as u32);
                s17 = u4 as i16;
            }
            let u16_ = (u12 as u16).wrapping_add(s17 as u16);
            let mut u5 = FUN_004783e0((u16_ % 0x3800) as i32, 0x3800);
            let mut u10 = DAT_0071ecf0;
            if 0x7fff < DAT_0071ecf0 {
                u10 = DAT_0071ecf0.wrapping_neg();
            }
            u12 = FUN_00478320(u10 as u32, u5 as u16 as u32);
            s17 = (((u16_ / 0x3800) as i32)
                .wrapping_mul(u10 as i32)
                .wrapping_add(u12 as i16 as i32)) as i16;
            if 0x7fff < DAT_0071ecf0 {
                s17 = s17.wrapping_neg();
            }
            _DAT_006d4860 = DAT_0071ecf0;
            if 0x7fff < DAT_0071ecf0 {
                _DAT_006d4860 = DAT_0071ecf0.wrapping_neg();
            }
            u12 = FUN_00478320(r16(iv14.wrapping_add(0x98)) as u32, u8_ as u32);
            local_4 = (local_4 & 0xffff0000) | (u12 as u16 as u32);
            if s9 < 1 {
                u4 = FUN_00478320(local_4, u15 as u32);
                s9 = (u4 as i16).wrapping_neg();
            } else {
                u4 = FUN_00478320(local_4, u15 as u32);
                s9 = u4 as i16;
            }
            u8_ = (u12 as u16).wrapping_add(s9 as u16);
            u5 = FUN_004783e0((u8_ % 0x1400) as i32, 0x1400);
            let mut u15b = DAT_0071eae4;
            if 0x7fff < DAT_0071eae4 {
                u15b = DAT_0071eae4.wrapping_neg();
            }
            u12 = FUN_00478320(u15b as u32, u5 as u16 as u32);
            s9 = ((u12 as i16 as i32)
                .wrapping_add(((u8_ / 0x1400) as i32).wrapping_mul(u15b as i32)))
                as i16;
            if 0x7fff < DAT_0071eae4 {
                s9 = s9.wrapping_neg();
            }
            if (r8(iv14.wrapping_add(0x1df)) & 0x20) == 0
                || (r32(iv14.wrapping_add(0x19)) != 0 && s16(iv14.wrapping_add(0x23e)) == 0)
            {
                s17 = 0;
                s9 = 0;
            }
            let u8v: u16 = ((DAT_0071e534 as u8 & 0xe) == 2) as u16;
            if (s16(base + 0x4b) as i32 as u32) == u8v as u32 {
                let s11 = s16(base + 0x4d);
                let s18 = (s17 as u16).wrapping_sub(s11 as u16) as i16;
                if s18 != 0 {
                    let mut s3 = s18;
                    if s17 < s11 {
                        s3 = s18.wrapping_neg();
                    }
                    if s3 < 8 {
                        w16(base + 0x4d, s11.wrapping_add(s18) as u16);
                    } else {
                        if 4 < DAT_0070a1d2 {
                            local_4 = DAT_0070a1d2 as u32;
                            s3 = ((s3 as i32) / (local_4 as i32)) as i16;
                            if s3 == 0 {
                                s3 = 1;
                            }
                            s3 = s3.wrapping_shl(2);
                        }
                        if s18 < 0 {
                            w16(base + 0x4d, s11.wrapping_sub(s3) as u16);
                        } else {
                            w16(base + 0x4d, s3.wrapping_add(s11) as u16);
                        }
                    }
                }
                let s17b = s16(base + 0x4f);
                let s11b = (s9 as u16).wrapping_sub(s17b as u16) as i16;
                if s11b != 0 {
                    let mut s18b = s11b;
                    if s9 < s17b {
                        s18b = s11b.wrapping_neg();
                    }
                    if s18b < 8 {
                        w16(base + 0x4f, s17b.wrapping_add(s11b) as u16);
                    } else {
                        if 4 < DAT_0070a1d2 {
                            s18b = ((s18b as i32) / (DAT_0070a1d2 as i32)) as i16;
                            if s18b == 0 {
                                s18b = 1;
                            }
                            s18b = s18b.wrapping_shl(2);
                        }
                        if s11b < 0 {
                            w16(base + 0x4f, s17b.wrapping_sub(s18b) as u16);
                        } else {
                            w16(base + 0x4f, s18b.wrapping_add(s17b) as u16);
                        }
                    }
                }
            } else {
                w16(base + 0x4d, 0);
                w16(base + 0x4f, 0);
            }
            w16(base + 0x4b, u8v);
            let mut i6 = FUN_00478290(s16(base + 0x4d) as i32, DAT_0071e6be as u32, 0xec);
            s17 = i6 as i16;
            i6 = FUN_00478290(s16(base + 0x4f) as i32, DAT_0071e6be as u32, 0xec);
            s9 = i6 as i16;
            if (r8(iv14.wrapping_add(0x1df)) & 0x20) == 0
                || (r32(iv14.wrapping_add(0x19)) != 0 && s16(iv14.wrapping_add(0x23e)) == 0)
            {
                s9 = 0;
                s17 = 0;
            }
            if u8v != 0 {
                if s9 != 0 {
                    FUN_00442e60(s9 as i32, 0, r32(base), iv14 as i32);
                    chain(base);
                }
                if s17 == 0 {
                    return;
                }
                let ps = ent(r32(base), 0x14);
                w16(
                    ps,
                    (r16(ps) as i16 as i32).wrapping_add((s17 as i32).wrapping_mul(-2)) as u16,
                );
                chain(base);
                return;
            }
            if s9 == 0 && s17 == 0 {
                return;
            }
            FUN_00442e60(
                s9 as i32,
                (s17 as i32).wrapping_neg(),
                r32(base),
                iv14 as i32,
            );
            chain(base);
            if s17 == 0 {
                return;
            }
            let ps = ent(r32(base), 0x14);
            w16(ps, (r16(ps) as i16).wrapping_sub(s17) as u16);
            return;
        }

        let mut b7 = r8(base + 0x13);
        if b7 != 0 {
            if b7 & 0x80 == 0 {
                let mut u8_ = (b7 & 0x7f) as u16;
                if (u8_ != 1 && (b7 & 0x7f) != 0) && DAT_00525550 == 0 {
                    if u8_ <= DAT_0071e6be {
                        u8_ = DAT_0071e6be.wrapping_add(1);
                    }
                    b7 = (u8_ as u8).wrapping_sub(DAT_0071e6be as u8);
                    w8(base + 0x13, b7);
                }
            } else {
                let mut u8_ = (b7 & 0x7f) as u16;
                if u8_ < 0x7f {
                    if DAT_00525550 == 0 {
                        if 0x7f < (DAT_0071e6be as i32).wrapping_add(u8_ as i16 as i32) {
                            u8_ = 0x7fi32.wrapping_sub(DAT_0071e6be as i32) as u16;
                        }
                        b7 = (u8_ as u8).wrapping_add(DAT_0071e6be as u8) | 0x80;
                        w8(base + 0x13, b7);
                    }
                } else {
                    if r16(base + 0x3f7) != 0xffff {
                        w32(base + 0x3eb, r32(ent(r16(base + 0x3f7) as u32, 4)));
                        w32(base + 0x3ef, r32(ent(r16(base + 0x3f7) as u32, 8)));
                        let iv6 = r32(ent(r16(base + 0x3f7) as u32, 0xc));
                        w32(base + 0x3f3, iv6);
                        w32(base + 0x3f3, r32(base + 0x416).wrapping_add(iv6));
                    }
                    w16(base + 0x3f7, 0xffff);
                }
            }
            if r8(base + 0x13) != 0 {
                let mut u8_ = DAT_0071ecf0;
                if (DAT_0071ecf0 as i16) < 0 {
                    u8_ = DAT_0071ecf0.wrapping_neg();
                }
                let mut u15 = DAT_0071eae4;
                if (DAT_0071eae4 as i16) < 0 {
                    u15 = DAT_0071eae4.wrapping_neg();
                }
                if (u8_ as i16) < 0x80 {
                    DAT_0071ecf0 = 0;
                }
                if (u15 as i16) < 0x30 {
                    DAT_0071eae4 = 0;
                }
            }
        }
        let mut i6 = FUN_00478290(DAT_0071ecf0 as i16 as i32, DAT_0071e6be as u32, 0xec);
        let mut s9 = i6 as i16;
        i6 = FUN_00478290(DAT_0071eae4 as i16 as i32, DAT_0071e6be as u32, 0xec);
        let mut s17 = i6 as i16;
        if r8(base + 0x13) & 0x80 == 0 {
            if s16(base + 0x3f7) == -1 {
                if s17 != 0 || s9 != 0 {
                    FUN_00436450(s17 as i32, (s9 as i32).wrapping_neg(), param_1);
                }
            } else {
                w16(base + 0x405, s16(base + 0x405).wrapping_add(s9) as u16);
                w16(base + 0x403, s16(base + 0x403).wrapping_add(s17) as u16);
            }
        } else {
            let m = ((r32(base + 0x3f3) as i32) >> 0xe).wrapping_add(1) as u32;
            w32(
                base + 0x3eb,
                r32(base + 0x3eb).wrapping_add(m.wrapping_mul(s9 as i32 as u32)),
            );
            let m = ((r32(base + 0x3f3) as i32) >> 0xe).wrapping_add(1) as u32;
            w32(
                base + 0x3ef,
                r32(base + 0x3ef).wrapping_add(m.wrapping_mul(s17 as i32 as u32)),
            );
        }
        let u8_ = DAT_0071e534 & 0xf;
        if u8_ != 1 && u8_ != 2 {
            if r8(base + 0x13) == 0 {
                w16(base + 0x412, 0x20);
                return;
            }
            let v = r16(base + 0x412);
            if v < 0x101 {
                w16(base + 0x412, 0x20);
                return;
            }
            w16(base + 0x412, ((v as i32) - ((v >> 3) as i32) - 0x20) as u16);
            return;
        }
        let p412 = base + 0x412;
        'lab: {
            if 1 < r8(base + 0x13) {
                if ((r32(base + 0x3f3) & 0xfffffffe) as i32) < 0xe001 {
                    w16(p412, ((r32(base + 0x3f3) as i32) >> 1) as u16);
                } else {
                    w16(p412, 0x7000);
                }
                if r16(p412) < 0x800 {
                    w16(p412, 0x800);
                }
                break 'lab;
            }
            if r8(base + 0x13) == 1 {
                if s16(base + 0x3f7) != -1 {
                    if ((r32(base + 0x416) & 0xfffffffe) as i32) < 0xe001 {
                        w16(p412, ((r32(base + 0x416) as i32) >> 1) as u16);
                    } else {
                        w16(p412, 0x7000);
                    }
                    if r16(p412) < 0x100 {
                        w16(p412, 0x100);
                    }
                    break 'lab;
                }
                if r16(base + 0x3f9) == 0xffff {
                    let mut u15 = (s16(base + 0x412) as i32).wrapping_add(0x20) as u16;
                    w16(p412, u15);
                    u15 = u15.wrapping_add(u15 >> 3);
                    w16(p412, u15);
                    if 0x4000 < u15 {
                        w16(p412, 0x4000);
                    }
                    break 'lab;
                }
                let pe = ent(r16(base + 0x3f9) as u32, 0);
                let iv6 = FUN_00412810(
                    r32(pe + 4).wrapping_sub(r32(base + 0x3eb)) as i32,
                    r32(pe + 8).wrapping_sub(r32(base + 0x3ef)) as i32,
                    r32(pe + 0xc).wrapping_sub(r32(base + 0x3f3)) as i32,
                );
                if iv6 < 0x7001 {
                    w16(p412, iv6 as u16);
                } else {
                    w16(p412, 0x7000);
                }
                if 0x3ff < r16(p412) {
                    break 'lab;
                }
            } else {
                let s = s16(base + 0x412);
                w16(base + 0x412, (s as i32 as u32).wrapping_add(0x20) as u16);
                if ((s as i32 as u32).wrapping_add(0x20) as u16) < 0x401 {
                    break 'lab;
                }
            }
            w16(base + 0x412, 0x400);
        }
        let ps = base + 0x412;
        if u8_ == 1 {
            if r8(base + 0x13) == 0 {
                let i = FUN_00478290(s16(ps) as i32, DAT_0071e6be as u32, 0xec);
                let v = r32(base + 0x416).wrapping_sub(i as i16 as i32 as u32) as i32;
                w32(base + 0x416, v as u32);
                if v < 0x30 {
                    w32(base + 0x416, 0x30);
                    return;
                }
            } else if s16(base + 0x3f7) == -1 {
                let (s11, s18, s9b, s17b);
                if s16(base + 0x3f9) == -1 {
                    s11 = s16(base + 0x405);
                    s18 = s16(base + 0x403);
                    s9b = s16(base + 0x3fd);
                    s17b = s16(base + 0x3fb);
                } else {
                    s9b = s16(base + 0x3fd);
                    s11 = 0;
                    s17b = s16(base + 0x3fb);
                    s18 = 0;
                }
                FUN_00483180(0, s17b as i32, s9b as i32, 0, s18 as i32, s11 as i32, 0);
                let mut i = FUN_00478290(s16(ps) as i32, DAT_0071e6be as u32, 0xec);
                let mut u20 = FUN_00413ca0(i as i16 as i32, DAT_00732568);
                let mut u12 = DAT_0071e6be as u32;
                w32(base + 0x3eb, r32(base + 0x3eb).wrapping_add(u20 as u32));
                i = FUN_00478290(s16(ps) as i32, u12, 0xec);
                u20 = FUN_00413ca0(i as i16 as i32, DAT_00732530);
                u12 = DAT_0071e6be as u32;
                w32(base + 0x3ef, r32(base + 0x3ef).wrapping_add(u20 as u32));
                i = FUN_00478290(s16(ps) as i32, u12, 0xec);
                u20 = FUN_00413ca0(i as i16 as i32, DAT_00732548);
                w32(base + 0x3f3, r32(base + 0x3f3).wrapping_add(u20 as u32));
                if r16(base + 0x3f9) != 0xffff {
                    let pe = ent(r16(base + 0x3f9) as u32, 0);
                    let mut iv6 = FUN_00412810(
                        r32(pe + 4).wrapping_sub(r32(base + 0x3eb)) as i32,
                        r32(pe + 8).wrapping_sub(r32(base + 0x3ef)) as i32,
                        r32(pe + 0xc).wrapping_sub(r32(base + 0x3f3)) as i32,
                    );
                    let t = ent(r16(base + 0x3f9) as u32, 3);
                    if iv6
                        < (r16(tbl.wrapping_add((r8(t) as u32).wrapping_mul(0x16))) as i32 + 0x200)
                    {
                        let u12b = r8(ent(r16(base + 0x3f7) as u32, 3)) as u32;
                        iv6 = (iv6 as u32).wrapping_sub(
                            (r16(tbl.wrapping_add(u12b.wrapping_mul(0x16))) as u32)
                                .wrapping_add(0x200),
                        ) as i32;
                        let mut u = FUN_00413ca0(iv6, DAT_00732568);
                        w32(base + 0x3eb, r32(base + 0x3eb).wrapping_add(u as u32));
                        u = FUN_00413ca0(iv6, DAT_00732530);
                        w32(base + 0x3ef, r32(base + 0x3ef).wrapping_add(u as u32));
                        u = FUN_00413ca0(iv6, DAT_00732548);
                        w32(base + 0x3f3, r32(base + 0x3f3).wrapping_add(u as u32));
                    }
                }
                if 1 < r8(base + 0x13) && (r32(base + 0x3f3) as i32) < 0x30 {
                    w32(base + 0x3f3, 0x30);
                }
                clamp(base + 0x3eb);
                clamp(base + 0x3ef);
                clamp(base + 0x3f3);
                return;
            } else {
                let i = FUN_00478290(s16(ps) as i32, DAT_0071e6be as u32, 0xec);
                let v = r32(base + 0x416).wrapping_sub(i as i16 as i32 as u32) as i32;
                w32(base + 0x416, v as u32);
                let t = ent(r16(base + 0x3f7) as u32, 3);
                if v < (r16(tbl.wrapping_add((r8(t) as u32).wrapping_mul(0x16))) as i32 + 0x200) {
                    let t = ent(r16(base + 0x3f7) as u32, 3);
                    w32(
                        base + 0x416,
                        (r16(tbl.wrapping_add((r8(t) as u32).wrapping_mul(0x16))) as u32)
                            .wrapping_add(0x200),
                    );
                    return;
                }
            }
        } else if r8(base + 0x13) == 0 {
            let i = FUN_00478290(s16(ps) as i32, DAT_0071e6be as u32, 0xec);
            let v = (i as i16 as i32 as u32).wrapping_add(r32(base + 0x416)) as i32;
            w32(base + 0x416, v as u32);
            if 0x1400 < v {
                w32(base + 0x416, 0x1400);
                return;
            }
        } else {
            if s16(base + 0x3f7) != -1 {
                let i = FUN_00478290(s16(ps) as i32, DAT_0071e6be as u32, 0xec);
                w32(
                    base + 0x416,
                    r32(base + 0x416).wrapping_add(i as i16 as i32 as u32),
                );
                return;
            }
            let (s11, s18, s9b, s17b);
            if s16(base + 0x3f9) == -1 {
                s11 = s16(base + 0x405);
                s18 = s16(base + 0x403);
                s9b = s16(base + 0x3fd);
                s17b = s16(base + 0x3fb);
            } else {
                s9b = s16(base + 0x3fd);
                s11 = 0;
                s17b = s16(base + 0x3fb);
                s18 = 0;
            }
            FUN_00483180(0, s17b as i32, s9b as i32, 0, s18 as i32, s11 as i32, 0);
            let mut i = FUN_00478290(s16(ps) as i32, DAT_0071e6be as u32, 0xec);
            let mut u20 = FUN_00413ca0(i as i16 as i32, DAT_00732568);
            let mut u12 = DAT_0071e6be as u32;
            w32(base + 0x3eb, r32(base + 0x3eb).wrapping_sub(u20 as u32));
            i = FUN_00478290(s16(ps) as i32, u12, 0xec);
            u20 = FUN_00413ca0(i as i16 as i32, DAT_00732530);
            u12 = DAT_0071e6be as u32;
            w32(base + 0x3ef, r32(base + 0x3ef).wrapping_sub(u20 as u32));
            i = FUN_00478290(s16(ps) as i32, u12, 0xec);
            u20 = FUN_00413ca0(i as i16 as i32, DAT_00732548);
            w32(base + 0x3f3, r32(base + 0x3f3).wrapping_sub(u20 as u32));
            clamp(base + 0x3eb);
            clamp(base + 0x3ef);
            clamp(base + 0x3f3);
        }
    }
}
