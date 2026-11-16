// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_007092c1: i8;
    static mut DAT_007092c0: i8;
    static mut DAT_0071e6a2: u16;
    static mut DAT_0071e6aa: u16;
    static mut DAT_007092e0: i32;
    static mut DAT_007092e4: u32;
    static mut DAT_0071b740: [i8; 0];
    static mut DAT_0071e528: [i8; 0];
    static mut DAT_0071b73c: [i8; 0];
    static mut DAT_0071f2dd: u8;
    static mut DAT_0071f2db: u8;
    static mut DAT_0071f2dc: u8;
    static mut DAT_006d49a0: i16;
    static mut DAT_006d49a2: i16;
    static mut DAT_00524900: [u8; 0];
    static mut DAT_005248f8: [u8; 0];
    static mut DAT_007092d4: i32;
    static mut DAT_0070ad5d: [u8; 0];
    static mut DAT_0070b252: [u8; 0];
    static mut DAT_006d4b36: [i8; 0];
    static mut DAT_0070aee9: [u8; 0];
    static mut DAT_0070aee6: [u8; 0];
    static mut DAT_0070aee7: [u8; 0];
    static mut DAT_0070aef3: [u8; 0];
    static mut DAT_0070aee5: [u8; 0];
    static mut DAT_0070aee8: [u8; 0];
    static mut DAT_007092e8: [i32; 0];
    static mut DAT_006daa00: [u8; 0];
    static mut DAT_006daa01: [u8; 0];
    static mut DAT_006daa02: [u8; 0];
    static mut DAT_006daa03: [u8; 0];
    static mut DAT_006daa04: [u8; 0];
    static mut DAT_006daa05: [u8; 0];
    static mut DAT_006daa06: [u8; 0];
    static mut DAT_006daa07: [u8; 0];
    static mut DAT_006daa0a: [u8; 0];
    static mut DAT_006daa0b: [u8; 0];
    static mut DAT_006daa0c: [u8; 0];
    static mut DAT_006daa0d: [u8; 0];
    static mut DAT_006daa0e: [u8; 0];
    static mut DAT_006daa0f: [u8; 0];
    static mut DAT_006daa10: [u8; 0];
    static mut DAT_006daa11: [u8; 0];
    static mut DAT_006daa12: [u8; 0];
    static mut DAT_006daa15: [u8; 0];
    static mut DAT_006daa27: [u8; 0];
    static mut DAT_006daa29: [u8; 0];
    static mut DAT_00645ad8: u16;
    static mut DAT_00645adc: u16;
    static mut DAT_00709d28: [u16; 0];
    static mut DAT_00709e18: [u16; 0];
    static mut DAT_00709d2a: [u16; 0];
    static mut DAT_00709e1a: [u16; 0];
    static mut DAT_00709d2c: [u16; 0];
    static mut DAT_00709e1c: [u16; 0];
    static mut DAT_00709d2e: [u16; 0];
    static mut DAT_00709e1e: [u16; 0];
    static mut DAT_00709cec: [i8; 0];
    static mut DAT_00709d0a: [i8; 0];
    static mut DAT_00709d0b: [i8; 0];
    static mut DAT_006d4a02: i8;
    static mut DAT_006d4a03: i8;
    static mut DAT_00709f08: [i32; 0];
    static mut DAT_00709ce8: i8;
    static mut DAT_00709ceb: i8;
    static mut DAT_00732fe2: u16;
    static mut DAT_00732950: i16;
    static mut DAT_00732952: i16;
    static mut DAT_00732e48: i16;
    static mut DAT_0071ecf4: i32;
    static mut DAT_0072f512: [i8; 0];
    static mut DAT_0072f592: [i8; 0];
    static mut DAT_0072f612: [i8; 0];
    static mut DAT_0070a004: u16;
    static mut DAT_00709f44: [i8; 0];
    static mut DAT_00709f84: [i8; 0];
    static mut DAT_007308fa: [i8; 0];
    static mut DAT_00730905: [i8; 0];
    static mut DAT_00730921: [i8; 0];
    static mut DAT_00730920: [i8; 0];
    static mut DAT_007308f0: [i8; 0];
    static mut DAT_007308b0: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0049d8b0(
        param_1: u16,
        param_2: i16,
        param_3: u16,
        param_4: u16,
        param_5: i16,
        param_6: u16,
    ) -> i8;
    fn FUN_004440b0(param_1: u32, param_2: i32) -> u8;
    fn FUN_004a1d70(param_1: i32, param_2: i32);
    fn FUN_004a3490(param_1: i32, param_2: *mut core::ffi::c_void);
    fn FUN_0040c290(param_1: i32);
    fn FUN_00432cc0(param_1: i32) -> *mut i32;
    fn FUN_0049d810(param_1: *mut i8, param_2: i32) -> u32;
}

