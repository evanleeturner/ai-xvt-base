// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1fe8: i32;
    static mut _DAT_00a07cec: i32;
    static mut DAT_009ecc34: i32;

    static mut DAT_009d9192: u8;
    static mut DAT_0051c5a6: u8;
    static mut DAT_0051c5a0: u8;
    static mut DAT_0051c573: u8;
    static mut DAT_0051c635: u8;
    static mut DAT_0051c633: u8;
    static mut DAT_0051a1d0: u8;
    static mut DAT_009e96b0: u8;
    static mut DAT_009e96b1: u8;
    static mut DAT_009e96b2: u8;
    static mut DAT_009e96b3: u8;
    static mut DAT_009e96b4: u8;
    static mut DAT_009e96b5: u8;
    static mut DAT_009e96ad: u8;
    static mut DAT_009e96ae: u8;
    static mut DAT_009e96af: u8;
    static mut DAT_009e96ab: u8;
    static mut DAT_009e9670: u8;
    static mut DAT_009e9674: u8;
    static mut DAT_009e9681: u8;
    static mut DAT_009e9684: u8;
    static mut DAT_009e9689: u8;
    static mut DAT_009e968a: u8;
    static mut DAT_009e968c: u8;
    static mut DAT_009e968e: u8;
    static mut DAT_009e9692: u8;
    static mut DAT_009e9696: u8;
    static mut DAT_009e9697: u8;
    static mut DAT_009e9698: u8;
    static mut DAT_009e9699: u8;
    static mut DAT_009e969b: u8;
    static mut DAT_009e969d: u8;
    static mut DAT_009e9c1d: u8;
    static mut DAT_009e9c21: u8;
    static mut DAT_009e96bb: u8;
    static mut DAT_009e96bd: u8;
    static mut DAT_009e96bf: u8;
    static mut DAT_009e96c1: u8;
    static mut DAT_009e96c3: u8;
    static mut DAT_009e96c5: u8;
    static mut DAT_009e96c9: u8;
    static mut DAT_009e96cd: u8;
    static mut DAT_009e96d1: u8;
    static mut DAT_009e96d5: u8;
    static mut DAT_009e96d9: u8;
    static mut DAT_009e9a79: u8;
    static mut DAT_009e9a73: u8;
    static mut DAT_009e9a75: u8;
    static mut DAT_009e9a84: u8;
    static mut DAT_009e9a86: u8;
    static mut DAT_009e9a80: u8;
    static mut DAT_009e9a8a: u8;
    static mut DAT_009e9a67: u8;
    static mut DAT_009d8b80: u8;
    static mut DAT_009d8b84: u8;
    static mut DAT_009d8b88: u8;
    static mut DAT_009d8b8c: u8;
    static mut DAT_009d8a54: u32;
    static mut DAT_009d8a58: u32;
    static mut DAT_009d8b60: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0041bb00(param_1: u16);
    fn FUN_004269e0(param_1: u32) -> u32;
    fn FUN_00485270(param_1: i32);
    fn FUN_0046b610(param_1: *mut u8) -> i32;
    fn FUN_004035d0(param_1: i32, param_2: i32, param_3: i16, param_4: i16);
    fn FUN_00440d60(param_1: u32, param_2: i32);
    fn FUN_004515d0();
    fn FUN_0042e4d0(param_1: i32, param_2: u32, param_3: i32);
    fn FUN_00462a90();
}

macro_rules! a {
    ($n:ident) => {
        (&raw mut $n).cast::<u8>()
    };
}

fn p(x: u32) -> *mut u8 {
    x as usize as *mut u8
}

fn g(base: *mut u8, off: i32) -> *mut u8 {
    base.wrapping_byte_add(off as u32 as usize)
}

unsafe fn r32(q: *const u8) -> u32 {
    // SAFETY: caller supplies a readable game address.
    unsafe { q.cast::<u32>().read_unaligned() }
}

unsafe fn r16(q: *const u8) -> u16 {
    // SAFETY: caller supplies a readable game address.
    unsafe { q.cast::<u16>().read_unaligned() }
}

unsafe fn w32(q: *mut u8, v: u32) {
    // SAFETY: caller supplies a writable game address.
    unsafe { q.cast::<u32>().write_unaligned(v) }
}

