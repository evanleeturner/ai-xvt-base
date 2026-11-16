// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals, non_snake_case)]
unsafe extern "C" {
    static mut DAT_009d6941: u8;
    static mut DAT_009ec5e4: u16;
    static mut DAT_00a07cec: u16;
    static mut DAT_009d1308: i32;
    static mut DAT_00a07ce8: i32;
    static mut DAT_009a1fe8: *mut u8;
    static mut DAT_00a08104: *mut u8;
    static mut DAT_00a90a60: *mut u8;
    static mut _DAT_00a90a5e: u16;
    static mut DAT_00a07cdc: i32;
    static mut DAT_00a07cd8: i32;
    static mut DAT_00a07cd4: i32;
    static mut DAT_00521f26: [u8; 0];
    static mut DAT_00a00870: [u32; 0];
    static mut DAT_00a00874: [i32; 0];
    static mut DAT_009d1320: [u8; 0];
    static mut DAT_0051a1d0: [i8; 0];
    static mut DAT_0051a1bc: [i8; 0];
    static mut DAT_0051a1a8: [i8; 0];
    static mut DAT_0051a0b0: [u8; 0];
    static mut DAT_0051c5b9: [u8; 0];

    fn __ftol() -> i64;
    fn FUN_004adc40(a: u32) -> i32;
    fn FUN_004ae2a0(a: u32, b: u32) -> i32;
    fn FUN_004af000(a: u32, b: u32);
    fn FUN_00458e20(a: u32, b: i32);
    fn FUN_00427bd0(a: i16, b: i16, c: *mut u8);
    fn FUN_00427d30(a: i16, b: i32, c: *mut u8);
    fn FUN_0046abf0(a: u32, b: u32) -> u32;
    fn FUN_004021a0(a: u32);
    fn FUN_004af5b0(a: u32) -> i32;
    fn FUN_004a7480(a: u32, b: i32);
    fn FUN_00458fa0(a: u32, b: i32);
    fn FUN_004592d0(a: u32);
    fn FUN_004ae6e0(a: i32, b: u32) -> i32;
    fn FUN_00450650(a: i32, b: i32);
    fn FUN_00459480(a: u32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_0042c0a0();
    fn FUN_00425cb0() -> u16;
}

unsafe fn r16(p: *const u8) -> u16 {
    // SAFETY: p points into the game's own structures.
    unsafe { p.cast::<u16>().read_unaligned() }
}

unsafe fn w16(p: *mut u8, v: u16) {
    // SAFETY: p points into the game's own structures.
    unsafe { p.cast::<u16>().write_unaligned(v) }
}

unsafe fn r32(p: *const u8) -> u32 {
    // SAFETY: p points into the game's own structures.
    unsafe { p.cast::<u32>().read_unaligned() }
}

unsafe fn rp(p: *const u8) -> *mut u8 {
    // SAFETY: p points into the game's own structures.
    unsafe { r32(p) as usize as *mut u8 }
}

unsafe fn a(off: i32) -> *mut u8 {
    // SAFETY: reads the game's global base pointer.
    unsafe { DAT_009a1fe8.wrapping_offset(off as isize) }
}

unsafe fn d(off: u32) -> *mut u8 {
    // SAFETY: reads the game's global record pointer.
    unsafe { DAT_00a08104.wrapping_add(off as usize) }
}

fn adj(iv: i32, mask: u32, sh: u32) -> u16 {
    (((iv as u32).wrapping_add(((iv >> 31) as u32) & mask)) as i32 >> sh) as u16
}

fn sh(x: i64) -> u32 {
    ((x as u32) >> 15) | ((((x as u64) >> 32) as u32) << 17)
}

unsafe fn t870(idx: u32) -> u32 {
    // SAFETY: the game's own table.
    unsafe {
        (&raw mut DAT_00a00870)
            .cast::<u32>()
            .wrapping_add(idx as usize)
            .read()
    }
}

unsafe fn t870_16(idx: u32) -> u16 {
    // SAFETY: the game's own table.
    unsafe {
        (&raw mut DAT_00a00870)
            .cast::<u32>()
            .wrapping_add(idx as usize)
            .cast::<u16>()
            .read_unaligned()
    }
}

unsafe fn t874(idx: u32) -> i32 {
    // SAFETY: the game's own table.
    unsafe {
        (&raw mut DAT_00a00874)
            .cast::<i32>()
            .wrapping_add(idx as usize)
            .read()
    }
}

unsafe fn cmpn(mut n: i32, mut p: *const i8, mut q: *const i8) -> bool {
    let mut r = false;
    loop {
        if n == 0 {
            break;
        }
        n -= 1;
        // SAFETY: the game's own strings, compared as the original does.
        r = unsafe { *p == *q };
        p = p.wrapping_add(1);
        q = q.wrapping_add(1);
        if !r {
            break;
        }
    }
    r
}

unsafe fn lab_00402123(p50: *mut u8, i35: i32, u9: u32) {
    // SAFETY: the game's own record and globals.
    unsafe {
        _DAT_00a90a5e = *p50.add(0x19) as u16;
        FUN_004021a0(u9);
        *a(i35.wrapping_add(0x19)) = _DAT_00a90a5e as u8;
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004015b0() {
    // SAFETY: every pointer and global is the game's own memory, accessed
    // where and in the order the original does.
    unsafe {
        if DAT_009d6941 != 0 {
            FUN_0042c0a0();
        }
        if DAT_009ec5e4 != 0 {
            return;
        }
        DAT_009ec5e4 = 0x1d;
        let mut bc: u16 = DAT_00a07cec;
        if !((bc as i32) < DAT_009d1308.wrapping_add(DAT_00a07ce8)) {
            return;
        }
        loop {
            let u9 = bc as u32;
            let i35 = u9.wrapping_mul(0x23) as i32;
            let b3 = *a(i35.wrapping_add(3));
            if 99 < b3 && b3 < 0x6a {
                let t = (u9.wrapping_sub(DAT_009d1308 as u32) as i32) >> 4;
                let i20 = (t as u32).wrapping_mul(0x1d) as i32;
                let p = a(i35.wrapping_add(0x14));
                w16(p, r16(p).wrapping_add(adj(i20, 0xf, 4)));

                let t = (u9.wrapping_sub(DAT_009d1308 as u32) as i32) >> 3;
                let i20 = (t as u32).wrapping_mul(0x1d) as i32;
                let p = a(i35.wrapping_add(0x12));
                w16(p, r16(p).wrapping_add(adj(i20, 0x1f, 5)));

                let t = (u9.wrapping_sub(DAT_009d1308 as u32) as i32) >> 4;
                let i20 = (4i32.wrapping_sub(t) as u32).wrapping_mul(0x1d) as i32;
                let p = a(i35.wrapping_add(0x10));
                w16(p, r16(p).wrapping_add(adj(i20, 0xf, 4)));
            }
            let p50 = a(i35);
            let p51 = rp(p50.add(0x1f));
            if p51.is_null() {
                if *p50.add(3) != 0 {
                    DAT_00a90a60 = rp((&raw mut DAT_00521f26)
                        .cast::<u8>()
                        .add(*p50.add(3) as usize * 0x1a));
                    if !DAT_00a90a60.is_null() {
                        lab_00402123(p50, i35, u9);
                    }
                }
            } else {
                let b3 = *p50.add(3);
                let u15 = b3 as u16;
                if u15 != 0 {
                    let u10 = u15 as u32;
                    DAT_00a90a60 = rp((&raw mut DAT_00521f26)
                        .cast::<u8>()
                        .add(u10 as usize * 0x1a));
                    let i5 = r32(p51.add(0xa9)) as i32;
                    let comb = ((b3 as u32) << 16) | (bc as u32);
                    match *p50.add(2) {
                        0..=5 => {
                            let u13: u16 = if u15 < 0x49 {
                                t870_16(u10.wrapping_mul(0x65))
                            } else {
                                FUN_004adc40(u10) as u16
                            };
                            let mut b11 = false;
                            let p52 = rp(a(i35.wrapping_add(0x1f)));
                            DAT_00a08104 = rp(p52.add(0xa9));
                            let s53 = r16(DAT_00a08104.add(0x1e5)) as i16;
                            let mut b27 = false;
                            if s53 != 0 {
                                let p25 = (&raw mut DAT_009d1320)
                                    .cast::<u8>()
                                    .add(*DAT_00a08104.add(0x34) as usize * 0x55)
                                    as *const i8;
                                let s1 = (&raw mut DAT_0051a1d0).cast::<i8>() as *const i8;
                                let s2 = (&raw mut DAT_0051a1bc).cast::<i8>() as *const i8;
                                let s3 = (&raw mut DAT_0051a1a8).cast::<i8>() as *const i8;
                                if !cmpn(8, p25, s1) && !cmpn(0x11, p25, s2) && !cmpn(0x11, p25, s3)
                                {
                                    b27 = true;
                                }
                            }
                            if b27 && *DAT_00a08104.add(0x21d) != 0 {
                                let mut a9: i32 = 0;
                                let mut a8: i32 = 0;
                                let mut a0: i32 = 0;
                                loop {
                                    if *d(0x370).wrapping_offset(a8 as isize) == 2 {
                                        let u22 = *(&raw mut DAT_0051c5b9).cast::<u8>().add(
                                            (a9 as u32)
                                                .wrapping_add((*d(4) as u32).wrapping_mul(0x101))
                                                as usize,
                                        ) as u32;
                                        if *d(u22 + 0x33c) != 0 {
                                            let i20: i32 = if u15 < 0x49 {
                                                let mut u21 = u22;
                                                if (t870(u10.wrapping_mul(0x65)) as i32)
                                                    <= (u22 as i32)
                                                {
                                                    u21 = t870(u10.wrapping_mul(0x65))
                                                        .wrapping_sub(1);
                                                }
                                                t874(u21.wrapping_add(u10.wrapping_mul(0x65)))
                                            } else {
                                                FUN_004ae2a0(u10, u22)
                                            };
                                            if i20 == 0x15 {
                                                let pu2 = d(0x3b2).wrapping_offset(a8 as isize);
                                                if r16(pu2) == 0xffff {
                                                    let b19 = *d(u22 + 0x30a);
                                                    let c12 = if (b19 & 1) == 0 {
                                                        b19.wrapping_sub(4)
                                                    } else {
                                                        b19.wrapping_add(4)
                                                    };
                                                    *d(u22 + 0x30a) = c12;
                                                    let u23 = FUN_00425cb0();
                                                    if u23 < 0x600 {
                                                        let p18 = d(u22 + 0x30a);
                                                        *p18 = *p18 ^ 1;
                                                    }
                                                } else {
                                                    FUN_004af000(u10, u22);
                                                    let p50b = a(i35);
                                                    FUN_00458e20(r16(pu2) as u32, 0);
                                                    DAT_00a07cdc = (DAT_00a07cdc as u32)
                                                        .wrapping_sub(r32(p50b.add(4)))
                                                        as i32;
                                                    DAT_00a07cd8 = (DAT_00a07cd8 as u32)
                                                        .wrapping_sub(r32(p50b.add(8)))
                                                        as i32;
                                                    DAT_00a07cd4 = (DAT_00a07cd4 as u32)
                                                        .wrapping_sub(r32(p50b.add(0xc)))
                                                        as i32;
                                                    let p51b = rp(p50b.add(0x1f));
                                                    if *p51b.add(0x92) != 0 {
                                                        let s56 = r16(p50b.add(0x12)) as i16;
                                                        let s57 = r16(p50b.add(0x10)) as i16;
                                                        FUN_00427bd0(s56, s57, p50b);
                                                        let s58 = r16(p50b.add(0x14)) as i16;
                                                        FUN_00427d30(s58, 0, p50b);
                                                    }
                                                    let p51b = rp(p50b.add(0x1f));
                                                    let mut s55 = [0i32; 9];
                                                    let mut k = 0usize;
                                                    while k < 9 {
                                                        s55[k] = r16(p51b.add(0x93 + 2 * k)) as i16
                                                            as i32;
                                                        k += 1;
                                                    }
                                                    let l29 =
                                                        (DAT_00a07cdc as i64) * (s55[3] as i64);
                                                    let l31 =
                                                        (DAT_00a07cd8 as i64) * (s55[4] as i64);
                                                    let l30 =
                                                        (DAT_00a07cd4 as i64) * (s55[5] as i64);
                                                    let l34 =
                                                        (DAT_00a07cdc as i64) * (s55[0] as i64);
                                                    let l33 =
                                                        (DAT_00a07cd8 as i64) * (s55[1] as i64);
                                                    let l32 =
                                                        (DAT_00a07cd4 as i64) * (s55[2] as i64);
                                                    let l6 =
                                                        (DAT_00a07cdc as i64) * (s55[6] as i64);
                                                    let l7 =
                                                        (DAT_00a07cd8 as i64) * (s55[7] as i64);
                                                    let l8 =
                                                        (DAT_00a07cd4 as i64) * (s55[8] as i64);
                                                    let l28 = __ftol();
                                                    DAT_00a07cdc = sh(l30)
                                                        .wrapping_add(sh(l29))
                                                        .wrapping_add(sh(l31))
                                                        .wrapping_sub(l28 as u32)
                                                        as i32;
                                                    let l29b = __ftol();
                                                    DAT_00a07cd8 = 0u32
                                                        .wrapping_sub(l29b as u32)
                                                        .wrapping_sub(
                                                            sh(l32)
                                                                .wrapping_add(sh(l34))
                                                                .wrapping_add(sh(l33)),
                                                        )
                                                        as i32;
                                                    let l29c = __ftol();
                                                    DAT_00a07cd4 = sh(l8)
                                                        .wrapping_add(sh(l6))
                                                        .wrapping_add(sh(l7))
                                                        .wrapping_sub(l29c as u32)
                                                        as i32;
                                                    let _ = __ftol();
                                                    let _ = __ftol();
                                                    let _ = __ftol();
                                                    let f30 = __ftol();
                                                    let f31 = __ftol();
                                                    let f29 = __ftol();
                                                    let m29 =
                                                        (DAT_00a07cdc as i64) * (f29 as i32 as i64);
                                                    let m31 =
                                                        (DAT_00a07cd8 as i64) * (f31 as i32 as i64);
                                                    let m30 =
                                                        (DAT_00a07cd4 as i64) * (f30 as i32 as i64);
                                                    let f32 = __ftol();
                                                    let f33 = __ftol();
                                                    let f34 = __ftol();
                                                    let m34 =
                                                        (DAT_00a07cdc as i64) * (f34 as i32 as i64);
                                                    let m33 =
                                                        (DAT_00a07cd8 as i64) * (f33 as i32 as i64);
                                                    let m32 =
                                                        (DAT_00a07cd4 as i64) * (f32 as i32 as i64);
                                                    let u16v = FUN_0046abf0(
                                                        sh(m32)
                                                            .wrapping_add(sh(m34))
                                                            .wrapping_add(sh(m33)),
                                                        sh(m30)
                                                            .wrapping_add(sh(m29))
                                                            .wrapping_add(sh(m31)),
                                                    );
                                                    *d(u22 + 0x30a) = (u16v >> 8) as u8;
                                                }
                                            }
                                        }
                                    }
                                    a9 = a9.wrapping_add(8);
                                    a8 = a8.wrapping_add(4);
                                    a0 = a0.wrapping_add(1);
                                    if !(a0 < (*d(0x21d) as i32)) {
                                        break;
                                    }
                                }
                            }
                            let mut u23: u16 = 0;
                            if u13 != 0 {
                                loop {
                                    let mut u22 = u23 as u32;
                                    let mut i20: i32;
                                    if u15 < 0x49 {
                                        if (t870(u10.wrapping_mul(0x65)) as i32) <= (u22 as i32) {
                                            u22 = t870(u10.wrapping_mul(0x65)).wrapping_sub(1);
                                        }
                                        i20 = t874(u22.wrapping_add(u10.wrapping_mul(0x65)));
                                    } else {
                                        i20 = FUN_004ae2a0(u10, u22);
                                    }
                                    if i20 == 3 {
                                        let p18 = (u13 as u32)
                                            .wrapping_add(0x2d8)
                                            .wrapping_add(i5 as u32)
                                            as usize
                                            as *mut u8;
                                        DAT_00a90a60 = (&raw mut DAT_0051a0b0).cast::<u8>();
                                        _DAT_00a90a5e = *p18 as u16;
                                        FUN_004021a0(u9);
                                        *p18 = _DAT_00a90a5e as u8;
                                    }
                                    if *d(7) == 3 {
                                        let i17 = FUN_004af5b0(u10);
                                        if i17 == 0 {
                                            u23 = u23.wrapping_add(3);
                                            FUN_004a7480(comb, 0);
                                        } else {
                                            FUN_00458fa0(comb, 0);
                                            let u14 = FUN_00425cb0();
                                            if u14 < 0x1800 {
                                                FUN_004592d0(comb);
                                            }
                                        }
                                    }
                                    if b27 && matches!(i20, 0xb | 0x17 | 0xc | 0x18 | 0xd | 0x19) {
                                        let u22 = u23 as u32;
                                        let b19 = *d(u22 + 0x30a);
                                        let c12 = if (b19 & 1) == 0 {
                                            b19.wrapping_sub(4)
                                        } else {
                                            b19.wrapping_add(4)
                                        };
                                        *d(u22 + 0x30a) = c12;
                                        let u14 = FUN_00425cb0();
                                        if u14 < 0x200 {
                                            let p18 = d(u22 + 0x30a);
                                            *p18 = *p18 ^ 1;
                                        }
                                    }
                                    if i20 == 7 && u15 == 4 && (*d(0x29) & 1) != 0 {
                                        if (*d(0x29) & 2) == 0 {
                                            let u22 = u23 as u32;
                                            let c12 = *d(u22 + 0x30a) as i8;
                                            if c12 != 0 {
                                                *d(u22 + 0x30a) = c12.wrapping_add(-4) as u8;
                                                if 0x80 < *d(u22 + 0x30a) {
                                                    *d(u22 + 0x30a) = 0;
                                                }
                                            }
                                        } else {
                                            let p18 = d(u23 as u32 + 0x30a);
                                            let b19 = *p18;
                                            if b19 < 0x40 {
                                                *p18 = b19.wrapping_add(4);
                                            }
                                        }
                                    }
                                    if i20 == 0x14 && (*d(0x29) & 1) != 0 {
                                        if (*d(0x29) & 2) == 0 {
                                            if u15 == 1 {
                                                let p18 = d(u23 as u32 + 0x30a);
                                                if *p18 != 0 {
                                                    *p18 = (*p18).wrapping_sub(1);
                                                    b11 = true;
                                                }
                                            } else if u15 == 4 {
                                                let p18 = d(u23 as u32 + 0x30a);
                                                if *p18 != 0 {
                                                    *p18 = (*p18).wrapping_sub(4);
                                                    b11 = true;
                                                }
                                            }
                                        } else if u15 == 1 {
                                            i20 = FUN_004ae6e0(1, u23 as u32);
                                            let lim: u8 = if -1 < i20 { 0xc } else { 8 };
                                            let p18 = d(u23 as u32 + 0x30a);
                                            let b4 = *p18;
                                            if b4 < lim {
                                                *p18 = b4.wrapping_add(1);
                                                b11 = true;
                                            }
                                        } else if u15 == 4 {
                                            let p18 = d(u23 as u32 + 0x30a);
                                            if *p18 < 0x40 {
                                                *p18 = (*p18).wrapping_add(4);
                                                b11 = true;
                                            }
                                        }
                                    }
                                    u23 = u23.wrapping_add(1);
                                    if !(u23 < u13) {
                                        break;
                                    }
                                }
                            }
                            if (*d(0x29) & 1) != 0 {
                                if (*d(0x29) & 2) == 0 {
                                    if !b11 {
                                        *d(0x29) = 0;
                                        let v = r32(a(i35.wrapping_add(0x1b))) as i32;
                                        FUN_00450650(0x80, v);
                                    }
                                } else if !b11 {
                                    *d(0x29) = 2;
                                    let v = r32(a(i35.wrapping_add(0x1b))) as i32;
                                    FUN_00450650(0x81, v);
                                }
                            }
                            if *d(7) == 0 && *d(0x246) == 1 {
                                let s = r16(d(0x248)) as i16;
                                if s != 0 {
                                    FUN_00459480(comb);
                                    FUN_00459480(comb);
                                    FUN_00459480(comb);
                                }
                            }
                        }
                        0xb | 0xd => {
                            if *p50.add(3) != b'Y' {
                                lab_00402123(p50, i35, u9);
                            } else {
                                *p50.add(0x1a) = 0;
                                let u = FUN_00425cb0();
                                if u < 0x800 {
                                    FUN_004592d0(comb);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            bc = bc.wrapping_add(1);
            if !((bc as i32) < DAT_009d1308.wrapping_add(DAT_00a07ce8)) {
                break;
            }
        }
    }
}
