// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005270b0: i32;
    static mut DAT_0060f1d8: u32;
    static mut DAT_0060f1ec: *mut i32;
    static mut DAT_0060f1c0: i32;
    static mut DAT_0060f1e8: i32;
    static mut _DAT_0060f1f0: i32;
    static mut DAT_0060f204: i32;
    static mut _DAT_0060f208: *mut f32;
    static mut _DAT_005233a0: i32;
    static mut DAT_005233a4: i32;
    static mut DAT_005233a8: f32;
    static mut DAT_009a8e34: i32;
    static mut DAT_005271d4: i32;
    static mut _DAT_005181c0: f32;
    static mut _DAT_005181dc: f32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00479de0(param_1: i32, param_2: *mut i8) -> *mut u32;
    fn FUN_00471e00(param_1: *mut u32);
    fn FUN_004210a0(param_1: *mut f32, param_2: *mut f32);
    fn FUN_00420fe0(param_1: *mut f32, param_2: *mut f32);
    fn FUN_004211d0(param_1: *mut f32, param_2: *mut f32);
    fn FUN_00421040(param_1: *mut f32, param_2: *mut f32) -> f64;
    fn FUN_00421060(param_1: *mut f32, param_2: i32) -> f64;
    fn FUN_00421080(param_1: *mut f32, param_2: i32) -> f64;
    fn FUN_00421300(param_1: *mut f32, param_2: *mut f32);
}

#[inline(always)]
unsafe fn g(p: *mut u32, i: usize) -> u32 {
    unsafe { p.add(i).read_unaligned() }
}

#[inline(always)]
unsafe fn s(p: *mut u32, i: usize, v: u32) {
    unsafe { p.add(i).write_unaligned(v) }
}

#[inline(always)]
unsafe fn rf(p: *mut u32, i: usize) -> f32 {
    unsafe { p.add(i).cast::<f32>().read_unaligned() }
}

#[inline(always)]
unsafe fn wf(p: *mut u32, i: usize, v: f32) {
    unsafe { p.add(i).cast::<f32>().write_unaligned(v) }
}

#[inline(always)]
unsafe fn ri(a: u32) -> i32 {
    unsafe { (a as *const i32).read_unaligned() }
}

#[inline(always)]
unsafe fn fp(p: *mut u32, i: usize) -> *mut f32 {
    unsafe { p.add(i).cast::<f32>() }
}

unsafe fn case10(p2: *mut u32, p3: *mut u32) {
    // SAFETY: game globals and caller-provided structures.
    unsafe {
        let iv = g(p2, 4) as i32;
        if iv == 8 || iv == 7 {
            s(p3, 0x22, DAT_0060f1d8);
        } else if iv == 6 || iv == 5 {
            s(p3, 0x27, DAT_0060f1d8);
        } else {
            s(p3, 0x1d, DAT_0060f1d8);
        }
    }
}