unsafe fn w16(q: *mut u8, v: u16) {
    // SAFETY: caller supplies a writable game address.
    unsafe { q.cast::<u16>().write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0045a1e0(
    param_1: i32,
    param_2: u32,
    param_3: u32,
    param_4: i32,
) -> u32 {
    // SAFETY: all globals, callees and the pointers derived from them are the
    // game's own memory, read and written exactly where the original does.
    unsafe {
        let mut b4 = false;
        let mut b5 = false;
        let mut local_c: i32 = 0;
        if param_3 != 0 {
            local_c = _DAT_00a07cec;
            if _DAT_00a07cec < DAT_009a1fe0 {
                let mut pc = p((_DAT_00a07cec as u32)
                    .wrapping_mul(0x23)
                    .wrapping_add(3)
                    .wrapping_add(DAT_009a1fe8 as u32));
                loop {
                    if *pc != 0 {
                        let i4 = r32(a!(DAT_009d9192)
                            .wrapping_byte_add((*pc.byte_add(0x13) as usize) * 0x566))
                            as i32;
                        if i4 == param_1 {
                            let mut u6 = r32(pc.byte_add(0x1c));
                            u6 = r32(p(u6).wrapping_byte_add(0xa9));
                            let c2 = *p(u6).wrapping_byte_add(7) as i8;
                            if c2 != 3 && c2 != 5 && c2 != 4 {
                                let l8 = r16(pc.wrapping_byte_sub(3));
                                if l8 as u32 == param_3 {
                                    b4 = true;
                                    b5 = true;
                                    break;
                                }
                            }
                        }
                    }
                    pc = pc.wrapping_byte_add(0x23);
                    local_c = local_c.wrapping_add(1);
                    if !(local_c < DAT_009a1fe0) {
                        break;
                    }
                }
            }
        }
        let mut u14: u32 = local_c as u32;
        if !b4 {
            u14 = param_2;
            let mut i11: i32 = DAT_009a1fe0.wrapping_sub(_DAT_00a07cec);
            loop {
                if i11 == 0 {
                    break;
                }
                u14 = u14.wrapping_add(1);
                if DAT_009a1fe0 <= u14 as i32 {
                    u14 = _DAT_00a07cec as u32;
                }
                let i13 = u14.wrapping_mul(0x23).wrapping_add(DAT_009a1fe8 as u32);
                if *p(i13).wrapping_byte_add(3) != 0 {
                    let i4 = r32(a!(DAT_009d9192)
                        .wrapping_byte_add((*p(i13).wrapping_byte_add(0x16) as usize) * 0x566))
                        as i32;
                    if i4 == param_1 {
                        let mut u6 = r32(p(i13).wrapping_byte_add(0x1f));
                        u6 = r32(p(u6).wrapping_byte_add(0xa9));
                        let c2 = *p(u6).wrapping_byte_add(7) as i8;
                        if c2 != 3 && c2 != 5 && c2 != 4 {
                            break;
                        }
                    }
                }
                i11 = i11.wrapping_sub(1);
            }
            if i11 == 0 {
                return 1;
            }
        }
        let i13: i32 = (u14.wrapping_mul(0x23)) as i32;
        let mut i1 = (DAT_009a1fe8 as u32).wrapping_add(i13 as u32);
        w32(p(i1).wrapping_byte_add(0x1b), param_1 as u32);
        let mut i2 = r32(p(i1).wrapping_byte_add(0x1f));
        *p(i2).wrapping_byte_add(0x92) = 1;
        i2 = r32(p(i1).wrapping_byte_add(0x1f));
        *p(i2).wrapping_byte_add(0x8b) = 1;
        let mut i12: i32 = 0;
        FUN_0041bb00(u14 as u16);
        i1 = (DAT_009a1fe8 as u32).wrapping_add(i13 as u32);
        i2 = r32(p(i1).wrapping_byte_add(0x1f));
        let mut i11: i32 = r32(p(i2).wrapping_byte_add(0xa9)) as i32;
        let b = p(i11 as u32);
        loop {
            *g(b, 0x220 + i12) = 1;
            *g(b, 0x222 + i12) = 0;
            *g(b, 0x224 + i12) = 0;
            w16(g(b, 0x226 + i12 * 2), 0);
            w32(g(b, 0x22a + i12 * 4), 0);
            let u8v = FUN_004269e0(
                *p((DAT_009a1fe8 as u32).wrapping_add(i13 as u32)).wrapping_byte_add(3) as u32,
            );
            if *g(b, 0x21e + i12) != 0 {
                let i9 = ((u8v & 0xffff).wrapping_mul(0x101)) as i32;
                if *g(a!(DAT_0051c5a6), i12.wrapping_add(i9)) != 2 {
                    *g(b, 0x224 + i12) = *g(a!(DAT_0051c5a0), i9.wrapping_add(i12));
                }
            }
            i12 += 1;
            if !(i12 < 2) {
                break;
            }
        }
        i12 = (param_1 as u32).wrapping_mul(0x5bd) as i32;
        *b.wrapping_byte_add(0x220) = *g(a!(DAT_009e96b1), i12);
        let mut i9: i32 = 0;
        *b.wrapping_byte_add(0x221) = *g(a!(DAT_009e96b2), i12);
        loop {
            let v = *g(b, 0x235 + i9);
            i9 += 1;
            *g(b, 0x234 + i9) = (v & 0x81) | 1;
            w16(g(b, 0x235 + i9 * 2), 0);
            if !(i9 < 2) {
                break;
            }
        }
        *b.wrapping_byte_add(0x235) =
            *g(a!(DAT_009e96b3), i12) | (*b.wrapping_byte_add(0x235) & 0x80);
        *b.wrapping_byte_add(0x236) =
            *g(a!(DAT_009e96b4), i12) | (*b.wrapping_byte_add(0x236) & 0x80);
        w16(b.wrapping_byte_add(0x23b), 0);
        let i3 = r32(a!(DAT_0051c573).wrapping_byte_add((*b.wrapping_byte_add(4) as usize) * 0x101))
            as i32;
        i9 = i3.wrapping_mul(2);
        let c2 = *g(a!(DAT_009e96b0), i12) as i8;
        *b.wrapping_byte_add(0x21a) = c2 as u8;
        if c2 == 0 {
            let i4 = r32(b.wrapping_byte_add(0x211)) as i32;
            if i9 < i4 {
                let i5 = i4.wrapping_sub(i3.wrapping_mul(2));
                w32(b.wrapping_byte_add(0x215), i5 as u32);
                w32(b.wrapping_byte_add(0x211), i9 as u32);
            }
        } else if c2 == 1 {
            let i4 = r32(b.wrapping_byte_add(0x211)) as i32;
            i9 = i4 >> 1;
            w32(b.wrapping_byte_add(0x211), i9 as u32);
            w32(b.wrapping_byte_add(0x215), i9 as u32);
        } else if c2 == 2 {
            let i4 = r32(b.wrapping_byte_add(0x211));
            w32(b.wrapping_byte_add(0x215), i4);
            w32(b.wrapping_byte_add(0x211), 0);
            let i4 = r32(b.wrapping_byte_add(0x215)) as i32;
            if i9 < i4 {
                let i5 = i4.wrapping_sub(i3.wrapping_mul(2));
                w32(b.wrapping_byte_add(0x211), i5 as u32);
                w32(b.wrapping_byte_add(0x215), i9 as u32);
            }
        }
        *b.wrapping_byte_add(0x219) = *g(a!(DAT_009e96ae), i12);
        *b.wrapping_byte_add(0x21c) = *g(a!(DAT_009e96ad), i12);
        *b.wrapping_byte_add(0x23e) = *g(a!(DAT_009e96af), i12);
        let i9b = DAT_009a1fe8;
        w32(g(a!(DAT_009e9670), i12), u14);
        let l8 = r16(p((i9b as u32).wrapping_add(i13 as u32)));
        w32(g(a!(DAT_009e9674), i12), l8 as u32);
        *g(a!(DAT_009e9681), i12) = 0;
        *g(a!(DAT_009e9684), i12) = 0;
        if b5 || param_2 != 0xffff_ffff {
            let l8 = r16(g(a!(DAT_009e96ab), i12));
            w16(b.wrapping_byte_add(0xbd), l8);
        }
        if !b5 {
            *g(a!(DAT_009e9697), i12) = 0;
            *g(a!(DAT_009e9698), i12) = 0;
        }
        w16(g(a!(DAT_009e968c), i12), 0xffff);
        w16(g(a!(DAT_009e969b), i12), 0xffff);
        w16(g(a!(DAT_009e9699), i12), 0);
        if param_4 == 1 {
            *g(a!(DAT_009e9689), i12) = 1;
            w16(g(a!(DAT_009e968a), i12), 0xffff);
            w32(g(a!(DAT_009e968e), i12), 0xffff_ffff);
            w32(g(a!(DAT_009e9692), i12), 0xffff_ffff);
        }
        if param_2 != 0xffff_ffff {
            let i4 = r32(p(param_2
                .wrapping_mul(0x23)
                .wrapping_add(0x1f)
                .wrapping_add(DAT_009a1fe8 as u32)));
            if i4 != 0 {
                w16(g(a!(DAT_009e968a), i12), 0xffff);
                let l8 = r16(b.wrapping_byte_add(0x42));
                if l8 < 0x8000 {
                    w16(g(a!(DAT_009e968a), i12), l8);
                    w16(g(a!(DAT_009e9699), i12), 0);
                }
            }
        }
        let l8 = r16(g(a!(DAT_009e968a), i12));
        if l8 as u32 == u14 {
            w16(g(a!(DAT_009e968a), i12), 0xffff);
        }
        FUN_00485270(param_1);
        *g(a!(DAT_009e9696), i12) = 0;
        let r = FUN_0046b610(a!(DAT_0051a1d0));
        *b.wrapping_byte_add(0x34) = r as u8;
        w16(b.wrapping_byte_add(0x42), 0xffff);
        w32(g(a!(DAT_009e9c1d), i12), 0);
        w32(g(a!(DAT_009e9c21), i12), 0);
        w16(g(a!(DAT_009e96bb), i12), 0);
        w16(g(a!(DAT_009e96bd), i12), 0);
        w16(g(a!(DAT_009e96bf), i12), 0);
        w16(g(a!(DAT_009e96c1), i12), 0);
        w16(g(a!(DAT_009e96c3), i12), 0);
        let i13b = (param_1 as u32).wrapping_mul(0x10) as i32;
        w16(g(a!(DAT_009e969d), i12), 0xffff);
        i11 = r32(g(a!(DAT_009e9670), i12)) as i32;
        w32(
            a!(DAT_009d8b80).wrapping_byte_add((param_1 as u32).wrapping_mul(8) as usize),
            0,
        );
        w32(g(a!(DAT_009d8b84), i13b), 0);
        w32(g(a!(DAT_009d8b88), i13b), 0);
        w32(g(a!(DAT_009d8b8c), i13b), 0);
        let u14b = FUN_004269e0(*p((i11 as u32)
            .wrapping_mul(0x23)
            .wrapping_add(3)
            .wrapping_add(DAT_009a1fe8 as u32)) as u32);
        i11 = ((u14b & 0xffff).wrapping_mul(0x101)) as i32;
        let s3 = r16(g(a!(DAT_0051c635), i11)) as i16;
        let s4 = r16(g(a!(DAT_0051c633), i11)) as i16;
        let i4 = r32(g(a!(DAT_009e9670), i12));
        FUN_004035d0(
            i4.wrapping_mul(0x23).wrapping_add(DAT_009a1fe8 as u32) as i32,
            0,
            s3,
            s4,
        );
        let u7 = DAT_009d8b60;
        let u6 = DAT_009d8a54;
        let u5 = DAT_009d8a58;
        w32(g(a!(DAT_009e96c5), i12), u5);
        w32(g(a!(DAT_009e96c9), i12), u6);
        w32(g(a!(DAT_009e96cd), i12), u7);
        let u5 = r32(g(a!(DAT_009e96c5), i12));
        w32(g(a!(DAT_009e96d1), i12), u5);
        let u5 = r32(g(a!(DAT_009e96c9), i12));
        w32(g(a!(DAT_009e96d5), i12), u5);
        let u5 = r32(g(a!(DAT_009e96cd), i12));
        w32(g(a!(DAT_009e96d9), i12), u5);
        *g(a!(DAT_009e9a79), i12) = 0;
        w16(g(a!(DAT_009e9a73), i12), 0);
        w16(g(a!(DAT_009e9a75), i12), 0);
        w16(g(a!(DAT_009e9a84), i12), 0);
        w32(g(a!(DAT_009e9a86), i12), 0x400);
        w16(g(a!(DAT_009e9a80), i12), 0);
        w16(g(a!(DAT_009e9a8a), i12), 0);
        let u6 = r32(g(a!(DAT_009e9670), i12));
        w16(g(a!(DAT_009e9a67), i12), u6 as u16);
        let u14c: u32 = if *g(a!(DAT_009e96b5), i12) == 0x13 {
            0x13
        } else {
            0
        };
        FUN_00440d60(u14c, param_1);
        if param_1 == DAT_009ecc34 {
            FUN_004515d0();
            FUN_0042e4d0(0xf, 0xffff_ffff, param_1);
        }
        FUN_00462a90();
        0
    }
}
