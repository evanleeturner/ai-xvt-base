// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00556358: u16;
    static mut DAT_00556354: u16;
    static mut DAT_00521128: u8;
    static mut DAT_009d738a: u8;
    static mut DAT_009d738c: u8;
    static mut DAT_009d7368: u8;
    static mut DAT_009d736b: u8;
    static mut DAT_009d75ba: u8;
    static mut DAT_009d75b0: u8;
    static mut DAT_009e9680: u8;
    static mut DAT_009e967c: u8;
    static mut DAT_009e967a: u8;
    static mut DAT_009a2000: i16;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1fe8: i32;
    static mut _DAT_00a07cec: i32;
    static mut DAT_009d8c94: u8;
    static mut DAT_009d8c88: u8;
    static mut DAT_009d8c87: u8;
    static mut DAT_009d8c82: u8;
    static mut DAT_009d8c80: u8;
    static mut DAT_009d9192: u8;
    static mut DAT_0051c573: u8;
    static mut DAT_0051c5aa: u8;
    static mut DAT_0051c5ac: u8;
    static mut DAT_009a20ca: u8;
    static mut DAT_009a20cc: u8;
    static mut DAT_009a20ce: u8;
    static mut DAT_009a20d0: u8;
    static mut DAT_009a20d2: u8;
    static mut DAT_009a20d4: u8;
    static mut DAT_009a20d6: u8;
    static mut DAT_009a20d8: u8;
    static mut DAT_009a20da: u8;
    static mut DAT_009a20dc: u8;
    static mut DAT_009a20de: u8;
    static mut DAT_009a20e0: u8;
    static mut DAT_009a20e2: u8;
    static mut DAT_009a20e4: u8;
    static mut DAT_009a20e6: u8;
    static mut DAT_009a20e8: u8;
    static mut DAT_009a20ea: u8;
    static mut DAT_009a20ec: u8;
    static mut DAT_009a20ee: u8;
    static mut DAT_009a20f0: u8;
    static mut DAT_009a20f2: u8;
    static mut DAT_009a20f4: u8;
    static mut DAT_009a20f6: u8;
    static mut DAT_009a20f8: u8;
    static mut DAT_009a20fa: u8;
    static mut DAT_009a20fc: u8;
    static mut DAT_009a20fe: u8;
    static mut DAT_009a2100: u8;
    static mut DAT_009a2102: u8;
    static mut DAT_009a2103: u8;
    static mut DAT_009a2104: u8;
    static mut DAT_009a2105: u8;
    static mut DAT_009a2106: u8;
    static mut DAT_009a2107: u8;
    static mut DAT_009a2108: u8;
    static mut DAT_009a2109: u8;
    static mut DAT_009a210a: u8;
    static mut DAT_009a210b: u8;
    static mut DAT_009a210c: u8;
    static mut DAT_009a210d: u8;
    static mut DAT_009a210e: u8;
    static mut DAT_009a210f: u8;
    static mut DAT_009a2110: u8;
    static mut DAT_009a2111: u8;
    static mut DAT_009a2112: u8;
    static mut DAT_009a2113: u8;
    static mut DAT_009a2114: u8;
    static mut DAT_009a2115: u8;
    static mut DAT_009a2116: u8;
    static mut DAT_009a2117: u8;
    static mut DAT_009a2118: u8;
    static mut DAT_009a2119: u8;
    static mut DAT_009a211a: u8;
    static mut DAT_009a211b: u8;
    static mut DAT_009a211c: u8;
    static mut DAT_009a211d: u8;
    static mut DAT_009a211e: u8;
    static mut DAT_009a2128: u8;
    static mut DAT_009a2132: u8;
    static mut DAT_009a213c: u8;
    static mut DAT_009a2146: u8;
    static mut DAT_009a2150: u8;
    static mut DAT_009a215a: u8;
    static mut DAT_009a2164: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00425c20(a: u16, b: u16) -> u32;
    fn FUN_00432fa0(a: u16, b: i16, c: u16) -> i16;
}

