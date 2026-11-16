// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00757870: [i32; 0];
    static mut DAT_007579b0: [i32; 0];
    static mut DAT_007579d0: [i32; 0];
    static mut DAT_007579f8: i32;
    static mut DAT_00757a00: [i32; 0];
    static mut DAT_00757a20: i32;
    static mut DAT_00757a28: i32;
    static mut DAT_0075637a: i8;
    static mut DAT_00756b0e: i8;
    static mut DAT_00757ce0: i8;
    static mut DAT_00757cee: i8;
    static mut DAT_00757cf2: i8;
    static mut DAT_00758620: i32;
    static mut DAT_00758624: i32;
    static mut DAT_00758628: i32;
    static mut DAT_0075862c: i32;
    static mut DAT_0081b740: [i8; 0];
    static mut DAT_0081b850: i32;
    static mut DAT_0081bc80: i8;
    static mut DAT_0081bcd0: u32;
    static mut DAT_0081bfba: i8;
    static mut DAT_0085e7c5: i8;
    static mut DAT_005321f4: i8;
    static mut DAT_005337d0: [i8; 0];
    static mut DAT_0053309c: [i8; 0];
    static mut DAT_00531334: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004dcfd0(p: *mut i32, a: i32, b: i32, c: i32, d: i32);
    fn FUN_004dcfa0(a: i32) -> *mut i8;
    fn FUN_004daac0(a: i32, s: *mut i8, r: *mut i32, b: i32, c: i32, d: u32);
    fn FUN_004dd030(p: *mut i32, a: i32, b: i32);
    fn FUN_004db630(a: i32) -> u8;
    fn FUN_004db5f0(a: i32) -> u8;
    fn FUN_004db610(a: i32) -> u8;
    fn FUN_004ddc70(a: *mut i32, b: *mut i32);
    fn FUN_004dd7d0(p: *mut i32, a: i32, b: i32) -> i32;
    fn FUN_004cb830(a: i32, p: *mut u32, b: i32);
    fn FUN_004de190(s: *mut i8, a: i32, b: i32, c: i32);
    fn FUN_004db520();
    fn FUN_004cde00() -> i32;
    fn FUN_004d5180(p: *mut i32);
    fn FUN_004d51b0(p: *mut i32);
    fn FUN_004cdbc0() -> i32;
    fn FUN_004cc810() -> i32;
    fn FUN_004db530() -> u8;
    fn FUN_004db550() -> u8;
    fn FUN_004cdbb0() -> i32;
    fn FUN_004db510(a: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004f0170(param_1: i32) {
    // SAFETY: all globals and callees are the game's own; pointers into its
    // arrays are walked exactly as the original does.
    unsafe {
        let a870 = (&raw mut DAT_00757870).cast::<i32>();
        let ab0 = (&raw mut DAT_007579b0).cast::<i32>();
        let ad0 = (&raw mut DAT_007579d0).cast::<i32>();
        let aa00 = (&raw mut DAT_00757a00).cast::<i32>();
        let buf = (&raw mut DAT_0081b740).cast::<i8>();
        let msg = (&raw mut DAT_00758620).cast::<u32>();
        let rec_e = (&raw mut DAT_00757cee).cast::<u8>();
        let rec_12 = (&raw mut DAT_00757cf2).cast::<u8>();
        let rec_0 = (&raw mut DAT_00757ce0) as usize as i32;

        let mut local_20 = [0i32; 4];
        let mut aux = [0i32; 4];
        let mut local_2c: i32 = 0;
        let mut local_30: i32 = 0;
        let l20 = local_20.as_mut_ptr();
        let aux_p = aux.as_mut_ptr();

        FUN_004dcfd0(l20, 0x5a, 0xcf, 0x100, 0xdd);
        let mut local_3c: i32 = 0;
        if 0 < DAT_00757a28 {
            let mut local_38: *mut i32 = a870;
            let mut local_44: i32 = 0;
            let mut local_34: *mut i32 = ad0;
            let mut local_40: *mut i8 = &raw mut DAT_0075637a;
            loop {
                if (DAT_00757a28 == 8 || DAT_00757a28 == 7) && local_40 == (&raw mut DAT_00756b0e) {
                    FUN_004dcfd0(l20, 0x106, 0xcf, 0x1ac, 0xdd);
                }
                let tmp: i32 = (&raw mut DAT_0081bfba).cast::<i32>().read_unaligned();
                if tmp == 3 {
                    _sprintf(buf, &raw const DAT_005321f4, local_40);
                } else {
                    let i5 = local_3c.wrapping_add(1);
                    let pc6 = FUN_004dcfa0(0xcc);
                    _sprintf(buf, (&raw const DAT_005337d0).cast::<i8>(), pc6, i5);
                }
                FUN_004daac0(0xc, buf, l20, 0, 1, 0xffff);
                FUN_004dd030(l20, 0, 0xf);
                let mut local_48: i32 = 0;
                if 0 < *local_34 {
                    loop {
                        let b1 = FUN_004db630(2);
                        if b1 != 0 && (FUN_004db5f0(2) != 0 || FUN_004db610(2) != 0) {
                            FUN_004ddc70(&mut local_2c, &mut local_30);
                            let i3 = FUN_004dd7d0(l20, local_2c, local_30);
                            let mut i10 = DAT_0081b850;
                            let mut i5 = DAT_007579f8;
                            if i3 != 0 {
                                let i3 = *a870.offset((local_44.wrapping_add(local_48)) as isize);
                                let i9: i32;
                                if i3 == 0 {
                                    let mut n = 0i32;
                                    let mut p = local_38;
                                    if 0 < local_48 {
                                        loop {
                                            if *p == 0 {
                                                break;
                                            }
                                            n = n.wrapping_add(1);
                                            p = p.add(1);
                                            if !(n < local_48) {
                                                break;
                                            }
                                        }
                                    }
                                    i9 = n;
                                } else {
                                    let mut p = ab0;
                                    loop {
                                        if *p == i3 {
                                            *p = 0;
                                        }
                                        p = p.add(1);
                                        if !(p < ad0) {
                                            break;
                                        }
                                    }
                                    if i10 != 2 {
                                        DAT_00758620 = 0x4f;
                                        DAT_00758628 = 10;
                                        DAT_00758624 = i3;
                                        FUN_004cb830(0, msg, 0xc);
                                        i10 = DAT_0081b850;
                                        i5 = DAT_007579f8;
                                    }
                                    i9 = local_48;
                                }
                                let mut i3 = 0i32;
                                *a870.offset((local_44.wrapping_add(i9)) as isize) = i5;
                                let mut p = ab0;
                                loop {
                                    if *p == i5 {
                                        break;
                                    }
                                    p = p.add(1);
                                    i3 = i3.wrapping_add(1);
                                    if !(p < ad0) {
                                        break;
                                    }
                                }
                                if i3 == 8 {
                                    i3 = 0;
                                    let mut p = ab0;
                                    loop {
                                        if *p == 0 {
                                            *ab0.offset(i3 as isize) = i5;
                                            break;
                                        }
                                        p = p.add(1);
                                        i3 = i3.wrapping_add(1);
                                        if !(p < ad0) {
                                            break;
                                        }
                                    }
                                }
                                if DAT_0085e7c5 != 0 {
                                    FUN_004de190((&raw mut DAT_0053309c).cast::<i8>(), 1, 0, 0xff);
                                    i10 = DAT_0081b850;
                                    i5 = DAT_007579f8;
                                }
                                if i10 != 2 {
                                    DAT_00758628 = local_3c;
                                    DAT_00758620 = 0x4f;
                                    DAT_00758624 = i5;
                                    DAT_0075862c = i9;
                                    FUN_004cb830(0, msg, 0x10);
                                    if DAT_0081b850 != 2 {
                                        DAT_00758624 = DAT_007579f8;
                                        DAT_00758620 = 0x67;
                                        FUN_004cb830(0, msg, 8);
                                    }
                                }
                                DAT_007579f8 = 0;
                                FUN_004db520();
                            }
                        }
                        if *a870.offset((local_44.wrapping_add(local_48)) as isize) != 0 {
                            let mut i5: i32 = 1;
                            if DAT_0081b850 != 2 {
                                i5 = FUN_004cde00();
                            }
                            let mut i10: i32 = 0;
                            if 0 < i5 {
                                let mut pc = rec_e;
                                loop {
                                    let t: i32 = pc.cast::<i32>().read_unaligned();
                                    if t == *a870.offset((local_44.wrapping_add(local_48)) as isize)
                                    {
                                        break;
                                    }
                                    pc = pc.add(0x2a);
                                    i10 = i10.wrapping_add(1);
                                    if !(i10 < i5) {
                                        break;
                                    }
                                }
                            }
                            if i10 < i5 {
                                let off = (i10 as u32).wrapping_mul(0x2a);
                                let i5r = rec_0.wrapping_add(off as i32);
                                let i10o = off as i32;
                                let uvar12: i32 = 1;
                                let t: i32 =
                                    rec_12.add(i10o as usize).cast::<i32>().read_unaligned();
                                let pc6 = FUN_004dcfa0((t as u32).wrapping_add(0x8e) as i32);
                                _sprintf(
                                    buf,
                                    (&raw const DAT_00531334).cast::<i8>(),
                                    6i32,
                                    pc6,
                                    uvar12,
                                    i5r,
                                );
                                FUN_004d5180(aux_p);
                                FUN_004d51b0(l20);
                                let i5 = FUN_004cdbc0();
                                let t: i32 =
                                    rec_e.add(i10o as usize).cast::<i32>().read_unaligned();
                                let mut uvar13: u32 = 0;
                                if i5 == t || {
                                    uvar13 = DAT_0081bcd0;
                                    DAT_0081b850 == 2
                                } {
                                    let idx =
                                        ((param_1 % 0x18) as u32 & 0xffff_fffe).wrapping_mul(2);
                                    uvar13 = (&raw mut DAT_0081bc80)
                                        .cast::<u8>()
                                        .add(idx as usize)
                                        .cast::<u32>()
                                        .read_unaligned();
                                }
                                FUN_004daac0(0xc, buf, l20, 0, 1, uvar13);
                                FUN_004d51b0(aux_p);
                                let b1 = FUN_004db630(2);
                                if b1 == 0
                                    && (FUN_004cc810() != 0 || DAT_0081b850 == 2)
                                    && (FUN_004db530() != 0 || FUN_004db550() != 0)
                                {
                                    FUN_004ddc70(&mut local_2c, &mut local_30);
                                    let i5 = FUN_004dd7d0(l20, local_2c, local_30);
                                    if i5 != 0 {
                                        if DAT_0081b850 == 2 {
                                            DAT_007579f8 = rec_e
                                                .add(i10o as usize)
                                                .cast::<i32>()
                                                .read_unaligned();
                                            FUN_004db510(2);
                                            let mut i5 = 0i32;
                                            let mut p = ab0;
                                            loop {
                                                if *p != DAT_007579f8 {
                                                    p = p.add(1);
                                                    i5 = i5.wrapping_add(1);
                                                    if (p as usize) > 0x7579cf {
                                                        break;
                                                    }
                                                    continue;
                                                }
                                                *ab0.offset(i5 as isize) = 0;
                                                let i5 = local_48.wrapping_add(1);
                                                if i5 < 8 {
                                                    let mut c = 8 - i5;
                                                    let mut q = a870.offset(
                                                        (local_44.wrapping_add(i5).wrapping_sub(1))
                                                            as isize,
                                                    );
                                                    loop {
                                                        c -= 1;
                                                        *q = *q.add(1);
                                                        q = q.add(1);
                                                        if c == 0 {
                                                            break;
                                                        }
                                                    }
                                                }
                                                *local_38.add(7) = 0;
                                                break;
                                            }
                                        } else {
                                            let mut i5 = 0i32;
                                            if 0 < DAT_00757a20 {
                                                let mut p = aa00;
                                                loop {
                                                    let t: i32 = rec_e
                                                        .add(i10o as usize)
                                                        .cast::<i32>()
                                                        .read_unaligned();
                                                    if *p == t {
                                                        break;
                                                    }
                                                    p = p.add(1);
                                                    i5 = i5.wrapping_add(1);
                                                    if !(i5 < DAT_00757a20) {
                                                        break;
                                                    }
                                                }
                                            }
                                            if i5 == DAT_00757a20 {
                                                DAT_00758624 = rec_e
                                                    .add(i10o as usize)
                                                    .cast::<i32>()
                                                    .read_unaligned();
                                                DAT_00757a20 = DAT_00757a20.wrapping_add(1);
                                                *aa00.offset(i5 as isize) = DAT_00758624;
                                                DAT_00758620 = 0x68;
                                                DAT_00758628 = FUN_004cdbb0();
                                                FUN_004cb830(0, msg, 0xc);
                                                DAT_007579f8 = rec_e
                                                    .add(i10o as usize)
                                                    .cast::<i32>()
                                                    .read_unaligned();
                                                FUN_004db510(2);
                                                DAT_00758620 = 0x4f;
                                                DAT_00758624 = DAT_007579f8;
                                                DAT_00758628 = 10;
                                                FUN_004cb830(0, msg, 0xc);
                                                let i10b = DAT_00757a28;
                                                let i5b = DAT_007579f8;
                                                let mut i3 = 0i32;
                                                let mut p = ab0;
                                                loop {
                                                    if *p == DAT_007579f8 {
                                                        *ab0.offset(i3 as isize) = 0;
                                                        break;
                                                    }
                                                    p = p.add(1);
                                                    i3 = i3.wrapping_add(1);
                                                    if !(p < ad0) {
                                                        break;
                                                    }
                                                }
                                                if 0 < i10b {
                                                    let mut p4 = a870;
                                                    let mut i3 = 0i32;
                                                    let mut p11 = ad0;
                                                    let mut cnt = i10b;
                                                    loop {
                                                        let mut k = 0i32;
                                                        let mut p7 = p4;
                                                        if 0 < *p11 {
                                                            loop {
                                                                if *p7 != i5b {
                                                                    k = k.wrapping_add(1);
                                                                    p7 = p7.add(1);
                                                                    if *p11 <= k {
                                                                        break;
                                                                    }
                                                                    continue;
                                                                }
                                                                k = k.wrapping_add(1);
                                                                if k < 8 {
                                                                    let mut c = 8 - k;
                                                                    let mut q = a870.offset(
                                                                        (i3.wrapping_add(k)
                                                                            .wrapping_sub(1))
                                                                            as isize,
                                                                    );
                                                                    loop {
                                                                        c -= 1;
                                                                        *q = *q.add(1);
                                                                        q = q.add(1);
                                                                        if c == 0 {
                                                                            break;
                                                                        }
                                                                    }
                                                                }
                                                                *p4.add(7) = 0;
                                                                break;
                                                            }
                                                        }
                                                        p4 = p4.add(8);
                                                        i3 = i3.wrapping_add(8);
                                                        p11 = p11.add(1);
                                                        cnt -= 1;
                                                        if cnt == 0 {
                                                            break;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        FUN_004dd030(l20, 0, 0xf);
                        local_48 = local_48.wrapping_add(1);
                        if !(local_48 < *local_34) {
                            break;
                        }
                    }
                }
                local_38 = local_38.add(8);
                local_44 = local_44.wrapping_add(8);
                local_34 = local_34.add(1);
                local_40 = local_40.add(0x1e5);
                local_3c = local_3c.wrapping_add(1);
                if !(local_3c < DAT_00757a28) {
                    break;
                }
            }
        }
    }
}
