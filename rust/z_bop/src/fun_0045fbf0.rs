// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a73b0: u16;
    static mut DAT_009a73d8: u8;
    static mut DAT_009a1fe8: u32;
    static mut DAT_00a08104: u32;
    static mut DAT_009d8c84: [u8; 0];
    static mut DAT_009d9155: [u8; 0];
    static mut DAT_00a08144: i32;
    static mut DAT_00a08140: i32;
    static mut DAT_00a08148: i32;
    static mut DAT_009a73c4: i32;
    static mut DAT_009a73c8: i32;
    static mut DAT_009a73cc: i32;
    static mut DAT_0051c5b2: [u8; 0];
    static mut DAT_009d8a58: i32;
    static mut DAT_009d8a54: i32;
    static mut DAT_009d8b60: i32;
    static mut DAT_009ec604: i32;
    static mut DAT_00a081e8: i32;
    static mut DAT_00a08298: i32;
    static mut DAT_009a1ff8: u32;
    static mut DAT_00521f1c: [u8; 0];
    static mut DAT_009a1fd4: i32;
    static mut DAT_009a1fd8: i32;
    static mut DAT_009a1fd0: i32;
    static mut DAT_00a07cdc: i32;
    static mut DAT_00a07cd8: i32;
    static mut DAT_00a07cd4: i32;
    static mut DAT_009a73b6: [u8; 4];
    static mut DAT_009d1320: [i8; 0];
    static mut DAT_00521690: [i8; 0];
    static mut s_disableldr1pln_0051a310: [i8; 0];
    static mut DAT_00a07cec: u16;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a73d0: u16;
    static mut DAT_005243c8: [i32; 0];
    static mut DAT_009e9640: u16;
    static mut DAT_00a00854: i32;
    static mut DAT_009a73c0: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004035d0(param_1: u32, param_2: i16, param_3: i16, param_4: i16);
    fn FUN_00403540(param_1: u32, param_2: u32);
    fn FUN_0041f410(param_1: i32, param_2: i32, param_3: i32) -> u32;
    fn FUN_00458e20(param_1: u32, param_2: i32);
    fn FUN_004a5490(param_1: u16, param_2: u16) -> i32;
    fn FUN_00403360(param_1: u32, param_2: i32) -> u8;
    fn FUN_00407090(param_1: u32, param_2: i32) -> u32;
}

#[inline(always)]
unsafe fn rb(a: u32) -> u8 {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *const u8).read() }
}

#[inline(always)]
unsafe fn wb(a: u32, v: u8) {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *mut u8).write(v) }
}

#[inline(always)]
unsafe fn r16(a: u32) -> u16 {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *const u16).read_unaligned() }
}

#[inline(always)]
unsafe fn r32(a: u32) -> u32 {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn w16(a: u32, v: u16) {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *mut u16).write_unaligned(v) }
}

#[inline(always)]
unsafe fn add16(a: u32, v: u16) {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe {
        let t = (a as usize as *const u16).read_unaligned();
        (a as usize as *mut u16).write_unaligned(t.wrapping_add(v));
    }
}

#[inline(always)]
unsafe fn g6() -> u32 {
    // SAFETY: DAT_009a73b6 is the game's own packed 4-byte pointer.
    unsafe { (&raw const DAT_009a73b6).cast::<u32>().read_unaligned() }
}

#[inline(always)]
unsafe fn ent(i: u32) -> u32 {
    // SAFETY: reads the game's own table base pointer.
    unsafe { DAT_009a1fe8.wrapping_add(i.wrapping_mul(0x23)) }
}

unsafe fn cmp(mut a: u32, mut s: u32, n: u32) -> bool {
    // SAFETY: both addresses point at the game's own strings, read as the original does.
    unsafe {
        let mut cnt = n;
        let mut bv = true;
        loop {
            if cnt == 0 {
                break;
            }
            cnt = cnt.wrapping_sub(1);
            bv = rb(a) == rb(s);
            a = a.wrapping_add(1);
            s = s.wrapping_add(1);
            if !bv {
                break;
            }
        }
        bv
    }
}