macro_rules! ad {
    ($n:ident) => {
        (&raw mut $n)
    };
}
macro_rules! S {
    ($n:ident, $o:expr) => {
        rs(&raw mut $n, $o)
    };
}
macro_rules! B {
    ($n:ident, $o:expr) => {
        rb(&raw mut $n, $o)
    };
}
macro_rules! w {
    ($a:expr $(, $b:expr)+ $(,)?) => {
        $a $(.wrapping_add($b))+
    };
}

unsafe fn rb(a: *mut u8, o: u32) -> u16 {
    // SAFETY: a points into one of the game's own tables, read where the original reads.
    unsafe { a.byte_add(o as usize).read() as u16 }
}

unsafe fn rs(a: *mut u8, o: u32) -> u16 {
    // SAFETY: a points into one of the game's own tables, read where the original reads.
    unsafe { a.byte_add(o as usize).cast::<u16>().read_unaligned() }
}

unsafe fn rd8(p: u32) -> u8 {
    // SAFETY: the address comes from the game's own object records.
    unsafe { (p as usize as *const u8).read() }
}

unsafe fn rd32(p: u32) -> u32 {
    // SAFETY: the address comes from the game's own object records.
    unsafe { (p as usize as *const u32).read_unaligned() }
}

fn hi_add(l: u32, v: u16) -> u32 {
    ((((l >> 16) as u16).wrapping_add(v) as u32) << 16) | (l & 0xffff)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00431c10(
    param_1: u16,
    param_2: i16,
    param_3: u16,
    param_4: u16,
    param_5: i16,
    param_6: u16,
) -> i8 {
    // SAFETY: all globals, tables and callees are the game's own memory; the
    // accesses mirror the original's, in the original's order.
    unsafe {
        DAT_00556358 = 0;
        DAT_00556354 = 0;
        let u12 = param_1 as u32;
        if rb(ad!(DAT_00521128), u12) == 0 {
            let mut c14: i8 = 4;
            let p3x3 = (param_3 as u32).wrapping_mul(3);
            match u12 {
                0 => return 1,
                10 => return 0,
                0xd => {
                    if param_2 == 0xc {
                        if B!(DAT_009d738a, p3x3) == 1 {
                            return 1;
                        }
                        if B!(DAT_009d738a, p3x3) == 2 {
                            return 2;
                        }
                    } else {
                        if DAT_009d7368 == 1 {
                            return 1;
                        }
                        if DAT_009d7368 == 2 {
                            return 2;
                        }
                    }
                }
                0xe => {
                    if param_2 == 0xc {
                        if B!(DAT_009d738a, p3x3) == 2 {
                            return 1;
                        }
                        if B!(DAT_009d738a, p3x3) == 1 {
                            return 2;
                        }
                    } else {
                        if DAT_009d7368 == 2 {
                            return 1;
                        }
                        if DAT_009d7368 == 1 {
                            return 2;
                        }
                    }
                }
                0x11 => {
                    if param_2 == 0xc {
                        if B!(DAT_009d738c, p3x3) == 2 {
                            return 1;
                        }
                        if B!(DAT_009d738c, p3x3) == 1 {
                            return 2;
                        }
                    } else {
                        if DAT_009d736b == 1 {
                            return 1;
                        }
                        if DAT_009d736b == 2 {
                            return 2;
                        }
                    }
                }
                0x12 => {
                    if DAT_009d736b == 2 {
                        return 1;
                    }
                    if DAT_009d736b == 1 {
                        return 2;
                    }
                }
                0x14 => {
                    return ((B!(DAT_009d75ba, param_3 as u32) == 0) as i8) + 1;
                }
                0x26 => return 2,
                0x29 | 0x2a => {
                    c14 = 2;
                    let mut stk = [0u16; 32];
                    let p = stk.as_mut_ptr();
                    let l40 = p;
                    let l34 = p.wrapping_add(6);
                    let l20 = p.wrapping_add(16);
                    let l14 = p.wrapping_add(22);
                    let mut k12: u32 = 0;
                    loop {
                        let i3 = k12.wrapping_mul(0x5bd);
                        if rb(ad!(DAT_009e9680), i3) != 0 {
                            let t = rs(ad!(DAT_009e967c), i3);
                            let q = l34.wrapping_add(t as usize);
                            q.write(q.read().wrapping_add(1));
                            let t = rs(ad!(DAT_009e967a), i3);
                            let q = l40.wrapping_add(t as usize);
                            q.write(q.read().wrapping_add(1));
                        }
                        let n = (k12 as u16).wrapping_add(1);
                        k12 = n as u32;
                        if !(n < 8) {
                            break;
                        }
                    }
                    k12 = 0;
                    if 0 < DAT_009a2000 {
                        loop {
                            let o = k12.wrapping_mul(0x566);
                            if rb(ad!(DAT_009d8c94), o) != 0 {
                                let q = l14.wrapping_add(rb(ad!(DAT_009d8c88), o) as usize);
                                q.write(q.read().wrapping_add(1));
                                let q = l20.wrapping_add(rb(ad!(DAT_009d8c87), o) as usize);
                                q.write(q.read().wrapping_add(1));
                            }
                            let n = (k12 as u16).wrapping_add(1);
                            k12 = n as u32;
                            if !((n as i32) < DAT_009a2000 as i32) {
                                break;
                            }
                        }
                    }
                    if param_2 == 0xd {
                        let mut k: u16 = 0;
                        if 0 < DAT_009a2000 {
                            loop {
                                let o = (k as u32).wrapping_mul(0x566);
                                let t32 = (ad!(DAT_009d9192))
                                    .byte_add(o as usize)
                                    .cast::<i32>()
                                    .read_unaligned();
                                let b = rb(ad!(DAT_009d8c94), o) as u32;
                                if b != 0 && b.wrapping_sub(param_3 as u32) == 1 {
                                    if param_1 == 0x29 {
                                        if t32 != -1 {
                                            c14 = 1;
                                        }
                                    } else if t32 == -1 {
                                        c14 = 1;
                                    }
                                }
                                k = k.wrapping_add(1);
                                if !((k as i32) < DAT_009a2000 as i32) {
                                    break;
                                }
                            }
                            return c14;
                        }
                    } else {
                        let mut a12: u32 = 0;
                        let mut a13: u32 = 0;
                        if param_2 == 0xc {
                            let q14 = l14.wrapping_add(param_3 as usize);
                            let q34 = l34.wrapping_add(param_3 as usize);
                            a12 = q14.read() as u32;
                            if param_1 == 0x29 {
                                a13 = q34.read() as u32;
                            } else {
                                a13 = q14.read().wrapping_sub(q34.read()) as u32;
                            }
                        } else if param_2 == 5 {
                            let q20 = l20.wrapping_add(param_3 as usize);
                            let q40 = l40.wrapping_add(param_3 as usize);
                            a12 = q20.read() as u32;
                            if param_1 == 0x29 {
                                a13 = q40.read() as u32;
                            } else {
                                a13 = q20.read().wrapping_sub(q40.read()) as u32;
                            }
                        }
                        let u9 = a13 as u16;
                        let u10 = a12 as u16;
                        match param_4 {
                            0 => {
                                if u9 == u10 {
                                    return 1;
                                }
                            }
                            1 => {
                                if (FUN_00425c20(u9, u10) as u16) > 0xbfff {
                                    return 1;
                                }
                            }
                            2 => {
                                if (FUN_00425c20(u9, u10) as u16) > 0x7fff {
                                    return 1;
                                }
                            }
                            3 => {
                                if (FUN_00425c20(u9, u10) as u16) > 0x3fff {
                                    return 1;
                                }
                            }
                            4 => {
                                if u9 != 0 {
                                    return 1;
                                }
                            }
                            5 => {
                                if a12.wrapping_sub(a13) == 1 {
                                    return 1;
                                }
                            }
                            0x10 => {
                                if (FUN_00425c20(u9, u10) as u16) > 0xaaa9 {
                                    return 1;
                                }
                            }
                            0x11 => {
                                if (FUN_00425c20(u9, u10) as u16) > 0x5554 {
                                    c14 = 1;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
            return c14;
        }
        if param_2 == 0 {
            return 2;
        }
        let mut u56: u16 = 0;
        let mut l54: u16 = 0;
        let mut u11: u16 = 0;
        let mut u9: u16 = 0;
        let mut u10: u16 = 0;
        let mut l50: u32 = 0;
        let mut s58: u16 = 0;
        if 0 < DAT_009a2000 {
            loop {
                let u13 = s58 as u32;
                let i3 = u13.wrapping_mul(0x566);
                let l20 = u13;
                'iter: {
                    if rb(ad!(DAT_009d8c82), i3) == 0 || FUN_00432fa0(s58, param_2, param_3) == 0 {
                        break 'iter;
                    }
                    let o8 = u13.wrapping_mul(0x126);
                    l50 = (l50 & 0xffff0000)
                        | ((l50 as u16).wrapping_add(S!(DAT_009a20ca, o8)) as u32);
                    l54 = w!(l54, S!(DAT_009a20cc, o8));
                    let u7 = B!(DAT_009a2102, o8);
                    u56 = w!(u56, u7);
                    if 0x2d < u12.wrapping_sub(1) {
                        break 'iter;
                    }
                    let add: u16 = match u12 {
                        1 => {
                            u11 = w!(u11, S!(DAT_009a20cc, o8));
                            u9 = w!(u9, B!(DAT_009a2103, o8));
                            break 'iter;
                        }
                        2 => {
                            u11 = w!(u11, S!(DAT_009a20ce, o8), S!(DAT_009a20d0, o8));
                            u9 = w!(u9, B!(DAT_009a2104, o8), B!(DAT_009a2105, o8));
                            if param_5 == 0 {
                                u10 = w!(u10, S!(DAT_009a20ec, o8));
                                B!(DAT_009a2113, o8)
                            } else {
                                u11 = w!(
                                    u11,
                                    S!(DAT_009a20ec, o8),
                                    S!(DAT_009a20ee, o8),
                                    S!(DAT_009a20f0, o8),
                                    S!(DAT_009a20f2, o8)
                                );
                                u9 = w!(
                                    u9,
                                    B!(DAT_009a2113, o8),
                                    B!(DAT_009a2114, o8),
                                    B!(DAT_009a2115, o8),
                                    B!(DAT_009a2116, o8)
                                );
                                break 'iter;
                            }
                        }
                        3 => {
                            u11 = w!(u11, S!(DAT_009a20d2, o8));
                            u10 = w!(u10, S!(DAT_009a20d4, o8));
                            u9 = w!(u9, B!(DAT_009a2106, o8));
                            B!(DAT_009a2107, o8)
                        }
                        4 => {
                            u11 = w!(u11, S!(DAT_009a20d6, o8));
                            u10 = w!(u10, S!(DAT_009a20d8, o8));
                            u9 = w!(u9, B!(DAT_009a2108, o8));
                            B!(DAT_009a2109, o8)
                        }
                        5 => {
                            if param_6 < 10 {
                                let o2 = (param_6 as u32).wrapping_add(o8);
                                u11 = w!(u11, B!(DAT_009a211e, o2));
                                u9 = w!(u9, B!(DAT_009a2128, o2));
                                u10 = w!(u10, B!(DAT_009a2132, o2));
                                B!(DAT_009a213c, o2)
                            } else {
                                u11 = w!(u11, S!(DAT_009a20da, o8));
                                u10 = w!(u10, S!(DAT_009a20dc, o8));
                                u9 = w!(u9, B!(DAT_009a210a, o8));
                                B!(DAT_009a210b, o8)
                            }
                        }
                        6 => {
                            u11 = w!(u11, S!(DAT_009a20de, o8));
                            u10 = w!(u10, S!(DAT_009a20e0, o8));
                            u9 = w!(u9, B!(DAT_009a210c, o8));
                            B!(DAT_009a210d, o8)
                        }
                        7 => {
                            u11 = w!(u11, S!(DAT_009a20e2, o8));
                            u10 = w!(u10, S!(DAT_009a20e4, o8));
                            u9 = w!(u9, B!(DAT_009a210e, o8));
                            B!(DAT_009a210f, o8)
                        }
                        8 => {
                            u11 = w!(u11, S!(DAT_009a20e6, o8));
                            u10 = w!(u10, S!(DAT_009a20e8, o8));
                            u9 = w!(u9, B!(DAT_009a2110, o8));
                            B!(DAT_009a2111, o8)
                        }
                        9 => {
                            u11 = u11.wrapping_sub(S!(DAT_009a20d0, o8)).wrapping_add(
                                S!(DAT_009a20ca, o8).wrapping_sub(S!(DAT_009a20ce, o8)),
                            );
                            u9 = u9
                                .wrapping_sub(B!(DAT_009a2104, o8))
                                .wrapping_add(u7.wrapping_sub(B!(DAT_009a2105, o8)));
                            u10 = w!(u10, S!(DAT_009a20ce, o8), S!(DAT_009a20d0, o8));
                            l50 = hi_add(l50, w!(B!(DAT_009a2104, o8), B!(DAT_009a2105, o8)));
                            break 'iter;
                        }
                        0xc => {
                            u11 = w!(u11, S!(DAT_009a20ea, o8));
                            u9 = w!(u9, B!(DAT_009a2112, o8));
                            u10 = w!(
                                u10,
                                S!(DAT_009a20f6, o8),
                                S!(DAT_009a20f4, o8),
                                S!(DAT_009a20ce, o8)
                            );
                            l50 = hi_add(l50, w!(B!(DAT_009a2118, o8), B!(DAT_009a2117, o8)));
                            B!(DAT_009a2104, o8)
                        }
                        0x15 | 0x16 | 0x17 | 0x18 | 0x22 | 0x23 | 0x24 | 0x25 => {
                            if _DAT_00a07cec < DAT_009a1fe0 {
                                let mut p4c: u32 = (_DAT_00a07cec as u32)
                                    .wrapping_mul(0x23)
                                    .wrapping_add(3)
                                    .wrapping_add(DAT_009a1fe8 as u32);
                                let mut cnt: i32 = DAT_009a1fe0.wrapping_sub(_DAT_00a07cec);
                                loop {
                                    if rd8(p4c) != 0 && (rd8(p4c.wrapping_add(0x13)) as u32) == l20
                                    {
                                        let p1 = rd32(p4c.wrapping_add(0x1c));
                                        let e = rd32(p1.wrapping_add(0xa9));
                                        let mut b2 = false;
                                        let l34 =
                                            (rd8(e.wrapping_add(4)) as u32).wrapping_mul(0x101);
                                        let iv = (ad!(DAT_0051c573))
                                            .byte_add(l34 as usize)
                                            .cast::<i32>()
                                            .read_unaligned();
                                        let iv4 = iv.wrapping_mul(2);
                                        let s215 = rd32(e.wrapping_add(0x215));
                                        let s211 = rd32(e.wrapping_add(0x211));
                                        let uc3 = rd32(e.wrapping_add(0xc3));
                                        let ucb = rd32(e.wrapping_add(0xcb));
                                        let sum = s215.wrapping_add(s211) as i32;
                                        if param_1 == 0x15 {
                                            if sum < 1 {
                                                b2 = true;
                                            }
                                        } else if param_1 == 0x23 {
                                            let q = ((iv4 as u32)
                                                .wrapping_add(((iv4 >> 31) as u32) & 3)
                                                as i32)
                                                >> 2;
                                            if sum <= q {
                                                b2 = true;
                                            }
                                        } else if param_1 == 0x22 {
                                            if sum <= iv4 / 2 {
                                                b2 = true;
                                            }
                                        } else if param_1 == 0x16 {
                                            if (ucb >> 1) < uc3 {
                                                b2 = true;
                                            }
                                        } else if param_1 == 0x24 {
                                            if (ucb >> 2) < uc3 {
                                                b2 = true;
                                            }
                                        } else if param_1 == 0x25 {
                                            if (ucb >> 2).wrapping_mul(3) < uc3 {
                                                b2 = true;
                                            }
                                        } else if param_1 == 0x17 {
                                            let mut s5: u16 = 0;
                                            let mut l5a: u16 = 0;
                                            let n = rd8(e.wrapping_add(0x232));
                                            if n != 0 {
                                                let l14 = n as u32;
                                                loop {
                                                    let nx = l5a.wrapping_add(1);
                                                    let idx = l34.wrapping_add(l5a as u32);
                                                    let ba = B!(DAT_0051c5aa, idx) as u32;
                                                    let bb = B!(DAT_0051c5ac, idx) as u32;
                                                    let a = rd8(e
                                                        .wrapping_add(0x372)
                                                        .wrapping_add(ba.wrapping_mul(4)))
                                                        as u16;
                                                    let b = rd8(e
                                                        .wrapping_add(0x372)
                                                        .wrapping_add(bb.wrapping_mul(4)))
                                                        as u16;
                                                    s5 = s5.wrapping_add(a).wrapping_add(b);
                                                    l5a = nx;
                                                    if !((nx as u32) < l14) {
                                                        break;
                                                    }
                                                }
                                            }
                                            if s5 == 0 {
                                                b2 = true;
                                            }
                                        } else if (rd8(e.wrapping_add(0x1e5)) & 0x10) == 0 {
                                            b2 = true;
                                        }
                                        let same = (ad!(DAT_009d8c80)).byte_add(i3 as usize).read()
                                            == rd8(e.wrapping_add(0xb0));
                                        if b2 {
                                            u11 = u11.wrapping_add(1);
                                            if same {
                                                u9 = u9.wrapping_add(1);
                                            }
                                        } else {
                                            u10 = u10.wrapping_add(1);
                                            if same {
                                                l50 = hi_add(l50, 1);
                                            }
                                        }
                                    }
                                    p4c = p4c.wrapping_add(0x23);
                                    cnt = cnt.wrapping_sub(1);
                                    if cnt == 0 {
                                        break;
                                    }
                                }
                            }
                            break 'iter;
                        }
                        0x19 => {
                            u10 = w!(u10, S!(DAT_009a20cc, o8));
                            l50 = hi_add(l50, B!(DAT_009a2103, o8));
                            break 'iter;
                        }
                        0x1a => {
                            u11 = w!(u11, S!(DAT_009a20d4, o8));
                            u10 = w!(u10, S!(DAT_009a20d2, o8));
                            u9 = w!(u9, B!(DAT_009a2107, o8));
                            B!(DAT_009a2106, o8)
                        }
                        0x1b => {
                            u11 = w!(u11, S!(DAT_009a20e8, o8));
                            u10 = w!(u10, S!(DAT_009a20e6, o8));
                            u9 = w!(u9, B!(DAT_009a2111, o8));
                            B!(DAT_009a2110, o8)
                        }
                        0x1c => {
                            u11 = w!(u11, S!(DAT_009a20d8, o8));
                            u10 = w!(u10, S!(DAT_009a20d6, o8));
                            u9 = w!(u9, B!(DAT_009a2109, o8));
                            B!(DAT_009a2108, o8)
                        }
                        0x1d => {
                            if param_6 < 10 {
                                let o2 = (param_6 as u32).wrapping_add(o8);
                                u11 = w!(u11, B!(DAT_009a2132, o2));
                                u9 = w!(u9, B!(DAT_009a213c, o2));
                                u10 = w!(u10, B!(DAT_009a211e, o2));
                                B!(DAT_009a2128, o2)
                            } else {
                                u11 = w!(u11, S!(DAT_009a20dc, o8));
                                u10 = w!(u10, S!(DAT_009a20da, o8));
                                u9 = w!(u9, B!(DAT_009a210b, o8));
                                B!(DAT_009a210a, o8)
                            }
                        }
                        0x1e => {
                            u11 = w!(u11, S!(DAT_009a20fc, o8));
                            u9 = w!(u9, B!(DAT_009a211b, o8));
                            break 'iter;
                        }
                        0x1f => {
                            u11 = w!(u11, S!(DAT_009a20e0, o8));
                            u10 = w!(u10, S!(DAT_009a20de, o8));
                            u9 = w!(u9, B!(DAT_009a210d, o8));
                            B!(DAT_009a210c, o8)
                        }
                        0x20 => {
                            u11 = w!(u11, S!(DAT_009a20fe, o8));
                            u9 = w!(u9, B!(DAT_009a211c, o8));
                            break 'iter;
                        }
                        0x21 => {
                            u11 = w!(u11, S!(DAT_009a20e4, o8));
                            u10 = w!(u10, S!(DAT_009a20e2, o8));
                            u9 = w!(u9, B!(DAT_009a210f, o8));
                            B!(DAT_009a210e, o8)
                        }
                        0x2b => {
                            u11 = w!(
                                u11,
                                S!(DAT_009a20ec, o8),
                                S!(DAT_009a20ee, o8),
                                S!(DAT_009a20f0, o8),
                                S!(DAT_009a20f2, o8),
                                S!(DAT_009a20ce, o8),
                                S!(DAT_009a20d0, o8)
                            );
                            u9 = w!(
                                u9,
                                B!(DAT_009a2113, o8),
                                B!(DAT_009a2114, o8),
                                B!(DAT_009a2115, o8),
                                B!(DAT_009a2116, o8),
                                B!(DAT_009a2104, o8),
                                B!(DAT_009a2105, o8)
                            );
                            break 'iter;
                        }
                        0x2c => {
                            if param_6 < 10 {
                                let o2 = (param_6 as u32).wrapping_add(o8);
                                u11 = w!(u11, B!(DAT_009a2146, o2));
                                u9 = w!(u9, B!(DAT_009a2150, o2));
                                u10 = w!(u10, B!(DAT_009a215a, o2));
                                B!(DAT_009a2164, o2)
                            } else {
                                let mut k13: u32 = 0;
                                loop {
                                    let oo = o8.wrapping_add(k13);
                                    u11 = w!(u11, B!(DAT_009a2146, oo));
                                    u9 = w!(u9, B!(DAT_009a2150, oo));
                                    if B!(DAT_009d75b0, k13) != 0 {
                                        u10 = w!(u10, B!(DAT_009a215a, oo));
                                        l50 = hi_add(l50, B!(DAT_009a2164, oo));
                                    }
                                    let n = (k13 as u16).wrapping_add(1);
                                    k13 = n as u32;
                                    if !(n < 10) {
                                        break;
                                    }
                                }
                                break 'iter;
                            }
                        }
                        0x2d => {
                            u11 = w!(
                                u11,
                                S!(DAT_009a20f6, o8),
                                S!(DAT_009a20f4, o8),
                                S!(DAT_009a20ce, o8),
                                S!(DAT_009a20d0, o8)
                            );
                            u9 = w!(
                                u9,
                                B!(DAT_009a2118, o8),
                                B!(DAT_009a2117, o8),
                                B!(DAT_009a2104, o8),
                                B!(DAT_009a2105, o8)
                            );
                            u10 = w!(u10, S!(DAT_009a20ea, o8), S!(DAT_009a2100, o8));
                            l50 = hi_add(l50, B!(DAT_009a2112, o8));
                            B!(DAT_009a211d, o8)
                        }
                        0x2e => {
                            u11 = w!(u11, S!(DAT_009a20f8, o8));
                            u10 = w!(u10, S!(DAT_009a20fa, o8));
                            u9 = w!(u9, B!(DAT_009a2119, o8));
                            B!(DAT_009a211a, o8)
                        }
                        _ => break 'iter,
                    };
                    l50 = hi_add(l50, add);
                }
                s58 = s58.wrapping_add(1);
                if !((s58 as i32) < DAT_009a2000 as i32) {
                    break;
                }
            }
        }
        let lo = l50 as u16;
        let hi = (l50 >> 16) as u16;
        let c: i8 = 'done: {
            if lo == 0 {
                break 'done 4;
            }
            'plain: {
                'join: {
                    match param_4 {
                        0 => {
                            if lo <= u11 {
                                break 'done 1;
                            }
                            break 'join;
                        }
                        1 => {
                            if (FUN_00425c20(u11, lo) as u16) > 0xbfff {
                                break 'done 1;
                            }
                            if (FUN_00425c20(u10, lo) as u16) < 0x4001 {
                                break 'done 4;
                            }
                            break 'plain;
                        }
                        2 => {
                            if (FUN_00425c20(u11, lo) as u16) > 0x7fff {
                                break 'done 1;
                            }
                            if (FUN_00425c20(u10, lo) as u16) < 0x8001 {
                                break 'done 4;
                            }
                            break 'plain;
                        }
                        3 => {
                            if (FUN_00425c20(u11, lo) as u16) > 0x3fff {
                                break 'done 1;
                            }
                            if (FUN_00425c20(u10, lo) as u16) < 0xc001 {
                                break 'done 4;
                            }
                            break 'plain;
                        }
                        4 | 0xe => {
                            if u11 != 0 {
                                break 'done 1;
                            }
                            if u10 != lo {
                                break 'done 4;
                            }
                            break 'plain;
                        }
                        5 => {
                            if ((lo as u32).wrapping_sub(1) as i32) <= (u11 as i32) {
                                break 'done 1;
                            }
                            if u10 < 2 {
                                break 'done 4;
                            }
                            break 'plain;
                        }
                        6 => {
                            if u56 == 0 {
                                break 'done 4;
                            }
                            if u9 == u56 {
                                break 'done 1;
                            }
                            u10 = hi;
                            break 'join;
                        }
                        7 => {
                            if (lo as u32).wrapping_sub(u11 as u32) == u56 as u32 {
                                break 'done 1;
                            }
                            if u10 == 0 || u9 != 0 {
                                let b = u10 < 2;
                                u10 = u9;
                                if b {
                                    break 'done 4;
                                }
                                break 'join;
                            }
                            break 'plain;
                        }
                        10 => {
                            if u11 < l54 {
                                break 'done 4;
                            }
                            break 'done 1;
                        }
                        0xb => {
                            if (FUN_00425c20(u11, l54) as u16) < 0xc000 {
                                break 'done 4;
                            }
                            break 'done 1;
                        }
                        0xc => {
                            if (FUN_00425c20(u11, l54) as u16) < 0x8000 {
                                break 'done 4;
                            }
                            break 'done 1;
                        }
                        0xd => {
                            if (FUN_00425c20(u11, l54) as u16) < 0x4000 {
                                break 'done 4;
                            }
                            break 'done 1;
                        }
                        0xf => {
                            if (u11 as i32) < (l54 as i32).wrapping_sub(1) {
                                break 'done 4;
                            }
                            break 'done 1;
                        }
                        0x10 => {
                            if (FUN_00425c20(u11, lo) as u16) > 0xaaa9 {
                                break 'done 1;
                            }
                            if (FUN_00425c20(u10, lo) as u16) < 0x5556 {
                                break 'done 4;
                            }
                            break 'plain;
                        }
                        0x11 => {
                            if (FUN_00425c20(u11, lo) as u16) > 0x5554 {
                                break 'done 1;
                            }
                            if (FUN_00425c20(u10, lo) as u16) < 0xaaab {
                                break 'done 4;
                            }
                            break 'plain;
                        }
                        _ => break 'done 4,
                    }
                }
                if u10 == 0 {
                    break 'done 4;
                }
            }
            2
        };
        DAT_00556358 = u11;
        DAT_00556354 = lo;
        c
    }
}
