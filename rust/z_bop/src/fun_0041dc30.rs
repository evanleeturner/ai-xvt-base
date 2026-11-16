// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523410: i32;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1fe8: u32;
    static mut DAT_009d113c: u16;
    static mut DAT_009d1308: i32;
    static mut DAT_009d6940: [u8; 2];
    static mut DAT_009d6954: i8;
    static mut DAT_009ec5e6: u16;
    static mut DAT_009ecc34: i32;
    static mut DAT_009fe7d0: u8;
    static mut DAT_00a00860: u16;
    static mut DAT_00a07be0: u16;
    static mut DAT_00a07be2: u16;
    static mut DAT_00a07ce8: i32;
    static mut DAT_00a07cec: i32;
    static mut DAT_00a08294: u16;
    static mut DAT_00bb7375: i8;

    static mut DAT_009d8c80: [u8; 0];
    static mut DAT_009d8c84: [u8; 0];
    static mut DAT_009d8c88: [u8; 0];
    static mut DAT_009d8c89: [u8; 0];
    static mut DAT_009d9154: [u8; 0];
    static mut DAT_009d9155: [u8; 0];
    static mut DAT_009d8b82: [u16; 0];
    static mut DAT_009d8b84: [u8; 0];
    static mut DAT_009a20e6: [u8; 0];
    static mut DAT_009a2110: [u8; 0];
    static mut DAT_009e9670: [u8; 0];
    static mut DAT_009e967c: [u8; 0];
    static mut DAT_009e967e: [u8; 0];
    static mut DAT_009e9681: [u8; 0];
    static mut DAT_009fd458: [u8; 0];
    static mut DAT_0051bf60: [u16; 0];
    static mut DAT_0051bf78: [u8; 0];
    static mut DAT_0051bf88: [u16; 0];
    static mut DAT_0051bfa0: [u8; 0];
    static mut DAT_0051c581: [u8; 0];
    static mut DAT_0051c591: [u8; 0];
    static mut DAT_00521f1a: [u8; 0];
    static mut DAT_00521f1c: [u8; 0];
    static mut DAT_00a00870: [u32; 0];
    static mut DAT_00a00874: [u32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0041fac0(a: u32, b: u32) -> u32;
    fn FUN_004aef60(a: u32, b: i32) -> u32;
    fn FUN_004aeec0(a: u32, b: i32) -> u32;
    fn FUN_004a6990(a: u16, b: u16, c: u32, d: u32) -> u32;
    fn FUN_0042fb70(a: i32, b: u32, c: i32, d: i32, e: u32, f: u16) -> i32;
    fn FUN_00430200(a: i32, b: i32, c: u32, d: u32) -> i32;
    fn FUN_00425cb0() -> u32;
    fn FUN_0042e4d0(a: u32, b: u32, c: i32) -> u32;
    fn FUN_00425ba0(a: u32, b: u32) -> u32;
    fn FUN_00450650(a: i32, b: i32);
    fn FUN_00451c20(a: u32, b: *mut core::ffi::c_void, c: i32);
    fn FUN_00425d10() -> u32;
    fn FUN_004aba50();
    fn FUN_0043f140();
    fn FUN_004abe50();
    fn FUN_00434500(a: u32, b: u32);
    fn FUN_00435250(a: u32, b: i32) -> u32;
    fn FUN_0045acd0(a: i32);
    fn FUN_00484a30(a: i32, b: u32, c: u32);
    fn FUN_00433d30(a: u16, b: u16, c: i32);
    fn FUN_004af5b0(a: u32) -> i32;
    fn FUN_004adc40(a: u32) -> i32;
    fn FUN_004ae2a0(a: u32, b: u32) -> i32;
    fn FUN_004ae5a0(a: u32, b: u32) -> i32;
    fn FUN_004591c0(a: u32, b: i8) -> u16;
}

macro_rules! s {
    ($a:expr $(, $b:expr)+) => { ($a)$(.wrapping_add($b))+ };
}

#[inline(always)]
unsafe fn r8(a: u32) -> u8 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *const u8).read_unaligned() }
}
#[inline(always)]
unsafe fn r16(a: u32) -> u16 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *const u16).read_unaligned() }
}
#[inline(always)]
unsafe fn r32(a: u32) -> u32 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *const u32).read_unaligned() }
}
#[inline(always)]
unsafe fn w8(a: u32, v: u8) {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *mut u8).write_unaligned(v) }
}
#[inline(always)]
unsafe fn w16(a: u32, v: u16) {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *mut u16).write_unaligned(v) }
}
#[inline(always)]
unsafe fn w32(a: u32, v: u32) {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *mut u32).write_unaligned(v) }
}
#[inline(always)]
unsafe fn gb() -> u32 {
    // SAFETY: the game's global table pointer.
    unsafe { DAT_009a1fe8 }
}
#[inline(always)]
unsafe fn ec() -> u32 {
    // SAFETY: the game's global.
    unsafe { (DAT_009ecc34 as u32).wrapping_mul(0x5bd) }
}
#[inline(always)]
fn ad<T>(p: *mut T) -> u32 {
    p as usize as u32
}

