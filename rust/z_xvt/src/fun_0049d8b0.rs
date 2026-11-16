// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00645adc: u16;
    static mut DAT_00645ad8: u16;
    static mut DAT_0052acd8: u8;
    static mut DAT_00709d0a: u8;
    static mut DAT_00709d0c: u8;
    static mut DAT_00709ce8: u8;
    static mut DAT_00709ceb: u8;
    static mut DAT_00709f3a: u8;
    static mut DAT_00709f30: u8;
    static mut DAT_0071b73a: u8;
    static mut DAT_0071b73c: u8;
    static mut DAT_0071b740: u8;
    static mut DAT_006d49a0: i16;
    static mut DAT_006d4980: i32;
    static mut DAT_006d4988: i32;
    static mut _DAT_00732a3c: i32;
    static mut DAT_0070ad40: i8;
    static mut DAT_0070ad42: u8;
    static mut DAT_0070ad47: u8;
    static mut DAT_0070ad48: u8;
    static mut DAT_0070ad54: u8;
    static mut DAT_0070b252: u8;
    static mut DAT_0052b083: u8;
    static mut DAT_0052b0b4: u8;
    static mut DAT_0052b0b6: u8;

    static mut DAT_006d4a6a: i16;
    static mut DAT_006d4a8c: i16;
    static mut DAT_006d4a8e: i16;
    static mut DAT_006d4a90: i16;
    static mut DAT_006d4a92: i16;
    static mut DAT_006d4a6c: u8;
    static mut DAT_006d4a6e: u8;
    static mut DAT_006d4a70: u8;
    static mut DAT_006d4a72: u8;
    static mut DAT_006d4a74: u8;
    static mut DAT_006d4a76: u8;
    static mut DAT_006d4a78: u8;
    static mut DAT_006d4a7a: u8;
    static mut DAT_006d4a7c: u8;
    static mut DAT_006d4a7e: u8;
    static mut DAT_006d4a80: u8;
    static mut DAT_006d4a82: u8;
    static mut DAT_006d4a84: u8;
    static mut DAT_006d4a86: u8;
    static mut DAT_006d4a88: u8;
    static mut DAT_006d4a8a: u8;
    static mut DAT_006d4a94: u8;
    static mut DAT_006d4a96: u8;
    static mut DAT_006d4a98: u8;
    static mut DAT_006d4a9a: u8;
    static mut DAT_006d4a9c: u8;
    static mut DAT_006d4a9e: u8;
    static mut DAT_006d4aa0: u8;
    static mut DAT_006d4aa2: u8;
    static mut DAT_006d4aa3: u8;
    static mut DAT_006d4aa4: u8;
    static mut DAT_006d4aa5: u8;
    static mut DAT_006d4aa6: u8;
    static mut DAT_006d4aa7: u8;
    static mut DAT_006d4aa8: u8;
    static mut DAT_006d4aa9: u8;
    static mut DAT_006d4aaa: u8;
    static mut DAT_006d4aab: u8;
    static mut DAT_006d4aac: u8;
    static mut DAT_006d4aad: u8;
    static mut DAT_006d4aae: u8;
    static mut DAT_006d4aaf: u8;
    static mut DAT_006d4ab0: u8;
    static mut DAT_006d4ab1: u8;
    static mut DAT_006d4ab2: u8;
    static mut DAT_006d4ab3: u8;
    static mut DAT_006d4ab4: u8;
    static mut DAT_006d4ab5: u8;
    static mut DAT_006d4ab6: u8;
    static mut DAT_006d4ab7: u8;
    static mut DAT_006d4ab8: u8;
    static mut DAT_006d4ab9: u8;
    static mut DAT_006d4aba: u8;
    static mut DAT_006d4abb: u8;
    static mut DAT_006d4abc: u8;
    static mut DAT_006d4abd: u8;
    static mut DAT_006d4abe: u8;
    static mut DAT_006d4ac8: u8;
    static mut DAT_006d4ad2: u8;
    static mut DAT_006d4adc: u8;
    static mut DAT_006d4ae6: u8;
    static mut DAT_006d4af0: u8;
    static mut DAT_006d4afa: u8;
    static mut DAT_006d4b04: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004783e0(param_1: u16, param_2: u16) -> u32;
    fn FUN_0049ec30(param_1: u16, param_2: i16, param_3: u16) -> i16;
}

unsafe fn gb(p: *mut u8, o: u32) -> u16 {
    // SAFETY: reads a byte of the game's own memory at a computed offset.
    unsafe { p.wrapping_add(o as usize).read() as u16 }
}

