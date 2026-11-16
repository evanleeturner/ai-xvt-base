// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a08104: *mut u8;
    static mut DAT_009a1fe8: *mut u8;
    static mut DAT_00a08294: u16;
    static mut DAT_00a05da8: i32;
    static mut DAT_00a00870: [i32; 0];
    static mut DAT_00a00874: [i32; 0];
    static mut DAT_009d6954: i8;
    static mut DAT_009d6941: u8;
    static mut DAT_009d6950: i16;
    static mut DAT_009d6944: i32;
    static mut DAT_009d8b74: u8;
    static mut DAT_009d8b75: u8;
    static mut DAT_009d8a58: u32;
    static mut DAT_009d8a54: i32;
    static mut DAT_009d8b60: i32;
    static mut DAT_009ecc34: i32;
    static mut DAT_009e9699: u8;
    static mut DAT_009e9670: u8;
    static mut DAT_009ec480: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ae2a0(a: u32, b: u32) -> i32;
    fn FUN_004adc40(a: u32) -> i32;
    fn FUN_004aeec0(a: u32, b: u32) -> u32;
    fn FUN_00484e10(a: u32, b: u32) -> u8;
    fn FUN_00459750(a: u32) -> u16;
    fn FUN_004ae5a0(a: u32, b: u32) -> u32;
    fn FUN_004ae640(a: u32, b: u32) -> i32;
    fn FUN_004ae6e0(a: u32, b: u32) -> i32;
    fn FUN_0046a3c0(a: i16) -> i64;
    fn FUN_0046a6d0(a: i16) -> i64;
    fn FUN_004037b0(a: *mut u8, b: u32, c: i32, d: i32);
    fn FUN_00425cb0() -> u32;
    fn FUN_0042e4d0(a: u32, b: u32, c: i32);
    fn FUN_004aedf0(a: u32, b: u32) -> i32;
    fn FUN_0042fb70(a: i32, b: u32, c: u32, d: u32, e: u32, f: u32);
}