unsafe fn case14(p2: *mut u32, p3: *mut u32) {
    // SAFETY: game globals and caller-provided structures.
    unsafe {
        s(p3, 0x29, g(p2, 0));
        DAT_0060f1ec = g(p2, 5) as *mut i32;
        s(p3, 0x2a, DAT_0060f1ec as u32);
        s(p3, 0x2b, g(p3, 0x2a));
        let i11 = g(p3, 0x2a).wrapping_add(0x18) as i32;
        s(p3, 0x2b, i11 as u32);
        let p6 = p3.add(0x2c).cast::<i32>();
        if DAT_0060f1ec.add(1).read_unaligned() == 0 {
            p6.write_unaligned(DAT_0060f1ec.read_unaligned());
        } else {
            p6.write_unaligned(i11);
            let i2 = g(p3, 0x2a);
            let mut i5 = ri(i2.wrapping_add(0x14)).wrapping_mul(ri(i2.wrapping_add(0x10)));
            if ri(i2.wrapping_add(8)) == i5 {
                i5 = ri(i2.wrapping_add(0xc));
            }
            p6.write_unaligned(i5.wrapping_add(i11));
        }
        let i11b = p6.read_unaligned();
        p6.write_unaligned(i11b.wrapping_add(0x1000));
        s(p3, 0x2d, (i11b as u32).wrapping_add(0x1000));
        p6.write_unaligned(p6.read_unaligned().wrapping_sub(0x1000));
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00472c90(param_1: i32, param_2: *mut u32, param_3: *mut u32) {
    let mut param_2 = param_2;
    let mut local_d0 = [0u32; 52];
    if param_2.is_null() {
        return;
    }
    // SAFETY: all pointers are the game's own structures, accessed as the original does.
    unsafe {
        loop {
            let mut i10: i32 = 0;
            let mut local_128: i32 = 0;
            while g(param_2, 1) == 7 {
                if DAT_005270b0 == 0 {
                    param_2 = FUN_00479de0(param_1, g(param_2, 5) as *mut i8);
                } else if *(g(param_2, 5) as *const i8) == 0 {
                    param_2 = g(param_2, 0) as *mut u32;
                } else {
                    let p4 = FUN_00479de0(param_1, g(param_2, 5) as *mut i8);
                    s(param_2, 0, p4 as u32);
                    *(g(param_2, 5) as *mut u8) = 0;
                    param_2 = g(param_2, 0) as *mut u32;
                }
                if param_2.is_null() {
                    return;
                }
            }
            let mut pf9 = g(param_2, 5) as *mut f32;
            if pf9.is_null() {
                match g(param_2, 1) {
                    10 => case10(param_2, param_3),
                    0x14 => case14(param_2, param_3),
                    0x18 => {
                        local_128 = _DAT_005233a0.wrapping_add(1);
                        let i11 = g(param_2, 2) as i32;
                        if i11 < local_128 {
                            local_128 = i11;
                        }
                    }
                    _ => {}
                }
            } else {
                match g(param_2, 1) {
                    1 | 0xf | 0x10 | 0x11 => {
                        let mut p7 = (pf9 as u32).wrapping_add(4);
                        s(param_3, 0x23, g(param_2, 4));
                        s(param_3, 0x24, (pf9 as *const u32).read_unaligned());
                        s(param_3, 0x28, p7);
                        p7 = p7.wrapping_add(g(param_2, 4).wrapping_mul(16).wrapping_mul(4));
                        s(param_3, 0x25, p7);
                        let i11 = g(param_2, 4);
                        s(
                            param_3,
                            0x26,
                            p7.wrapping_add(i11.wrapping_mul(3).wrapping_mul(4)),
                        );
                        let mut p6 = DAT_0060f1ec as u32;
                        let i2 = g(param_2, 4);
                        if g(param_3, 0x2a) == 0 {
                            s(param_3, 0x2a, DAT_0060f1ec as u32);
                            s(param_3, 0x2b, p6);
                            p6 = p6.wrapping_add(0x18);
                            s(param_3, 0x2b, p6);
                            if DAT_0060f1ec.add(1).read_unaligned() == 0 {
                                p6 = DAT_0060f1ec.read_unaligned() as u32;
                                s(param_3, 0x2c, p6);
                            } else {
                                s(param_3, 0x2c, p6);
                                let i5 = g(param_3, 0x2a);
                                let i8 = ri(i5.wrapping_add(0x14))
                                    .wrapping_mul(ri(i5.wrapping_add(0x10)));
                                if ri(i5.wrapping_add(8)) != i8 {
                                    p6 = p6.wrapping_add(i8 as u32);
                                    s(param_3, 0x2c, p6);
                                } else {
                                    s(
                                        param_3,
                                        0x2c,
                                        (ri(i5.wrapping_add(0xc)) as u32).wrapping_add(p6),
                                    );
                                }
                            }
                            let i5 = g(param_3, 0x2c);
                            s(param_3, 0x2c, i5.wrapping_add(0x1000));
                            s(param_3, 0x2d, i5.wrapping_add(0x1000));
                            s(param_3, 0x2c, g(param_3, 0x2c).wrapping_sub(0x1000));
                        }
                        if g(param_3, 0x21) == 0 {
                            s(
                                param_3,
                                0x21,
                                p7.wrapping_add(i11.wrapping_mul(3).wrapping_mul(4))
                                    .wrapping_add(i2.wrapping_mul(6).wrapping_mul(4)),
                            );
                            FUN_00471e00(param_3);
                            s(param_3, 0x21, 0);
                        } else {
                            FUN_00471e00(param_3);
                        }
                    }
                    2 => {
                        FUN_004210a0(fp(param_3, 5), pf9.add(3));
                        FUN_00420fe0(fp(param_3, 2), pf9.add(3));
                        wf(param_3, 2, *pf9 + rf(param_3, 2));
                        wf(param_3, 3, *pf9.add(1) + rf(param_3, 3));
                        wf(param_3, 4, rf(param_3, 4) + *pf9.add(2));
                        FUN_004211d0(fp(param_3, 0x11), pf9.add(3));
                        let mut f12 = FUN_00421040(pf9, fp(param_3, 0x11));
                        wf(param_3, 0xe, (rf(param_3, 0xe) as f64 - f12) as f32);
                        f12 = FUN_00421060(pf9, param_3.add(0x11) as i32);
                        wf(param_3, 0xf, (rf(param_3, 0xf) as f64 - f12) as f32);
                        f12 = FUN_00421080(pf9, param_3.add(0x11) as i32);
                        wf(param_3, 0x10, (rf(param_3, 0x10) as f64 - f12) as f32);
                    }
                    3 => {
                        s(param_3, 0x1e, g(param_2, 4));
                        s(param_3, 0x1f, pf9 as u32);
                    }
                    4 => {
                        wf(param_3, 2, rf(param_3, 2) + *pf9);
                        wf(param_3, 3, rf(param_3, 3) + *pf9.add(1));
                        wf(param_3, 4, rf(param_3, 4) + *pf9.add(2));
                        let mut f12 = FUN_00421040(pf9, fp(param_3, 0x11));
                        wf(param_3, 0xe, (rf(param_3, 0xe) as f64 - f12) as f32);
                        f12 = FUN_00421060(pf9, param_3.add(0x11) as i32);
                        wf(param_3, 0xf, (rf(param_3, 0xf) as f64 - f12) as f32);
                        f12 = FUN_00421080(pf9, param_3.add(0x11) as i32);
                        wf(param_3, 0x10, (rf(param_3, 0x10) as f64 - f12) as f32);
                    }
                    5 => {
                        FUN_004210a0(fp(param_3, 5), pf9);
                        FUN_00420fe0(fp(param_3, 2), pf9);
                        FUN_004211d0(fp(param_3, 0x11), pf9);
                    }
                    6 => {
                        let a = *pf9;
                        let b = *pf9.add(1);
                        let c = *pf9.add(2);
                        wf(param_3, 5, rf(param_3, 5) * a);
                        wf(param_3, 6, rf(param_3, 6) * b);
                        wf(param_3, 7, rf(param_3, 7) * c);
                        wf(param_3, 8, rf(param_3, 8) * a);
                        wf(param_3, 9, rf(param_3, 9) * b);
                        wf(param_3, 10, rf(param_3, 10) * c);
                        wf(param_3, 0xb, rf(param_3, 0xb) * a);
                        wf(param_3, 0xc, rf(param_3, 0xc) * b);
                        wf(param_3, 0xd, rf(param_3, 0xd) * c);
                        wf(param_3, 2, rf(param_3, 2) * a);
                        wf(param_3, 3, rf(param_3, 3) * b);
                        wf(param_3, 4, rf(param_3, 4) * c);
                        let mut f3 = _DAT_005181c0 / a;
                        wf(param_3, 0x11, rf(param_3, 0x11) * f3);
                        wf(param_3, 0x12, rf(param_3, 0x12) * f3);
                        wf(param_3, 0x13, f3 * rf(param_3, 0x13));
                        f3 = _DAT_005181c0 / b;
                        wf(param_3, 0x14, rf(param_3, 0x14) * f3);
                        wf(param_3, 0x15, rf(param_3, 0x15) * f3);
                        wf(param_3, 0x16, f3 * rf(param_3, 0x16));
                        f3 = _DAT_005181c0 / c;
                        wf(param_3, 0x17, rf(param_3, 0x17) * f3);
                        wf(param_3, 0x18, rf(param_3, 0x18) * f3);
                        wf(param_3, 0x19, f3 * rf(param_3, 0x19));
                    }
                    10 => case10(param_2, param_3),
                    0xb => {
                        _DAT_0060f208 = pf9;
                        s(param_3, 0x21, pf9 as u32);
                    }
                    0xd => {
                        s(param_3, 0x20, pf9 as u32);
                    }
                    0x13 => {
                        let q = pf9 as *const u32;
                        s(param_3, 0x1a, q.read_unaligned());
                        s(param_3, 0x1b, q.add(1).read_unaligned());
                        s(param_3, 0x1c, q.add(2).read_unaligned());
                    }
                    0x14 => case14(param_2, param_3),
                    0x15 => {
                        if DAT_009a8e34 < 1 || DAT_005233a4 != 0 {
                            if DAT_005233a4 == 0 {
                                i10 = 1;
                            } else {
                                i10 = DAT_005233a4;
                                if (g(param_2, 2) as i32) < DAT_005233a4 {
                                    i10 = -1;
                                }
                            }
                        } else {
                            let mut local_12c: f32 = 1.0;
                            if 0 < (DAT_005233a8 as i32) {
                                local_12c = _DAT_005181c0 / (DAT_009a8e34 as f32 * DAT_005233a8);
                            }
                            i10 = 1;
                            let i11 = g(param_2, 2) as i32;
                            'sw: {
                                if 0 < i11 {
                                    loop {
                                        if *pf9 <= local_12c {
                                            break;
                                        }
                                        pf9 = pf9.add(1);
                                        i10 = i10.wrapping_add(1);
                                        if !(i10 <= i11) {
                                            break;
                                        }
                                    }
                                    if i10 <= i11 {
                                        break 'sw;
                                    }
                                }
                                i10 = -1;
                            }
                        }
                    }
                    0x17 => {
                        if (g(param_3, 1) & 0x7fffffff) != 0 {
                            wf(param_3, 0xe, rf(param_3, 0xe) - *pf9);
                            wf(param_3, 0xf, rf(param_3, 0xf) - *pf9.add(1));
                            wf(param_3, 0x10, rf(param_3, 0x10) - *pf9.add(2));
                            let mut f12 = FUN_00421040(pf9, fp(param_3, 5));
                            wf(param_3, 2, (f12 + rf(param_3, 2) as f64) as f32);
                            f12 = FUN_00421060(pf9, param_3.add(5) as i32);
                            wf(param_3, 3, (f12 + rf(param_3, 3) as f64) as f32);
                            f12 = FUN_00421080(pf9, param_3.add(5) as i32);
                            wf(param_3, 4, (f12 + rf(param_3, 4) as f64) as f32);
                            let mut local_120 = [0f32; 4];
                            let mut local_110 = [0f32; 16];
                            local_120[0] = *pf9.add(3) * _DAT_005181dc;
                            local_120[1] = *pf9.add(4) * _DAT_005181dc;
                            local_120[2] = *pf9.add(5) * _DAT_005181dc;
                            local_120[3] = f32::from_bits(g(param_3, 1));
                            FUN_00421300(local_110.as_mut_ptr(), local_120.as_mut_ptr());
                            FUN_004210a0(fp(param_3, 0x11), local_110.as_mut_ptr());
                            FUN_00420fe0(fp(param_3, 0xe), local_110.as_mut_ptr());
                            FUN_004211d0(fp(param_3, 5), local_110.as_mut_ptr());
                            wf(param_3, 0xe, rf(param_3, 0xe) + *pf9);
                            wf(param_3, 0xf, rf(param_3, 0xf) + *pf9.add(1));
                            wf(param_3, 0x10, rf(param_3, 0x10) + *pf9.add(2));
                            f12 = FUN_00421040(pf9, fp(param_3, 5));
                            wf(param_3, 2, (rf(param_3, 2) as f64 - f12) as f32);
                            f12 = FUN_00421060(pf9, param_3.add(5) as i32);
                            wf(param_3, 3, (rf(param_3, 3) as f64 - f12) as f32);
                            f12 = FUN_00421080(pf9, param_3.add(5) as i32);
                            wf(param_3, 4, (rf(param_3, 4) as f64 - f12) as f32);
                        }
                    }
                    0x18 => {
                        let i11 = g(param_2, 2) as i32;
                        local_128 = _DAT_005233a0.wrapping_add(1);
                        if i11 < _DAT_005233a0.wrapping_add(1) {
                            local_128 = i11;
                        }
                    }
                    _ => {}
                }
            }
            if g(param_2, 2) == 0 {
                return;
            }
            if local_128 == 0 {
                if i10 == 0 {
                    for i in 0..0x34usize {
                        local_d0[i] = g(param_3, i);
                    }
                    let mut i11: i32 = 0;
                    DAT_0060f1c0 = 0;
                    DAT_0060f1e8 = 0;
                    i10 = 0;
                    _DAT_0060f208 = core::ptr::null_mut();
                    _DAT_0060f1f0 = 0;
                    DAT_0060f1d8 = 0;
                    DAT_0060f204 = 0;
                    if (g(param_2, 2) as i32) < 1 {
                        DAT_0060f1c0 = 0;
                        DAT_0060f1d8 = 0;
                        DAT_0060f1e8 = 0;
                        _DAT_0060f1f0 = 0;
                        DAT_0060f204 = 0;
                        _DAT_0060f208 = core::ptr::null_mut();
                        return;
                    }
                    loop {
                        DAT_005271d4 = DAT_005271d4.wrapping_add(1);
                        i10 = i10.wrapping_add(1);
                        i11 = i11.wrapping_add(4);
                        let a = g(param_2, 3).wrapping_sub(4).wrapping_add(i11 as u32);
                        let child = (a as *const u32).read_unaligned() as *mut u32;
                        FUN_00472c90(param_1, child, local_d0.as_mut_ptr());
                        if !(i10 < g(param_2, 2) as i32) {
                            break;
                        }
                    }
                    return;
                }
                if i10 == -1 {
                    return;
                }
                let a = g(param_2, 3)
                    .wrapping_sub(4)
                    .wrapping_add((i10 as u32).wrapping_mul(4));
                param_2 = (a as *const u32).read_unaligned() as *mut u32;
            } else {
                let a = g(param_2, 3)
                    .wrapping_sub(4)
                    .wrapping_add((local_128 as u32).wrapping_mul(4));
                param_2 = (a as *const u32).read_unaligned() as *mut u32;
            }
            DAT_005271d4 = DAT_005271d4.wrapping_add(1);
            if param_2.is_null() {
                return;
            }
        }
    }
}
