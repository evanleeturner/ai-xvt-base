// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071e6a6: u16;
    static mut DAT_0071e6a8: i16;
    static mut DAT_0073b374: u16;
    static mut DAT_006d49a0: i16;
    static mut DAT_006d4980: u32;
    static mut DAT_006d4988: *mut u8;
    static mut DAT_00732a38: u32;
    static mut DAT_00732a3c: u32;
    static mut DAT_00703c88: u32;
    static mut DAT_00732e54: i32;

    static mut DAT_006d4a02: u8;
    static mut DAT_006d4a60: u8;
    static mut DAT_006d4a61: u8;
    static mut DAT_006d4a62: u16;
    static mut DAT_006d4a64: u8;
    static mut DAT_006d4a6a: u16;
    static mut DAT_006d4a6c: u16;
    static mut DAT_006d4a74: u16;
    static mut DAT_006d4a78: u16;
    static mut DAT_006d4a7c: u16;
    static mut DAT_006d4a80: u16;
    static mut DAT_006d4a88: u16;
    static mut DAT_006d4a8a: u16;
    static mut DAT_006d4a8c: u16;
    static mut DAT_006d4a8e: u16;
    static mut DAT_006d4a96: u16;
    static mut DAT_006d4aa2: u8;
    static mut DAT_006d4aa3: u8;
    static mut DAT_006d4aa7: u8;
    static mut DAT_006d4aa9: u8;
    static mut DAT_006d4aab: u8;
    static mut DAT_006d4aad: u8;
    static mut DAT_006d4ab1: u8;
    static mut DAT_006d4ab8: u8;

    static mut DAT_0070ad42: u8;
    static mut DAT_0070ad43: u8;
    static mut DAT_0070ad48: u8;
    static mut DAT_0070ad51: u8;
    static mut DAT_0070ad53: u8;
    static mut DAT_0070ad54: u8;
    static mut DAT_0070ad55: u8;
    static mut DAT_0070ad5d: u8;
    static mut DAT_0070ad5e: u8;
    static mut DAT_0070ad69: u8;
    static mut DAT_0070ad74: u8;
    static mut DAT_0070ad75: u8;
    static mut DAT_0070ad76: u8;
    static mut DAT_0070ad77: u8;
    static mut DAT_0070ad78: u8;
    static mut DAT_0070ad7c: u8;
    static mut DAT_0070ad86: u8;
    static mut DAT_0070ad8d: u8;
    static mut DAT_0070ad8f: u8;
    static mut DAT_0070b252: u8;

    static mut DAT_007092d4: u8;
    static mut DAT_007092d7: u8;
    static mut DAT_007092db: u8;
    static mut DAT_00709d0a: u8;
    static mut DAT_00709d0b: u8;

    static mut DAT_005247e8: u8;
    static mut DAT_005248f8: u8;
    static mut DAT_00524900: u8;
    static mut DAT_0052687a: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0049d810(param_1: *mut u8, param_2: i32) -> u32;
    fn FUN_00478470() -> u16;
    fn FUN_0041eea0(param_1: i32);
    fn FUN_004215c0(param_1: i32);
}

unsafe fn rb(p: *mut u8, i: u32) -> u8 {
    // SAFETY: p is the address of one of the game's tables.
    unsafe { p.add(i as usize).read() }
}

unsafe fn wb(p: *mut u8, i: u32, v: u8) {
    // SAFETY: p is the address of one of the game's tables.
    unsafe { p.add(i as usize).write(v) }
}

unsafe fn rw(p: *mut u16, i: u32) -> u16 {
    // SAFETY: p is the address of one of the game's tables.
    unsafe { p.add(i as usize).read_unaligned() }
}

unsafe fn ww(p: *mut u16, i: u32, v: u16) {
    // SAFETY: p is the address of one of the game's tables.
    unsafe { p.add(i as usize).write_unaligned(v) }
}

unsafe fn ri(p: *mut u8, i: u32) -> i32 {
    // SAFETY: p is the address of one of the game's tables.
    unsafe { p.add(i as usize).cast::<i32>().read_unaligned() }
}

unsafe fn fl(i: u32) -> u8 {
    // SAFETY: the game's own lookup tables.
    unsafe {
        let a = rb(&raw mut DAT_0070ad42, i) as u32;
        let b = rb(&raw mut DAT_005247e8, a) as u32;
        rb(&raw mut DAT_0052687a, b.wrapping_mul(0x16))
    }
}