unsafe fn r32(p: *const u8) -> u32 {
    // SAFETY: caller supplies a valid game address.
    unsafe { p.cast::<u32>().read_unaligned() }
}
unsafe fn w32(p: *mut u8, v: u32) {
    // SAFETY: caller supplies a valid game address.
    unsafe { p.cast::<u32>().write_unaligned(v) }
}
unsafe fn r16(p: *const u8) -> u16 {
    // SAFETY: caller supplies a valid game address.
    unsafe { p.cast::<u16>().read_unaligned() }
}
unsafe fn w16(p: *mut u8, v: u16) {
    // SAFETY: caller supplies a valid game address.
    unsafe { p.cast::<u16>().write_unaligned(v) }
}
fn off(p: *mut u8, o: usize) -> *mut u8 {
    p.wrapping_add(o)
}
unsafe fn ent(i: u32) -> *mut u8 {
    // SAFETY: reads the game's entity table pointer.
    unsafe { off(DAT_009a1fe8, i.wrapping_mul(0x23) as usize) }
}
unsafe fn ent3(i: u32) -> u8 {
    // SAFETY: entity table is the game's own memory.
    unsafe { *off(ent(i), 3) }
}
unsafe fn t0(i: u32) -> i32 {
    // SAFETY: game table read.
    unsafe {
        (&raw mut DAT_00a00870)
            .cast::<i32>()
            .wrapping_add(i as usize)
            .read()
    }
}
unsafe fn t4(i: u32) -> i32 {
    // SAFETY: game table read.
    unsafe {
        (&raw mut DAT_00a00874)
            .cast::<i32>()
            .wrapping_add(i as usize)
            .read()
    }
}
unsafe fn kind(b: u32, idx: i32) -> i32 {
    // SAFETY: game tables and callee are the game's own.
    unsafe {
        if b < 0x49 {
            if idx < 0 {
                0
            } else {
                let mut i = idx;
                let n = t0(b.wrapping_mul(0x65));
                if n <= i {
                    i = n.wrapping_sub(1);
                }
                t4((i as u32).wrapping_add(b.wrapping_mul(0x65)))
            }
        } else {
            FUN_004ae2a0(b, idx as u32)
        }
    }
}
unsafe fn count(b: u32) -> i32 {
    // SAFETY: game table and callee.
    unsafe {
        if b < 0x49 {
            t0(b.wrapping_mul(0x65))
        } else {
            FUN_004adc40(b)
        }
    }
}
unsafe fn kind2(u6: u32) -> i32 {
    // SAFETY: game globals.
    unsafe {
        if (u6 as i32) < 0 {
            0
        } else {
            let mut u = u6;
            if DAT_00a05da8 <= u6 as i32 {
                u = DAT_00a05da8.wrapping_sub(1) as u32;
            }
            t4(u.wrapping_add(0x154e))
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a6990(
    param_1: u16,
    param_2: i16,
    param_3: u32,
    param_4: u32,
) -> u32 {
    let mut param_3 = param_3;
    let mut u15: u16 = param_2.wrapping_sub(1) as u16;
    let mut u6: u32 = u15 as u32;
    let p1 = param_1 as u32;
    // SAFETY: all globals, tables and entity memory are the game's own,
    // accessed where the original accesses them.
    unsafe {
        let c2 = *off(DAT_00a08104, u6 as usize + 0x33c) as i8;
        if c2 == 0 {
            return param_3;
        }
        'a: {
            if c2 != -1 {
                break 'a;
            }
            if ent3(p1) == 0x36 {
                if kind2(u6) == 7 {
                    break 'a;
                }
            }
            return param_3;
        }
        if param_3 == 0 {
            param_3 = 1;
        }
        let mut iv7: u32 = p1.wrapping_mul(0x23);
        if DAT_00a08294 == 0xe && ent3(p1) == 0x36 && kind2(u6) == 7 {
            let local_34 = count(ent3(p1) as u32);
            let mut i8_: i32 = 0;
            let mut cnt: i32 = 0;
            if 0 < local_34 {
                loop {
                    let b = ent3(p1) as u32;
                    let k = kind(b, i8_);
                    if k == 8 && *off(DAT_00a08104, 0x33c + i8_ as usize) != 0 {
                        cnt = cnt.wrapping_add(1);
                    }
                    i8_ = i8_.wrapping_add(1);
                    if !(i8_ < local_34) {
                        break;
                    }
                }
            }
            if cnt != 0 {
                return param_3;
            }
            if r32(off(DAT_00a08104, 0x211)) != 0 {
                return param_3;
            }
            if r32(off(DAT_00a08104, 0x215)) != 0 {
                return param_3;
            }
            let e1 = ent(param_4 & 0xffff);
            let c = *off(e1, 3) as i8;
            let t32: u32;
            let u14: u32;
            if c == 0x28 {
                u14 = r32(off(DAT_00a08104, 0xcb));
                let t4v = r32(off(DAT_00a08104, 0xc3));
                let bb = *off(DAT_00a08104, u6 as usize + 0x33c) as u32;
                t32 = bb
                    .wrapping_mul(0x10)
                    .wrapping_add((u14 / 100).wrapping_mul(0u32.wrapping_sub(5)))
                    .wrapping_sub(t4v);
            } else {
                if c != 3 {
                    return param_3;
                }
                let pp = r32(off(e1, 0x1f)) as *mut u8;
                let pq = r32(off(pp, 0xa9)) as *mut u8;
                if *off(pq, 7) as i8 != 3 {
                    return param_3;
                }
                u14 = r32(off(DAT_00a08104, 0xcb));
                let t4v = r32(off(DAT_00a08104, 0xc3));
                let bb = *off(DAT_00a08104, u6 as usize + 0x33c) as u32;
                t32 = bb.wrapping_mul(0x10).wrapping_sub(t4v);
            }
            param_3 = t32.wrapping_add(u14);
        }
        let mut u14: u32 = *off(DAT_00a08104, u6 as usize + 0x33c) as u32;
        let puv1 = off(DAT_00a08104, u6 as usize + 0x33c);
        if param_3 < u14.wrapping_mul(0x10) {
            let mut v = (u14.wrapping_mul(0x10).wrapping_sub(param_3) as i32) >> 4;
            if v == 0 {
                v = 1;
            }
            param_3 = 0;
            *puv1 = v as u8;
        } else {
            *puv1 = 0;
            param_3 = param_3.wrapping_sub(u14.wrapping_mul(0x10));
            if DAT_00a08294 == 0xe && DAT_009d6954 == 0 {
                iv7 = p1.wrapping_mul(0x23);
                let b = ent3(p1) as u32;
                let k = kind(b, u6 as i32);
                if k == 8 {
                    let local_34 = count(ent3(p1) as u32);
                    let mut u14b: u32 = 0;
                    let mut n: i32 = 0;
                    if 0 < local_34 {
                        loop {
                            if u6 != u14b {
                                let b2 = ent3(p1) as u32;
                                let k2 = kind(b2, u14b as i32);
                                if k2 == 8 && *off(DAT_00a08104, 0x33c + u14b as usize) != 0 {
                                    n = n.wrapping_add(1);
                                }
                            }
                            u14b = u14b.wrapping_add(1);
                            if !((u14b as i32) < local_34) {
                                break;
                            }
                        }
                    }
                    if n == 0 {
                        w32(off(DAT_00a08104, 0x215), 0);
                        let t = r32(off(DAT_00a08104, 0x215));
                        w32(off(DAT_00a08104, 0x211), t);
                    }
                }
            }
            u14 = p1;
            iv7 = u14.wrapping_mul(0x23);
            let u10 = FUN_004aeec0(ent3(p1) as u32, u6);
            if u10 != 0 {
                let mut p17: *mut u8 = &raw mut DAT_009e9699;
                *off(DAT_00a08104, u6 as usize + 0x2d8) = 2;
                loop {
                    let t16 = r16(p17.wrapping_sub(0xf));
                    let t16b = r16(p17);
                    if *p17.wrapping_sub(0x19) != 0 && t16 == param_1 && t16b == u15 {
                        let iv8 = count(ent3(p1) as u32);
                        loop {
                            let u4 = r16(p17);
                            w16(p17, u4.wrapping_add(1));
                            if iv8 <= u4.wrapping_add(1) as i32 {
                                w16(p17, 0);
                            }
                            let t = r16(p17);
                            if *off(DAT_00a08104, t as usize + 0x2d8) == 0 {
                                if FUN_00484e10(ent3(p1) as u32, t as u32) != 0 {
                                    break;
                                }
                            }
                            let t = r16(p17);
                            if t == u15 {
                                break;
                            }
                        }
                    }
                    p17 = p17.wrapping_add(0x5bd);
                    if !((p17 as usize) < ((&raw mut DAT_009ec480) as usize).wrapping_add(1)) {
                        break;
                    }
                }
                let mut b3 = DAT_009d8b75;
                if DAT_009d6941 != 0 {
                    DAT_009d6950 = DAT_009d6950.wrapping_add(1);
                    let mut iv8 = (DAT_009d6944 as u32).wrapping_add(0x32) as i32;
                    if *off(DAT_00a08104, u6 as usize + 0x30a) != 0 {
                        iv8 = (DAT_009d6944 as u32).wrapping_add(100) as i32;
                    }
                    DAT_009d6944 = iv8;
                    b3 = DAT_009d8b75.wrapping_add(2);
                    if 0x3b < DAT_009d8b75.wrapping_add(2) {
                        DAT_009d8b74 = DAT_009d8b74.wrapping_add(1);
                        b3 = DAT_009d8b75.wrapping_sub(0x3a);
                    }
                }
                DAT_009d8b75 = b3;
                u15 = FUN_00459750(0xd);
                if u15 != 0xffff {
                    let u10 = u15 as u32;
                    let iv8: u32 = u10.wrapping_mul(0x23);
                    let mut e1 = off(DAT_009a1fe8, iv7 as usize);
                    let mut e2 = off(DAT_009a1fe8, iv8 as usize);
                    w32(off(e2, 4), r32(off(e1, 4)));
                    w32(off(e2, 8), r32(off(e1, 8)));
                    w32(off(e2, 0xc), r32(off(e1, 0xc)));
                    let mut local_2c = FUN_004ae5a0(ent3(p1) as u32, u6);
                    let iv16 = FUN_004ae640(ent3(p1) as u32, u6);
                    let mut local_28 = FUN_004ae6e0(ent3(p1) as u32, u6);
                    if DAT_009d6941 != 0 {
                        let b3 = *off(DAT_00a08104, u6 as usize + 0x30a);
                        if b3 != 0 {
                            let s13 = ((b3 as u32).wrapping_mul(0u32.wrapping_sub(0x100))) as i16;
                            let l18 = FUN_0046a3c0(s13);
                            let l19 = FUN_0046a6d0(s13);
                            let mut iv9 = ((l19 as i16 as i32 as u32)
                                .wrapping_mul(local_2c)
                                .wrapping_add(
                                    (0u32.wrapping_sub(l18 as i16 as i32 as u32))
                                        .wrapping_mul(local_28 as u32),
                                )) as i32;
                            if 0x3fffffff < iv9 {
                                iv9 = 0x3fffffff;
                            }
                            if iv9 < -0x3fffffff {
                                iv9 = -0x3fff0000;
                            }
                            local_28 = ((l18 as i16 as i32 as u32)
                                .wrapping_mul(local_2c)
                                .wrapping_add(
                                    (l19 as i16 as i32 as u32).wrapping_mul(local_28 as u32),
                                )) as i32;
                            if 0x3fffffff < local_28 {
                                local_28 = 0x3fffffff;
                            }
                            if local_28 < -0x3fffffff {
                                local_28 = -0x3fff0000;
                            }
                            local_28 >>= 0xf;
                            local_2c = ((iv9 >> 0xf) as u32) & 0xffff;
                        }
                    }
                    DAT_009d8a58 = local_2c;
                    DAT_009d8a54 = local_28;
                    DAT_009d8b60 = 0u32.wrapping_sub(iv16 as u32) as i32;
                    FUN_004037b0(
                        off(DAT_009a1fe8, iv7 as usize),
                        local_2c,
                        local_28,
                        DAT_009d8b60,
                    );
                    e2 = off(DAT_009a1fe8, iv8 as usize);
                    w32(off(e2, 4), r32(off(e2, 4)).wrapping_add(DAT_009d8a58));
                    w32(
                        off(e2, 8),
                        r32(off(e2, 8)).wrapping_add(DAT_009d8a54 as u32),
                    );
                    w32(
                        off(e2, 0xc),
                        r32(off(e2, 0xc)).wrapping_add(DAT_009d8b60 as u32),
                    );
                    *off(DAT_009a1fe8, 3 + iv8 as usize) = 0x81;
                    *off(DAT_009a1fe8, 2 + iv8 as usize) = 0xd;
                    let mut pp = r32(off(e2, 0x1f)) as *mut u8;
                    *pp = 5;
                    *off(DAT_009a1fe8, 0x19 + iv8 as usize) = 2;
                    pp = r32(off(e2, 0x1f)) as *mut u8;
                    w16(off(pp, 0x83), 0);
                    pp = r32(off(e2, 0x1f)) as *mut u8;
                    w16(off(pp, 0x81), 0);
                    pp = r32(off(e2, 0x1f)) as *mut u8;
                    e1 = off(DAT_009a1fe8, iv7 as usize);
                    let pq = r32(off(e1, 0x1f)) as *mut u8;
                    w16(off(pp, 0x79), r16(off(pq, 0x79)));
                    w16(off(e2, 0x12), r16(off(e1, 0x12)));
                    w16(off(e2, 0x10), r16(off(e1, 0x10)));
                    w16(off(e2, 0x14), 0);
                    pp = r32(off(e2, 0x1f)) as *mut u8;
                    *off(pp, 0x92) = 1;
                    pp = r32(off(e2, 0x1f)) as *mut u8;
                    *off(pp, 0x8b) = *off(pp, 0x92);
                    let iv16b = DAT_009ecc34;
                    let u11 = FUN_00425cb0();
                    FUN_0042e4d0((u11 & 3).wrapping_add(0x16), u10, iv16b);
                    let mut v = FUN_004aedf0(ent3(p1) as u32, u6) >> 9;
                    if 0xff < v {
                        v = 0xff;
                    }
                    pp = r32(off(e2, 0x1f)) as *mut u8;
                    *off(pp, 1) = v as u8;
                }
                let t32 = r32(off(
                    &raw mut DAT_009e9670,
                    (DAT_009ecc34 as u32).wrapping_mul(0x5bd) as usize,
                ));
                if t32 == (param_4 & 0xffff) {
                    let u10 = ent3(p1) as u32;
                    let u12: u32;
                    if u10 < 0x49 {
                        if (u6 as i32) < 0 {
                            u12 = 0;
                        } else {
                            if t0(u10.wrapping_mul(0x65)) <= u6 as i32 {
                                u6 = t0(u10.wrapping_mul(0x65)).wrapping_sub(1) as u32;
                            }
                            u12 = t4(u6.wrapping_add(u10.wrapping_mul(0x65))) as u32;
                        }
                    } else {
                        u12 = FUN_004ae2a0(u10, u6) as u32;
                    }
                    match u12 {
                        4 | 5 | 0x15 => {
                            FUN_0042fb70(DAT_009ecc34, 0xffffffff, 0x17, 2, u14, 0x4000);
                        }
                        8 => {
                            FUN_0042fb70(DAT_009ecc34, 0xffffffff, 0x17, 3, u14, 0xffff);
                        }
                        10 | 0x16 => {
                            FUN_0042fb70(DAT_009ecc34, 0xffffffff, 0x17, 1, u14, 0xffff);
                        }
                        0xb | 0xc | 0x17 | 0x18 => {
                            FUN_0042fb70(DAT_009ecc34, 0xffffffff, 0x17, 4, u14, 0xffff);
                        }
                        _ => {}
                    }
                }
            }
        }
        param_3
    }
}
