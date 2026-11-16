// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0060f200: *mut i32;
    static mut DAT_0060f204: i32;
    static mut DAT_005272b0: i32;
    static mut DAT_0060f1c0: i32;
    static mut DAT_0060f1c8: *mut i32;
    static mut DAT_0060f1cc: *mut i32;
    static mut DAT_0060f1d8: i32;
    static mut DAT_0060f1dc: *mut i32;
    static mut DAT_0060f1e8: i32;
    static mut DAT_0060f1f4: *mut i32;
    static mut _DAT_0060f208: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00476280(param_1: i32, param_2: *mut i32, param_3: i32, param_4: i32);
    fn FUN_00479de0(param_1: i32, param_2: *mut i8) -> *mut i32;
    fn FUN_00475740(param_1: i32, param_2: *mut i32, param_3: i32, param_4: *mut u32);
    fn FUN_00475850(param_1: i32, param_2: *mut i32, param_3: i32, param_4: *mut u32);
    fn FUN_00475940(param_1: i32, param_2: *mut i32, param_3: i32, param_4: i32);
    fn FUN_00474910(param_1: *mut i8, param_2: i32, param_3: i32) -> i32;
}

unsafe fn gi(p: *const i32, i: usize) -> i32 {
    // SAFETY: caller supplies a valid game structure pointer.
    unsafe { p.add(i).read_unaligned() }
}

unsafe fn si(p: *mut i32, i: usize, v: i32) {
    // SAFETY: caller supplies a valid game structure pointer.
    unsafe { p.add(i).write_unaligned(v) }
}

unsafe fn wi(addr: u32, v: i32) {
    // SAFETY: the address is one the original writes through.
    unsafe { (addr as usize as *mut i32).write_unaligned(v) }
}

fn ip(v: i32) -> *mut i32 {
    v as usize as *mut i32
}

fn pi(p: *mut i32) -> i32 {
    p as usize as i32
}

fn pa(p: *mut i32, n: u32) -> *mut i32 {
    p.wrapping_add(n as usize)
}

unsafe fn copy_words(dst: *mut u8, src: *const u8, mut count: u32) {
    let mut d = dst;
    let mut s = src;
    while count != 0 {
        // SAFETY: mirrors the original's forward dword copy.
        unsafe {
            (d as *mut u32).write_unaligned((s as *const u32).read_unaligned());
        }
        d = d.wrapping_add(4);
        s = s.wrapping_add(4);
        count = count.wrapping_sub(1);
    }
}

unsafe fn copy_bytes(dst: *mut u8, src: *const u8, mut count: u32) {
    let mut d = dst;
    let mut s = src;
    while count != 0 {
        // SAFETY: mirrors the original's forward byte copy.
        unsafe {
            d.write(s.read());
        }
        d = d.wrapping_add(1);
        s = s.wrapping_add(1);
        count = count.wrapping_sub(1);
    }
}

unsafe fn copy_name(dst: *mut i32, src: *const u8) -> *mut i32 {
    let mut n: usize = 0;
    loop {
        // SAFETY: src is a NUL-terminated string; dst has room for it.
        let c = unsafe { src.add(n).read() };
        unsafe { (dst as *mut u8).add(n).write(c) };
        n += 1;
        if c == 0 {
            break;
        }
    }
    dst.wrapping_byte_add(n)
}

unsafe fn rf(p: *mut f32, i: isize) -> f32 {
    // SAFETY: caller supplies a valid float array.
    unsafe { p.offset(i).read_unaligned() }
}