#[unsafe(no_mangle)]
#[allow(
    non_snake_case,
    unused_variables,
    unused_assignments,
    unused_mut,
    unused_labels,
    unused_unsafe
)]
pub unsafe extern "C" fn FUN_0041dc30(
    param_1: u32,
    param_2: u16,
    param_3: u32,
    param_4: u32,
    param_5: u32,
    param_6: u16,
    param_7: u32,
    param_8: u16,
    param_9: u32,
) -> u32 {
    let mut param_3 = param_3;
    let mut param_4 = param_4;
    // SAFETY: all globals, callees and pointers are the game's own memory,
    // accessed exactly where the original does.
    unsafe {
        let x8c80 = ad(&raw mut DAT_009d8c80);
        let x8c84 = ad(&raw mut DAT_009d8c84);
        let x8c88 = ad(&raw mut DAT_009d8c88);
        let x8c89 = ad(&raw mut DAT_009d8c89);
        let x9154 = ad(&raw mut DAT_009d9154);
        let x9155 = ad(&raw mut DAT_009d9155);
        let x8b82 = ad(&raw mut DAT_009d8b82);
        let x8b84 = ad(&raw mut DAT_009d8b84);
        let x20e6 = ad(&raw mut DAT_009a20e6);
        let x2110 = ad(&raw mut DAT_009a2110);
        let x9670 = ad(&raw mut DAT_009e9670);
        let x967c = ad(&raw mut DAT_009e967c);
        let x967e = ad(&raw mut DAT_009e967e);
        let x9681 = ad(&raw mut DAT_009e9681);
        let xd458 = ad(&raw mut DAT_009fd458);
        let xbf60 = ad(&raw mut DAT_0051bf60);
        let xbf78 = ad(&raw mut DAT_0051bf78);
        let xbf88 = ad(&raw mut DAT_0051bf88);
        let xbfa0 = ad(&raw mut DAT_0051bfa0);
        let xc581 = ad(&raw mut DAT_0051c581);
        let xc591 = ad(&raw mut DAT_0051c591);
        let x1f1a = ad(&raw mut DAT_00521f1a);
        let x1f1c = ad(&raw mut DAT_00521f1c);
        let x0870 = ad(&raw mut DAT_00a00870);
        let x0874 = ad(&raw mut DAT_00a00874);

        let mut u15: u32 = param_4;
        let u9 = DAT_009d113c;
        let mut b8: u8 = 0;
        let mut b25: u8 = 0;
        let mut local_1f: u8 = 1;
        let mut local_1c: u16 = 0;
        let mut local_10: u32 = 0;
        let mut local_c: u16 = 0;
        if DAT_00523410 != 0 {
            return 1;
        }
        let u14 = param_1 & 0xffff;
        let i23 = u14.wrapping_mul(0x23);
        let mut u20 = r8(s!(gb(), i23, 0x16)) as u32;
        let mut i17: i32 = u20.wrapping_mul(0x566) as i32;
        if r8(s!(x8c84, u20.wrapping_mul(0x566))) == 0x14 || r8(s!(x9155, i17 as u32)) == 0x14 {
            return (i17 as u32 & 0xffff0000) | 1;
        }
        u20 = param_3 & 0xffff;
        if DAT_00a07cec <= u20 as i32 && (u20 as i32) < DAT_009a1fe0 {
            i17 = (r8(s!(u20.wrapping_mul(0x23), 0x16, gb())) as u32).wrapping_mul(0x566) as i32;
            if r8(s!(x8c84, i17 as u32)) == 0x14 || r8(s!(x9155, i17 as u32)) == 0x14 {
                return (i17 as u32 & 0xffff0000) | 1;
            }
        }
        let mut m_u0: u32 = r32(s!(gb(), i23, 0x1f));
        let p6: u32 = r32(s!(m_u0, 0xa9));
        DAT_009d113c = r16(s!(p6, 0x40));

        if param_3 as i16 == -2 {
            b25 = 1;
            param_3 = (param_3 & 0xffff0000) | 0xffff;
            local_1c = 0x35;
            param_4 = 0;
            u15 &= 0xffff;
        } else if param_3 as i16 == -1 {
            local_1c = 0x35;
            u15 = 0x20000;
        } else {
            let i21 = s!(u20.wrapping_mul(0x23), gb());
            i17 = r32(s!(i21, 0x1f)) as i32;
            if i17 == 0 {
                local_1c = r8(s!(i21, 3)) as u16;
                m_u0 = r32(s!(x1f1c, (local_1c as u32).wrapping_mul(0x1a)));
                if 0x7fff < m_u0 {
                    u15 = 0x20000;
                } else {
                    m_u0 = r32(s!(x1f1c, (local_1c as u32).wrapping_mul(0x1a)));
                    u15 = m_u0 << 2;
                }
            } else {
                local_1c = r8(s!(i21, 3)) as u16;
                let m_i0 = r32(s!(i17 as u32, 0xa9));
                if m_i0 == 0 {
                    u15 = r32(s!(i17 as u32, 0x7d));
                } else {
                    u15 = FUN_0041fac0(param_1, param_3);
                }
            }
        }
        let mut c4: i8 = r8(s!(gb(), 2, i23)) as i8;
        if c4 == 4 || c4 == 5 {
            u15 >>= 4;
        }
        if c4 == 3 {
            u15 >>= 2;
        }
        if DAT_00a08294 == 0xe {
            c4 = r8(s!(gb(), i23, 3)) as i8;
            if c4 == 0x25 || c4 == 0x26 {
                u15 >>= 2;
            }
        }
        u20 = 0xffff;
        let m_i0 = r32(s!(p6, 0xd3));
        w32(s!(p6, 0xd3), m_i0.wrapping_add(u15));

        let mut u10: u16;
        let mut i22: i32 = 0;
        let mut goto_dfdf = param_3 as i16 == -1;
        if !goto_dfdf {
            i17 = (param_3 & 0xffff).wrapping_mul(0x23) as i32;
            let i21 = s!(i17 as u32, gb());
            let m_p0 = r32(s!(i21, 0x1f));
            if m_p0 == 0 {
                goto_dfdf = true;
            } else {
                c4 = r8(m_p0) as i8;
                if c4 == 0 {
                    u20 = param_3 & 0xffff;
                    i22 = r32(s!(i21, 0x1b)) as i32;
                } else {
                    u10 = r16(s!(m_p0, 0x85));
                    u20 = u10 as u32;
                    i22 = r32(s!(
                        u20.wrapping_mul(8).wrapping_sub(u10 as u32).wrapping_mul(5),
                        0x1b,
                        gb()
                    )) as i32;
                    if c4 == 1 {
                        let p7 = r32(s!(m_p0, 0xa5));
                        if p7 != 0 {
                            i22 = r8(p7) as i8 as i32;
                        }
                    }
                }
                if i22 == -1 {
                    if c4 == 0 {
                        let v = r32(s!(p6, 0xdb));
                        w32(s!(p6, 0xdb), v.wrapping_add(u15));
                    } else {
                        i17 = s!(u20.wrapping_mul(0x23), gb()) as i32;
                        if r8(s!(i17 as u32, 2)) == 0 {
                            let idx = (r8(s!(i17 as u32, 0x16)) as u32).wrapping_mul(0x566);
                            let pi2 = s!(p6, (r8(s!(x8c89, idx)) as u32).wrapping_mul(4), 0x1c7);
                            let v = r32(pi2);
                            w32(pi2, v.wrapping_add(u15));
                        } else if (u20 as i32) < DAT_00a07ce8
                            || ((DAT_009d1308 as u32).wrapping_add(DAT_00a07ce8 as u32) as i32)
                                <= u20 as i32
                        {
                            let v = r32(s!(p6, 0xdf));
                            w32(s!(p6, 0xdf), v.wrapping_add(u15));
                        } else {
                            let v = r32(s!(p6, 0xe3));
                            w32(s!(p6, 0xe3), v.wrapping_add(u15));
                        }
                    }
                } else {
                    let a = s!(p6, (i22 as u32).wrapping_mul(4), 0x1a7);
                    let v = r32(a);
                    w32(a, v.wrapping_add(u15));
                    let mp = r32(s!(gb(), 0x1f, i17 as u32));
                    if r8(mp) == 0 {
                        let v = r32(s!(p6, 0xdb));
                        w32(s!(p6, 0xdb), v.wrapping_add(u15));
                    }
                }
                let pi2 = s!(
                    p6,
                    (r8(s!(u20.wrapping_mul(0x23), 0x16, gb())) as u32).wrapping_mul(4),
                    0xe7
                );
                let v = r32(pi2);
                w32(pi2, v.wrapping_add(u15));
            }
        }
        if goto_dfdf {
            let m_i0: i32;
            if param_3 as i16 == -1 {
                m_i0 = 1;
            } else {
                i17 = s!((param_3 & 0xffff).wrapping_mul(0x23), gb()) as i32;
                let m_i1 = r32(s!(i17 as u32, 0x1f)) as i32;
                m_i0 = (m_i1 != 0) as i32;
            }
            if m_i0 != 0 {
                if b25 != 0 {
                    let v = r32(s!(p6, 0xdf));
                    w32(s!(p6, 0xdf), v.wrapping_add(u15));
                }
            } else if r8(s!(i17 as u32, 2)) == 10 {
                let v = r32(s!(p6, 0xdb));
                w32(s!(p6, 0xdb), v.wrapping_add(u15));
            }
        }
        if r32(s!(gb(), 0x1b, i23)) as i32 != -1 {
            let v = r32(s!(p6, 0xd7));
            w32(s!(p6, 0xd7), v.wrapping_add(u15));
        }
        u10 = param_1 as u16;
        if param_2 != 0xffff {
            let m_i0 = r32(s!(p6, 0x215));
            let m_i1 = r32(s!(p6, 0x211));
            let go = if m_i0.wrapping_add(m_i1) == 0 {
                true
            } else {
                DAT_009d6954 == 0
            };
            if go {
                let q = FUN_004aef60(
                    r8(s!(gb(), 3, i23)) as u32,
                    (param_2 as i32).wrapping_sub(1),
                );
                if q != 0 {
                    u15 = FUN_004a6990(u10, param_2, u15, u20);
                }
            }
        }
        c4 = r8(s!(gb(), 2, i23)) as i8;
        if c4 == 0x0e {
            u15 = 0;
        }
        let pi2 = s!(p6, (param_4 & 0xffff).wrapping_mul(4), 0x211);
        i17 = r32(pi2) as i32;
        let mut do_ea1a = false;
        if (u15 as i32) < i17 {
            w32(pi2, (i17 as u32).wrapping_sub(u15));
            i17 = r32(s!(gb(), 0x1b, i23)) as i32;
            if i17 != -1 {
                w16(
                    s!(x8b82, (i17 as u32).wrapping_mul(8).wrapping_mul(2)),
                    0x3b,
                );
                DAT_009fe7d0 = param_4 as u8;
                u15 = (param_4 & 0xffff) ^ 1;
                i17 = r32(s!(p6, u15.wrapping_mul(4), 0x211)) as i32;
                let m0 = r32(pi2) as i32;
                if m0 < i17 {
                    let m0 = r32(pi2) as i32;
                    let i21 = i17.wrapping_sub(m0) / 2;
                    w32(
                        s!(p6, u15.wrapping_mul(4), 0x211),
                        i17.wrapping_sub(i21) as u32,
                    );
                    w32(pi2, m0.wrapping_add(i21) as u32);
                    w8(s!(p6, 0x21a), 1);
                }
            }
            if r32(s!(x9670, ec())) != u14 {
                FUN_0042fb70(DAT_009ecc34, u14, 3, -1, u14, 0x2000);
            }
            do_ea1a = true;
        } else {
            'b: {
                if DAT_00bb7375 == 2 && c4 != 0 && i17 != 0 && r32(s!(p6, 0xc3)) == 0 {
                    FUN_00430200(1, 0x10, u14, 0xffff);
                }
                let m0 = r32(pi2);
                u15 = u15.wrapping_sub(m0);
                w32(pi2, 0);
                if r32(s!(gb(), 0x1b, i23)) as i32 != -1 {
                    if param_4 as i16 == 0 && b25 == 0 {
                        let u19 = FUN_00425cb0() as u16;
                        if u19 < 0x1000 {
                            let m = r32(s!(gb(), 0x1b, i23)) as i32;
                            FUN_0042e4d0(0x59, 0xffffffff, m);
                        }
                    }
                    let q16 = (param_4 & 0xffff) ^ 1;
                    i17 = r32(s!(p6, q16.wrapping_mul(4), 0x211)) as i32;
                    let pi3 = s!(p6, q16.wrapping_mul(4), 0x211);
                    if i17 != 0 {
                        i17 /= 2;
                        let m = r32(pi3);
                        w32(pi3, m.wrapping_sub(i17 as u32));
                        let m = r32(pi2);
                        w32(pi2, m.wrapping_add(i17 as u32));
                        w8(s!(p6, 0x21a), 1);
                    }
                }
                if u15 == 0 {
                    do_ea1a = true;
                    break 'b;
                }
                if local_1c == 0x8d || local_1c == 0x8e || local_1c == 0x93 {
                    if r16(s!(p6, 0xcf)) < 1000 {
                        if local_1c == 0x8d {
                            let m = r16(s!(p6, 0xcf));
                            w16(s!(p6, 0xcf), m.wrapping_add(1));
                        }
                        if local_1c == 0x8e {
                            let m = r16(s!(p6, 0xcf));
                            w16(s!(p6, 0xcf), m.wrapping_add(2));
                        }
                        if local_1c == 0x93 {
                            let m = r16(s!(p6, 0xcf));
                            w16(s!(p6, 0xcf), m.wrapping_add(4));
                        }
                    }
                    let m_h0 = r16(s!(xc581, (r8(s!(p6, 4)) as u32).wrapping_mul(0x101))) as i16;
                    let m_h1 = r16(s!(p6, 0xcf)) as i16;
                    let s11: i16 = m_h0.wrapping_sub(m_h1);
                    let m_h0 = r16(s!(p6, 0x1e5)) as i16;
                    if m_h0 != 0 && s11 < 0xb {
                        if 0 < u15 as i32 {
                            u15 = u15.wrapping_add(199) / 200;
                            loop {
                                let mut q16: u32 = 0;
                                loop {
                                    let mut m_s0 = r16(s!(p6, 0x1e5));
                                    let f = r16(s!(xbf60, q16.wrapping_mul(2)));
                                    if (m_s0 & f) != 0 {
                                        b8 = 1;
                                        m_s0 = (f ^ 0x3ff) & m_s0;
                                        w16(s!(p6, 0x1e5), m_s0);
                                        break;
                                    }
                                    let u19 = (q16 as u16).wrapping_add(1);
                                    q16 = u19 as u32;
                                    if !(u19 < 10) {
                                        break;
                                    }
                                }
                                let m_i0 = r32(s!(gb(), 0x1b, i23)) as i32;
                                if (q16 as u16) < 10 && m_i0 != -1 {
                                    w16(s!(p6, q16.wrapping_mul(2), 0x2ac), 0);
                                    w16(
                                        s!(p6, q16.wrapping_mul(2), 0x2c2),
                                        r16(s!(xbf88, q16.wrapping_mul(2))),
                                    );
                                }
                                u15 = u15.wrapping_sub(1);
                                if u15 == 0 {
                                    break;
                                }
                            }
                        }
                        let mut u19: u16 = 0;
                        let m_i0 = r32(s!(gb(), 0x1b, i23)) as i32;
                        if s11 < 1 && m_i0 != -1 {
                            loop {
                                u15 = u19 as u32;
                                let mut m_s0 = r16(s!(p6, 0x1e5));
                                let f = r16(s!(xbf60, u15.wrapping_mul(2)));
                                if (m_s0 & f) != 0 {
                                    m_s0 = (f ^ 0x3ff) & m_s0;
                                    w16(s!(p6, 0x1e5), m_s0);
                                    w16(s!(p6, u15.wrapping_mul(2), 0x2ac), 0);
                                    w16(
                                        s!(p6, u15.wrapping_mul(2), 0x2c2),
                                        r16(s!(xbf88, u15.wrapping_mul(2))),
                                    );
                                }
                                u19 = u19.wrapping_add(1);
                                if !(u19 < 10) {
                                    break;
                                }
                            }
                            w16(s!(p6, 0x1e5), 0);
                        }
                        let mut b24: u8 = 0;
                        let m_h0 = r16(s!(p6, 0x1e5)) as i16;
                        if m_h0 == 0 {
                            let m_s0 = r16(s!(xc581, (r8(s!(p6, 4)) as u32).wrapping_mul(0x101)));
                            w16(s!(p6, 0xcf), m_s0);
                            w32(s!(p6, 0x211), 0);
                            w32(s!(p6, 0x215), 0);
                            FUN_00451c20(param_1, p6 as usize as *mut core::ffi::c_void, 0x89);
                            i17 = FUN_00430200(1, 0x1c, u14, 0xffff);
                            if i17 != 0 {
                                let m_s0 = r16(s!(x967c, ec()));
                                u15 = m_s0 as u32;
                                let idx = (r8(s!(gb(), 0x16, i23)) as u32).wrapping_mul(0x566);
                                let t = r8(s!(x8c88, idx));
                                if t as u32 != u15 {
                                    b24 = (r8(s!(
                                        xd458,
                                        (t as u32).wrapping_add(u15.wrapping_mul(0x1e5))
                                    )) == 0) as u8;
                                }
                                if b24 == 0 {
                                    i17 = r32(s!(x9670, ec())) as i32;
                                    if i17 != -1 {
                                        let c = r8(s!((i17 as u32).wrapping_mul(0x23), 3, gb()));
                                        b24 = (c == 1 || c == 2 || c == 3 || c == 0x0e || c == 4)
                                            as u8;
                                        u15 = if b24 != 0 { 0x57 } else { 0x24 };
                                        FUN_0042e4d0(u15, 0xffffffff, DAT_009ecc34);
                                    }
                                }
                            }
                            let m_i0 = r32(s!(gb(), 0x1b, i23)) as i32;
                            if m_i0 == -1 {
                                u15 = 0;
                                if r8(s!(p6, 0x21d)) != 0 {
                                    loop {
                                        if r8(s!(p6, u15.wrapping_add(0xdc).wrapping_mul(4))) == 2 {
                                            w16(s!(p6, u15.wrapping_mul(4), 0x3b2), 0xffff);
                                        }
                                        u15 = u15.wrapping_add(1);
                                        if !(u15 < r8(s!(p6, 0x21d)) as u32) {
                                            break;
                                        }
                                    }
                                }
                                u15 = 0;
                                if r8(s!(p6, 0x21b)) != 0 {
                                    loop {
                                        w8(s!(p6, u15, 0x220), 0);
                                        u15 = u15.wrapping_add(1);
                                        if !(u15 < r8(s!(p6, 0x21b)) as u32) {
                                            break;
                                        }
                                    }
                                }
                            }
                            u15 = r8(s!(gb(), 0x16, i23)) as u32;
                            i17 = u15.wrapping_mul(0x126) as i32;
                            let m = r16(s!(x20e6, i17 as u32));
                            w16(s!(x20e6, i17 as u32), m.wrapping_add(1));
                            if r8(s!(x8c80, u15.wrapping_mul(0x566))) as i32
                                == r8(s!(p6, 0xb0)) as i8 as i32
                            {
                                w8(s!(x2110, i17 as u32), 1);
                            }
                        }
                    }
                    let m_i0 = r32(s!(gb(), 0x1b, i23)) as i32;
                    if m_i0 == DAT_009ecc34 {
                        FUN_0042e4d0(0x21, u14, DAT_009ecc34);
                    }
                } else {
                    if param_2 != 0xffff {
                        let q = FUN_004aeec0(
                            r8(s!(gb(), 3, i23)) as u32,
                            (param_2 as i32).wrapping_sub(1),
                        );
                        if q != 0 {
                            u15 = FUN_004a6990(u10, param_2, u15, u20);
                        }
                    }
                    let q16 = r32(s!(p6, 0xc3));
                    w32(s!(p6, 0xc3), q16.wrapping_add(u15));
                    'e3a6: {
                        if DAT_00bb7375 == 2 && r8(s!(gb(), 2, i23)) != 0 {
                            let mut v: i32;
                            m_u0 = r32(s!(p6, 0xcb));
                            u15 = FUN_00425ba0(m_u0, 0xf333);
                            let m_u1 = r32(s!(p6, 0xc3));
                            if q16 < u15 && u15 <= m_u1 {
                                v = 0x16;
                            } else {
                                m_u0 = r32(s!(p6, 0xcb));
                                u15 = FUN_00425ba0(m_u0, 0xc000);
                                let m_u1 = r32(s!(p6, 0xc3));
                                if q16 < u15 && u15 <= m_u1 {
                                    v = 0x15;
                                } else {
                                    m_u0 = r32(s!(p6, 0xcb));
                                    u15 = FUN_00425ba0(m_u0, 0x4000);
                                    let m_u1 = r32(s!(p6, 0xc3));
                                    if u15 <= q16 || m_u1 < u15 {
                                        break 'e3a6;
                                    }
                                    v = 0x14;
                                }
                            }
                            FUN_00430200(1, v, u14, 0xffff);
                        }
                    }
                    i17 = r32(s!(gb(), 0x1b, i23)) as i32;
                    if i17 != -1 && b25 == 0 {
                        if i17 != -1 {
                            let a = s!(x8b84, (i17 as u32).wrapping_mul(0x10));
                            let m = r16(a);
                            w16(a, m.wrapping_add(0x3b));
                        }
                        let u19 = FUN_00425cb0() as u16;
                        if u19 < 0x4000 {
                            let u19 = FUN_00425cb0() as u16;
                            let u12 = FUN_00425cb0() as u16;
                            let u13 = FUN_00425cb0() as u16;
                            u15 = ((u19 & 7).wrapping_add(u12 & 1).wrapping_add(u13 & 1)) as u32;
                            let mut m_s0 = r16(s!(p6, 0x1e5));
                            let f = r16(s!(xbf60, u15.wrapping_mul(2)));
                            if (m_s0 & f) != 0 {
                                m_s0 = (f ^ 0x3ff) & m_s0;
                                w16(s!(p6, 0x1e5), m_s0);
                                DAT_00a07be2 = 0x57;
                                DAT_00a07be0 = r8(s!(xbf78, u15)) as u16;
                                let m_i0 = r32(s!(gb(), 0x1b, i23)) as i32;
                                if m_i0 == DAT_009ecc34 && r8(s!(x9681, ec())) == 0 {
                                    FUN_00450650(0x56, DAT_009ecc34);
                                }
                                w16(s!(p6, u15.wrapping_mul(2), 0x2ac), 0);
                                w16(
                                    s!(p6, u15.wrapping_mul(2), 0x2c2),
                                    r16(s!(xbf88, u15.wrapping_mul(2))),
                                );
                            }
                        }
                    }
                }
                let m_u0 = r32(s!(p6, 0xc3));
                let m_u1 = r32(s!(p6, 199));
                let mut call_e9cc = false;
                let mut e_u19: u16 = 0;
                let mut e_i17: i32 = 0;
                if m_u0 < m_u1 || b25 != 0 {
                    let m_i0 = r32(s!(gb(), 0x1b, i23)) as i32;
                    if m_i0 == DAT_009ecc34 && b25 == 0 {
                        u15 = FUN_00425d10();
                        u15 = if (u15 & 0x8000) == 0 { 0x20 } else { 0x1f };
                        FUN_0042e4d0(u15, u14, DAT_009ecc34);
                        local_1f = 0;
                    }
                    let m_u0 = r32(s!(x9670, ec()));
                    if m_u0 != u14 {
                        e_u19 = 0x4000;
                        e_i17 = 3;
                        call_e9cc = true;
                    }
                } else {
                    u15 = FUN_00425cb0();
                    let mut u19 = r16(s!(xbfa0, (u15 & 0xf).wrapping_mul(2)));
                    let m_s0 = r16(s!(p6, 0x1df));
                    if (DAT_009d6940[1] == 0 || u19 != 1) && (m_s0 & u19) != 0 {
                        i17 = r32(s!(gb(), 0x1b, i23)) as i32;
                        if i17 != -1 {
                            FUN_0042e4d0(0x1e, u14, i17);
                            b25 = 0;
                            local_1f = 0;
                            let c = r8(s!(gb(), 3, i23));
                            if c == 1 || c == 2 || c == 3 || c == 0x0e || c == 4 {
                                b25 = 1;
                            }
                            if b25 != 0 {
                                if u19 < 9 {
                                    if u19 == 8 || u19 == 2 || u19 == 4 {
                                        u19 = 0xe;
                                    }
                                } else if u19 < 0x101 {
                                    if u19 == 0x100 || u19 == 0x80 {
                                        u19 = 0x180;
                                    }
                                } else if u19 < 0x401 {
                                    if u19 == 0x400 || u19 == 0x200 {
                                        u19 = 0xe00;
                                    }
                                } else if u19 == 0x800 || u19 == 0x1000 {
                                    u19 = 0xe00;
                                }
                            }
                        }
                        b8 = 1;
                        let m_s0 = r16(s!(p6, 0x1e1));
                        w16(s!(p6, 0x1e1), m_s0 & !u19);
                    }
                    let m_u0 = r32(s!(x9670, ec()));
                    if r8(s!(p6, 7)) == 0 && m_u0 != u14 {
                        e_u19 = 0x6000;
                        e_i17 = 5;
                        call_e9cc = true;
                    }
                }
                if call_e9cc {
                    FUN_0042fb70(DAT_009ecc34, u14, e_i17, -1, u14, e_u19);
                }
                let m_i0 = r32(s!(gb(), 0x1b, i23)) as i32;
                if !(b8 == 0 || m_i0 != DAT_009ecc34 || DAT_00a00860 != 0) {
                    if DAT_00523410 == 0 {
                        FUN_004aba50();
                        FUN_0043f140();
                        FUN_004abe50();
                        do_ea1a = true;
                    }
                } else {
                    do_ea1a = true;
                }
            }
        }
        'f2c3: {
            if do_ea1a && DAT_00523410 == 0 {
                u15 = 0;
                let c7 = r8(s!(p6, 7));
                if c7 != 0 && c7 != 5 {
                    break 'f2c3;
                }
                u15 = r32(s!(p6, 0xc3));
                m_u0 = r32(s!(p6, 0xcb));
                if u15 < m_u0 {
                    break 'f2c3;
                }
                if param_3 as i16 != -1 {
                    let pc = r32(s!((param_3 & 0xffff).wrapping_mul(0x23), 0x1f, gb()));
                    if pc != 0 {
                        if r8(pc) != 0 {
                            let m_s0 = r16(s!(pc, 0x85));
                            param_3 = (pc & 0xffff0000) | m_s0 as u32;
                        }
                        FUN_00434500(param_3, param_1);
                    }
                }
                u15 = FUN_00435250(u14, 0);
                i17 = r32(s!(gb(), 0x1b, i23)) as i32;
                if i17 != -1 {
                    FUN_0045acd0(i17);
                }
                if DAT_009d6940[1] == 0 {
                    i17 = r32(s!(gb(), 0x1b, i23)) as i32;
                    if i17 != -1 {
                        FUN_00484a30(i17, u20, u15);
                    }
                } else {
                    let m_i0 = r32(s!(gb(), 0x1b, i23)) as i32;
                    if m_i0 == DAT_009ecc34 {
                        DAT_009d6940[0] = 1;
                    }
                }
                if r32(s!(gb(), i23, 0x1b)) as i32 != -1 {
                    DAT_009ec5e6 = 0;
                }
                FUN_00433d30(u10, r8(s!(gb(), i23, 0x16)) as u16, 2);
                let mut u19 = DAT_00a07cec as u16;
                if (((DAT_00a07cec as u32) & 0xffff) as i32) < DAT_009a1fe0 {
                    loop {
                        let i = s!((u19 as u32).wrapping_mul(0x23), gb());
                        if r8(s!(i, 3)) != 0 {
                            let mu = r32(s!(i, 0x1f));
                            let ii = r32(s!(mu, 0xa9));
                            let m_s0 = r16(s!(ii, 0x66));
                            if m_s0 == u10 {
                                w16(s!(ii, 0x66), 0xffff);
                            }
                        }
                        u19 = u19.wrapping_add(1);
                        if !((u19 as i32) < DAT_009a1fe0) {
                            break;
                        }
                    }
                }
                u19 = DAT_00a07cec as u16;
                if (((DAT_00a07cec as u32) & 0xffff) as i32) < DAT_009a1fe0 {
                    loop {
                        let i = s!((u19 as u32).wrapping_mul(0x23), gb());
                        if r8(s!(i, 3)) != 0 {
                            let mu = r32(s!(i, 0x1f));
                            let ii = r32(s!(mu, 0xa9));
                            let m_s0 = r16(s!(ii, 0x68));
                            if m_s0 == u10 {
                                w16(s!(ii, 0x68), 0xffff);
                            }
                        }
                        u19 = u19.wrapping_add(1);
                        if !((u19 as i32) < DAT_009a1fe0) {
                            break;
                        }
                    }
                }
                FUN_00451c20(param_1, p6 as usize as *mut core::ffi::c_void, 0x88);
                u10 = r16(s!(x967e, ec()));
                let mut do_ecf2 = false;
                let mut e_u10: u16 = 0;
                let mut e_i17: i32 = 0;
                if r8(s!(gb(), i23, 0x16)) as u32 == u10 as u32 {
                    i17 = FUN_0042fb70(DAT_009ecc34, u14, 6, -1, u14, 0xc000);
                    if i17 == 0 {
                        e_u10 = 0xc000;
                        e_i17 = 0x10;
                        u20 = 0xffffffff;
                        do_ecf2 = true;
                    }
                } else if u20 != 0xffffffff
                    && (u20 as i32) < DAT_009a1fe0
                    && r32(s!(x9670, ec())) != u20
                    && r8(s!(u20.wrapping_mul(0x23), 0x16, gb())) as u32 == u10 as u32
                {
                    let c = r8(s!(gb(), i23, 2));
                    if c == 0 || c == 1 {
                        e_u10 = 0xffff;
                        e_i17 = 9;
                    } else {
                        e_u10 = 0xa000;
                        e_i17 = 10;
                    }
                    do_ecf2 = true;
                }
                if do_ecf2 {
                    u10 = e_u10;
                    FUN_0042fb70(DAT_009ecc34, u20, e_i17, -1, u14, u10);
                }
                i17 = FUN_00430200(1, 0x1b, u14, 0xffff);
                if i17 != 0 {
                    let m_s0 = r16(s!(x967c, ec()));
                    u15 = m_s0 as u32;
                    let idx = (r8(s!(gb(), 0x16, i23)) as u32).wrapping_mul(0x566);
                    let t = r8(s!(x8c88, idx));
                    if t as u32 == u15 {
                        b25 = 0;
                    } else {
                        b25 = (r8(s!(xd458, (t as u32).wrapping_add(u15.wrapping_mul(0x1e5)))) == 0)
                            as u8;
                    }
                    if b25 == 0 {
                        i17 = r32(s!(x9670, ec())) as i32;
                        if i17 != -1 {
                            let c = r8(s!((i17 as u32).wrapping_mul(0x23), 3, gb()));
                            b25 = (c == 1 || c == 2 || c == 3 || c == 0x0e || c == 4) as u8;
                            u15 = if b25 != 0 { 0x57 } else { 0x24 };
                            FUN_0042e4d0(u15, 0xffffffff, DAT_009ecc34);
                        }
                    }
                }
                let mut b5 = r8(s!(gb(), 3, i23));
                u15 = b5 as u32;
                i17 = FUN_004af5b0(u15);
                if i17 == 0 {
                    b5 = r8(s!(p6, 4));
                    u10 = FUN_00425cb0() as u16;
                    u10 = ((u10 & 0x3fff) as u32).wrapping_add(0x2000) as u16;
                    while r16(s!(xc591, (b5 as u32).wrapping_mul(0x101))) < u10 {
                        u10 >>= 1;
                    }
                    let u19 = FUN_00425cb0() as u16;
                    if u19 < 0x8000 {
                        u10 = 0u16.wrapping_sub(u10);
                    }
                    m_u0 = r32(s!(gb(), 0x1f, i23));
                    w16(s!(m_u0, 0x77), u10);
                    let a23 = s!(gb(), i23);
                    let idx = (r8(s!(a23, 0x16)) as u32).wrapping_mul(0x566);
                    let t = r8(s!(x9154, idx));
                    if t == 0 {
                        u15 = FUN_00425cb0();
                        let s11 = (((u15 & 7) as u16 as i16 as i32)
                            .wrapping_add(8)
                            .wrapping_mul(0xec)) as i16;
                        u15 = (u15 & 0xffff0000) | (s11 as u16 as u32);
                        m_u0 = r32(s!(a23, 0x1f));
                        w16(s!(m_u0, 0x81), s11 as u16);
                        w8(s!(p6, 7), 3);
                    } else {
                        u15 = r32(s!(a23, 0x1f));
                        let m_s0 = ((t as i32).wrapping_mul(0x49c).wrapping_sub(0x49b)) as u16;
                        w16(s!(u15, 0x81), m_s0);
                        w8(s!(p6, 7), 3);
                    }
                } else if ((r32(s!(x1f1c, (local_1c as u32).wrapping_mul(0x1a))) as i32) < 0x448
                    || r8(s!(x1f1a, (local_1c as u32).wrapping_mul(0x1a))) != 0)
                    && (local_1c != 0x3a && {
                        m_u0 = r32(s!(gb(), 0x1f, i23));
                        (r16(s!(m_u0, 0x79)) as i16) != 0
                    })
                {
                    u10 = FUN_00425cb0() as u16;
                    let m_i0 = r32(s!(gb(), i23, 0x1b)) as i32;
                    if u10 < 0x4000 && m_i0 != -1 {
                        m_u0 = r32(s!(gb(), i23, 0x1f));
                        w16(s!(m_u0, 0x81), 1);
                        u15 = FUN_0042e4d0(0x15, u14, DAT_009ecc34);
                        local_1f = 0;
                        w8(s!(p6, 7), 4);
                    } else {
                        if b5 < 0x49 {
                            u10 = r16(s!(x0870, u15.wrapping_mul(0x65).wrapping_mul(4)));
                        } else {
                            i17 = FUN_004adc40(u15);
                            u10 = i17 as u16;
                        }
                        local_1c = 0;
                        local_c = 0;
                        let mut u19: u16 = 0;
                        if 1 < u10 {
                            u19 = FUN_00425cb0() as u16;
                            local_10 = 0;
                            u19 &= 1;
                            if u10 != 0 {
                                loop {
                                    if b5 < 0x49 {
                                        u20 = local_10;
                                        let cnt =
                                            r32(s!(x0870, u15.wrapping_mul(0x65).wrapping_mul(4)));
                                        if (cnt as i32) <= local_10 as i32 {
                                            u20 = cnt.wrapping_sub(1);
                                        }
                                        i17 = r32(s!(
                                            x0874,
                                            u20.wrapping_add(u15.wrapping_mul(0x65))
                                                .wrapping_mul(4)
                                        )) as i32;
                                    } else {
                                        i17 = FUN_004ae2a0(u15, local_10);
                                    }
                                    if r8(s!(p6, local_10, 0x2d8)) == 0 && (i17 == 2 || i17 == 0x14)
                                    {
                                        if u19 == 0 {
                                            i17 = FUN_004ae5a0(u15, local_10);
                                            if 0 < i17 {
                                                break;
                                            }
                                        } else {
                                            i17 = FUN_004ae5a0(u15, local_10);
                                            if i17 < 0 {
                                                break;
                                            }
                                        }
                                    }
                                    let u12 = (local_10 as u16).wrapping_add(1);
                                    local_10 = u12 as u32;
                                    if !(u12 < u10) {
                                        break;
                                    }
                                }
                            }
                            let mut u12: u16 = 0;
                            if (local_10 as u16) < u10 && {
                                u12 = FUN_004591c0(param_1, local_10 as i8);
                                u12 != 0xffff
                            } {
                                let u13 = FUN_00425cb0() as u16;
                                local_c = ((u13 & 0x3fff) as u32).wrapping_add(0x4000) as u16;
                                let u13 = FUN_00425cb0() as u16;
                                local_1c = ((u13 & 0x7ff) as u32).wrapping_add(0x800) as u16;
                                let mut u13 = local_1c;
                                if u19 != 0 {
                                    u13 = 0u16.wrapping_sub(local_1c);
                                    local_c = 0u16.wrapping_sub(local_c);
                                }
                                let i21 = (u12 as u32).wrapping_mul(0x23);
                                m_u0 = r32(s!(gb(), 0x1f, i21));
                                w16(s!(m_u0, 0x77), local_c);
                                let ps = s!(gb(), 0x10, i21);
                                let m_h0 = r16(ps);
                                w16(ps, m_h0.wrapping_add(u13));
                                m_u0 = r32(s!(gb(), 0x1f, i21));
                                w8(s!(m_u0, 0x92), 1);
                                i17 = r32(s!(gb(), 0x1f, i21)) as i32;
                                w8(s!(i17 as u32, 0x8b), r8(s!(i17 as u32, 0x92)));
                                w8(s!(gb(), 0x1a, i21), 2);
                                w8(s!(p6, local_10, 0x2d8), 4);
                                u15 = FUN_00425d10();
                                u15 = if (u15 & 0x8000) == 0 { 0x1b } else { 0x1a };
                                FUN_0042e4d0(u15, u14, DAT_009ecc34);
                                local_1f = 0;
                            }
                        }
                        b5 = r8(s!(p6, 4));
                        let mut u12 = FUN_00425cb0() as u16;
                        u12 = ((u12 & 0x3fff) as u32).wrapping_add(0x2000) as u16;
                        while r16(s!(xc591, (b5 as u32).wrapping_mul(0x101))) < u12 {
                            u12 >>= 1;
                        }
                        if local_c < 0x8000 {
                            u12 = 0u16.wrapping_sub(u12);
                        }
                        m_u0 = r32(s!(gb(), 0x1f, i23));
                        w16(s!(m_u0, 0x77), u12);
                        if local_1c != 0 {
                            local_1c >>= 1;
                            if u19 == 0 {
                                local_1c = 0u16.wrapping_sub(local_1c);
                            }
                            let ps = s!(gb(), 0x10, i23);
                            let m_h0 = r16(ps);
                            w16(ps, m_h0.wrapping_add(local_1c));
                            m_u0 = r32(s!(gb(), 0x1f, i23));
                            w8(s!(m_u0, 0x92), 1);
                            i17 = r32(s!(gb(), 0x1f, i23)) as i32;
                            w8(s!(i17 as u32, 0x8b), r8(s!(i17 as u32, 0x92)));
                        }
                        w8(s!(p6, 7), 3);
                        let a23 = s!(gb(), i23);
                        let m_i0 = r32(s!(a23, 0x1b)) as i32;
                        if m_i0 == -1 {
                            let u19 = FUN_00425cb0() as u16;
                            let u12 = FUN_00425cb0() as u16;
                            m_u0 = r32(s!(a23, 0x1f));
                            let m_s0 = ((((u19 & 3) as u32)
                                .wrapping_add((u12 & 7) as u32)
                                .wrapping_add(1))
                            .wrapping_mul(0xec)) as u16;
                            w16(s!(m_u0, 0x81), m_s0);
                        } else {
                            m_u0 = r32(s!(a23, 0x1f));
                            w16(s!(m_u0, 0x81), 0x2c4);
                        }
                        u15 = u10 as u32;
                        w8(s!(p6, u15, 0x2d8), 2);
                    }
                } else {
                    m_u0 = r32(s!(gb(), 0x1f, i23));
                    w16(s!(m_u0, 0x81), 1);
                    u15 = FUN_0042e4d0(0x15, u14, DAT_009ecc34);
                    local_1f = 0;
                    w8(s!(p6, 7), 4);
                }
                break 'f2c3;
            }
            let c = r8(s!(p6, 7));
            u15 = (((DAT_00523410 as u32) >> 8) << 8) | c as u32;
            if c == 0 || c == 5 {
                u15 = r32(s!(p6, 0xcb));
                m_u0 = r32(s!(p6, 0xc3));
                if u15 <= m_u0 {
                    u15 = u15.wrapping_sub(1);
                    w32(s!(p6, 0xc3), u15);
                }
            }
        }
        let u18 = (u15 & 0xffff0000) | DAT_009d113c as u32;
        w16(s!(p6, 0x40), DAT_009d113c);
        DAT_009d113c = u9;
        (((u18 >> 8) << 8) | local_1f as u32) & 0xffff00ff
    }
}