unsafe fn valid(u9: u32) -> u32 {
    // SAFETY: the globals, records and callee are the game's own.
    unsafe {
        if u9 == 0xffff {
            return 0;
        }
        let iv7 = u9.wrapping_mul(0x23);
        let mut iv5 = r32(DAT_009a1fe8.wrapping_add(iv7).wrapping_add(0x1f));
        if iv5 == 0 {
            let bv13 = rb(DAT_009a1fe8.wrapping_add(iv7).wrapping_add(3)) == 0;
            return 1u32.wrapping_sub(bv13 as u32);
        }
        if rb(DAT_009a1fe8.wrapping_add(iv7).wrapping_add(3)) == 0 {
            return 0;
        }
        iv5 = r32(iv5.wrapping_add(0xa9));
        if iv5 == 0 {
            return 1;
        }
        if rb(iv5.wrapping_add(0x5e)) == 0x15 && rb(iv5.wrapping_add(0x5f)) != 0 {
            return 0;
        }
        let c = rb(iv5.wrapping_add(7)) as i8;
        if c == 1 || c == 3 || c == 4 {
            return 0;
        }
        if rb(iv5.wrapping_add(0x23d)) == 3 && rb(iv5.wrapping_add(0x241)) != 0 {
            FUN_00403540(u9, DAT_009a73b0 as u32);
            let c2 = rb(ent(DAT_009a73b0 as u32).wrapping_add(2)) as i8;
            if c2 == 0 || c2 == 1 || c2 == 2 {
                if (DAT_009a1ff8 as i32) >= 0x8001 {
                    return 0;
                }
            } else if (DAT_009a1ff8 as i32) >= 0x10001 {
                return 0;
            }
        }
        let i1 = r32(DAT_009a1fe8.wrapping_add(iv7).wrapping_add(0x1b));
        if i1 == 0xffff_ffff {
            let t1 = r32(DAT_009a1fe8.wrapping_add(iv7).wrapping_add(0x1f));
            let t2 = r32(t1.wrapping_add(0xa9));
            let t3 = r32(t2.wrapping_add(0xcb));
            let t1b = r32(iv5.wrapping_add(0xc3));
            return 1u32.wrapping_sub((t3 < t1b) as u32);
        }
        1
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0045fbf0() -> u32 {
    // SAFETY: every global, record pointer and callee is the game's own memory
    // and code; reads and writes occur where the original performs them.
    unsafe {
        let iv3 = ent(DAT_009a73b0 as u32);
        DAT_009a73d8 = 0;
        let b14 = rb(iv3.wrapping_add(3)) == b'6';
        let u6 = rb(iv3.wrapping_add(0x16)) as u32;
        let a8c84 = (&raw mut DAT_009d8c84) as usize as u32;
        let a9155 = (&raw mut DAT_009d9155) as usize as u32;
        let a51c5b2 = (&raw mut DAT_0051c5b2) as usize as u32;
        let a521f1c = (&raw mut DAT_00521f1c) as usize as u32;
        let ad1320 = (&raw mut DAT_009d1320) as usize as u32;
        let a521690 = (&raw mut DAT_00521690) as usize as u32;
        let a51a310 = (&raw mut s_disableldr1pln_0051a310) as usize as u32;

        let mut local_12: u16 = 0;
        if rb(a8c84.wrapping_add(u6.wrapping_mul(0x566))) != 0x0e
            && rb(a9155.wrapping_add(u6.wrapping_mul(0x566))) != 0x0e
            && {
                local_12 = 0;
                rb(DAT_00a08104.wrapping_add(0x21d)) != 0
            }
        {
            loop {
                'iter: {
                    let u6 = local_12 as u32;
                    let iv3 = DAT_00a08104.wrapping_add(u6.wrapping_mul(4));
                    let p4 = iv3.wrapping_add(0x3b2);
                    let sh1 = r16(iv3.wrapping_add(0x3b4)) as i16;
                    if !(rb(iv3.wrapping_add(0x370)) == 2 && sh1 < 1) {
                        break 'iter;
                    }
                    w16(p4, 0xffff);
                    wb(
                        u6.wrapping_mul(4)
                            .wrapping_add(0x372)
                            .wrapping_add(DAT_00a08104),
                        0,
                    );
                    let sh2 = r16(DAT_00a08104.wrapping_add(0x1e5)) as i16;
                    let sh3 = r16(DAT_00a08104.wrapping_add(0x1e7)) as i16;
                    if !(rb(DAT_00a08104.wrapping_add(7)) != 3 && sh2 != 0 && sh3 == 0) {
                        break 'iter;
                    }
                    DAT_00a08144 = DAT_009a73c4;
                    DAT_00a08140 = DAT_009a73c8;
                    DAT_00a08148 = DAT_009a73cc;
                    if !b14 {
                        let iv7 = (rb(DAT_00a08104.wrapping_add(4)) as u32).wrapping_mul(0x101);
                        let iv5 = iv7.wrapping_add(u6.wrapping_mul(8));
                        let s1 = r16(a51c5b2.wrapping_add(iv5)) as i16;
                        let s2 = r16(a51c5b2.wrapping_add(2).wrapping_add(iv5)) as i16;
                        let s3 = r16(a51c5b2
                            .wrapping_add(4)
                            .wrapping_add(u6.wrapping_mul(8))
                            .wrapping_add(iv7)) as i16;
                        FUN_004035d0(ent(DAT_009a73b0 as u32), s1, s2, s3);
                        if rb(ent(DAT_009a73b0 as u32).wrapping_add(3)) == b'5' {
                            DAT_009d8a58 = (DAT_009d8a58 as u32).wrapping_mul(2) as i32;
                            DAT_009d8a54 = (DAT_009d8a54 as u32).wrapping_mul(2) as i32;
                            DAT_009d8b60 = (DAT_009d8b60 as u32).wrapping_mul(2) as i32;
                        }
                        DAT_009ec604 =
                            (DAT_00a08144 as u32).wrapping_add(DAT_009d8a58 as u32) as i32;
                        DAT_00a08140 =
                            (DAT_00a08140 as u32).wrapping_add(DAT_009d8a54 as u32) as i32;
                        DAT_00a08148 =
                            (DAT_00a08148 as u32).wrapping_add(DAT_009d8b60 as u32) as i32;
                        DAT_00a08144 = DAT_009ec604;
                        DAT_00a081e8 = DAT_00a08140;
                        DAT_00a08298 = DAT_00a08148;
                    }
                    let u9 = r16(DAT_00a08104.wrapping_add(0x6a));
                    let ok = valid(u9 as u32);
                    'l159: {
                        if ok == 0 {
                            w16(DAT_00a08104.wrapping_add(0x6a), 0xffff);
                        } else {
                            let iv5 = ent(u9 as u32);
                            let i1 = r32(iv5.wrapping_add(4));
                            let i2 = r32(iv5.wrapping_add(8));
                            let i3 = r32(iv5.wrapping_add(0xc));
                            DAT_009a1ff8 = FUN_0041f410(
                                i1.wrapping_sub(DAT_00a08144 as u32) as i32,
                                i2.wrapping_sub(DAT_00a08140 as u32) as i32,
                                i3.wrapping_sub(DAT_00a08148 as u32) as i32,
                            );
                            let mut lim: u32 = 0x10000;
                            if b14 {
                                let t1 = r32(a521f1c.wrapping_add(
                                    (rb(ent(DAT_009a73b0 as u32).wrapping_add(3)) as u32)
                                        .wrapping_mul(0x1a),
                                ));
                                lim = t1.wrapping_add(0x10000);
                            }
                            if DAT_009a1ff8 < lim {
                                let mut bv13 = true;
                                if !b14 {
                                    FUN_00458e20(u9 as u32, 0);
                                    DAT_009a1fd4 = DAT_00a07cdc;
                                    DAT_009a1fd8 = DAT_00a07cd8;
                                    DAT_009a1fd0 = DAT_00a07cd4;
                                    let r = FUN_004a5490(DAT_009a73b0, DAT_009a73b0) as u32;
                                    bv13 = r == 0;
                                }
                                if bv13 {
                                    let g = g6();
                                    let eq = cmp(
                                        ad1320.wrapping_add(
                                            (rb(g.wrapping_add(0xb)) as u32).wrapping_mul(0x55),
                                        ),
                                        a521690,
                                        10,
                                    );
                                    let s1 = r16(g6().wrapping_add(0x18));
                                    if !eq || s1 != u9 {
                                        let g = g6();
                                        let eq2 = cmp(
                                            ad1320.wrapping_add(
                                                (rb(g.wrapping_add(0xb)) as u32).wrapping_mul(0x55),
                                            ),
                                            a51a310,
                                            0xf,
                                        );
                                        let s1 = r16(g6().wrapping_add(0x18));
                                        if eq2 && u9 == s1 {
                                            let s2 = r16(g6().wrapping_add(0x18));
                                            let t1 = r32((s2 as u32)
                                                .wrapping_mul(0x23)
                                                .wrapping_add(0x1f)
                                                .wrapping_add(DAT_009a1fe8));
                                            let t2 = r32(t1.wrapping_add(0xa5 + 4));
                                            let t3 = r32(t2.wrapping_add(0x211));
                                            if t3 == 0 {
                                                break 'l159;
                                            }
                                            w16(p4, u9);
                                            add16(iv3.wrapping_add(0x3b4), 0x1d8);
                                        } else {
                                            if b14 {
                                                let mut iv7: u32 = 0;
                                                let mut iv5: i32 = 0;
                                                if rb(DAT_00a08104.wrapping_add(0x21d)) != 0 {
                                                    let mut p8 = DAT_00a08104.wrapping_add(0x3b2);
                                                    loop {
                                                        let s2 = r16(p8);
                                                        if s2 == u9 {
                                                            iv7 = iv7.wrapping_add(1);
                                                            if iv7 == 2 {
                                                                break 'iter;
                                                            }
                                                        }
                                                        p8 = p8.wrapping_add(4);
                                                        iv5 = iv5.wrapping_add(1);
                                                        if !(iv5
                                                            < rb(DAT_00a08104.wrapping_add(0x21d))
                                                                as i32)
                                                        {
                                                            break;
                                                        }
                                                    }
                                                }
                                                if iv7 == 2 {
                                                    break 'iter;
                                                }
                                            }
                                            w16(p4, u9);
                                            add16(iv3.wrapping_add(0x3b4), 0x1d8);
                                        }
                                        break 'iter;
                                    }
                                }
                            }
                        }
                    }
                    let mut local_4: u32 = 0x10000;
                    let mut local_14: u16 = 0xffff;
                    if b14 {
                        let t1 = r32(a521f1c.wrapping_add(
                            (rb(ent(DAT_009a73b0 as u32).wrapping_add(3)) as u32)
                                .wrapping_mul(0x1a),
                        ));
                        local_4 = t1.wrapping_add(0x10000);
                    }
                    let mut u9 = DAT_00a07cec;
                    if (u9 as i32) < DAT_009a1fe0 {
                        loop {
                            let u6 = u9 as u32;
                            let e = ent(u6);
                            if rb(e.wrapping_add(3)) != 0 {
                                let t1 = r32(e.wrapping_add(0x1f));
                                let o = r32(t1.wrapping_add(0xa9));
                                let s1 = r16(o.wrapping_add(0x42));
                                if s1 == DAT_009a73b0 {
                                    let c = rb(o.wrapping_add(0x5e)) as i8;
                                    if c == 0x0c || c == 0x17 {
                                        let ok2 = valid(u9 as u32);
                                        if ok2 != 0 {
                                            let iv5 = ent(u6);
                                            let i1 = r32(iv5.wrapping_add(4));
                                            let i2 = r32(iv5.wrapping_add(8));
                                            let i3 = r32(iv5.wrapping_add(0xc));
                                            DAT_009a1ff8 = FUN_0041f410(
                                                i1.wrapping_sub(DAT_00a08144 as u32) as i32,
                                                i2.wrapping_sub(DAT_00a08140 as u32) as i32,
                                                i3.wrapping_sub(DAT_00a08148 as u32) as i32,
                                            );
                                            if b14 {
                                                let mut iv5: i32 = 0;
                                                if rb(DAT_00a08104.wrapping_add(0x21d)) != 0 {
                                                    let mut p8 = DAT_00a08104.wrapping_add(0x3b2);
                                                    loop {
                                                        let s2 = r16(p8);
                                                        if s2 == u9 {
                                                            DAT_009a1ff8 =
                                                                DAT_009a1ff8.wrapping_add(0x8000);
                                                        }
                                                        p8 = p8.wrapping_add(4);
                                                        iv5 = iv5.wrapping_add(1);
                                                        if !(iv5
                                                            < rb(DAT_00a08104.wrapping_add(0x21d))
                                                                as i32)
                                                        {
                                                            break;
                                                        }
                                                    }
                                                }
                                            }
                                            if DAT_009a1ff8 < local_4 {
                                                let mut bv13 = true;
                                                if !b14 {
                                                    FUN_00458e20(u6, 0);
                                                    DAT_009a1fd4 = DAT_00a07cdc;
                                                    DAT_009a1fd8 = DAT_00a07cd8;
                                                    DAT_009a1fd0 = DAT_00a07cd4;
                                                    let r = FUN_004a5490(DAT_009a73b0, DAT_009a73b0)
                                                        as u32;
                                                    bv13 = r == 0;
                                                }
                                                if bv13 {
                                                    local_4 = DAT_009a1ff8;
                                                    local_14 = u9;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            u9 = u9.wrapping_add(1);
                            if !((u9 as i32) < DAT_009a1fe0) {
                                break;
                            }
                        }
                    }
                    if local_14 != 0xffff {
                        w16(p4, local_14);
                        let sh1 = r16(iv3.wrapping_add(0x3b4)) as i16;
                        if sh1 < 1 {
                            add16(iv3.wrapping_add(0x3b4), 0x1d8);
                        }
                    }
                }
                local_12 = local_12.wrapping_add(1);
                if !((local_12 as u32) < rb(DAT_00a08104.wrapping_add(0x21d)) as u32) {
                    break;
                }
            }
        }

        let mut u6 = DAT_009a1fe8;
        let mut cond = false;
        if rb(ent(DAT_009a73b0 as u32).wrapping_add(2)) == 0 {
            u6 = DAT_00a08104;
            if rb(DAT_00a08104.wrapping_add(0x246)) != 0
                && rb(DAT_00a08104.wrapping_add(0x247)) != 0
            {
                cond = true;
            }
        }
        if cond {
            let a5243c8 = (&raw mut DAT_005243c8) as usize as u32;
            let iv3 = r32(a5243c8.wrapping_add((DAT_009a73d0 as u32).wrapping_mul(4)));
            u6 = DAT_009e9640 as u32;
            let mut u9: u16 = DAT_009e9640;
            if (u6 as i32) < DAT_00a00854 {
                loop {
                    let e = ent(u9 as u32);
                    let c1 = rb(e.wrapping_add(3)) as i8;
                    if c1 != 0 {
                        let t1 = r32(e.wrapping_add(0x1f));
                        let o = r32(t1.wrapping_add(0xa5));
                        if rb(o.wrapping_add(1)) != 0 {
                            let s1 = r16(o.wrapping_add(4));
                            if s1 == DAT_009a73b0 {
                                let mut iv5 = iv3;
                                if c1 == -0x70 || c1 == -0x6b {
                                    iv5 = iv3.wrapping_mul(3);
                                }
                                let mut b14: i32 = FUN_00403360(u9 as u32, iv5 as i32) as i32;
                                if b14 == 1 {
                                    u6 = DAT_00a08104;
                                    if (rb(DAT_00a08104.wrapping_add(0x1e5)) & 2) == 0 {
                                        break;
                                    }
                                    let c2 = rb(DAT_00a08104.wrapping_add(0x246)) as i8;
                                    u6 = ((DAT_00a08104 >> 8) << 8) | (c2 as u8 as u32);
                                    if c2 == 1 {
                                        let s1 = r16(DAT_00a08104.wrapping_add(0x248));
                                        u6 = (DAT_00a08104 & 0xffff0000) | (s1 as u32);
                                        if s1 == 0 {
                                            w16(DAT_00a08104.wrapping_add(0x248), 10);
                                            let u2 = DAT_00a08104;
                                            u6 = (DAT_009a73c0 as u32).wrapping_mul(0x566);
                                            if rb(a8c84.wrapping_add(u6)) != 0x15
                                                && rb(a9155.wrapping_add(u6)) != 0x15
                                            {
                                                wb(
                                                    DAT_00a08104.wrapping_add(0x247),
                                                    rb(DAT_00a08104.wrapping_add(0x247))
                                                        .wrapping_sub(1),
                                                );
                                                return u2 & 0xffff0000;
                                            }
                                        }
                                        break;
                                    }
                                    if c2 != 2 {
                                        break;
                                    }
                                    b14 = 0;
                                    u6 = DAT_009e9640 as u32;
                                    let mut u11: u16 = DAT_009e9640;
                                    if (u6 as i32) < DAT_00a00854 {
                                        loop {
                                            let iv3b = ent(u11 as u32);
                                            if rb(iv3b.wrapping_add(3)) == 0x9b {
                                                let t1 = r32(iv3b.wrapping_add(0x1f));
                                                u6 = r32(t1.wrapping_add(0xa5));
                                                if rb(u6.wrapping_add(1)) != 0 {
                                                    let s1 = r16(u6.wrapping_add(4));
                                                    if s1 == u9 {
                                                        b14 = 1;
                                                        break;
                                                    }
                                                }
                                            }
                                            u6 = u11.wrapping_add(1) as u32;
                                            u11 = u11.wrapping_add(1);
                                            if DAT_00a00854 <= u6 as i32 {
                                                break;
                                            }
                                        }
                                    }
                                    if b14 == 0 {
                                        u6 = DAT_00a08104;
                                        let s1 = r16(DAT_00a08104.wrapping_add(0x24a));
                                        if s1 == 0 {
                                            u6 = FUN_00407090(DAT_009a73b0 as u32, 0x9b);
                                        }
                                    }
                                    return u6 & 0xffff0000;
                                }
                            }
                        }
                    }
                    u9 = u9.wrapping_add(1);
                    if DAT_00a00854 <= u9 as i32 {
                        return 0;
                    }
                }
            }
        }
        u6 & 0xffff0000
    }
}