macro_rules! ar {
    ($n:ident, $t:ty) => {
        (&raw mut $n).cast::<$t>()
    };
}

unsafe fn g<T: Copy>(p: *mut T, i: u32) -> T {
    // SAFETY: the caller indexes the game's own table.
    unsafe { p.wrapping_add(i as usize).read_unaligned() }
}

unsafe fn s<T>(p: *mut T, i: u32, v: T) {
    // SAFETY: the caller indexes the game's own table.
    unsafe { p.wrapping_add(i as usize).write_unaligned(v) }
}

fn sel(c: bool) -> u8 {
    (((0u32.wrapping_sub(c as u32)) & 2).wrapping_add(2)) as u8
}

unsafe fn emit(base: *mut i8, off: u32, a004: u16, iv19: i32, set_ds: bool, p13: *mut i8) {
    // SAFETY: the globals and callees are the game's own.
    unsafe {
        let mut uv1c: u16 = 0;
        loop {
            let iv21 = (uv1c as u32).wrapping_mul(0x40);
            let q = base.wrapping_add(iv21.wrapping_add(off) as usize);
            if q.read() != 0 {
                FUN_004a3490(0, q.cast());
                DAT_0070a004 = a004;
                if uv1c != 0 {
                    DAT_0070a004 = 0;
                }
                if set_ds {
                    DAT_00732fe2 = p13.wrapping_sub(6).cast::<u16>().read_unaligned();
                }
                let b6 = q.read() as u8;
                let code = if b6 < 0x31 || 0x36 < b6 { 0xc3 } else { 0xce };
                FUN_004a1d70(code, iv19);
            }
            uv1c = uv1c.wrapping_add(1);
            if !(uv1c < 2) {
                break;
            }
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0049c790() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // tables are indexed exactly as the original does.
    unsafe {
        if DAT_007092c1 != 0 {
            return;
        }
        if DAT_0071e6a2 == 0 || DAT_007092c0 != 0 {
            let end = ar!(DAT_0071e528, i8);
            let b740 = ar!(DAT_0071b740, i8);
            DAT_007092e0 = 0;
            let mut p13 = b740;
            loop {
                if p13.read() != 0 {
                    DAT_007092e0 = DAT_007092e0.wrapping_add(1);
                }
                p13 = p13.wrapping_add(0x5bd);
                if !(p13 < end) {
                    break;
                }
            }
            if DAT_007092e4 < DAT_007092e0 as u32 {
                DAT_007092e4 = DAT_007092e0 as u32;
            }
            let mut uv12: u16 = 0;
            let uv14: u32 = (DAT_0071f2dd as u32).wrapping_add(
                ((DAT_0071f2db as u32)
                    .wrapping_mul(0x3c)
                    .wrapping_add(DAT_0071f2dc as u32))
                .wrapping_mul(0x3c),
            );
            let d4b36 = ar!(DAT_006d4b36, i8);
            let aee9 = ar!(DAT_0070aee9, u8);
            let aee5 = ar!(DAT_0070aee5, u8);
            let e8 = ar!(DAT_007092e8, i32);
            if 0 < DAT_006d49a0 {
                loop {
                    let uv22 = uv12 as u32;
                    let mut uv24: u16 = 0;
                    let iv1 = uv22.wrapping_mul(0x566);
                    loop {
                        let itmp = ar!(DAT_0070b252, u8)
                            .wrapping_add(iv1 as usize)
                            .cast::<i32>()
                            .read_unaligned();
                        let m1 = g(ar!(DAT_00524900, u8), DAT_007092d4 as u32);
                        let m2 = g(ar!(DAT_005248f8, u8), g(ar!(DAT_0070ad5d, u8), iv1) as u32);
                        if (m1 & m2) == 0 && itmp == -1 {
                            let mut uv17: u32 = 0;
                            loop {
                                let uv23 = (uv17 as u16).wrapping_add(1);
                                s(
                                    d4b36,
                                    (uv24 as u32)
                                        .wrapping_add(uv22.wrapping_mul(0x126))
                                        .wrapping_add(uv17.wrapping_mul(8)),
                                    0,
                                );
                                uv17 = uv23 as u32;
                                if !(uv23 < 10) {
                                    break;
                                }
                            }
                        } else {
                            let mut us20: u16 = 0;
                            let iv19 = iv1.wrapping_add((uv24 as u32).wrapping_mul(0x4e));
                            loop {
                                let uv17 = us20 as u32;
                                if g(aee9, iv19.wrapping_add(uv17)) == 0 {
                                    s(
                                        d4b36,
                                        (uv24 as u32)
                                            .wrapping_add(uv17.wrapping_mul(8))
                                            .wrapping_add(uv22.wrapping_mul(0x126)),
                                        0,
                                    );
                                } else {
                                    let iv20 =
                                        uv17.wrapping_mul(8).wrapping_add(uv22.wrapping_mul(0x126));
                                    let idx = (uv24 as u32).wrapping_add(iv20);
                                    if g(d4b36, idx) == 4 {
                                        let b6 = g(ar!(DAT_0070aee6, u8), iv19);
                                        if b6 == 0 || b6 == 10 {
                                            s(d4b36, idx, 0);
                                        } else {
                                            let c11 = FUN_0049d8b0(
                                                b6 as u16,
                                                1,
                                                uv12,
                                                g(ar!(DAT_0070aee7, u8), iv19) as u16,
                                                0,
                                                us20,
                                            );
                                            let mut uv15 = c11 as u8 as u32;
                                            if (uv15 as i16) == 1 {
                                                uv15 = 8;
                                                let f3 = g(ar!(DAT_0070aef3, u8), iv19);
                                                if f3 != 0 {
                                                    let c =
                                                        ((f3 as u32).wrapping_mul(5) < uv14) as u32;
                                                    uv15 = ((0u32.wrapping_sub(c) & 0xfffa)
                                                        .wrapping_add(8))
                                                        as u16
                                                        as u32;
                                                }
                                            }
                                            let e7 = g(ar!(DAT_0070aee7, u8), iv19);
                                            if (uv15 as i16) == 8
                                                && (g(aee5, iv19) != 2
                                                    || (e7 != 0x12 && e7 != 0x13))
                                            {
                                                uv15 = 1;
                                                let add = (g(ar!(DAT_0070aee8, u8), iv19) as i8
                                                    as i32)
                                                    .wrapping_mul(0xfa);
                                                s(e8, uv17, g(e8, uv17).wrapping_add(add));
                                            }
                                            s(d4b36, idx, uv15 as i8);
                                        }
                                    }
                                }
                                us20 = us20.wrapping_add(1);
                                if !(us20 < 10) {
                                    break;
                                }
                            }
                        }
                        uv24 = uv24.wrapping_add(1);
                        if !(uv24 < 8) {
                            break;
                        }
                    }
                    uv12 = uv12.wrapping_add(1);
                    if !((uv12 as i32) < DAT_006d49a0 as i32) {
                        break;
                    }
                }
            }
            let mut us20: u16 = 0;
            let a00 = ar!(DAT_006daa00, u8);
            let d0a = ar!(DAT_00709d0a, i8);
            let d0b = ar!(DAT_00709d0b, i8);
            let cec = ar!(DAT_00709cec, i8);
            loop {
                let mut us22: u16 = 0;
                let uv22 = us20 as u32;
                let iv1 = uv22.wrapping_mul(3);
                loop {
                    let mut bv10: u8 = 0;
                    let mut bs1e: u8 = (us22 != 1) as u8;
                    let mut iv19 = uv22.wrapping_mul(7).wrapping_add(us22 as u32);
                    let mut b6 = g(a00, iv19.wrapping_mul(0x2a));
                    let mut b7 = g(ar!(DAT_006daa04, u8), iv19.wrapping_mul(0x2a));
                    let mut b9 = g(ar!(DAT_006daa0f, u8), iv19.wrapping_mul(0x2a));
                    iv19 = iv19.wrapping_mul(0x2a);
                    let mut b8 = g(ar!(DAT_006daa0b, u8), iv19);
                    let iv20 = (us22 as u32).wrapping_add(iv1);
                    let mut uv12b: u16 = 0;
                    loop {
                        let uv24 = uv12b.wrapping_add(1);
                        let iv21 = (uv12b as u32).wrapping_add(iv20.wrapping_mul(4));
                        s(ar!(DAT_00709d28, u16), iv21, 0);
                        s(ar!(DAT_00709e18, u16), iv21, 0);
                        uv12b = uv24;
                        if !(uv24 < 4) {
                            break;
                        }
                    }
                    let mut c11: i8;
                    if ((b6 == 10 || b6 == 0) && (b7 == 10 || b7 == 0))
                        && ((b8 == 10 || b8 == 0) && (b9 == 10 || b9 == 0))
                    {
                        c11 = 0;
                    } else {
                        bv10 = 1;
                        b6 = FUN_0049d8b0(
                            b6 as u16,
                            g(ar!(DAT_006daa01, u8), iv19) as i16,
                            g(ar!(DAT_006daa02, u8), iv19) as u16,
                            g(ar!(DAT_006daa03, u8), iv19) as u16,
                            0,
                            10,
                        ) as u8;
                        let u4 = DAT_00645ad8;
                        s(ar!(DAT_00709d28, u16), iv20.wrapping_mul(4), DAT_00645adc);
                        s(ar!(DAT_00709e18, u16), iv20.wrapping_mul(4), u4);
                        b7 = FUN_0049d8b0(
                            b7 as u16,
                            g(ar!(DAT_006daa05, u8), iv19) as i16,
                            g(ar!(DAT_006daa06, u8), iv19) as u16,
                            g(ar!(DAT_006daa07, u8), iv19) as u16,
                            0,
                            10,
                        ) as u8;
                        let u4 = DAT_00645ad8;
                        s(ar!(DAT_00709d2a, u16), iv20.wrapping_mul(4), DAT_00645adc);
                        c11 = g(ar!(DAT_006daa0a, u8), iv19) as i8;
                        s(ar!(DAT_00709e1a, u16), iv20.wrapping_mul(4), u4);
                        let c37 = if c11 == 1 {
                            if ((b7 | b6) & 1) == 0 {
                                b7 &= b6;
                                true
                            } else {
                                false
                            }
                        } else if (b7 & b6 & 1) == 0 {
                            b7 |= b6;
                            true
                        } else {
                            false
                        };
                        if c37 {
                            b7 = sel((b7 & 2) == 0);
                            b6 = 0;
                        } else {
                            b7 = 0;
                            b6 = 1;
                        }
                        b8 = FUN_0049d8b0(
                            b8 as u16,
                            g(ar!(DAT_006daa0c, u8), iv19) as i16,
                            g(ar!(DAT_006daa0d, u8), iv19) as u16,
                            g(ar!(DAT_006daa0e, u8), iv19) as u16,
                            0,
                            10,
                        ) as u8;
                        let u4 = DAT_00645ad8;
                        s(ar!(DAT_00709d2c, u16), iv20.wrapping_mul(4), DAT_00645adc);
                        s(ar!(DAT_00709e1c, u16), iv20.wrapping_mul(4), u4);
                        b9 = FUN_0049d8b0(
                            b9 as u16,
                            g(ar!(DAT_006daa10, u8), iv19) as i16,
                            g(ar!(DAT_006daa11, u8), iv19) as u16,
                            g(ar!(DAT_006daa12, u8), iv19) as u16,
                            0,
                            10,
                        ) as u8;
                        let u4 = DAT_00645ad8;
                        s(ar!(DAT_00709d2e, u16), iv20.wrapping_mul(4), DAT_00645adc);
                        c11 = g(ar!(DAT_006daa15, u8), iv19) as i8;
                        s(ar!(DAT_00709e1e, u16), iv20.wrapping_mul(4), u4);
                        let c32 = if c11 == 1 {
                            if ((b9 | b8) & 1) == 0 {
                                b9 &= b8;
                                true
                            } else {
                                false
                            }
                        } else if (b9 & b8 & 1) == 0 {
                            b9 |= b8;
                            true
                        } else {
                            false
                        };
                        let b18: u8;
                        let b3: u8;
                        if c32 {
                            b18 = sel((b9 & 2) == 0);
                            b3 = 0;
                        } else {
                            b18 = 0;
                            b3 = 1;
                        }
                        if g(ar!(DAT_006daa27, u8), iv19) == 1 {
                            if (b6 | b3) != 0 {
                                c11 = 1;
                            } else {
                                c11 = sel((b7 & b18 & 2) == 0) as i8;
                            }
                        } else if (b6 & b3) != 0 {
                            c11 = 1;
                        } else {
                            c11 = sel(((b7 | b18) & 2) == 0) as i8;
                        }
                    }
                    let mut joined = false;
                    if us22 == 1 {
                        if c11 == 1 {
                            bs1e = 1;
                            joined = true;
                        }
                    } else if c11 == 1 {
                        joined = true;
                    } else if c11 == 2 {
                        bs1e = 1;
                    } else if c11 == 4 {
                        bs1e = 0;
                    }
                    if joined {
                        let c2 = g(cec, iv20);
                        bv10 = 1;
                        if c2 != 1 {
                            bv10 = 1;
                            let add =
                                (g(ar!(DAT_006daa29, u8), iv19) as i8 as i32).wrapping_mul(0xfa);
                            s(e8, uv22, g(e8, uv22).wrapping_add(add));
                        }
                    }
                    let s5 = DAT_006d49a0;
                    let mut uv17: u32 = 0;
                    s(cec, iv20, c11);
                    if 0 < s5 {
                        loop {
                            let mut uv15: u32 = 0;
                            loop {
                                let iv19b = uv15
                                    .wrapping_mul(0x4e)
                                    .wrapping_add(uv17.wrapping_mul(0x566));
                                if g(aee9, iv19b.wrapping_add(uv22)) != 0
                                    && (g(aee5, iv19b) as u16) == us22
                                {
                                    let di = uv15
                                        .wrapping_add(uv22.wrapping_mul(8))
                                        .wrapping_add(uv17.wrapping_mul(0x126));
                                    if us22 == 1 {
                                        if g(d4b36, di) == 1 {
                                            bs1e = 1;
                                            bv10 = 1;
                                        }
                                    } else {
                                        let c = g(d4b36, di);
                                        if c == 1 {
                                            bv10 = 1;
                                        } else if c == 2 {
                                            bs1e = 1;
                                        } else if c == 4 && bs1e != 1 {
                                            bs1e = 0;
                                        }
                                    }
                                }
                                let u = (uv15 as i16).wrapping_add(1) as u16;
                                uv15 = u as u32;
                                if !(u < 8) {
                                    break;
                                }
                            }
                            let u = (uv17 as i16).wrapping_add(1) as u16;
                            uv17 = u as u32;
                            if !((u as i32) < DAT_006d49a0 as i32) {
                                break;
                            }
                        }
                    }
                    if bs1e == 1 && bv10 == 0 {
                        bs1e = 0;
                    }
                    if DAT_006d4a02 == 2 && us22 == 1 && bs1e == 1 && g(d0a, iv1) == 1 {
                        bs1e = 0;
                    }
                    if us22 == 0 && DAT_006d4a02 == 2 && DAT_006d4a03 != 0 {
                        bs1e = 0;
                    }
                    if ((g(d0a, iv20) != 0) as u8) != bs1e
                        && bs1e != 0
                        && g(d0a, iv20) != 1
                        && DAT_006d4a03 == 0
                    {
                        s(d0a, iv20, bs1e as i8);
                        if bs1e == 1 {
                            let mut iv19c: i32 = 10;
                            let mut is18: i32 = 0;
                            let mut p13 = d0a.wrapping_add(us22 as usize);
                            loop {
                                if p13.read() == 1 {
                                    is18 = is18.wrapping_add(1);
                                }
                                p13 = p13.wrapping_add(3);
                                iv19c = iv19c.wrapping_sub(1);
                                if iv19c == 0 {
                                    break;
                                }
                            }
                            if us22 == 0 {
                                let mut iv19d: i32 = 0;
                                if DAT_006d4a02 == 4 {
                                    let mut iv20b: i32 = 0;
                                    let mut p13 = d0a;
                                    loop {
                                        let bv = FUN_004440b0(uv22, iv20b);
                                        if bv != 0 && p13.read() == 1 {
                                            iv19d = iv19d.wrapping_add(1);
                                        }
                                        p13 = p13.wrapping_add(3);
                                        iv20b = iv20b.wrapping_add(1);
                                        if !(iv20b < 10) {
                                            break;
                                        }
                                    }
                                }
                                if iv19d == 0 {
                                    s(e8, uv22, g(e8, uv22).wrapping_add(10000));
                                }
                                s(ar!(DAT_00709f08, i32), uv22, uv14 as i32);
                                DAT_00709ce8 = 1;
                            } else if us22 == 2 {
                                DAT_00709ceb = 1;
                                s(e8, uv22, g(e8, uv22).wrapping_add(0x9c4));
                            }
                            if us22 == 0 {
                                let mut iv19e: i32 = 0;
                                let mut p13 = b740;
                                let mut is10: i32 = -1;
                                loop {
                                    if p13.read() != 0 {
                                        let ut = p13.wrapping_sub(4).cast::<u16>().read_unaligned();
                                        if ut == us20 {
                                            is10 = iv19e;
                                        }
                                    }
                                    p13 = p13.wrapping_add(0x5bd);
                                    iv19e = iv19e.wrapping_add(1);
                                    if !(p13 < end) {
                                        break;
                                    }
                                }
                                iv19e = 0;
                                p13 = b740;
                                loop {
                                    let c11b = DAT_006d4a02;
                                    if p13.read() != 0 {
                                        let ut = p13.wrapping_sub(4).cast::<u16>().read_unaligned();
                                        if ut == us20 {
                                            let mut bv10b: u8 = 1;
                                            p13.wrapping_add(0x71)
                                                .cast::<i32>()
                                                .write_unaligned(is18);
                                            DAT_00732fe2 =
                                                p13.wrapping_sub(6).cast::<u16>().read_unaligned();
                                            if c11b == 4 && g(d0b, iv1) == 1 {
                                                bv10b = 0;
                                            }
                                            if bv10b != 0 {
                                                FUN_004a1d70(0xc0, iv19e);
                                                if DAT_006d4a02 == 2 && 1 < DAT_00732e48 {
                                                    DAT_00732950 =
                                                        (is18 as i16).wrapping_add(0x11c);
                                                    FUN_004a1d70(0x127, iv19e);
                                                }
                                                if iv19e == DAT_0071ecf4 {
                                                    emit(
                                                        ar!(DAT_0072f512, i8),
                                                        uv22.wrapping_mul(0x1e5),
                                                        0x6c,
                                                        iv19e,
                                                        false,
                                                        p13,
                                                    );
                                                    if DAT_006d4a02 == 4 {
                                                        FUN_0040c290(1);
                                                    } else {
                                                        FUN_0040c290(0);
                                                    }
                                                }
                                            } else {
                                                let code = if us20 == 0 { 0xcf } else { 0xd0 };
                                                FUN_004a1d70(code, iv19e);
                                            }
                                        } else {
                                            DAT_00732952 = (is18 as i16).wrapping_add(0x11c);
                                            if (DAT_006d4a02 == 2 || DAT_006d4a02 == 1)
                                                && is10 != -1
                                            {
                                                let pv = FUN_00432cc0(is10);
                                                FUN_004a3490(0, pv.cast());
                                                FUN_004a1d70(0x128, iv19e);
                                            }
                                        }
                                    }
                                    p13 = p13.wrapping_add(0x5bd);
                                    iv19e = iv19e.wrapping_add(1);
                                    if !(p13 < end) {
                                        break;
                                    }
                                }
                            } else if us22 == 1 {
                                let mut iv19e: i32 = 0;
                                let mut p13 = b740;
                                loop {
                                    if p13.read() != 0 {
                                        let ut = p13.wrapping_sub(4).cast::<u16>().read_unaligned();
                                        if ut == us20 && g(d0a, iv1) != 2 && iv19e == DAT_0071ecf4 {
                                            let mut bv10b: u8 = 1;
                                            if DAT_006d4a02 == 4 && g(d0a, iv1) == 1 {
                                                bv10b = 0;
                                            }
                                            if bv10b != 0 {
                                                DAT_00732fe2 = p13
                                                    .wrapping_sub(6)
                                                    .cast::<u16>()
                                                    .read_unaligned();
                                                FUN_004a1d70(0xcd, iv19e);
                                                emit(
                                                    ar!(DAT_0072f592, i8),
                                                    uv22.wrapping_mul(0x1e5),
                                                    0x6e,
                                                    iv19e,
                                                    false,
                                                    p13,
                                                );
                                                if DAT_006d4a02 == 4 {
                                                    FUN_0040c290(7);
                                                } else {
                                                    FUN_0040c290(6);
                                                }
                                            } else {
                                                let code = if us20 == 0 { 0xd1 } else { 0xd2 };
                                                FUN_004a1d70(code, iv19e);
                                            }
                                        }
                                    }
                                    p13 = p13.wrapping_add(0x5bd);
                                    iv19e = iv19e.wrapping_add(1);
                                    if !(p13 < end) {
                                        break;
                                    }
                                }
                            } else {
                                let mut iv19e: i32 = 0;
                                let mut p13 = b740;
                                loop {
                                    if p13.read() != 0 {
                                        let ut = p13.wrapping_sub(4).cast::<u16>().read_unaligned();
                                        if ut == us20 && iv19e == DAT_0071ecf4 {
                                            emit(
                                                ar!(DAT_0072f612, i8),
                                                uv22.wrapping_mul(0x1e5),
                                                0x6d,
                                                iv19e,
                                                true,
                                                p13,
                                            );
                                        }
                                    }
                                    p13 = p13.wrapping_add(0x5bd);
                                    iv19e = iv19e.wrapping_add(1);
                                    if !(p13 < end) {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    us22 = us22.wrapping_add(1);
                    if !(us22 < 3) {
                        break;
                    }
                }
                us20 = us20.wrapping_add(1);
                if !(us20 < 10) {
                    break;
                }
            }
            DAT_0071e6a2 = 0xec;
        }
        if (DAT_0071e6aa as i16) < 1 {
            let mut uv12: u16 = 0;
            let f44 = ar!(DAT_00709f44, i8);
            let f84 = ar!(DAT_00709f84, i8);
            let h308f0 = ar!(DAT_007308f0, i8);
            let h308b0 = ar!(DAT_007308b0, i8);
            if 0 < DAT_006d49a2 {
                loop {
                    let uv14 = uv12 as u32;
                    let mut fire = false;
                    if g(f44, uv14) == 0 {
                        let iv1 = uv14.wrapping_mul(0x72);
                        let uv22 =
                            FUN_0049d810(ar!(DAT_007308fa, i8).wrapping_add(iv1 as usize), 0);
                        let uv17 =
                            FUN_0049d810(ar!(DAT_00730905, i8).wrapping_add(iv1 as usize), 0);
                        let uv24: u16 = if g(ar!(DAT_00730921, i8), iv1) == 1 {
                            (uv22 as u16) | (uv17 as u16)
                        } else {
                            (uv22 as u16) & (uv17 as u16)
                        };
                        if (uv24 & 1) != 0 {
                            s(f44, uv14, 1);
                            let c11 = g(ar!(DAT_00730920, i8), iv1);
                            s(f84, uv14, c11);
                            let ut = ar!(DAT_0071b73c, i8)
                                .wrapping_add(DAT_0071ecf4.wrapping_mul(0x5bd) as u32 as usize)
                                .cast::<u16>()
                                .read_unaligned();
                            if c11 == 0 && g(h308f0, iv1.wrapping_add(ut as u32)) != 0 {
                                FUN_004a3490(0, h308b0.wrapping_add(iv1 as usize).cast());
                                fire = true;
                            }
                        }
                    } else if g(f84, uv14) != 0 {
                        let c11 = g(f84, uv14).wrapping_sub(1);
                        s(f84, uv14, c11);
                        if c11 == 0 {
                            let ut = ar!(DAT_0071b73c, i8)
                                .wrapping_add(DAT_0071ecf4.wrapping_mul(0x5bd) as u32 as usize)
                                .cast::<u16>()
                                .read_unaligned();
                            if g(h308f0, uv14.wrapping_mul(0x72).wrapping_add(ut as u32)) != 0 {
                                FUN_004a3490(
                                    0,
                                    h308b0.wrapping_add(uv14.wrapping_mul(0x72) as usize).cast(),
                                );
                                fire = true;
                            }
                        }
                    }
                    if fire {
                        if uv12 < 0x10 {
                            DAT_0070a004 = uv12.wrapping_add(0x5c);
                        } else {
                            DAT_0070a004 = 0;
                        }
                        FUN_004a1d70(0xce, DAT_0071ecf4);
                    }
                    uv12 = uv12.wrapping_add(1);
                    if !((uv12 as i32) < DAT_006d49a2 as i32) {
                        break;
                    }
                }
            }
            DAT_0071e6aa = 0x49c;
        }
    }
}
