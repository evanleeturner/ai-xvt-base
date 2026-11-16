// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a08104: *mut u8;
    static mut DAT_009a73b6: [u8; 4];
    static mut DAT_009a1fe8: *mut u8;
    static mut DAT_009a73c0: u16;
    static mut DAT_009a73b0: u16;
    static mut DAT_00a07cec: u16;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a8c00: i32;
    static mut DAT_009d8a58: i32;
    static mut DAT_009d8a54: i32;
    static mut DAT_009d8b60: i32;

    static mut DAT_009d8ccc: [u8; 1];
    static mut DAT_009d8cce: [u8; 1];
    static mut DAT_009d8ccf: [u8; 1];
    static mut DAT_009d8cd0: [u8; 1];
    static mut DAT_009d8c80: [i8; 1];
    static mut DAT_009d8c88: [u8; 1];
    static mut DAT_0051c641: [u8; 1];
    static mut DAT_0051c645: [u8; 1];
    static mut DAT_0051c649: [u8; 1];
    static mut DAT_009a20ea: [u8; 1];
    static mut DAT_009a20f8: [u8; 1];
    static mut DAT_009a20fa: [u8; 1];
    static mut DAT_009a2100: [u8; 1];
    static mut DAT_009a2112: [u8; 1];
    static mut DAT_009a2119: [u8; 1];
    static mut DAT_009a211a: [u8; 1];
    static mut DAT_009a211d: [u8; 1];
    static mut DAT_009a2146: [u8; 1];
    static mut DAT_009a2150: [u8; 1];
    static mut DAT_009a215a: [u8; 1];
    static mut DAT_009a2164: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00402e20(a: u16) -> u64;
    fn FUN_004037b0(a: *mut u8, b: i32, c: i32, d: i32);
    fn FUN_00403990();
    fn FUN_004a4d90(a: u32, b: u32);
    fn FUN_00435850(a: i32, b: u32, c: u32) -> u32;
    fn FUN_00451c20(a: u32, b: *mut core::ffi::c_void, c: i32);
    fn FUN_00433d30(a: u16, b: u16, c: u16);
    fn FUN_00458780(a: i32) -> u32;
    fn FUN_00435930(a: i32, b: u32, c: u32, d: u32);
}

macro_rules! sym {
    ($d:ident) => {
        (&raw mut $d).cast::<u8>()
    };
}

fn g(p: *mut u8, off: u32) -> *mut u8 {
    p.wrapping_byte_add(off as usize)
}

fn concat22(h: u32, l: u32) -> u32 {
    ((h as u16 as u32) << 16) | (l as u16 as u32)
}

unsafe fn r32(p: *mut u8) -> u32 {
    // SAFETY: p points at game memory holding a 32-bit value.
    unsafe { p.cast::<u32>().read_unaligned() }
}

unsafe fn r16(p: *mut u8) -> u16 {
    // SAFETY: p points at game memory holding a 16-bit value.
    unsafe { p.cast::<u16>().read_unaligned() }
}

unsafe fn rp(p: *mut u8) -> *mut u8 {
    // SAFETY: p points at game memory holding a 4-byte pointer.
    unsafe { p.cast::<*mut u8>().read_unaligned() }
}

unsafe fn w32(p: *mut u8, v: u32) {
    // SAFETY: p points at writable game memory.
    unsafe { p.cast::<u32>().write_unaligned(v) }
}

unsafe fn w16(p: *mut u8, v: u16) {
    // SAFETY: p points at writable game memory.
    unsafe { p.cast::<u16>().write_unaligned(v) }
}

unsafe fn inc16(p: *mut u8) {
    // SAFETY: p points at a writable 16-bit game value.
    unsafe { w16(p, r16(p).wrapping_add(1)) }
}

unsafe fn inc8(p: *mut u8) {
    // SAFETY: p points at a writable byte of game memory.
    unsafe { *p = (*p).wrapping_add(1) }
}