unsafe fn fix_bbox(start: *mut f32, count: i32, cnt_ptr: *mut i32) {
    let mut pf = start;
    let mut cnt = count;
    // SAFETY: pf walks the float array the callee just filled.
    unsafe {
        let mut a = rf(pf, 0);
        let mut b = rf(pf, 1);
        let mut c = rf(pf, 2);
        let mut d = a;
        let mut e = b;
        let mut f = c;
        if 0 < cnt {
            loop {
                if rf(pf, 0) < a {
                    a = rf(pf, 0);
                }
                if rf(pf, 1) < b {
                    b = rf(pf, 1);
                }
                if rf(pf, 2) < c {
                    c = rf(pf, 2);
                }
                if d < rf(pf, 0) {
                    d = rf(pf, 0);
                }
                if e < rf(pf, 1) {
                    e = rf(pf, 1);
                }
                if f < rf(pf, 2) {
                    f = rf(pf, 2);
                }
                pf = pf.wrapping_add(3);
                cnt = cnt.wrapping_sub(1);
                if cnt == 0 {
                    break;
                }
            }
        }
        if rf(pf, -6) != a
            || rf(pf, -5) != b
            || rf(pf, -4) != c
            || rf(pf, -3) != d
            || rf(pf, -2) != e
            || rf(pf, -1) != f
        {
            pf.write_unaligned(a);
            pf.add(1).write_unaligned(b);
            pf.add(2).write_unaligned(c);
            pf.add(3).write_unaligned(d);
            pf.add(4).write_unaligned(e);
            pf.add(5).write_unaligned(f);
            cnt_ptr.write_unaligned((cnt_ptr.read_unaligned() as u32).wrapping_add(2) as i32);
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00474960(
    param_1: *mut i32,
    param_2: *mut i32,
    param_3: i32,
    param_4: i32,
    param_5: *mut u32,
) -> i32 {
    if param_2.is_null() {
        return 0;
    }
    // SAFETY: the globals and callees are the game's own; all pointers are
    // the game's structures, read and written where the original does.
    unsafe {
        let mut b3: u8 = 1;
        let mut p11: *mut i32 = core::ptr::null_mut();
        let i4 = gi(param_2, 1);
        let mut uf0: u32 = 0;
        let mut p14: *mut i32 = param_1;

        match i4 {
            1 | 0xf | 0x10 | 0x11 => {
                if gi(ip(gi(param_2, 5)), 0) < 0 {
                    b3 = 0;
                } else if DAT_0060f200.is_null() {
                    p11 = param_1.add(6);
                    si(param_1, 1, i4);
                    if gi(param_2, 0) == 0 {
                        si(param_1, 0, 0);
                    } else {
                        si(param_1, 0, pi(p11));
                        p11 = copy_name(p11, gi(param_2, 0) as usize as *const u8);
                    }
                    si(param_1, 4, gi(param_2, 4));
                    si(param_1, 5, pi(p11));
                    let n4 = gi(param_2, 4) as u32;
                    let iv4: u32 = if DAT_005272b0 == 0 {
                        n4.wrapping_shl(6)
                    } else {
                        n4.wrapping_mul(0x30)
                    };
                    copy_words(
                        p11 as *mut u8,
                        gi(param_2, 5) as usize as *const u8,
                        iv4.wrapping_add(4) >> 2,
                    );
                    let mut pc12 = (p11 as *mut u8).wrapping_add(iv4.wrapping_add(4) as usize);
                    let n = gi(param_2, 4) as u32;
                    copy_words(
                        pc12,
                        gi(param_2, 5) as usize as *const u8,
                        (n & 0xfffffff) << 2,
                    );
                    pc12 = pc12.wrapping_add(n.wrapping_mul(0x10) as usize);
                    let iv4b = gi(param_2, 4) as u32;
                    let off = iv4b.wrapping_mul(0x24);
                    uf0 = off;
                    let iv10: u32 = if DAT_005272b0 == 0 {
                        iv4b.wrapping_shl(6)
                    } else {
                        iv4b.wrapping_mul(0x30)
                    };
                    let pc9 = (gi(param_2, 5) as u32).wrapping_add(iv10).wrapping_add(4);
                    copy_words(pc12, pc9 as usize as *const u8, uf0 >> 2);
                    let iv5 = DAT_0060f204;
                    p14 = pc12.wrapping_add(off as usize) as *mut i32;
                    if param_5.add(0x21).read() == 0 {
                        uf0 = (DAT_0060f204 as u32).wrapping_mul(0xc);
                        let src = pc9.wrapping_add(off);
                        copy_words(
                            p14 as *mut u8,
                            src as usize as *const u8,
                            (DAT_0060f204 as u32).wrapping_mul(3) & 0x3fffffff,
                        );
                        p14 = pa(p14, (iv5 as u32).wrapping_mul(3));
                    }
                    b3 = 0;
                    p11 = param_1;
                } else {
                    p14 = param_1.add(6);
                    si(param_1, 1, i4);
                    if gi(param_2, 0) == 0 {
                        si(param_1, 0, 0);
                    } else {
                        si(param_1, 0, pi(p14));
                        p14 = copy_name(p14, gi(param_2, 0) as usize as *const u8);
                    }
                    si(param_1, 4, 0);
                    si(param_1, 5, pi(p14));
                    p14.write_unaligned(0);
                    FUN_00476280(pi(param_1), param_2, param_3, param_5 as usize as i32);
                    b3 = 0;
                    p11 = param_1;
                    p14 = pa(p14, (gi(param_1, 4) as u32).wrapping_mul(0x19)).wrapping_add(1);
                }
            }
            2 => {
                uf0 = 0x30;
            }
            3 => {
                DAT_0060f1c0 = gi(param_2, 5);
                DAT_0060f204 = gi(param_2, 4);
                if DAT_0060f200.is_null() {
                    uf0 = (gi(param_2, 4) as u32).wrapping_mul(0xc);
                } else {
                    b3 = 0;
                }
            }
            4 => {
                uf0 = 0xc;
            }
            5 => {
                uf0 = 0x24;
            }
            6 => {
                uf0 = 0xc;
            }
            7 => {
                uf0 = 0xffffffff;
                let mut pc = gi(param_2, 5) as usize as *const i8;
                loop {
                    if uf0 == 0 {
                        break;
                    }
                    uf0 = uf0.wrapping_sub(1);
                    let c = pc.read();
                    pc = pc.wrapping_add(1);
                    if c == 0 {
                        break;
                    }
                }
                uf0 = !uf0;
                p11 = param_2;
                loop {
                    if gi(p11, 1) != 7 {
                        break;
                    }
                    p11 = FUN_00479de0(param_3, gi(p11, 5) as usize as *mut i8);
                    if p11.is_null() {
                        break;
                    }
                }
                if !p11.is_null() && gi(p11, 1) == 0x14 {
                    DAT_0060f1dc = p11;
                }
            }
            9 => {
                uf0 = (gi(param_2, 4) as u32).wrapping_mul(0x38);
                DAT_0060f1d8 = gi(param_2, 5);
            }
            0xb => {
                _DAT_0060f208 = gi(param_2, 5);
                param_5.add(0x21).write(_DAT_0060f208 as u32);
                if DAT_0060f1cc.is_null() {
                    uf0 = (gi(param_2, 4) as u32).wrapping_mul(0xc);
                } else {
                    b3 = 0;
                }
            }
            0xd => {
                DAT_0060f1e8 = gi(param_2, 5);
                if DAT_0060f1f4.is_null() {
                    uf0 = (gi(param_2, 4) as u32).wrapping_shl(3);
                } else {
                    b3 = 0;
                }
            }
            0x13 => {
                uf0 = 0xc;
            }
            0x14 => {
                DAT_0060f1dc = param_2;
                let q = ip(gi(param_2, 5));
                let mut iv4 = (gi(q, 5) as u32).wrapping_mul(gi(q, 4) as u32);
                let mut iv10 = iv4;
                if iv4.wrapping_sub(gi(q, 2) as u32) == 0 {
                    iv10 = gi(q, 3) as u32;
                }
                uf0 = iv10.wrapping_add(0x18);
                if gi(q, 1) == 0 {
                    if iv4.wrapping_sub(gi(q, 2) as u32) == 0 {
                        iv4 = gi(q, 3) as u32;
                    }
                    if gi(q, 0) as u32 == (pi(q) as u32).wrapping_add(iv4).wrapping_add(0x18) {
                        uf0 = iv10.wrapping_add(0x3018);
                    }
                } else {
                    uf0 = uf0.wrapping_add((gi(q, 1) as u32).wrapping_mul(0x300));
                }
            }
            0x15 => {
                DAT_0060f1c8 = param_2;
                if DAT_0060f200.is_null() {
                    si(param_1, 1, 0);
                    p11 = param_1.add(6);
                    if gi(param_2, 0) == 0 {
                        si(param_1, 0, 0);
                    } else {
                        let start = p11;
                        p11 = copy_name(p11, gi(param_2, 0) as usize as *const u8);
                        si(param_1, 0, pi(start));
                    }
                    p14 = p11.add(4);
                    si(param_1, 5, 0);
                    si(param_1, 4, 0);
                    si(param_1, 2, 4);
                    si(param_1, 3, pi(p11));
                    p11.write_unaligned(pi(p14));
                    si(p11, 5, 3);
                    p14.write_unaligned(0);
                    si(p11, 9, pi(p11.add(10)));
                    si(p11, 8, 0);
                    si(p11, 6, 0);
                    si(p11, 7, 0);
                    FUN_00475740(pi(p14), param_2, param_3, param_5);
                    DAT_0060f200 = p14;
                    fix_bbox(gi(p11, 9) as usize as *mut f32, gi(p11, 8), p11.add(8));
                    p11 = pa(p11.add(10), (gi(p11, 8) as u32).wrapping_mul(3));
                    wi((gi(param_1, 3) as u32).wrapping_add(4), pi(p11));
                    si(p11, 1, 0xd);
                    p11.write_unaligned(0);
                    si(p11, 5, pi(p11.add(6)));
                    si(p11, 4, 0);
                    si(p11, 2, 0);
                    si(p11, 3, 0);
                    FUN_00475850(pi(p11), param_2, param_3, param_5);
                    p14 = pa(p11.add(6), (gi(p11, 4) as u32).wrapping_mul(2));
                    DAT_0060f1f4 = p11;
                    wi((gi(param_1, 3) as u32).wrapping_add(8), pi(p14));
                    si(p14, 1, 0xb);
                    let pu18 = param_5.add(0x21);
                    p14.write_unaligned(0);
                    si(p14, 5, pi(p14.add(6)));
                    si(p14, 4, 0);
                    si(p14, 2, 0);
                    si(p14, 3, 0);
                    let iv4 = DAT_0060f204;
                    let uv2 = pu18.read();
                    pu18.write(0);
                    FUN_00475940(pi(p14), param_2, param_3, param_5 as usize as i32);
                    pu18.write(uv2);
                    p11 = pa(p14.add(6), (gi(p14, 4) as u32).wrapping_mul(3));
                    DAT_0060f1cc = p14;
                    DAT_0060f204 = iv4;
                    wi((gi(param_1, 3) as u32).wrapping_add(0xc), pi(p11));
                    si(p11, 1, 0x15);
                    p14 = p11.add(6);
                    p11.write_unaligned(0);
                    si(p11, 5, pi(p14));
                    si(p11, 4, gi(param_2, 4));
                    copy_words(
                        p14 as *mut u8,
                        gi(param_2, 5) as usize as *const u8,
                        (gi(param_2, 4) as u32) & 0x3fffffff,
                    );
                    let a = gi(param_2, 4);
                    let b = gi(param_2, 2);
                    p14 = pa(p14, a as u32);
                    let diff = (gi(param_2, 2) as u32).wrapping_sub(gi(param_2, 4) as u32);
                    if a < b {
                        si(p11, 4, b);
                        let mut z = p14;
                        let mut cnt = diff & 0x3fffffff;
                        while cnt != 0 {
                            z.write_unaligned(0);
                            z = z.wrapping_add(1);
                            cnt = cnt.wrapping_sub(1);
                        }
                        b3 = 0;
                        p14 = pa(p14, diff);
                    } else {
                        if b < a {
                            si(p11, 4, b);
                            p14 = pa(p14, diff);
                        }
                        b3 = 0;
                    }
                } else {
                    p14 = param_1.add(6);
                    p11 = param_1.add(4);
                    si(param_1, 1, 0x15);
                    param_1.write_unaligned(0);
                    si(param_1, 5, pi(p14));
                    p11.write_unaligned(gi(param_2, 4));
                    copy_words(
                        p14 as *mut u8,
                        gi(param_2, 5) as usize as *const u8,
                        (gi(param_2, 4) as u32) & 0x3fffffff,
                    );
                    let a = gi(param_2, 4);
                    let b = gi(param_2, 2);
                    p14 = pa(p14, a as u32);
                    let diff = (gi(param_2, 2) as u32).wrapping_sub(gi(param_2, 4) as u32);
                    if a < b {
                        p11.write_unaligned(b);
                        let mut z = p14;
                        let mut cnt = diff & 0x3fffffff;
                        while cnt != 0 {
                            z.write_unaligned(0);
                            z = z.wrapping_add(1);
                            cnt = cnt.wrapping_sub(1);
                        }
                        b3 = 0;
                        p11 = param_1;
                        p14 = pa(p14, diff);
                    } else {
                        if b < a {
                            p11.write_unaligned(b);
                            p14 = pa(p14, diff);
                        }
                        b3 = 0;
                        p11 = param_1;
                    }
                }
            }
            0x16 => {
                uf0 = 0x10;
            }
            0x17 => {
                uf0 = 0x30;
            }
            0x19 => {
                uf0 = 0x48;
            }
            _ => {}
        }

        let mut p13: *mut i32 = p14;
        if b3 != 0 {
            p13 = p14.add(6);
            si(p14, 1, gi(param_2, 1));
            if gi(param_2, 0) == 0 {
                p14.write_unaligned(0);
            } else {
                p14.write_unaligned(pi(p13));
                p13 = copy_name(p13, gi(param_2, 0) as usize as *const u8);
            }
            si(p14, 4, gi(param_2, 4));
            si(p14, 5, pi(p13));
            let src = gi(param_2, 5) as usize as *const u8;
            copy_words(p13 as *mut u8, src, uf0 >> 2);
            copy_bytes(
                (p13 as *mut u8).wrapping_add((uf0 & !3) as usize),
                src.wrapping_add((uf0 & !3) as usize),
                uf0 & 3,
            );
            p13 = p13.wrapping_byte_add(uf0 as usize);
            p11 = p14;
            if gi(param_2, 1) == 0x14 {
                let pu18 = gi(param_2, 5) as usize as *mut u32;
                if pu18.add(1).read_unaligned() == 0 {
                    let mut iv4 = pu18
                        .add(5)
                        .read_unaligned()
                        .wrapping_mul(pu18.add(4).read_unaligned());
                    if pu18.add(2).read_unaligned() == iv4 {
                        iv4 = pu18.add(3).read_unaligned();
                    }
                    if pu18.read_unaligned()
                        == (pu18 as usize as u32).wrapping_add(iv4).wrapping_add(0x18)
                    {
                        p14 = ip(gi(p14, 5));
                        let mut iv4 = (gi(p14, 5) as u32).wrapping_mul(gi(p14, 4) as u32);
                        if gi(p14, 2) as u32 == iv4 {
                            iv4 = gi(p14, 3) as u32;
                        }
                        p14.write_unaligned(
                            (pi(p14) as u32).wrapping_add(iv4).wrapping_add(0x18) as i32
                        );
                    } else {
                        let r = FUN_00474910(
                            pu18.read_unaligned() as usize as *mut i8,
                            param_4,
                            pi(p14),
                        );
                        if r == 0 {
                            let p16 = pu18.read_unaligned() as usize as *const u8;
                            wi(gi(p14, 5) as u32, pi(p13));
                            p14 = p13;
                            copy_words(p14 as *mut u8, p16, 0xc00);
                            p13 = p13.wrapping_add(0xc00);
                        } else {
                            wi(gi(p14, 5) as u32, r);
                        }
                    }
                }
            }
        }

        p14 = p13;
        if p11.is_null() {
            if gi(param_2, 2) == 0 {
                return 0;
            }
            p14 = p13.add(6);
            p13.write_unaligned(0);
            si(p13, 1, 0);
            si(p13, 4, 0);
            si(p13, 5, 0);
            p11 = p13;
        }
        si(p11, 2, 0);
        si(p11, 3, 0);
        let iv4 = gi(param_2, 2);
        if iv4 != 0 {
            let mut iv10: i32;
            if DAT_0060f200.is_null() {
                si(p11, 2, (iv4 as u32).wrapping_add(3) as i32);
                si(p11, 3, pi(p14));
                p13 = pa(p14, gi(p11, 2) as u32);
                p14.write_unaligned(pi(p13));
                si(p13, 1, 3);
                p13.write_unaligned(0);
                si(p13, 5, pi(p13.add(6)));
                si(p13, 4, 0);
                si(p13, 2, 0);
                si(p13, 3, 0);
                FUN_00475740(pi(p13), param_2, param_3, param_5);
                DAT_0060f200 = p13;
                fix_bbox(gi(p13, 5) as usize as *mut f32, gi(p13, 4), p13.add(4));
                p14 = pa(p13.add(6), (gi(p13, 4) as u32).wrapping_mul(3));
                wi((gi(p11, 3) as u32).wrapping_add(4), pi(p14));
                si(p14, 1, 0xd);
                p14.write_unaligned(0);
                si(p14, 5, pi(p14.add(6)));
                si(p14, 4, 0);
                si(p14, 2, 0);
                si(p14, 3, 0);
                FUN_00475850(pi(p14), param_2, param_3, param_5);
                p13 = pa(p14.add(6), (gi(p14, 4) as u32).wrapping_mul(2));
                DAT_0060f1f4 = p14;
                wi((gi(p11, 3) as u32).wrapping_add(8), pi(p13));
                si(p13, 1, 0xb);
                let pu18 = param_5.add(0x21);
                p13.write_unaligned(0);
                si(p13, 5, pi(p13.add(6)));
                si(p13, 4, 0);
                si(p13, 2, 0);
                si(p13, 3, 0);
                let sv = DAT_0060f204;
                let uv2 = pu18.read();
                pu18.write(0);
                FUN_00475940(pi(p13), param_2, param_3, param_5 as usize as i32);
                pu18.write(uv2);
                iv10 = 3;
                p14 = pa(p13.add(6), (gi(p13, 4) as u32).wrapping_mul(3));
                DAT_0060f1cc = p13;
                DAT_0060f204 = sv;
            } else {
                iv10 = 0;
                si(p11, 2, iv4);
                si(p11, 3, pi(p14));
                p14 = pa(p14, gi(p11, 2) as u32);
            }
            let mut aux: [u32; 52] = [0; 52];
            let mut i = 0usize;
            while i < 0x34 {
                aux[i] = param_5.add(i).read_unaligned();
                i += 1;
            }
            let mut iv4: i32 = 0;
            let mut cnt: i32 = 0;
            if 0 < gi(param_2, 2) {
                iv10 = (iv10 as u32).wrapping_mul(4) as i32;
                loop {
                    wi((gi(p11, 3) as u32).wrapping_add(iv10 as u32), pi(p14));
                    let child = ip(((gi(param_2, 3) as u32).wrapping_add(iv4 as u32) as usize
                        as *const i32)
                        .read_unaligned());
                    let iv5 = FUN_00474960(p14, child, param_3, param_4, aux.as_mut_ptr());
                    if iv5 == 0 {
                        wi((gi(p11, 3) as u32).wrapping_add(iv10 as u32), 0);
                    }
                    p14 = p14.wrapping_byte_add(iv5 as usize);
                    iv4 = iv4.wrapping_add(4);
                    iv10 = iv10.wrapping_add(4);
                    cnt = cnt.wrapping_add(1);
                    if !(cnt < gi(param_2, 2)) {
                        break;
                    }
                }
            }
        }
        (pi(p14) as u32).wrapping_sub(pi(param_1) as u32) as i32
    }
}