unsafe fn count_free() -> u32 {
    // SAFETY: the game's own record array.
    unsafe {
        let mut n: u32 = 0;
        if DAT_00732a3c < DAT_006d4980 {
            let mut p = DAT_006d4988.add(DAT_00732a3c.wrapping_mul(0x23).wrapping_add(3) as usize);
            let mut k = DAT_006d4980.wrapping_sub(DAT_00732a3c) as i32;
            loop {
                if *p == 0 {
                    n = n.wrapping_add(1);
                }
                p = p.add(0x23);
                k = k.wrapping_sub(1);
                if k == 0 {
                    break;
                }
            }
        }
        n
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0041e080() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are read where the original reads them.
    unsafe {
        macro_rules! gb {
            ($n:ident, $i:expr) => {
                rb(&raw mut $n, $i)
            };
        }
        macro_rules! sb {
            ($n:ident, $i:expr, $v:expr) => {
                wb(&raw mut $n, $i, $v)
            };
        }
        macro_rules! gw {
            ($n:ident, $i:expr) => {
                rw(&raw mut $n, $i)
            };
        }
        macro_rules! sw {
            ($n:ident, $i:expr, $v:expr) => {
                ww(&raw mut $n, $i, $v)
            };
        }

        if DAT_0071e6a6 == 0 {
            DAT_0071e6a6 = 0xec;
            DAT_0073b374 = 0;
            if 0 < DAT_006d49a0 {
                loop {
                    let u5 = DAT_0073b374 as u32;
                    'body: {
                        if gb!(DAT_006d4a60, u5.wrapping_mul(0x126)) != 0
                            || gb!(DAT_006d4a64, u5.wrapping_mul(0x126)) != 0
                        {
                            let mut i11 = u5.wrapping_mul(0x566);
                            let i15 = ri(&raw mut DAT_0070b252, i11);
                            if (gb!(DAT_00524900, DAT_007092d4 as u32)
                                & gb!(DAT_005248f8, gb!(DAT_0070ad5d, i11) as u32))
                                == 0
                                || gb!(DAT_006d4a61, u5.wrapping_mul(0x126)) == 0
                                || gw!(DAT_006d4a6a, u5.wrapping_mul(0x93)) == 0
                                || i15 != -1
                            {
                                break 'body;
                            }
                            if gb!(DAT_0070ad54, i11) != 0 {
                                let i2 = (gb!(DAT_0070ad48, i11) as u32).wrapping_mul(3);
                                if (gb!(DAT_00709d0a, i2) == 1
                                    && (gb!(DAT_00709d0b, i2) == 1 || gb!(DAT_00709d0b, i2) == 2))
                                    || (gb!(DAT_00709d0a, i2) == 2 && gb!(DAT_00709d0b, i2) == 1)
                                {
                                    break 'body;
                                }
                            }
                            let mut bv14 = true;
                            if fl(i11) & 0x80 == 0 {
                                if DAT_00732a3c < DAT_006d4980 {
                                    let mut p = DAT_006d4988
                                        .add(DAT_00732a3c.wrapping_mul(0x23).wrapping_add(0x1f)
                                            as usize);
                                    let mut u6 = DAT_00732a3c;
                                    loop {
                                        if *p.sub(0x1c) != 0 {
                                            let pb = p.cast::<u32>().read_unaligned() as *mut u8;
                                            if *pb == 0 && *p.sub(9) as u32 == u5 {
                                                DAT_00732e54 =
                                                    pb.add(0xa7).cast::<i32>().read_unaligned();
                                                let v = (DAT_00732e54 as *const u8).add(7).read();
                                                if v != 3 && v != 4 {
                                                    bv14 = false;
                                                    break;
                                                }
                                            }
                                        }
                                        p = p.add(0x23);
                                        u6 = u6.wrapping_add(1);
                                        if !(u6 < DAT_006d4980) {
                                            break;
                                        }
                                    }
                                }
                            } else if DAT_00732a38 < DAT_00703c88.wrapping_add(DAT_00732a38) {
                                let mut p = DAT_006d4988
                                    .add(DAT_00732a38.wrapping_mul(0x23).wrapping_add(3) as usize);
                                let mut u6 = DAT_00732a38;
                                loop {
                                    if *p != 0 && *p.add(0x13) as u32 == u5 {
                                        bv14 = false;
                                        break;
                                    }
                                    p = p.add(0x23);
                                    u6 = u6.wrapping_add(1);
                                    if !(u6 < DAT_00703c88.wrapping_add(DAT_00732a38)) {
                                        break;
                                    }
                                }
                            }
                            if bv14 {
                                bv14 = false;
                                let i15 = ri(&raw mut DAT_0070b252, i11);
                                if i15 == -1 || DAT_007092db != 2 {
                                    if (gb!(DAT_0070ad78, i11) != 0 || gb!(DAT_0070ad7c, i11) != 0)
                                        && (FUN_0049d810(
                                            (&raw mut DAT_0070ad78).add(i11 as usize),
                                            0,
                                        ) & 1)
                                            != 0
                                    {
                                        bv14 = true;
                                    }
                                    i11 = (DAT_0073b374 as u32).wrapping_mul(0x566);
                                    let c3 = gb!(DAT_0070ad53, i11) as i8;
                                    let t = (gb!(DAT_0070ad48, i11) as u32).wrapping_mul(3);
                                    if c3 == 1
                                        && gw!(
                                            DAT_006d4a8a,
                                            (DAT_0073b374 as u32).wrapping_mul(0x93)
                                        ) != 0
                                    {
                                        bv14 = true;
                                    }
                                    if c3 == 2 && gb!(DAT_00709d0a, t) == 1 {
                                        bv14 = true;
                                    }
                                    if c3 == 3
                                        && (gb!(DAT_00709d0a, t) == 2 || gb!(DAT_00709d0b, t) == 1)
                                    {
                                        bv14 = true;
                                    }
                                }
                                if bv14 {
                                    let u5 = DAT_0073b374 as u32;
                                    let i11 = u5.wrapping_mul(0x126);
                                    let h = u5.wrapping_mul(0x93);
                                    let s8 =
                                        gw!(DAT_006d4a6a, h).wrapping_sub(gw!(DAT_006d4a6c, h));
                                    sw!(DAT_006d4a6c, h, gw!(DAT_006d4a6c, h).wrapping_add(s8));
                                    let c3 =
                                        gb!(DAT_006d4aa2, i11).wrapping_sub(gb!(DAT_006d4aa3, i11));
                                    sw!(DAT_006d4a96, h, gw!(DAT_006d4a96, h).wrapping_add(s8));
                                    sb!(DAT_006d4ab8, i11, gb!(DAT_006d4ab8, i11).wrapping_add(c3));
                                    sw!(DAT_006d4a7c, h, gw!(DAT_006d4a7c, h).wrapping_add(s8));
                                    sb!(DAT_006d4aab, i11, gb!(DAT_006d4aab, i11).wrapping_add(c3));
                                    sw!(DAT_006d4a88, h, gw!(DAT_006d4a88, h).wrapping_add(s8));
                                    sb!(DAT_006d4ab1, i11, gb!(DAT_006d4ab1, i11).wrapping_add(c3));
                                    sw!(DAT_006d4a78, h, gw!(DAT_006d4a78, h).wrapping_add(s8));
                                    sb!(DAT_006d4aa9, i11, gb!(DAT_006d4aa9, i11).wrapping_add(c3));
                                    sw!(DAT_006d4a74, h, gw!(DAT_006d4a74, h).wrapping_add(s8));
                                    sb!(DAT_006d4aa7, i11, gb!(DAT_006d4aa7, i11).wrapping_add(c3));
                                    sw!(DAT_006d4a80, h, gw!(DAT_006d4a80, h).wrapping_add(s8));
                                    sb!(DAT_006d4aad, i11, gb!(DAT_006d4aad, i11).wrapping_add(c3));
                                    sb!(DAT_006d4a61, i11, 0);
                                    let k = u5.wrapping_mul(0x566);
                                    if gb!(DAT_0070ad8d, k) == 0 && gb!(DAT_0070ad8f, k) == 0 {
                                        sw!(DAT_006d4a8c, h, gw!(DAT_006d4a8c, h).wrapping_add(s8));
                                    } else {
                                        sw!(DAT_006d4a8e, h, gw!(DAT_006d4a8e, h).wrapping_add(s8));
                                    }
                                } else {
                                    let k = (DAT_0073b374 as u32).wrapping_mul(0x566);
                                    if fl(k) & 0x80 == 0 {
                                        let n = count_free();
                                        bv14 = (gb!(DAT_0070ad43, k) as u32) <= n;
                                    } else {
                                        bv14 = true;
                                    }
                                    if bv14 {
                                        if fl(k) & 0x80 == 0 {
                                            FUN_0041eea0(0xffff);
                                        } else {
                                            FUN_004215c0(0xffff);
                                        }
                                        let u5 = DAT_0073b374 as u32;
                                        if gb!(DAT_006d4a61, u5.wrapping_mul(0x126)) != 0
                                            && (gb!(DAT_0070ad54, u5.wrapping_mul(0x566)) == 0
                                                || DAT_007092db != 2)
                                        {
                                            let q = u5.wrapping_mul(0x126);
                                            sb!(
                                                DAT_006d4a61,
                                                q,
                                                gb!(DAT_006d4a61, q).wrapping_sub(1)
                                            );
                                        }
                                    }
                                }
                            }
                        } else {
                            let i11 = u5.wrapping_mul(0x566);
                            let i15 = ri(&raw mut DAT_0070b252, i11);
                            if ((gb!(DAT_005248f8, gb!(DAT_0070ad5d, i11) as u32)
                                & gb!(DAT_00524900, DAT_007092d4 as u32))
                                != 0
                                || i15 != -1)
                                && gw!(DAT_006d4a6a, u5.wrapping_mul(0x93)) != 0
                            {
                                let v5 = FUN_0049d810((&raw mut DAT_0070ad5e).add(i11 as usize), 1);
                                let v6 = FUN_0049d810(
                                    (&raw mut DAT_0070ad69)
                                        .add((DAT_0073b374 as u32).wrapping_mul(0x566) as usize),
                                    1,
                                );
                                let v9 = DAT_0073b374 as u32;
                                let i11 = v9.wrapping_mul(0x566);
                                let b10: u8 = if gb!(DAT_0070ad74, i11) == 1 {
                                    (v5 as u8) | (v6 as u8)
                                } else {
                                    (v5 as u8) & (v6 as u8)
                                };
                                let i15 = ri(&raw mut DAT_0070b252, i11);
                                if (b10 & 1) != 0 && (i15 != -1 || gb!(DAT_0070ad55, i11) == 0) {
                                    let b10 = gb!(DAT_0070ad86, i11);
                                    let s8 = (gb!(DAT_0070ad76, i11) as u16)
                                        .wrapping_mul(0x3c)
                                        .wrapping_add(gb!(DAT_0070ad77, i11) as u16);
                                    let h = v9.wrapping_mul(0x93);
                                    sw!(DAT_006d4a62, h, s8);
                                    let u1 = (gb!(DAT_0070ad75, i11) as u16)
                                        .wrapping_mul(0x3c)
                                        .wrapping_add(b10 as u16);
                                    if DAT_007092d7 == 0 {
                                        sw!(DAT_006d4a62, h, (u1 >> 1).wrapping_add(s8));
                                    } else if u1 != 0 {
                                        let u4 = FUN_00478470();
                                        let hh = (DAT_0073b374 as u32).wrapping_mul(0x93);
                                        let r = (u4 as u32) % (u1 as u32);
                                        sw!(
                                            DAT_006d4a62,
                                            hh,
                                            gw!(DAT_006d4a62, hh).wrapping_add(r as u16)
                                        );
                                    }
                                    sb!(DAT_006d4a64, (DAT_0073b374 as u32).wrapping_mul(0x126), 1);
                                }
                            }
                        }
                    }
                    DAT_0073b374 = DAT_0073b374.wrapping_add(1);
                    if (DAT_006d49a0 as i32) <= (DAT_0073b374 as i32) {
                        break;
                    }
                }
            }
        }
        if DAT_0071e6a8 == 0 {
            DAT_0071e6a8 = 0xec;
            DAT_0073b374 = 0;
            if 0 < DAT_006d49a0 {
                loop {
                    let u5 = DAT_0073b374 as u32;
                    let i11 = u5.wrapping_mul(0x126);
                    if gb!(DAT_006d4a60, i11) == 0 && gb!(DAT_006d4a64, i11) == 1 {
                        if gw!(DAT_006d4a62, u5.wrapping_mul(0x93)) == 0 {
                            let i2 = u5.wrapping_mul(0x566);
                            let b10 = fl(i2);
                            let bv14: bool;
                            if b10 & 0x80 == 0 {
                                let n = count_free();
                                bv14 = (gb!(DAT_0070ad43, i2) as u32) <= n;
                            } else {
                                bv14 = true;
                            }
                            if bv14 {
                                sb!(DAT_006d4a60, i11, 1);
                                if b10 & 0x80 == 0 {
                                    sb!(DAT_006d4a61, i11, gb!(DAT_0070ad51, i2));
                                    FUN_0041eea0(0xffff);
                                } else {
                                    if DAT_006d4a02 == 2 {
                                        sb!(DAT_006d4a61, i11, gb!(DAT_0070ad51, i2));
                                    } else {
                                        sb!(DAT_006d4a61, i11, 0);
                                    }
                                    FUN_004215c0(0xffff);
                                }
                            }
                        } else {
                            let h = u5.wrapping_mul(0x93);
                            sw!(DAT_006d4a62, h, gw!(DAT_006d4a62, h).wrapping_sub(1));
                        }
                    }
                    DAT_0073b374 = DAT_0073b374.wrapping_add(1);
                    if !((DAT_0073b374 as i32) < (DAT_006d49a0 as i32)) {
                        break;
                    }
                }
            }
        }
    }
}