unsafe fn gp() -> *mut u8 {
    // SAFETY: DAT_009a73b6 is the game's packed pointer global.
    unsafe { rp(sym!(DAT_009a73b6)) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00466a70() -> u32 {
    // SAFETY: all globals, callees and pointers are the game's own memory,
    // accessed at the offsets and in the order the original does.
    unsafe {
        let mut unaff_edi: u32 = 0;
        let mut u_var13: u16 = 0;
        let mut u_var16: u16 = 0;
        let mut u_stack8: u16 = 0;
        let mut u_var9: u32;

        *g(DAT_00a08104, 0xaf) = 1;
        let p = gp();
        w32(g(p, 0xe), 0x1d);

        if *g(DAT_00a08104, 0x1eb) == 0 {
            let idx = (DAT_009a73c0 as u32).wrapping_mul(0x566);
            u_var16 = FUN_00402e20(*g(sym!(DAT_009d8ccc), idx) as u16) as u16;
            u_stack8 = 0x12;
            if u_var16 == 0xffff {
                let idx = (DAT_009a73c0 as u32).wrapping_mul(0x566);
                if *g(sym!(DAT_009d8ccf), idx) != 0 {
                    let idx = (DAT_009a73c0 as u32).wrapping_mul(0x566);
                    u_var16 = FUN_00402e20(*g(sym!(DAT_009d8cce), idx) as u16) as u16;
                    u_stack8 = 0x13;
                }
            }
        } else {
            let idx = (DAT_009a73c0 as u32).wrapping_mul(0x566);
            u_var16 = FUN_00402e20(*g(sym!(DAT_009d8cd0), idx) as u16) as u16;
            u_stack8 = 0x14;
        }

        if u_var16 != 0xffff {
            let p = gp();
            w16(g(p, 0x18), u_var16);
            unaff_edi = (u_var16 as u32).wrapping_mul(0x23);
            let t16 = r16(g(DAT_009a1fe8, unaff_edi));
            let p = gp();
            w16(g(p, 0x1a), t16);
            let p = gp();
            *g(p, 0x1e) = 0;
            let obj = rp(g(DAT_009a1fe8, unaff_edi.wrapping_add(0x1f)));
            let tp = rp(g(obj, 0xa9));
            let u_var2 = *g(tp, 4);
            let u_var8 = ((u_var2 as u32) << 8) | (u_var2 as u32);
            let a1 = r32(g(sym!(DAT_0051c641), u_var8)) as i32;
            let a2 = r32(g(sym!(DAT_0051c645), u_var8)) as i32;
            let a3 = r32(g(sym!(DAT_0051c649), u_var8)) as i32;
            FUN_004037b0(g(DAT_009a1fe8, unaff_edi), a1, a2, a3);
            let mut t32 = r32(g(DAT_009a1fe8, unaff_edi.wrapping_add(4)));
            t32 = t32.wrapping_add(DAT_009d8a58 as u32);
            let p = gp();
            w32(g(p, 0x1f), t32);
            t32 = r32(g(DAT_009a1fe8, unaff_edi.wrapping_add(8)));
            t32 = t32.wrapping_add(DAT_009d8a54 as u32);
            let p = gp();
            w32(g(p, 0x23), t32);
            t32 = r32(g(DAT_009a1fe8, unaff_edi.wrapping_add(0xc)));
            t32 = t32.wrapping_add(DAT_009d8b60 as u32);
            let p = gp();
            w32(g(p, 0x27), t32);
        }

        FUN_00403990();

        if u_var16 == 0xffff {
            FUN_004a4d90(DAT_009a73b0 as u32, 0x23);
            return 1;
        }

        let tp = rp(g(
            DAT_009a1fe8,
            (u_var16 as u32).wrapping_mul(0x23).wrapping_add(0x1f),
        ));
        u_var16 = r16(g(tp, 0x79));
        let u_var9a: u32 = if u_var16 < 0x19 {
            0x28
        } else {
            (u_var16 as u32).wrapping_add(0x19)
        };
        FUN_004a4d90(DAT_009a73b0 as u32, u_var9a);

        let off2 = (DAT_009a73b0 as u32).wrapping_mul(0x23).wrapping_add(2);
        u_var9 =
            (0u32.wrapping_sub((*g(DAT_009a1fe8, off2) == 4) as u32) & 0x200).wrapping_add(0x200);

        'end: {
            if (u_var9 as i32) <= DAT_009a8c00 {
                break 'end;
            }

            u_var9 = ((unaff_edi >> 16) << 16) | (DAT_00a07cec as u32);
            if (DAT_00a07cec as i32) < DAT_009a1fe0 {
                loop {
                    let i_var10 = (u_var9 & 0xffff).wrapping_mul(0x23);
                    let e = g(DAT_009a1fe8, i_var10);
                    if *g(e, 3) != 0 {
                        let u_var11 = DAT_009a73c0 as u32;
                        if *g(e, 0x16) as u32 == u_var11 {
                            let obj = rp(g(e, 0x1f));
                            let puvar6 = rp(g(obj, 0xa9));
                            if *g(puvar6, 5) != 0xff && *g(puvar6, 0x5e) == 10 {
                                let t32 = r32(g(e, 0x1b));
                                if t32 as i32 == -1 {
                                    if *g(puvar6, 0x1eb) == 0 {
                                        if *g(puvar6, 0x33) == 0
                                            && (*g(puvar6, 0x72) != 0
                                                || (*g(puvar6, 0x73) == 0 && *g(puvar6, 0x74) == 0))
                                        {
                                            let i_var14 = u_var11.wrapping_mul(0x126);
                                            inc16(g(sym!(DAT_009a20ea), i_var14));
                                            let c3 = *g(puvar6, 0xb0) as i8;
                                            let c4 =
                                                *g(sym!(DAT_009d8c80), u_var11.wrapping_mul(0x566))
                                                    as i8;
                                            if c4 == c3 {
                                                *g(sym!(DAT_009a2112), i_var14) = 1;
                                            }
                                            let local_4 = (c4 == c3) as u32;
                                            FUN_00435850(
                                                0xc,
                                                concat22(i_var14 >> 16, DAT_009a73c0 as u32),
                                                local_4,
                                            );
                                        }
                                        if *g(puvar6, 0x1eb) == 0
                                            && *g(puvar6, 0x33) == 1
                                            && *g(puvar6, 0x73) == 0
                                            && *g(puvar6, 0x74) == 0
                                        {
                                            let u_var11 = DAT_009a73c0 as u32;
                                            inc16(g(
                                                sym!(DAT_009a2100),
                                                u_var11.wrapping_mul(0x126),
                                            ));
                                            if *g(sym!(DAT_009d8c80), u_var11.wrapping_mul(0x566))
                                                as i8
                                                == *g(puvar6, 0xb0) as i8
                                            {
                                                *g(
                                                    sym!(DAT_009a211d),
                                                    u_var11.wrapping_mul(0x126),
                                                ) = 1;
                                            }
                                        }
                                    }
                                    FUN_00451c20(u_var9, puvar6.cast(), 0x8d);
                                    FUN_00433d30(u_var9 as u16, DAT_009a73c0, u_stack8);
                                    *g(DAT_009a1fe8, i_var10.wrapping_add(3)) = 0;
                                    FUN_00458780(puvar6 as i32);
                                }
                            }
                        }
                    }
                    u_var16 = (u_var9 as u16).wrapping_add(1);
                    u_var9 = ((u_var9 >> 16) << 16) | (u_var16 as u32);
                    if !((u_var16 as i32) < DAT_009a1fe0) {
                        break;
                    }
                }
            }

            let mut u_var12: u32 = (DAT_009a1fe0 as u32) >> 8;
            let mut cond = false;
            if *g(DAT_00a08104, 0x1eb) == 0 {
                let p = gp();
                if *g(p, 9) == 0
                    && (*g(DAT_00a08104, 0x72) != 0
                        || (*g(DAT_00a08104, 0x73) == 0 && *g(DAT_00a08104, 0x74) == 0))
                {
                    cond = true;
                }
            }
            if cond {
                let u9 = DAT_009a73c0 as u32;
                let i_var10 = u9.wrapping_mul(0x126);
                inc16(g(sym!(DAT_009a20ea), i_var10));
                let c3 = *g(sym!(DAT_009d8c80), u9.wrapping_mul(0x566)) as i8;
                let c4 = *g(DAT_00a08104, 0xb0) as i8;
                if c3 == c4 {
                    *g(sym!(DAT_009a2112), i_var10) = 1;
                }
                let extra = FUN_00435850(
                    0xc,
                    concat22(i_var10 >> 16, DAT_009a73c0 as u32),
                    (c3 == c4) as u32,
                ) >> 8;
                u_var12 = extra;
            }

            let pc15 = g(DAT_00a08104, 0x1eb);
            let mut do_fa8 = *pc15 != 0;
            if !do_fa8 {
                let p = gp();
                let mut i_var10: u32 = (u_var12 << 8) | (*g(p, 9) as u32);
                let p = gp();
                if *g(p, 9) == 1 {
                    i_var10 = (u_var12 << 8) | (*g(DAT_00a08104, 0x73) as u32);
                    if *g(DAT_00a08104, 0x73) == 0 {
                        i_var10 = (u_var12 << 8) | (*g(DAT_00a08104, 0x74) as u32);
                        if *g(DAT_00a08104, 0x74) == 0 {
                            let u9 = DAT_009a73c0 as u32;
                            i_var10 = u9.wrapping_mul(0x126);
                            inc16(g(sym!(DAT_009a2100), i_var10));
                            if *g(sym!(DAT_009d8c80), u9.wrapping_mul(0x566)) as i8
                                == *g(DAT_00a08104, 0xb0) as i8
                            {
                                *g(sym!(DAT_009a211d), i_var10) = 1;
                            }
                        }
                    }
                }
                u_var13 = (i_var10 >> 16) as u16;
                if *pc15 != 0 {
                    do_fa8 = true;
                }
            }
            if do_fa8 {
                let u9 = DAT_009a73c0 as u32;
                let obj = rp(g(
                    DAT_009a1fe8,
                    (DAT_009a73b0 as u32).wrapping_mul(0x23).wrapping_add(0x1f),
                ));
                let b5 = *g(obj, 0x89);
                let u_var11 = b5 as u32;
                let i_var10 = u_var11.wrapping_add(u9.wrapping_mul(0x126));
                *g(sym!(DAT_009a2146), i_var10) = (*g(
                    sym!(DAT_009a2146),
                    u9.wrapping_mul(0x126).wrapping_add(u_var11),
                ))
                .wrapping_add(1);
                let c3 = *g(DAT_00a08104, 0xb0) as i8;
                let c4 = *g(sym!(DAT_009d8c80), u9.wrapping_mul(0x566)) as i8;
                if c4 == c3 {
                    inc8(g(sym!(DAT_009a2150), i_var10));
                }
                FUN_00435930(
                    0x2c,
                    concat22(i_var10 >> 16, DAT_009a73c0 as u32),
                    (c4 == c3) as u32,
                    b5 as u32,
                );
                let mut u9: u32 = 0;
                u_var13 = 0;
                loop {
                    let puvar6 = DAT_00a08104;
                    if u9 != u_var11 {
                        let u17 = DAT_009a73c0 as u32;
                        if *g(sym!(DAT_009d8c88), u17.wrapping_mul(0x566)) as u32 != u9 {
                            let i_var10 = u9.wrapping_add(u17.wrapping_mul(0x126));
                            inc8(g(sym!(DAT_009a215a), i_var10));
                            if *g(sym!(DAT_009d8c80), u17.wrapping_mul(0x566)) as i8
                                == *g(puvar6, 0xb0) as i8
                            {
                                *g(sym!(DAT_009a2164), i_var10) = 1;
                            }
                        }
                    }
                    u9 = u9.wrapping_add(1);
                    if !(u9 < 10) {
                        break;
                    }
                }
            }

            FUN_00451c20(
                concat22(u_var13 as u32, DAT_009a73b0 as u32),
                DAT_00a08104.cast(),
                0x8d,
            );
            FUN_00433d30(DAT_009a73b0, DAT_009a73c0, u_stack8);
            *g(
                DAT_009a1fe8,
                (DAT_009a73b0 as u32).wrapping_mul(0x23).wrapping_add(3),
            ) = 0;
            u_var9 = FUN_00458780(DAT_00a08104 as i32);

            u_var16 = r16(g(DAT_00a08104, 0x66));
            if u_var16 != 0xffff {
                let i_var10 = (u_var16 as u32).wrapping_mul(0x23);
                let e = g(DAT_009a1fe8, i_var10);
                u_var9 = e as u32;
                let t32 = r32(g(e, 0x1f));
                if t32 != 0 {
                    let u_var8 = *g(e, 0x16) as u32;
                    FUN_00433d30(u_var16, u_var8 as u16, u_stack8);
                    let obj = rp(g(DAT_009a1fe8, i_var10.wrapping_add(0x1f)));
                    let i_var7 = rp(g(obj, 0xa9));
                    if *g(i_var7, 0x1eb) == 0 {
                        let i_var14 = u_var8.wrapping_mul(0x126);
                        inc16(g(sym!(DAT_009a20f8), i_var14));
                        let b18 = *g(sym!(DAT_009d8c80), u_var8.wrapping_mul(0x566)) as i8
                            == *g(i_var7, 0xb0) as i8;
                        if b18 {
                            *g(sym!(DAT_009a2119), i_var14) = 1;
                        }
                        FUN_00435850(0x2e, concat22(u_stack8 as u32, u_var8), b18 as u32);
                    } else {
                        let b5 = *g(obj, 0x89);
                        u_var9 = u_var8;
                        u_stack8 = b5 as u16;
                        let i_var14 = u_var9.wrapping_mul(0x126);
                        let k = (u_stack8 as u32).wrapping_add(i_var14);
                        *g(sym!(DAT_009a2146), k) = (*g(sym!(DAT_009a2146), k)).wrapping_add(1);
                        let i_var1 = u_var9.wrapping_mul(0x566);
                        let b18 = *g(sym!(DAT_009d8c80), i_var1) as i8 == *g(i_var7, 0xb0) as i8;
                        if b18 {
                            inc8(g(sym!(DAT_009a2150), k));
                        }
                        FUN_00435930(
                            0x2c,
                            ((b5 as u32) << 16) | (u_var8 as u16 as u32),
                            b18 as u32,
                            b5 as u32,
                        );
                        let mut u11: u32 = 0;
                        loop {
                            if u11 != u_stack8 as u32
                                && *g(sym!(DAT_009d8c88), i_var1) as u32 != u11
                            {
                                inc8(g(sym!(DAT_009a215a), u11.wrapping_add(i_var14)));
                                if *g(sym!(DAT_009d8c80), i_var1) as i8 == *g(i_var7, 0xb0) as i8 {
                                    *g(sym!(DAT_009a2164), u11.wrapping_add(i_var14)) = 1;
                                }
                            }
                            u11 = u11.wrapping_add(1);
                            if !(u11 < 10) {
                                break;
                            }
                        }
                        let c3 = *g(sym!(DAT_009d8c80), i_var1) as i8;
                        inc16(g(sym!(DAT_009a20fa), u_var9.wrapping_mul(0x126)));
                        if c3 == *g(i_var7, 0xb0) as i8 {
                            *g(sym!(DAT_009a211a), u_var9.wrapping_mul(0x126)) = 1;
                        }
                    }
                    *g(DAT_009a1fe8, i_var10.wrapping_add(3)) = 0;
                    u_var9 = FUN_00458780(i_var7 as i32);
                }
            }
        }

        u_var9 & 0xffff0000
    }
}
