// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0064ef50: i32;
    static mut DAT_0064ef54: i32;
    static mut DAT_0064ef58: i32;
    static mut DAT_0064ef5c: i32;
    static mut DAT_0081bcb0: u32;
    static mut DAT_0081bcd0: u32;
    static mut DAT_00859c2c: i32;
    static mut DAT_0085e6c0: i32;
    static mut DAT_0085e7c5: i8;
    static mut DAT_0081b740: [i8; 1];
    static mut DAT_00859c40: [i8; 1];
    static mut DAT_00859c41: [i8; 1];
    static mut DAT_00859c55: [i8; 1];
    static mut DAT_0085e7a0: [i8; 1];
    static mut DAT_0085e7b0: [i8; 1];
    static mut DAT_005319c4: [i8; 1];
    static mut DAT_005319d8: [i8; 1];
    static mut DAT_005319e8: [i8; 1];
    static mut DAT_005319cc: [i8; 1];
    static mut DAT_005319b8: [i8; 1];
    static mut DAT_005319ac: [i8; 1];
    static mut DAT_0053138c: [i8; 1];
    static mut DAT_0053199c: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ddc70(param_1: *mut i32, param_2: *mut i32);
    fn FUN_004dcfd0(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004dcfa0(param_1: u32) -> *mut i8;
    fn FUN_004da920(param_1: i32, param_2: *mut i8, param_3: *mut i32, param_4: u32);
    fn _sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004daac0(
        param_1: i32,
        param_2: *mut i8,
        param_3: *mut i32,
        param_4: i32,
        param_5: i32,
        param_6: u32,
    );
    fn FUN_004dd030(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_004df360(param_1: i32) -> u8;
    fn FUN_004df2c0(param_3: i32) -> u64;
    fn FUN_004de190(param_1: *mut i8, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_004df3a0(param_1: i32) -> u8;
    fn FUN_004df380(param_1: i32) -> u8;
    fn FUN_004d9010(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
    ) -> i32;
    fn FUN_004dd3e0(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004dd070(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_004dd7d0(param_1: *mut i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_004dd570(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i16);
    fn FUN_004db570() -> u8;
    fn FUN_004db590() -> u8;
    fn FUN_004bdea0() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004bd330(
    param_1: u32,
    param_2: u32,
    param_3: *mut i32,
    param_4: u32,
    param_5: *mut i32,
    param_6: u32,
    param_7: *mut i8,
    param_8: *mut i8,
    param_9: *mut i32,
    param_10: *mut i32,
    param_11: i32,
    param_12: *mut u8,
    param_13: i32,
) {
    let _ = (
        param_1, param_2, param_3, param_4, param_5, param_6, param_7, param_8, param_9, param_10,
        param_11, param_12, param_13,
    );
    // SAFETY: all globals and callees are the game's own; pointers are to
    // locals or the game's buffers, accessed as the original does.
    unsafe {
        let mut local_1c: [i32; 4] = [0; 4];
        let mut local_8: i32 = 0;
        let mut local_4: i32 = 0;
        let mut i_stack_c: i32;
        let l = local_1c.as_mut_ptr();
        let buf = (&raw mut DAT_0081b740).cast::<i8>();
        let d59c40 = (&raw mut DAT_00859c40).cast::<i8>();
        let d59c41 = (&raw mut DAT_00859c41).cast::<i8>();
        let d59c55 = (&raw mut DAT_00859c55).cast::<i8>();
        let e7a0 = (&raw mut DAT_0085e7a0).cast::<i8>();
        let e7b0 = (&raw mut DAT_0085e7b0).cast::<i8>();
        let cfg = (&raw mut DAT_0053138c).cast::<i8>();
        let set = (&raw mut DAT_0053199c).cast::<i8>();

        FUN_004ddc70(&raw mut local_4, &raw mut local_8);
        FUN_004dcfd0(l, 0x54, 0x5a, 0x25c, 0x6a);
        let mut pc3 = FUN_004dcfa0(0x192);
        FUN_004da920(0xf, pc3, l, 0xffff);
        FUN_004dcfd0(l, 0x58, 0x6f, 0x142, 0x7d);
        let pc4: *mut i8;
        let pc5: *mut i8;
        let fmt: *const i8;
        if DAT_0064ef58 < 0x10 {
            pc4 = (DAT_0064ef58 as u32).wrapping_add(1) as usize as *mut i8;
            pc3 = FUN_004dcfa0(0x194);
            pc5 = FUN_004dcfa0(0x193);
            fmt = (&raw mut DAT_005319d8).cast::<i8>();
        } else {
            let u = (DAT_0064ef58 as u32).wrapping_add(0x269);
            pc3 = FUN_004dcfa0(0x194);
            pc4 = FUN_004dcfa0(u);
            pc5 = FUN_004dcfa0(0x278);
            fmt = (&raw mut DAT_005319e8).cast::<i8>();
        }
        _sprintf(buf, fmt, 4i32, pc5, pc4, 1i32, pc3);
        FUN_004daac0(0xc, buf, l, 0, 1, 0xffff);
        FUN_004dd030(l, 0, 0xf);
        if DAT_0085e6c0 != 0 {
            if DAT_0064ef5c != 0 {
                let off = (DAT_0064ef5c as u32).wrapping_mul(0x95);
                _sprintf(
                    buf,
                    (&raw mut DAT_005319cc).cast::<i8>(),
                    2i32,
                    (d59c41 as u32).wrapping_add(off) as i32,
                    1i32,
                    (d59c55 as u32).wrapping_add(off) as i32,
                );
            } else {
                _sprintf(buf, (&raw mut DAT_005319c4).cast::<i8>(), 2i32, d59c41);
            }
            FUN_004daac0(0xc, buf, l, 0, 1, 0xffff);
        }
        FUN_004dd030(l, 0, 0x1e);
        let ids: [u32; 5] = [0x27f, 0x280, 0x281, 0x282, 0x283];
        let mut k = 0;
        while k < 5 {
            let p = FUN_004dcfa0(ids[k]);
            FUN_004daac0(0xc, p, l, 0, 1, DAT_0081bcb0);
            if k < 4 {
                FUN_004dd030(l, 0, 0xf);
            }
            k += 1;
        }
        let u2 = FUN_004df360(0);
        let mut i12 = u2 as i32;
        let u10 = FUN_004df2c0(0);
        let mut i14 = u10 as i32;
        if (i14 != -1 && i14 < 0x10) || i12 != 0 {
            if i14 == -1 {
                if i12 != 0 {
                    if (i12 as u32).wrapping_sub(DAT_0064ef58 as u32) != 0xfffffff1u32
                        && DAT_0085e7c5 != 0
                    {
                        FUN_004de190(cfg, 1, 0, 0xff);
                    }
                    DAT_0064ef58 = (i12 as u32).wrapping_add(0xf) as i32;
                    let u = FUN_004df3a0(0);
                    i12 = (i12 as u32).wrapping_sub(1).wrapping_add(u as u32) as i32;
                    if i12 < DAT_0064ef54
                        || 9 < (i12 as u32).wrapping_sub(DAT_0064ef54 as u32) as i32
                    {
                        DAT_0064ef54 = i12;
                    }
                }
            } else {
                if i14 != DAT_0064ef58 && DAT_0085e7c5 != 0 {
                    FUN_004de190(cfg, 1, 0, 0xff);
                }
                DAT_0064ef58 = i14;
                if i14 < DAT_0064ef54 || 9 < (i14 as u32).wrapping_sub(DAT_0064ef54 as u32) as i32 {
                    DAT_0064ef54 = i14;
                }
            }
            i12 = 0;
            if 0 < DAT_0085e6c0 {
                let mut p = d59c40;
                loop {
                    if *p == *e7a0.offset(DAT_0064ef58 as isize) {
                        DAT_0064ef5c = i12;
                    }
                    p = p.byte_add(0x95);
                    i12 = (i12 as u32).wrapping_add(1) as i32;
                    if !(i12 < DAT_0085e6c0) {
                        break;
                    }
                }
            }
        }
        let u2 = FUN_004df3a0(0);
        i12 = u2 as i32;
        i_stack_c = i12;
        let u2 = FUN_004df380(0);
        if u2 as i32 != 0 {
            i_stack_c = (i12 as u32).wrapping_add(4) as i32;
        }
        FUN_004dcfd0(l, 0x58, 0xf6, 0x142, 0x104);
        let p = FUN_004dcfa0(0x27e);
        FUN_004da920(0xc, p, l, DAT_0081bcd0);
        let mut u13: i32 = 0x142;
        if 10 < i_stack_c {
            FUN_004dcfd0(l, 0x139, 0x105, 0x142, 0x19f);
            DAT_0064ef54 = FUN_004d9010(l, DAT_0064ef54, i_stack_c, 0, 5, DAT_00859c2c, 1);
            u13 = 0x138;
        }
        FUN_004dcfd0(l, 0x58, 0x105, u13, 0x19f);
        FUN_004dd3e0(l, 0, 0, 0xffff, 0);
        FUN_004dd070(l, 2, 2);
        FUN_004dd3e0(l, 0, 0, 0, 0);
        local_1c[3] = (local_1c[1] as u32).wrapping_add(0xe) as i32;
        FUN_004dd070(l, 2, 0);
        i14 = DAT_0064ef54;
        if DAT_0064ef54 < (DAT_0064ef54 as u32).wrapping_add(10) as i32 {
            loop {
                if i_stack_c <= i14 {
                    break;
                }
                let mut i8_: i32 = 0;
                if 0 < DAT_0085e6c0 {
                    let mut p = d59c40;
                    loop {
                        if i14 < i12 {
                            if *e7a0.offset(i14 as isize) == *p {
                                break;
                            }
                        } else if *e7b0
                            .offset((i14 as u32).wrapping_sub(i12 as u32) as i32 as isize)
                            == *p
                        {
                            break;
                        }
                        p = p.byte_add(0x95);
                        i8_ = (i8_ as u32).wrapping_add(1) as i32;
                        if !(i8_ < DAT_0085e6c0) {
                            break;
                        }
                    }
                }
                if i8_ < DAT_0085e6c0 {
                    let i6 = FUN_004dd7d0(l, local_4, local_8);
                    let mut i7 = DAT_0064ef58;
                    let mut i1 = DAT_0064ef5c;
                    if i6 != 0 {
                        FUN_004dd570(l, 0, 0, DAT_0081bcb0 as i16);
                        let mut u = FUN_004db570();
                        if u == 0 {
                            u = FUN_004db590();
                            i7 = DAT_0064ef58;
                            i1 = DAT_0064ef5c;
                        }
                        if u != 0 {
                            i1 = i8_;
                            if i14 < i12 {
                                i7 = i14;
                                if i14 != DAT_0064ef58 && DAT_0085e7c5 != 0 {
                                    FUN_004de190(cfg, 1, 0, 0xff);
                                }
                            } else {
                                if (i14 as u32)
                                    .wrapping_sub(DAT_0064ef58 as u32)
                                    .wrapping_add(i12 as u32)
                                    != 0x10
                                    && DAT_0085e7c5 != 0
                                {
                                    FUN_004de190(cfg, 1, 0, 0xff);
                                }
                                i7 =
                                    (i14 as u32).wrapping_sub(i12 as u32).wrapping_add(0x10) as i32;
                            }
                        }
                    }
                    DAT_0064ef5c = i1;
                    DAT_0064ef58 = i7;
                    let u15: u32;
                    let p3: *mut i8;
                    let p4: *mut i8;
                    let f: *const i8;
                    let a8: i32;
                    if i14 < i12 {
                        u15 = if i14 == i7 {
                            DAT_0081bcd0 & 0xffff
                        } else {
                            0xffff
                        };
                        p3 = (i14 as u32).wrapping_add(1) as usize as *mut i8;
                        a8 = (d59c41 as u32).wrapping_add((i8_ as u32).wrapping_mul(0x95)) as i32;
                        p4 = FUN_004dcfa0(0x193);
                        f = (&raw mut DAT_005319b8).cast::<i8>();
                    } else {
                        u15 = if (i14 as u32)
                            .wrapping_sub(i7 as u32)
                            .wrapping_sub(i12 as u32)
                            == 0xfffffff0u32
                        {
                            DAT_0081bcd0 & 0xffff
                        } else {
                            0xffff
                        };
                        a8 = (d59c41 as u32).wrapping_add((i8_ as u32).wrapping_mul(0x95)) as i32;
                        p3 =
                            FUN_004dcfa0((i14 as u32).wrapping_sub(i12 as u32).wrapping_add(0x279));
                        p4 = FUN_004dcfa0(0x278);
                        f = (&raw mut DAT_005319ac).cast::<i8>();
                    }
                    _sprintf(buf, f, p4, p3, 2i32, a8);
                    FUN_004daac0(0xc, buf, l, 0, 1, u15);
                }
                i14 = (i14 as u32).wrapping_add(1) as i32;
                FUN_004dd030(l, 0, 0xf);
                if !(i14 < (DAT_0064ef54 as u32).wrapping_add(10) as i32) {
                    break;
                }
            }
        }
        FUN_004dcfd0(l, 0x14a, 0x6f, 0x24e, 0x7d);
        let p = FUN_004dcfa0(0x27d);
        FUN_004da920(0xc, p, l, DAT_0081bcd0);
        FUN_004dcfd0(l, 0x24f, 0x7e, 600, 0x19f);
        DAT_0064ef50 = FUN_004d9010(l, DAT_0064ef50, DAT_0085e6c0, 0, 5, DAT_00859c2c, 2);
        FUN_004dcfd0(l, 0x14a, 0x7e, 0x24e, 0x19f);
        FUN_004dd3e0(l, 0, 0, 0xffff, 0);
        FUN_004dd070(l, 2, 2);
        FUN_004dd3e0(l, 0, 0, 0, 0);
        local_1c[3] = (local_1c[1] as u32).wrapping_add(0xe) as i32;
        FUN_004dd070(l, 2, 0);
        if DAT_0064ef50 < (DAT_0064ef50 as u32).wrapping_add(0x13) as i32 {
            let mut p9 = d59c40
                .cast::<u8>()
                .byte_add((DAT_0064ef50 as u32).wrapping_mul(0x95) as usize);
            let mut i12b = DAT_0064ef50;
            loop {
                if DAT_0085e6c0 <= i12b {
                    break;
                }
                let r = FUN_004dd7d0(l, local_4, local_8);
                if r != 0 {
                    FUN_004dd570(l, 0, 0, DAT_0081bcb0 as i16);
                    let mut u = FUN_004db570();
                    if u != 0 || {
                        u = FUN_004db590();
                        u != 0
                    } {
                        if DAT_0064ef5c != i12b && DAT_0085e7c5 != 0 {
                            FUN_004de190(set, 1, 0, 0xff);
                        }
                        DAT_0064ef5c = i12b;
                        *e7a0.offset(DAT_0064ef58 as isize) = *p9 as i8;
                    }
                }
                _sprintf(
                    buf,
                    (&raw mut DAT_005319cc).cast::<i8>(),
                    2i32,
                    p9.byte_add(1),
                    1i32,
                    p9.byte_add(0x15),
                );
                let mut u15 = DAT_0081bcd0;
                if i12b != DAT_0064ef5c {
                    u15 = 0xffff;
                }
                p9 = p9.byte_add(0x95);
                i12b = (i12b as u32).wrapping_add(1) as i32;
                FUN_004daac0(0xc, buf, l, 0, 1, u15);
                FUN_004dd030(l, 0, 0xf);
                if !(i12b < (DAT_0064ef50 as u32).wrapping_add(0x13) as i32) {
                    break;
                }
            }
        }
        let mut i12c: i32 = 0;
        let u15 = FUN_004bdea0();
        if 0 < DAT_0085e6c0 {
            let mut p = d59c40;
            let mut i14c = DAT_0085e6c0;
            let mut i8c = DAT_0064ef58;
            loop {
                if (u15 as i8) != 0 && *p == (u15 as i8) {
                    if DAT_0064ef5c != i12c && DAT_0085e7c5 != 0 {
                        FUN_004de190(set, 1, 0, 0xff);
                        i14c = DAT_0085e6c0;
                        i8c = DAT_0064ef58;
                    }
                    DAT_0064ef50 = i12c;
                    DAT_0064ef5c = i12c;
                    *e7a0.offset(i8c as isize) = *p;
                }
                p = p.byte_add(0x95);
                i12c = (i12c as u32).wrapping_add(1) as i32;
                if !(i12c < i14c) {
                    break;
                }
            }
        }
    }
}
