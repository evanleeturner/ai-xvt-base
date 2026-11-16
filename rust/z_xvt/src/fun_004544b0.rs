// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732e54: *mut u8;
    static mut DAT_006d9d56: [u8; 4];
    static mut DAT_006d9d50: u16;
    static mut DAT_006d9d60: u16;
    static mut DAT_006d4988: *mut u8;
    static mut DAT_006d4980: i32;
    static mut DAT_006db580: u32;
    static mut DAT_0070ab14: i32;
    static mut DAT_0070ab18: i32;
    static mut DAT_0070ac20: i32;
    static mut DAT_00732a3c: u16;
    static mut DAT_0070ad8c: [u8; 1];
    static mut DAT_0070ad8e: [u8; 1];
    static mut DAT_0070ad8f: [u8; 1];
    static mut DAT_0070ad90: [u8; 1];
    static mut DAT_0070ad40: [u8; 1];
    static mut DAT_0070ad48: [u8; 1];
    static mut DAT_006d4a8a: [u8; 1];
    static mut DAT_006d4a98: [u8; 1];
    static mut DAT_006d4a9a: [u8; 1];
    static mut DAT_006d4aa0: [u8; 1];
    static mut DAT_006d4ab2: [u8; 1];
    static mut DAT_006d4ab9: [u8; 1];
    static mut DAT_006d4aba: [u8; 1];
    static mut DAT_006d4abd: [u8; 1];
    static mut DAT_006d4ae6: [u8; 1];
    static mut DAT_006d4af0: [u8; 1];
    static mut DAT_006d4afa: [u8; 1];
    static mut DAT_006d4b04: [u8; 1];
    static mut DAT_0052b14b: [u8; 1];
    static mut DAT_0052b14d: [u8; 1];
    static mut DAT_0052b14f: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00429880(a: u16) -> u64;
    fn FUN_0042a230(a: i32, b: u32, c: i32, d: i16, e: i16, f: u16);
    fn FUN_0042a3a0();
    fn FUN_00498450(a: u32, b: u32);
    fn FUN_0042b150(a: i32) -> i32;
    fn FUN_004a17b0(a: u32, b: u32, c: u32);
    fn FUN_004a34c0(a: u32, b: *mut c_void, c: u32);
    fn FUN_0049f9c0(a: u16, b: u16, c: u16);
    fn FUN_00421580(a: i32) -> u32;
}

macro_rules! a {
    ($n:ident) => {
        ((&raw mut $n).cast::<u8>() as usize as u32)
    };
}

macro_rules! bs {
    () => {
        (DAT_006d4988 as usize as u32)
    };
}

macro_rules! p0 {
    () => {
        (DAT_00732e54 as usize as u32)
    };
}

unsafe fn r8(a: u32) -> u8 {
    // SAFETY: the address is the game's own memory, read as the original does.
    unsafe { (a as usize as *const u8).read() }
}

unsafe fn r16(a: u32) -> u16 {
    // SAFETY: the address is the game's own memory, read as the original does.
    unsafe { (a as usize as *const u16).read_unaligned() }
}

unsafe fn r32(a: u32) -> u32 {
    // SAFETY: the address is the game's own memory, read as the original does.
    unsafe { (a as usize as *const u32).read_unaligned() }
}

unsafe fn w8(a: u32, v: u8) {
    // SAFETY: the address is the game's own memory, written as the original does.
    unsafe { (a as usize as *mut u8).write(v) }
}

unsafe fn w16(a: u32, v: u16) {
    // SAFETY: the address is the game's own memory, written as the original does.
    unsafe { (a as usize as *mut u16).write_unaligned(v) }
}

unsafe fn w32(a: u32, v: u32) {
    // SAFETY: the address is the game's own memory, written as the original does.
    unsafe { (a as usize as *mut u32).write_unaligned(v) }
}

unsafe fn inc8(a: u32) {
    // SAFETY: the address is the game's own memory.
    unsafe { w8(a, r8(a).wrapping_add(1)) }
}