unsafe fn gw(p: *mut u8, o: u32) -> u16 {
    // SAFETY: reads a word of the game's own memory at a computed offset.
    unsafe { p.wrapping_add(o as usize).cast::<u16>().read_unaligned() }
}

unsafe fn ab(a: u32) -> u32 {
    // SAFETY: reads a byte at an absolute address the original dereferences.
    unsafe { (a as usize as *const u8).read() as u32 }
}

unsafe fn aw(a: u32) -> u16 {
    // SAFETY: reads a word at an absolute address the original dereferences.
    unsafe { (a as usize as *const u16).read_unaligned() }
}

macro_rules! p {
    ($n:ident) => {
        (&raw mut $n).cast::<u8>()
    };
}
macro_rules! w {
    ($n:ident, $o:expr) => {
        gw((&raw mut $n).cast::<u8>(), $o)
    };
}
macro_rules! b {
    ($n:ident, $o:expr) => {
        gb((&raw mut $n).cast::<u8>(), $o)
    };
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0049d8b0(
    param_1: u16,
    param_2: i16,
    param_3: u16,
    param_4: u16,
    param_5: i16,
    param_6: u16,
) -> i8 {
    // SAFETY: all globals, callees and pointers are the game's own memory,
    // read and written where the original does.
    unsafe {
        DAT_00645adc = 0;
        DAT_00645ad8 = 0;
        let uvar11 = param_1 as u32;
        let p3 = param_3 as u32;
        if b!(DAT_0052acd8, uvar11) == 0 {
            let mut cvar13: i8 = 4;
            match uvar11 {
                0 => return 1,
                10 => return 0,
                0xd => {
                    if param_2 == 0xc {
                        let v = b!(DAT_00709d0a, p3.wrapping_mul(3));
                        if v == 1 {
                            return 1;
                        }
                        if v == 2 {
                            return 2;
                        }
                    } else {
                        if DAT_00709ce8 == 1 {
                            return 1;
                        }
                        if DAT_00709ce8 == 2 {
                            return 2;
                        }
                    }
                }
                0xe => {
                    if param_2 == 0xc {
                        let v = b!(DAT_00709d0a, p3.wrapping_mul(3));
                        if v == 2 {
                            return 1;
                        }
                        if v == 1 {
                            return 2;
                        }
                    } else {
                        if DAT_00709ce8 == 2 {
                            return 1;
                        }
                        if DAT_00709ce8 == 1 {
                            return 2;
                        }
                    }
                }
                0x11 => {
                    if param_2 == 0xc {
                        let v = b!(DAT_00709d0c, p3.wrapping_mul(3));
                        if v == 2 {
                            return 1;
                        }
                        if v == 1 {
                            return 2;
                        }
                    } else {
                        if DAT_00709ceb == 1 {
                            return 1;
                        }
                        if DAT_00709ceb == 2 {
                            return 2;
                        }
                    }
                }
                0x12 => {
                    if DAT_00709ceb == 2 {
                        return 1;
                    }
                    if DAT_00709ceb == 1 {
                        return 2;
                    }
                }
                0x14 => {
                    return ((b!(DAT_00709f3a, p3) == 0) as i8) + 1;
                }
                0x26 => return 2,
                0x29 | 0x2a => {
                    cvar13 = 2;
                    let mut l14 = [0u16; 10];
                    let mut l28 = [0u16; 10];
                    let mut l40 = [0u16; 6];
                    let mut l34 = [0u16; 6];
                    let p14 = l14.as_mut_ptr();
                    let p28 = l28.as_mut_ptr();
                    let p40 = l40.as_mut_ptr();
                    let p34 = l34.as_mut_ptr();
                    let mut i: u16 = 0;
                    loop {
                        *p14.add(i as usize) = 0;
                        *p28.add(i as usize) = 0;
                        i = i.wrapping_add(1);
                        if !(i < 10) {
                            break;
                        }
                    }
                    i = 0;
                    loop {
                        *p40.add(i as usize) = 0;
                        *p34.add(i as usize) = 0;
                        i = i.wrapping_add(1);
                        if !(i < 6) {
                            break;
                        }
                    }
                    let mut u11: u32 = 0;
                    loop {
                        let iv1 = u11.wrapping_mul(0x5bd);
                        if b!(DAT_0071b740, iv1) != 0 {
                            let l6a = w!(DAT_0071b73c, iv1);
                            let q = p14.add(l6a as usize);
                            *q = (*q).wrapping_add(1);
                            let l6a = w!(DAT_0071b73a, iv1);
                            let q = p40.add(l6a as usize);
                            *q = (*q).wrapping_add(1);
                        }
                        let n = (u11 as u16).wrapping_add(1);
                        u11 = n as u32;
                        if !(n < 8) {
                            break;
                        }
                    }
                    u11 = 0;
                    if 0 < DAT_006d49a0 {
                        loop {
                            let o = u11.wrapping_mul(0x566);
                            if b!(DAT_0070ad54, o) != 0 {
                                let q = p28.add(b!(DAT_0070ad48, o) as usize);
                                *q = (*q).wrapping_add(1);
                                let q = p34.add(b!(DAT_0070ad47, o) as usize);
                                *q = (*q).wrapping_add(1);
                            }
                            let n = (u11 as u16).wrapping_add(1);
                            u11 = n as u32;
                            if !((n as i32) < (DAT_006d49a0 as i32)) {
                                break;
                            }
                        }
                    }
                    if param_2 == 0xd {
                        let mut u9: u16 = 0;
                        if 0 < DAT_006d49a0 {
                            loop {
                                let o = (u9 as u32).wrapping_mul(0x566);
                                let t = b!(DAT_0070ad54, o);
                                if t != 0 && (t as u32).wrapping_sub(p3) == 1 {
                                    let l60 = p!(DAT_0070b252)
                                        .wrapping_add(o as usize)
                                        .cast::<i32>()
                                        .read_unaligned();
                                    if param_1 == 0x29 {
                                        if l60 != -1 {
                                            cvar13 = 1;
                                        }
                                    } else if l60 == -1 {
                                        cvar13 = 1;
                                    }
                                }
                                u9 = u9.wrapping_add(1);
                                if !((u9 as i32) < (DAT_006d49a0 as i32)) {
                                    break;
                                }
                            }
                        }
                    } else {
                        let mut a: u32 = 0;
                        let mut c: u32 = 0;
                        if param_2 == 0xc {
                            a = *p28.add(param_3 as usize) as u32;
                            if param_1 == 0x29 {
                                c = *p14.add(param_3 as usize) as u32;
                            } else {
                                c = (*p28.add(param_3 as usize))
                                    .wrapping_sub(*p14.add(param_3 as usize))
                                    as u32;
                            }
                        } else if param_2 == 5 {
                            a = *p34.add(param_3 as usize) as u32;
                            if param_1 == 0x29 {
                                c = *p40.add(param_3 as usize) as u32;
                            } else {
                                c = (*p34.add(param_3 as usize))
                                    .wrapping_sub(*p40.add(param_3 as usize))
                                    as u32;
                            }
                        }
                        let u8v = c as u16;
                        let u9 = a as u16;
                        match param_4 {
                            0 => {
                                if u8v == u9 {
                                    return 1;
                                }
                            }
                            1 => {
                                if (FUN_004783e0(u8v, u9) as u16) > 0xbfff {
                                    return 1;
                                }
                            }
                            2 => {
                                if (FUN_004783e0(u8v, u9) as u16) > 0x7fff {
                                    return 1;
                                }
                            }
                            3 => {
                                if (FUN_004783e0(u8v, u9) as u16) > 0x3fff {
                                    return 1;
                                }
                            }
                            4 => {
                                if u8v != 0 {
                                    return 1;
                                }
                            }
                            5 => {
                                if a.wrapping_sub(c) == 1 {
                                    return 1;
                                }
                            }
                            0x10 => {
                                if (FUN_004783e0(u8v, u9) as u16) > 0xaaa9 {
                                    return 1;
                                }
                            }
                            0x11 => {
                                if (FUN_004783e0(u8v, u9) as u16) > 0x5554 {
                                    cvar13 = 1;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
            return cvar13;
        }
        if param_2 == 0 {
            return 2;
        }
        let mut u_stack_56: u16 = 0;
        let mut l54: u16 = 0;
        let mut u10: u16 = 0;
        let mut u8v: u16 = 0;
        let mut u9: u16 = 0;
        let mut lo: u16 = 0;
        let mut hi: u16 = 0;
        let mut u58: u16 = 0;
        let p6 = param_6 as u32;
        if 0 < DAT_006d49a0 {
            loop {
                'body: {
                    let uvar12 = u58 as u32;
                    let iv1 = uvar12.wrapping_mul(0x566);
                    if b!(DAT_0070ad42, iv1) == 0 {
                        break 'body;
                    }
                    if FUN_0049ec30(u58, param_2, param_3) == 0 {
                        break 'body;
                    }
                    let iv2 = uvar12.wrapping_mul(0x126);
                    lo = lo.wrapping_add(w!(DAT_006d4a6a, iv2));
                    u_stack_56 = u_stack_56.wrapping_add(b!(DAT_006d4aa2, iv2));
                    l54 = l54.wrapping_add(w!(DAT_006d4a6c, iv2));
                    let mut u6: u16 = b!(DAT_006d4aa3, iv2);
                    if 0x2d < uvar11.wrapping_sub(1) {
                        break 'body;
                    }
                    match uvar11 {
                        1 => {
                            u10 = u10.wrapping_add(w!(DAT_006d4a6c, iv2));
                            u8v = u8v.wrapping_add(b!(DAT_006d4aa3, iv2));
                            break 'body;
                        }
                        2 => {
                            u10 = u10
                                .wrapping_add(w!(DAT_006d4a6e, iv2))
                                .wrapping_add(w!(DAT_006d4a70, iv2));
                            u8v = u8v
                                .wrapping_add(b!(DAT_006d4aa4, iv2))
                                .wrapping_add(b!(DAT_006d4aa5, iv2));
                            if param_5 == 0 {
                                u9 = u9.wrapping_add(w!(DAT_006d4a8c, iv2));
                                u6 = b!(DAT_006d4ab3, iv2);
                            } else {
                                u10 = u10
                                    .wrapping_add(w!(DAT_006d4a8c, iv2))
                                    .wrapping_add(w!(DAT_006d4a8e, iv2))
                                    .wrapping_add(w!(DAT_006d4a90, iv2))
                                    .wrapping_add(w!(DAT_006d4a92, iv2));
                                u8v = u8v
                                    .wrapping_add(b!(DAT_006d4ab3, iv2))
                                    .wrapping_add(b!(DAT_006d4ab4, iv2))
                                    .wrapping_add(b!(DAT_006d4ab5, iv2))
                                    .wrapping_add(b!(DAT_006d4ab6, iv2));
                                break 'body;
                            }
                        }
                        3 => {
                            u10 = u10.wrapping_add(w!(DAT_006d4a72, iv2));
                            u9 = u9.wrapping_add(w!(DAT_006d4a74, iv2));
                            u8v = u8v.wrapping_add(b!(DAT_006d4aa6, iv2));
                            u6 = b!(DAT_006d4aa7, iv2);
                        }
                        4 => {
                            u10 = u10.wrapping_add(w!(DAT_006d4a76, iv2));
                            u9 = u9.wrapping_add(w!(DAT_006d4a78, iv2));
                            u8v = u8v.wrapping_add(b!(DAT_006d4aa8, iv2));
                            u6 = b!(DAT_006d4aa9, iv2);
                        }
                        5 => {
                            if param_6 < 10 {
                                let iv1b = p6.wrapping_add(iv2);
                                u10 = u10.wrapping_add(b!(DAT_006d4abe, iv1b));
                                u8v = u8v.wrapping_add(b!(DAT_006d4ac8, iv1b));
                                u9 = u9.wrapping_add(b!(DAT_006d4ad2, iv1b));
                                u6 = b!(DAT_006d4adc, iv1b);
                            } else {
                                u10 = u10.wrapping_add(w!(DAT_006d4a7a, iv2));
                                u9 = u9.wrapping_add(w!(DAT_006d4a7c, iv2));
                                u8v = u8v.wrapping_add(b!(DAT_006d4aaa, iv2));
                                u6 = b!(DAT_006d4aab, iv2);
                            }
                        }
                        6 => {
                            u10 = u10.wrapping_add(w!(DAT_006d4a7e, iv2));
                            u9 = u9.wrapping_add(w!(DAT_006d4a80, iv2));
                            u8v = u8v.wrapping_add(b!(DAT_006d4aac, iv2));
                            u6 = b!(DAT_006d4aad, iv2);
                        }
                        7 => {
                            u10 = u10.wrapping_add(w!(DAT_006d4a82, iv2));
                            u9 = u9.wrapping_add(w!(DAT_006d4a84, iv2));
                            u8v = u8v.wrapping_add(b!(DAT_006d4aae, iv2));
                            u6 = b!(DAT_006d4aaf, iv2);
                        }
                        8 => {
                            u10 = u10.wrapping_add(w!(DAT_006d4a86, iv2));
                            u9 = u9.wrapping_add(w!(DAT_006d4a88, iv2));
                            u8v = u8v.wrapping_add(b!(DAT_006d4ab0, iv2));
                            u6 = b!(DAT_006d4ab1, iv2);
                        }
                        9 => {
                            u10 = u10.wrapping_sub(w!(DAT_006d4a70, iv2)).wrapping_add(
                                w!(DAT_006d4a6a, iv2).wrapping_sub(w!(DAT_006d4a6e, iv2)),
                            );
                            u6 = b!(DAT_006d4aa5, iv2);
                            u8v = u8v.wrapping_sub(b!(DAT_006d4aa5, iv2)).wrapping_add(
                                b!(DAT_006d4aa2, iv2).wrapping_sub(b!(DAT_006d4aa4, iv2)),
                            );
                            u9 = u9
                                .wrapping_add(w!(DAT_006d4a6e, iv2))
                                .wrapping_add(w!(DAT_006d4a70, iv2));
                            hi = hi.wrapping_add(b!(DAT_006d4aa4, iv2));
                        }
                        0xc => {
                            u10 = u10.wrapping_add(w!(DAT_006d4a8a, iv2));
                            u8v = u8v.wrapping_add(b!(DAT_006d4ab2, iv2));
                            u9 = u9
                                .wrapping_add(w!(DAT_006d4a96, iv2))
                                .wrapping_add(w!(DAT_006d4a94, iv2))
                                .wrapping_add(w!(DAT_006d4a6e, iv2));
                            hi = hi
                                .wrapping_add(b!(DAT_006d4ab8, iv2))
                                .wrapping_add(b!(DAT_006d4ab7, iv2));
                            u6 = b!(DAT_006d4aa4, iv2);
                        }
                        0x15 | 0x16 | 0x17 | 0x18 | 0x22 | 0x23 | 0x24 | 0x25 => {
                            if _DAT_00732a3c < DAT_006d4980 {
                                let mut l4c = (_DAT_00732a3c as u32)
                                    .wrapping_mul(0x23)
                                    .wrapping_add(3)
                                    .wrapping_add(DAT_006d4988 as u32)
                                    as usize
                                    as *const i8;
                                let mut cnt =
                                    (DAT_006d4980 as u32).wrapping_sub(_DAT_00732a3c as u32);
                                loop {
                                    if *l4c != 0 && (*l4c.wrapping_add(0x13) as u8 as u32) == uvar12
                                    {
                                        let l60 =
                                            l4c.wrapping_add(0x1c).cast::<i32>().read_unaligned();
                                        let ent = ((l60 as u32).wrapping_add(0xa7) as usize
                                            as *const i32)
                                            .read_unaligned()
                                            as u32;
                                        let mut b4 = false;
                                        let l34 = ab(ent.wrapping_add(4)).wrapping_mul(0xef);
                                        let l62 = w!(DAT_0052b083, l34) as i16;
                                        let sv5 = l62.wrapping_mul(2);
                                        let l64 = aw(ent.wrapping_add(0x20b)) as i16;
                                        let l66 = aw(ent.wrapping_add(0x20d)) as i16;
                                        let sum = (l64 as i32).wrapping_add(l66 as i32);
                                        if param_1 == 0x15 {
                                            if sum < 1 {
                                                b4 = true;
                                            }
                                        } else if param_1 == 0x23 {
                                            let s = sv5 as i32;
                                            let t = ((s as u32).wrapping_add(((s >> 31) as u32) & 3)
                                                as i32)
                                                >> 2;
                                            if sum <= (t as i16 as i32) {
                                                b4 = true;
                                            }
                                        } else if param_1 == 0x22 {
                                            if sum <= ((sv5 as i32) / 2) {
                                                b4 = true;
                                            }
                                        } else if param_1 == 0x16 {
                                            let l68 = aw(ent.wrapping_add(199));
                                            let l6c = aw(ent.wrapping_add(0xc3));
                                            if (l68 >> 1) < l6c {
                                                b4 = true;
                                            }
                                        } else if param_1 == 0x24 {
                                            let l68 = aw(ent.wrapping_add(199));
                                            let l6c = aw(ent.wrapping_add(0xc3));
                                            if (l68 >> 2) < l6c {
                                                b4 = true;
                                            }
                                        } else if param_1 == 0x25 {
                                            let l68 = aw(ent.wrapping_add(199));
                                            let l6c = aw(ent.wrapping_add(0xc3));
                                            if (((l68 >> 2) as u32).wrapping_mul(3) as u16) < l6c {
                                                b4 = true;
                                            }
                                        } else if param_1 == 0x17 {
                                            let mut sv: u16 = 0;
                                            let mut l5a: u16 = 0;
                                            if ab(ent.wrapping_add(0x228)) != 0 {
                                                let l28 = ab(ent.wrapping_add(0x228));
                                                loop {
                                                    let nx = l5a.wrapping_add(1);
                                                    let i1 = b!(
                                                        DAT_0052b0b4,
                                                        l34.wrapping_add(l5a as u32)
                                                    )
                                                        as u32;
                                                    let i2 = b!(
                                                        DAT_0052b0b6,
                                                        l34.wrapping_add(l5a as u32)
                                                    )
                                                        as u32;
                                                    sv = sv
                                                        .wrapping_add(ab(ent
                                                            .wrapping_add(0x346)
                                                            .wrapping_add(i1.wrapping_mul(4)))
                                                            as u16)
                                                        .wrapping_add(ab(ent
                                                            .wrapping_add(0x346)
                                                            .wrapping_add(i2.wrapping_mul(4)))
                                                            as u16);
                                                    l5a = nx;
                                                    if !((nx as u32) < l28) {
                                                        break;
                                                    }
                                                }
                                            }
                                            if sv == 0 {
                                                b4 = true;
                                            }
                                        } else if (ab(ent.wrapping_add(0x1df)) & 0x10) == 0 {
                                            b4 = true;
                                        }
                                        let same = p!(DAT_0070ad40)
                                            .wrapping_add(iv1 as usize)
                                            .cast::<i8>()
                                            .read()
                                            == (ab(ent.wrapping_add(0xb0)) as u8 as i8);
                                        if b4 {
                                            u10 = u10.wrapping_add(1);
                                            if same {
                                                u8v = u8v.wrapping_add(1);
                                            }
                                        } else {
                                            u9 = u9.wrapping_add(1);
                                            if same {
                                                hi = hi.wrapping_add(1);
                                            }
                                        }
                                    }
                                    l4c = l4c.wrapping_add(0x23);
                                    cnt = cnt.wrapping_sub(1);
                                    if cnt == 0 {
                                        break;
                                    }
                                }
                            }
                            break 'body;
                        }
                        0x19 => {
                            u9 = u9.wrapping_add(w!(DAT_006d4a6c, iv2));
                        }
                        0x1a => {
                            u10 = u10.wrapping_add(w!(DAT_006d4a74, iv2));
                            u9 = u9.wrapping_add(w!(DAT_006d4a72, iv2));
                            u8v = u8v.wrapping_add(b!(DAT_006d4aa7, iv2));
                            u6 = b!(DAT_006d4aa6, iv2);
                        }
                        0x1b => {
                            u10 = u10.wrapping_add(w!(DAT_006d4a88, iv2));
                            u9 = u9.wrapping_add(w!(DAT_006d4a86, iv2));
                            u8v = u8v.wrapping_add(b!(DAT_006d4ab1, iv2));
                            u6 = b!(DAT_006d4ab0, iv2);
                        }
                        0x1c => {
                            u10 = u10.wrapping_add(w!(DAT_006d4a78, iv2));
                            u9 = u9.wrapping_add(w!(DAT_006d4a76, iv2));
                            u8v = u8v.wrapping_add(b!(DAT_006d4aa9, iv2));
                            u6 = b!(DAT_006d4aa8, iv2);
                        }
                        0x1d => {
                            if param_6 < 10 {
                                let iv1b = p6.wrapping_add(iv2);
                                u10 = u10.wrapping_add(b!(DAT_006d4ad2, iv1b));
                                u8v = u8v.wrapping_add(b!(DAT_006d4adc, iv1b));
                                u9 = u9.wrapping_add(b!(DAT_006d4abe, iv1b));
                                u6 = b!(DAT_006d4ac8, iv1b);
                            } else {
                                u10 = u10.wrapping_add(w!(DAT_006d4a7c, iv2));
                                u9 = u9.wrapping_add(w!(DAT_006d4a7a, iv2));
                                u8v = u8v.wrapping_add(b!(DAT_006d4aab, iv2));
                                u6 = b!(DAT_006d4aaa, iv2);
                            }
                        }
                        0x1e => {
                            u10 = u10.wrapping_add(w!(DAT_006d4a9c, iv2));
                            u8v = u8v.wrapping_add(b!(DAT_006d4abb, iv2));
                            break 'body;
                        }
                        0x1f => {
                            u10 = u10.wrapping_add(w!(DAT_006d4a80, iv2));
                            u9 = u9.wrapping_add(w!(DAT_006d4a7e, iv2));
                            u8v = u8v.wrapping_add(b!(DAT_006d4aad, iv2));
                            u6 = b!(DAT_006d4aac, iv2);
                        }
                        0x20 => {
                            u10 = u10.wrapping_add(w!(DAT_006d4a9e, iv2));
                            u8v = u8v.wrapping_add(b!(DAT_006d4abc, iv2));
                            break 'body;
                        }
                        0x21 => {
                            u10 = u10.wrapping_add(w!(DAT_006d4a84, iv2));
                            u9 = u9.wrapping_add(w!(DAT_006d4a82, iv2));
                            u8v = u8v.wrapping_add(b!(DAT_006d4aaf, iv2));
                            u6 = b!(DAT_006d4aae, iv2);
                        }
                        0x2b => {
                            u10 = u10
                                .wrapping_add(w!(DAT_006d4a8c, iv2))
                                .wrapping_add(w!(DAT_006d4a8e, iv2))
                                .wrapping_add(w!(DAT_006d4a90, iv2))
                                .wrapping_add(w!(DAT_006d4a92, iv2))
                                .wrapping_add(w!(DAT_006d4a6e, iv2))
                                .wrapping_add(w!(DAT_006d4a70, iv2));
                            u8v = u8v
                                .wrapping_add(b!(DAT_006d4ab3, iv2))
                                .wrapping_add(b!(DAT_006d4ab4, iv2))
                                .wrapping_add(b!(DAT_006d4ab5, iv2))
                                .wrapping_add(b!(DAT_006d4ab6, iv2))
                                .wrapping_add(b!(DAT_006d4aa4, iv2))
                                .wrapping_add(b!(DAT_006d4aa5, iv2));
                            break 'body;
                        }
                        0x2c => {
                            if param_6 < 10 {
                                let iv1b = p6.wrapping_add(iv2);
                                u10 = u10.wrapping_add(b!(DAT_006d4ae6, iv1b));
                                u8v = u8v.wrapping_add(b!(DAT_006d4af0, iv1b));
                                u9 = u9.wrapping_add(b!(DAT_006d4afa, iv1b));
                                u6 = b!(DAT_006d4b04, iv1b);
                            } else {
                                let mut u12: u32 = 0;
                                loop {
                                    let o = iv2.wrapping_add(u12);
                                    u10 = u10.wrapping_add(b!(DAT_006d4ae6, o));
                                    u8v = u8v.wrapping_add(b!(DAT_006d4af0, o));
                                    if b!(DAT_00709f30, u12) != 0 {
                                        u9 = u9.wrapping_add(b!(DAT_006d4afa, o));
                                        hi = hi.wrapping_add(b!(DAT_006d4b04, o));
                                    }
                                    let n = (u12 as u16).wrapping_add(1);
                                    u12 = n as u32;
                                    if !(n < 10) {
                                        break;
                                    }
                                }
                                break 'body;
                            }
                        }
                        0x2d => {
                            u10 = u10
                                .wrapping_add(w!(DAT_006d4a96, iv2))
                                .wrapping_add(w!(DAT_006d4a94, iv2))
                                .wrapping_add(w!(DAT_006d4a6e, iv2))
                                .wrapping_add(w!(DAT_006d4a70, iv2));
                            u8v = u8v
                                .wrapping_add(b!(DAT_006d4ab8, iv2))
                                .wrapping_add(b!(DAT_006d4ab7, iv2))
                                .wrapping_add(b!(DAT_006d4aa4, iv2))
                                .wrapping_add(b!(DAT_006d4aa5, iv2));
                            u9 = u9
                                .wrapping_add(w!(DAT_006d4a8a, iv2))
                                .wrapping_add(w!(DAT_006d4aa0, iv2));
                            hi = hi.wrapping_add(b!(DAT_006d4ab2, iv2));
                            u6 = b!(DAT_006d4abd, iv2);
                        }
                        0x2e => {
                            u10 = u10.wrapping_add(w!(DAT_006d4a98, iv2));
                            u9 = u9.wrapping_add(w!(DAT_006d4a9a, iv2));
                            u8v = u8v.wrapping_add(b!(DAT_006d4ab9, iv2));
                            u6 = b!(DAT_006d4aba, iv2);
                        }
                        _ => {
                            break 'body;
                        }
                    }
                    hi = hi.wrapping_add(u6);
                }
                u58 = u58.wrapping_add(1);
                if !((u58 as i32) < (DAT_006d49a0 as i32)) {
                    break;
                }
            }
        }
        let mut cvar13: i8 = 4;
        'end: {
            if lo == 0 {
                break 'end;
            }
            'sw: {
                match param_4 {
                    0 => {
                        if lo <= u10 {
                            cvar13 = 1;
                            break 'end;
                        }
                        if u9 == 0 {
                            cvar13 = 4;
                            break 'end;
                        }
                    }
                    1 => {
                        if (FUN_004783e0(u10, lo) as u16) > 0xbfff {
                            cvar13 = 1;
                            break 'end;
                        }
                        if (FUN_004783e0(u9, lo) as u16) < 0x4001 {
                            cvar13 = 4;
                            break 'end;
                        }
                    }
                    2 => {
                        if (FUN_004783e0(u10, lo) as u16) > 0x7fff {
                            cvar13 = 1;
                            break 'end;
                        }
                        if (FUN_004783e0(u9, lo) as u16) < 0x8001 {
                            cvar13 = 4;
                            break 'end;
                        }
                    }
                    3 => {
                        if (FUN_004783e0(u10, lo) as u16) > 0x3fff {
                            cvar13 = 1;
                            break 'end;
                        }
                        if (FUN_004783e0(u9, lo) as u16) < 0xc001 {
                            cvar13 = 4;
                            break 'end;
                        }
                    }
                    4 | 0xe => {
                        if u10 != 0 {
                            cvar13 = 1;
                            break 'end;
                        }
                        if u9 != lo {
                            cvar13 = 4;
                            break 'end;
                        }
                    }
                    5 => {
                        if ((lo as u32).wrapping_sub(1) as i32) <= (u10 as i32) {
                            cvar13 = 1;
                            break 'end;
                        }
                        if u9 < 2 {
                            cvar13 = 4;
                            break 'end;
                        }
                    }
                    6 => {
                        if u_stack_56 == 0 {
                            cvar13 = 4;
                            break 'end;
                        }
                        if u8v == u_stack_56 {
                            cvar13 = 1;
                            break 'end;
                        }
                        u9 = hi;
                        if u9 == 0 {
                            cvar13 = 4;
                            break 'end;
                        }
                    }
                    7 => {
                        if (lo as u32).wrapping_sub(u_stack_56 as u32) == (u10 as u32) {
                            cvar13 = 1;
                            break 'end;
                        }
                        if u9 == 0 || u8v != 0 {
                            let lt = u9 < 2;
                            u9 = u8v;
                            if lt {
                                cvar13 = 4;
                                break 'end;
                            }
                            if u9 == 0 {
                                cvar13 = 4;
                                break 'end;
                            }
                        }
                    }
                    10 => {
                        if u10 < l54 {
                            cvar13 = 4;
                        } else {
                            cvar13 = 1;
                        }
                        break 'end;
                    }
                    0xb => {
                        if (FUN_004783e0(u10, l54) as u16) < 0xc000 {
                            cvar13 = 4;
                        } else {
                            cvar13 = 1;
                        }
                        break 'end;
                    }
                    0xc => {
                        if (FUN_004783e0(u10, l54) as u16) < 0x8000 {
                            cvar13 = 4;
                        } else {
                            cvar13 = 1;
                        }
                        break 'end;
                    }
                    0xd => {
                        if (FUN_004783e0(u10, l54) as u16) < 0x4000 {
                            cvar13 = 4;
                        } else {
                            cvar13 = 1;
                        }
                        break 'end;
                    }
                    0xf => {
                        if (u10 as i32) < (l54 as i32).wrapping_sub(1) {
                            cvar13 = 4;
                        } else {
                            cvar13 = 1;
                        }
                        break 'end;
                    }
                    0x10 => {
                        if (FUN_004783e0(u10, lo) as u16) > 0xaaa9 {
                            cvar13 = 1;
                            break 'end;
                        }
                        if (FUN_004783e0(u9, lo) as u16) < 0x5556 {
                            cvar13 = 4;
                            break 'end;
                        }
                    }
                    0x11 => {
                        if (FUN_004783e0(u10, lo) as u16) > 0x5554 {
                            cvar13 = 1;
                            break 'end;
                        }
                        if (FUN_004783e0(u9, lo) as u16) < 0xaaab {
                            cvar13 = 4;
                            break 'end;
                        }
                    }
                    _ => {
                        break 'end;
                    }
                }
                break 'sw;
            }
            cvar13 = 2;
        }
        DAT_00645adc = u10;
        DAT_00645ad8 = lo;
        cvar13
    }
}