unsafe fn inc16(a: u32) {
    // SAFETY: the address is the game's own memory.
    unsafe { w16(a, r16(a).wrapping_add(1)) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004544b0() -> u32 {
    // SAFETY: all globals, pointers and callees belong to the game; the
    // addresses are read and written exactly where the original does.
    unsafe {
        let mut unaff_ebp: u32 = 0;
        let mut local_8: u16 = 0;
        let local_c: u16;
        let mut u14: u16;
        let mut u8_: u32;

        w8(p0!() + 0xaf, 1);
        let g = r32(a!(DAT_006d9d56));
        w32(g.wrapping_add(0xe), 0x1d);
        let dev = DAT_006d9d60 as u32;
        if r8(p0!() + 0x1e5) == 0 {
            u14 = FUN_00429880(r8(a!(DAT_0070ad8c) + dev.wrapping_mul(0x566)) as u16) as u16;
            local_c = {
                let mut lc = 0x12u16;
                if u14 == 0xffff {
                    if r8(a!(DAT_0070ad8f) + (DAT_006d9d60 as u32).wrapping_mul(0x566)) != 0 {
                        u14 = FUN_00429880(r8(
                            a!(DAT_0070ad8e) + (DAT_006d9d60 as u32).wrapping_mul(0x566)
                        ) as u16) as u16;
                        lc = 0x13;
                    }
                }
                lc
            };
        } else {
            u14 = FUN_00429880(r8(a!(DAT_0070ad90) + dev.wrapping_mul(0x566)) as u16) as u16;
            local_c = 0x14;
        }

        if u14 != 0xffff {
            let g = r32(a!(DAT_006d9d56));
            w16(g.wrapping_add(0x18), u14);
            let iv1 = (u14 as u32).wrapping_mul(0x23);
            let t16 = r16(bs!().wrapping_add(iv1));
            let g = r32(a!(DAT_006d9d56));
            w16(g.wrapping_add(0x1a), t16);
            let g = r32(a!(DAT_006d9d56));
            w8(g.wrapping_add(0x1e), 0);
            let iv10 = bs!().wrapping_add(iv1);
            let t32 = r32(iv10.wrapping_add(0x1f));
            let t32 = r32(t32.wrapping_add(0xa7));
            let iv9 = (r8(t32.wrapping_add(4)) as u32).wrapping_mul(0xef);
            let t16 = r16(a!(DAT_0052b14f).wrapping_add(iv9));
            let s1 = r16(a!(DAT_0052b14b).wrapping_add(iv9));
            let s2 = r16(a!(DAT_0052b14d).wrapping_add(iv9));
            FUN_0042a230(
                iv10 as i32,
                t16 as u32,
                iv10 as i32,
                s1 as i16,
                s2 as i16,
                t16,
            );
            let t32 =
                r32(bs!().wrapping_add(4).wrapping_add(iv1)).wrapping_add(DAT_0070ab18 as u32);
            let g = r32(a!(DAT_006d9d56));
            w32(g.wrapping_add(0x1f), t32);
            unaff_ebp = DAT_0070ab14 as u32;
            let t32 =
                r32(bs!().wrapping_add(8).wrapping_add(iv1)).wrapping_add(DAT_0070ab14 as u32);
            let g = r32(a!(DAT_006d9d56));
            w32(g.wrapping_add(0x23), t32);
            let t32 =
                r32(bs!().wrapping_add(0xc).wrapping_add(iv1)).wrapping_add(DAT_0070ac20 as u32);
            let g = r32(a!(DAT_006d9d56));
            w32(g.wrapping_add(0x27), t32);
        }

        FUN_0042a3a0();
        if u14 == 0xffff {
            FUN_00498450(DAT_006d9d50 as u32, 0x23);
            return 1;
        }
        let t32 = r32(bs!()
            .wrapping_add((u14 as u32).wrapping_mul(0x23))
            .wrapping_add(0x1f));
        let u14b = r16(t32.wrapping_add(0x79));
        let arg = if u14b < 0x19 {
            0x28
        } else {
            (u14b as u32).wrapping_add(0x19)
        };
        FUN_00498450(DAT_006d9d50 as u32, arg);
        u8_ = DAT_006db580;
        if 0x1ff < (u8_ as i32) {
            return u8_ & 0xffff0000;
        }
        u8_ = (((unaff_ebp >> 0x10) & 0xffff) << 0x10) | (DAT_00732a3c as u32);
        if (DAT_00732a3c as i32) < DAT_006d4980 {
            loop {
                let iv1 = (u8_ & 0xffff).wrapping_mul(0x23);
                let iv9 = bs!().wrapping_add(iv1);
                if r8(iv9.wrapping_add(3)) != 0
                    && (r8(iv9.wrapping_add(0x16)) as u16) == DAT_006d9d60
                {
                    let t32 = r32(iv9.wrapping_add(0x1f));
                    let p6 = r32(t32.wrapping_add(0xa7));
                    let iv9 = FUN_0042b150(p6 as i32) as u32;
                    if (r8(p6.wrapping_add(5)) as i8) != -1 {
                        if (r8(iv9.wrapping_add(0x34)) as i8) == 10 {
                            let t32 = r32(bs!().wrapping_add(0x1b).wrapping_add(iv1));
                            if (t32 as i32) == -1 {
                                if r8(p6.wrapping_add(0x1e5)) == 0 {
                                    if r8(iv9.wrapping_add(9)) == 0
                                        && (r8(p6.wrapping_add(0x72)) != 0
                                            || (r8(p6.wrapping_add(0x73)) == 0
                                                && r8(p6.wrapping_add(0x74)) == 0))
                                    {
                                        let u11 = DAT_006d9d60 as u32;
                                        let iv10 = u11.wrapping_mul(0x126);
                                        inc16(a!(DAT_006d4a8a).wrapping_add(iv10));
                                        let c3 = r8(p6.wrapping_add(0xb0)) as i8;
                                        let c4 =
                                            r8(a!(DAT_0070ad40)
                                                .wrapping_add(u11.wrapping_mul(0x566)))
                                                as i8;
                                        if c4 == c3 {
                                            w8(a!(DAT_006d4ab2).wrapping_add(iv10), 1);
                                        }
                                        local_8 = (c4 == c3) as u16;
                                        FUN_004a17b0(
                                            0xc,
                                            (((iv10 >> 0x10) & 0xffff) << 0x10)
                                                | (DAT_006d9d60 as u32),
                                            local_8 as u32,
                                        );
                                    }
                                    if r8(p6.wrapping_add(0x1e5)) == 0
                                        && r8(iv9.wrapping_add(9)) == 1
                                        && r8(p6.wrapping_add(0x73)) == 0
                                        && r8(p6.wrapping_add(0x74)) == 0
                                    {
                                        let u11 = DAT_006d9d60 as u32;
                                        let iv9b = u11.wrapping_mul(0x126);
                                        inc16(a!(DAT_006d4aa0).wrapping_add(iv9b));
                                        if (r8(
                                            a!(DAT_0070ad40).wrapping_add(u11.wrapping_mul(0x566))
                                        ) as i8)
                                            == (r8(p6.wrapping_add(0xb0)) as i8)
                                        {
                                            w8(a!(DAT_006d4abd).wrapping_add(iv9b), 1);
                                        }
                                    }
                                }
                                FUN_004a34c0(u8_, p6 as usize as *mut c_void, 0x8c);
                                FUN_0049f9c0(u8_ as u16, DAT_006d9d60, local_c);
                                w8(bs!().wrapping_add(3).wrapping_add(iv1), 0);
                                FUN_00421580(p6 as i32);
                            }
                        }
                    }
                }
                let n = (u8_ as u16).wrapping_add(1);
                u8_ = (((u8_ >> 0x10) & 0xffff) << 0x10) | (n as u32);
                if !((n as i32) < DAT_006d4980) {
                    break;
                }
            }
        }
        let mut u12: u16 = ((DAT_006d4980 as u32) >> 0x10) as u16;
        let g = r32(a!(DAT_006d9d56));
        if r8(p0!() + 0x1e5) == 0
            && r8(g.wrapping_add(9)) == 0
            && (r8(p0!() + 0x72) != 0 || (r8(p0!() + 0x73) == 0 && r8(p0!() + 0x74) == 0))
        {
            let u8v = DAT_006d9d60 as u32;
            let iv1 = u8v.wrapping_mul(0x126);
            inc16(a!(DAT_006d4a8a).wrapping_add(iv1));
            let p6 = p0!();
            let c3 = r8(a!(DAT_0070ad40).wrapping_add(u8v.wrapping_mul(0x566))) as i8;
            let c4 = r8(p0!() + 0xb0) as i8;
            if c3 == c4 {
                w8(a!(DAT_006d4ab2).wrapping_add(iv1), 1);
            }
            FUN_004a17b0(
                0xc,
                (((p6 >> 0x10) & 0xffff) << 0x10) | (DAT_006d9d60 as u32),
                (c3 == c4) as u32,
            );
            u12 = 0;
        }
        let do_block: bool;
        if r8(p0!() + 0x1e5) == 0 {
            let g = r32(a!(DAT_006d9d56));
            if r8(g.wrapping_add(9)) == 1 {
                if r8(p0!() + 0x73) == 0 && r8(p0!() + 0x74) == 0 {
                    let u8v = DAT_006d9d60 as u32;
                    let iv1 = u8v.wrapping_mul(0x126);
                    inc16(a!(DAT_006d4aa0).wrapping_add(iv1));
                    u12 = (p0!() >> 0x10) as u16;
                    if (r8(a!(DAT_0070ad40).wrapping_add(u8v.wrapping_mul(0x566))) as i8)
                        == (r8(p0!() + 0xb0) as i8)
                    {
                        w8(a!(DAT_006d4abd).wrapping_add(iv1), 1);
                    }
                }
            }
            do_block = r8(p0!() + 0x1e5) != 0;
        } else {
            do_block = true;
        }
        if do_block {
            let u8v = DAT_006d9d60 as u32;
            let t32 = r32(bs!()
                .wrapping_add((DAT_006d9d50 as u32).wrapping_mul(0x23))
                .wrapping_add(0x1f));
            let u11 = r8(t32.wrapping_add(0x87)) as u32;
            let iv1 = u11.wrapping_add(u8v.wrapping_mul(0x126));
            w8(
                a!(DAT_006d4ae6).wrapping_add(iv1),
                r8(a!(DAT_006d4ae6).wrapping_add(u8v.wrapping_mul(0x126).wrapping_add(u11)))
                    .wrapping_add(1),
            );
            let c3 = r8(p0!() + 0xb0) as i8;
            let c4 = r8(a!(DAT_0070ad40).wrapping_add(u8v.wrapping_mul(0x566))) as i8;
            if c4 == c3 {
                inc8(a!(DAT_006d4af0).wrapping_add(iv1));
            }
            FUN_004a17b0(
                0x2c,
                (((iv1 >> 0x10) & 0xffff) << 0x10) | (DAT_006d9d60 as u32),
                (c4 == c3) as u32,
            );
            let mut k: u32 = 0;
            u12 = 0;
            loop {
                let p6 = p0!();
                if k != u11 {
                    let u15 = DAT_006d9d60 as u32;
                    if (r8(a!(DAT_0070ad48).wrapping_add(u15.wrapping_mul(0x566))) as u32) != k {
                        let iv1 = k.wrapping_add(u15.wrapping_mul(0x126));
                        w8(
                            a!(DAT_006d4afa).wrapping_add(iv1),
                            r8(a!(DAT_006d4afa)
                                .wrapping_add(u15.wrapping_mul(0x126).wrapping_add(k)))
                            .wrapping_add(1),
                        );
                        if (r8(a!(DAT_0070ad40).wrapping_add(u15.wrapping_mul(0x566))) as i8)
                            == (r8(p6.wrapping_add(0xb0)) as i8)
                        {
                            w8(a!(DAT_006d4b04).wrapping_add(iv1), 1);
                        }
                    }
                }
                k = k.wrapping_add(1);
                if !(k < 10) {
                    break;
                }
            }
        }
        FUN_004a34c0(
            ((u12 as u32) << 0x10) | (DAT_006d9d50 as u32),
            DAT_00732e54 as *mut c_void,
            0x8c,
        );
        FUN_0049f9c0(DAT_006d9d50, DAT_006d9d60, local_c);
        w8(
            bs!()
                .wrapping_add((DAT_006d9d50 as u32).wrapping_mul(0x23))
                .wrapping_add(3),
            0,
        );
        u8_ = FUN_00421580(p0!() as i32);
        let u14c = r16(p0!() + 0x66);
        if u14c != 0xffff {
            let iv1 = (u14c as u32).wrapping_mul(0x23);
            u8_ = bs!().wrapping_add(iv1);
            let t32 = r32(u8_.wrapping_add(0x1f));
            if t32 != 0 {
                let u7 = r8(u8_.wrapping_add(0x16)) as u16;
                FUN_0049f9c0(u14c, u7, local_c);
                let iv9 = r32(bs!().wrapping_add(0x1f).wrapping_add(iv1));
                let iv10 = r32(iv9.wrapping_add(0xa7));
                if r8(iv10.wrapping_add(0x1e5)) == 0 {
                    let iv9 = (u7 as u32).wrapping_mul(0x126);
                    inc16(a!(DAT_006d4a98).wrapping_add(iv9));
                    let b16 =
                        ((r8(a!(DAT_0070ad40).wrapping_add((u7 as u32).wrapping_mul(0x566))) as i8)
                            == (r8(iv10.wrapping_add(0xb0)) as i8)) as u32;
                    if b16 != 0 {
                        w8(a!(DAT_006d4ab9).wrapping_add(iv9), 1);
                    }
                    FUN_004a17b0(0x2e, ((local_8 as u32) << 0x10) | (u7 as u32), b16);
                } else {
                    let b5 = r8(iv9.wrapping_add(0x87));
                    let u8x = u7 as u32;
                    local_8 = b5 as u16;
                    let iv9 = u8x.wrapping_mul(0x126);
                    let l8 = local_8 as u32;
                    inc8(a!(DAT_006d4ae6).wrapping_add(iv9.wrapping_add(l8)));
                    let iv2 = u8x.wrapping_mul(0x566);
                    let b16 = ((r8(a!(DAT_0070ad40).wrapping_add(iv2)) as i8)
                        == (r8(iv10.wrapping_add(0xb0)) as i8))
                        as u32;
                    if b16 != 0 {
                        inc8(a!(DAT_006d4af0).wrapping_add(iv9.wrapping_add(l8)));
                    }
                    FUN_004a17b0(0x2c, ((b5 as u32) << 0x10) | (u7 as u32), b16);
                    let mut k: u32 = 0;
                    loop {
                        if k != l8 && (r8(a!(DAT_0070ad48).wrapping_add(iv2)) as u32) != k {
                            inc8(a!(DAT_006d4afa).wrapping_add(iv9.wrapping_add(k)));
                            if (r8(a!(DAT_0070ad40).wrapping_add(iv2)) as i8)
                                == (r8(iv10.wrapping_add(0xb0)) as i8)
                            {
                                w8(a!(DAT_006d4b04).wrapping_add(iv9.wrapping_add(k)), 1);
                            }
                        }
                        k = k.wrapping_add(1);
                        if !(k < 10) {
                            break;
                        }
                    }
                    let c3 = r8(a!(DAT_0070ad40).wrapping_add(iv2)) as i8;
                    inc16(a!(DAT_006d4a9a).wrapping_add(u8x.wrapping_mul(0x126)));
                    if c3 == (r8(iv10.wrapping_add(0xb0)) as i8) {
                        w8(a!(DAT_006d4aba).wrapping_add(u8x.wrapping_mul(0x126)), 1);
                    }
                }
                w8(bs!().wrapping_add(3).wrapping_add(iv1), 0);
                u8_ = FUN_00421580(iv10 as i32);
            }
        }
        u8_ & 0xffff0000
    }
}
