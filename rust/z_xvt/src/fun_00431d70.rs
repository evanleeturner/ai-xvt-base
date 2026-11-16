// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005251b0: u32;
    static mut DAT_005251b4: i32;
    static mut DAT_00525558: i32;
    static mut DAT_005ef21c: u32;
    static mut DAT_007330d4: i32;
    static mut DAT_00739290: u32;
    static mut DAT_00739298: [u32; 0];
    static mut DAT_007392a8: u8;
    static mut DAT_0085e78f: i8;

    static mut DAT_00539a08: [u32; 0];
    static mut DAT_00539a0c: [u32; 0];
    static mut DAT_00539a10: [u32; 0];
    static mut DAT_00539a14: [u8; 0];
    static mut DAT_00539a15: [i8; 0];
    static mut DAT_00539a16: [u8; 0];
    static mut DAT_00539a17: [u8; 0];
    static mut DAT_00539a18: [i32; 0];
    static mut DAT_00539a1c: [u32; 0];

    static mut DAT_0073379c: [i32; 0];
    static mut DAT_007337a4: [i32; 0];
    static mut DAT_007337ac: [i32; 0];
    static mut DAT_007339bc: [u32; 0];
    static mut DAT_007339c0: [i32; 0];
    static mut DAT_007339c8: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042ff10();
    fn FUN_00433160();
    fn FUN_00433540(param_1: u32) -> u32;
    fn FUN_00433340(param_1: u32, param_2: u32, param_3: u32, param_4: u32, param_5: u32) -> u32;
    fn FUN_00432a30(param_1: i32, param_2: *mut u32, param_3: i32);
    fn FUN_00433450(param_1: u32) -> i32;
}

unsafe extern "stdcall" {
    fn timeGetTime() -> u32;
}

macro_rules! arr {
    ($n:ident, $t:ty) => {
        (&raw mut $n).cast::<$t>()
    };
}

unsafe fn ld<T: Copy>(p: *mut T, i: u32) -> T {
    // SAFETY: the game's own table memory, read where the original reads.
    unsafe { p.wrapping_add(i as usize).read_unaligned() }
}

unsafe fn st<T>(p: *mut T, i: u32, v: T) {
    // SAFETY: the game's own table memory, written where the original writes.
    unsafe { p.wrapping_add(i as usize).write_unaligned(v) }
}

unsafe fn copy_buf(src: *const u32) {
    // SAFETY: copies 0x84 dwords from the game's slot into its copy buffer.
    unsafe {
        let dst = arr!(DAT_00739298, u32);
        let mut i = 0usize;
        while i < 0x84 {
            dst.wrapping_add(i)
                .write_unaligned(src.wrapping_add(i).read_unaligned());
            i += 1;
        }
    }
}

unsafe fn get_seq(a: u32, b: u32, idx: u32) -> i32 {
    // SAFETY: the game's own sequence tables.
    unsafe {
        let i = idx.wrapping_mul(0x8e);
        if a == 0 {
            if b == 0 {
                ld(arr!(DAT_0073379c, i32), i)
            } else {
                ld(arr!(DAT_007337ac, i32), i)
            }
        } else {
            ld(arr!(DAT_007337a4, i32), i)
        }
    }
}

unsafe fn set_seq(a: u32, b: u32, idx: u32, v: u32) {
    // SAFETY: the game's own sequence tables.
    unsafe {
        let i = idx.wrapping_mul(0x8e);
        if a == 0 {
            if b == 0 {
                st(arr!(DAT_0073379c, i32), i, v as i32);
            } else {
                st(arr!(DAT_007337ac, i32), i, v as i32);
            }
        } else {
            st(arr!(DAT_007337a4, i32), i, v as i32);
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00431d70(param_1: *mut u32, param_2: *mut u32) -> *mut u8 {
    // SAFETY: all globals, tables and callees are the game's own; param_1 and
    // param_2 point at the caller's output dwords; stack buffers are indexed
    // through raw pointers exactly as the original does.
    unsafe {
        FUN_0042ff10();
        FUN_00433160();
        if DAT_005251b4 == 0 {
            return core::ptr::null_mut();
        }
        let mut ab2c8 = [0u8; 40];
        let mut ab2f0 = [0u8; 40];
        let mut ab228 = [0x5au8; 40];
        let mut ab2a0 = [0u8; 120];
        let mut u200 = [0u32; 3];
        let p2c8 = ab2c8.as_mut_ptr();
        let p2f0 = ab2f0.as_mut_ptr();
        let p228 = ab228.as_mut_ptr();
        let p2a0 = ab2a0.as_mut_ptr();
        let pu200 = u200.as_mut_ptr();

        let a08 = arr!(DAT_00539a08, u32);
        let a0c = arr!(DAT_00539a0c, u32);
        let a10 = arr!(DAT_00539a10, u32);
        let a14 = arr!(DAT_00539a14, u8);
        let a15 = arr!(DAT_00539a15, i8);
        let a16 = arr!(DAT_00539a16, u8);
        let a17 = arr!(DAT_00539a17, u8);
        let a18 = arr!(DAT_00539a18, i32);
        let a1c = arr!(DAT_00539a1c, u32);
        let d39bc = arr!(DAT_007339bc, u32);
        let d39c0 = arr!(DAT_007339c0, i32);

        let mut u32c: u32 = DAT_005251b0;
        let mut u328: u32 = 0;
        let mut u314: u32 = 0;
        let mut i310: i32 = 0;
        let mut uvar17: u32 = DAT_00739290;
        let mut ivar12: i32 = DAT_005251b4;

        if (DAT_00739290 as i32) > 0 {
            let mut u4 = DAT_00739290;
            let mut p13 = arr!(DAT_007337a4, u8);
            let mut idx: u32 = 0;
            loop {
                st(p2a0, idx, p13.read());
                st(p2a0, idx.wrapping_add(1), p13.wrapping_sub(8).read());
                st(p2a0, idx.wrapping_add(2), p13.wrapping_add(8).read());
                u4 = u4.wrapping_sub(1);
                p13 = p13.wrapping_add(0x238);
                idx = idx.wrapping_add(3);
                if u4 == 0 {
                    break;
                }
            }
        }

        loop {
            DAT_00739290 = uvar17;
            if ivar12 < 1 {
                uvar17 = DAT_005251b0;
                ivar12 = DAT_005251b4;
                if 0x3fe < DAT_005251b4 {
                    while 0 < ivar12 {
                        let mut iv18: u32 = uvar17.wrapping_mul(0x210);
                        if ld(a08, uvar17.wrapping_mul(0x84)) == 0 {
                            uvar17 = uvar17.wrapping_add(1);
                        } else {
                            let cvar2 = ld(a15, iv18);
                            let uvar4 = ld(a16, iv18) as u32;
                            let uvar10 = FUN_00433540(ld(a08, uvar17.wrapping_mul(0x84)));
                            let c4: u32 = (cvar2 == 0) as u32;
                            let c5: u32 = (cvar2 == 2) as u32;
                            if uvar10 < DAT_00739290 && uvar10 < 0x28 {
                                let iv14 = get_seq(c4, c5, uvar10);
                                u328 = (iv14 as u32).wrapping_add(1);
                                if 0x7f < (u328 as i32) {
                                    u328 = 0;
                                }
                            }
                            let iv14 = (uvar4 as i32).wrapping_sub(u328 as i32);
                            if uvar10 < DAT_00739290 && (iv14 < -0x1b || (-1 < iv14 && iv14 < 100))
                            {
                                if ld(a14, iv18) != 0 {
                                    set_seq(c4, c5, uvar10, uvar4);
                                    DAT_005ef21c = uvar4;
                                    let k = uvar10.wrapping_mul(0x8e);
                                    st(d39c0, k, ld(d39c0, k).wrapping_add(1));
                                    copy_buf(a08.wrapping_add(uvar17.wrapping_mul(0x84) as usize));
                                    FUN_00433450(uvar17);
                                    *param_1 = ld(arr!(DAT_00739298, u32), 0);
                                    *param_2 = ld(arr!(DAT_00739298, u32), 1);
                                    return &raw mut DAT_007392a8;
                                }
                                uvar17 = uvar17.wrapping_add(1);
                            } else {
                                iv18 = FUN_00433450(uvar17) as u32;
                                if iv18 != 0 {
                                    uvar17 = uvar17.wrapping_add(1);
                                }
                            }
                        }
                        if 0x3ff < (uvar17 as i32) {
                            uvar17 = 0;
                        }
                        ivar12 = ivar12.wrapping_sub(1);
                    }
                }
                return core::ptr::null_mut();
            }

            'lab: {
                'joined: {
                    let iv18: u32 = u32c.wrapping_mul(0x210);
                    let p20: *mut u32 = a08.wrapping_add(u32c.wrapping_mul(0x84) as usize);
                    if ld(a08, u32c.wrapping_mul(0x84)) == 0 {
                        if u32c == DAT_005251b0 {
                            copy_buf(p20);
                            *param_1 = ld(a08, u32c.wrapping_mul(0x84));
                            *param_2 = ld(a0c, u32c.wrapping_mul(0x84));
                            DAT_005251b0 = DAT_005251b0.wrapping_add(1);
                            DAT_005251b4 = DAT_005251b4.wrapping_sub(1);
                            if 0x3ff < (DAT_005251b0 as i32) {
                                DAT_005251b0 = 0;
                            }
                            return &raw mut DAT_007392a8;
                        }
                        break 'joined;
                    } else {
                        let mut bvar11: u8 = ld(a16, iv18);
                        let uvar10: u32 = bvar11 as u32;
                        let uvar4: u32 = (ld(a15, iv18) == 0) as u32;
                        let uvar5: u32 = (ld(a15, iv18) == 2) as u32;
                        let uvar6 = FUN_00433540(ld(a08, u32c.wrapping_mul(0x84)));
                        if DAT_00739290 != uvar17 && uvar6 < 0x28 {
                            let k = uvar6.wrapping_mul(3);
                            st(p2a0, k, 0x7f);
                            st(p2a0, k.wrapping_add(1), 0x7f);
                            st(p2a0, k.wrapping_add(2), 0x7f);
                        }
                        if uvar6 < DAT_00739290 {
                            if ld(p228, uvar6) < ld(p2f0, uvar6) {
                                break 'joined;
                            }
                            if ld(a17, iv18) == 0 {
                                st(p2f0, uvar6, ld(p2f0, uvar6).wrapping_add(1));
                            }
                            let bvar1: u8;
                            u328 = (get_seq(uvar4, uvar5, uvar6) as u32).wrapping_add(1);
                            if 0x7f < (u328 as i32) {
                                u328 = 0;
                            }
                            let k = uvar6.wrapping_mul(3);
                            if uvar4 == 0 {
                                if uvar5 == 0 {
                                    bvar1 = ld(p2a0, k.wrapping_add(1));
                                } else {
                                    bvar1 = ld(p2a0, k.wrapping_add(2));
                                }
                            } else {
                                bvar1 = ld(p2a0, k);
                            }
                            u314 = (bvar1 as u32).wrapping_add(1);
                            if 0x7f < u314 {
                                u314 = 0;
                            }
                        }
                        let iv14: i32 = ld(a18, u32c.wrapping_mul(0x84));
                        let mut iv7: i32 = (uvar10 as i32).wrapping_sub(u328 as i32);
                        if uvar6 < DAT_00739290 && (iv7 < -0x1c || (-1 < iv7 && iv7 < 100)) {
                            let k8e = uvar6.wrapping_mul(0x8e);
                            if u328 == uvar10 && u314 == u328 {
                                st(d39bc, k8e, timeGetTime());
                                set_seq(uvar4, uvar5, uvar6, uvar10);
                                DAT_005ef21c = uvar10;
                                if iv14 != 1 || DAT_0085e78f != 1 {
                                    st(d39c0, k8e, ld(d39c0, k8e).wrapping_add(1));
                                }
                                copy_buf(p20);
                                FUN_00433450(u32c);
                                *param_1 = ld(arr!(DAT_00739298, u32), 0);
                                *param_2 = ld(arr!(DAT_00739298, u32), 1);
                                return &raw mut DAT_007392a8;
                            }
                            if iv14 == 1 && DAT_0085e78f == 1 {
                                st(d39bc, k8e, timeGetTime());
                                set_seq(uvar4, uvar5, uvar6, uvar10);
                                DAT_005ef21c = uvar10;
                                copy_buf(p20);
                                FUN_00433450(u32c);
                                *param_1 = ld(arr!(DAT_00739298, u32), 0);
                                *param_2 = ld(arr!(DAT_00739298, u32), 1);
                                return &raw mut DAT_007392a8;
                            }
                            if ld(a17, iv18) == 0 {
                                iv7 = 0;
                                if uvar4 == 0 {
                                    iv7 = 2 - (uvar5 == 0) as i32;
                                }
                                iv7 = (uvar6.wrapping_mul(3) as i32).wrapping_add(iv7);
                                uvar17 = (ld(p2a0, iv7 as u32) as u32).wrapping_add(1);
                                st(p2a0, iv7 as u32, bvar11);
                                if 0x7f < uvar17 {
                                    uvar17 = 0;
                                }
                                i310 = (uvar10 as i32).wrapping_sub(uvar17 as i32);
                                if i310 < 0 {
                                    i310 = i310.wrapping_add(0x80);
                                }
                                let mut bvar3: u8 = 0;
                                iv7 = i310.wrapping_mul(DAT_00525558);
                                if ld(p2f0, uvar6) < 0x7f {
                                    st(p2f0, uvar6, ld(p2f0, uvar6).wrapping_add(i310 as u8));
                                }
                                let mut uvar16 = uvar17;
                                if ld(p2c8, uvar6) < 0x1a {
                                    while uvar16 != uvar10 {
                                        let uvar8 = FUN_00433340(u32c, uvar16, uvar4, uvar5, uvar6);
                                        if (uvar8 as i32) < 0x400 && -1 < (uvar8 as i32) {
                                            if uvar16 == u328 {
                                                st(d39bc, k8e, timeGetTime());
                                                copy_buf(a08.wrapping_add(
                                                    uvar8.wrapping_mul(0x84) as usize,
                                                ));
                                                FUN_00433450(uvar8);
                                                set_seq(uvar4, uvar5, uvar6, u328);
                                                DAT_005ef21c = u328;
                                                st(d39c0, k8e, ld(d39c0, k8e).wrapping_add(1));
                                                *param_1 = ld(arr!(DAT_00739298, u32), 0);
                                                *param_2 = ld(arr!(DAT_00739298, u32), 1);
                                                if i310 < 2 {
                                                    st(a14, iv18, 0);
                                                    st(a10, u32c.wrapping_mul(0x84), 0);
                                                }
                                                return &raw mut DAT_007392a8;
                                            }
                                            i310 = i310.wrapping_sub(1);
                                        } else {
                                            let cvar2 = ld(a14, iv18);
                                            st(p2c8, uvar6, ld(p2c8, uvar6).wrapping_add(1));
                                            if cvar2 == 0 {
                                                *pu200.add(2) = 0;
                                                if uvar4 == 0 {
                                                    *pu200.add(2) = 2 - (uvar5 == 0) as u32;
                                                }
                                                let iv23: i32;
                                                if iv14 == 2 {
                                                    *pu200 = 0x33;
                                                    iv23 = p20.read_unaligned() as i32;
                                                    *pu200.add(1) =
                                                        (ld(a1c, u32c.wrapping_mul(0x84))
                                                            & 0x7fffffff)
                                                            .wrapping_sub(iv7 as u32);
                                                    *pu200.add(2) = uvar16;
                                                } else {
                                                    iv23 = p20.read_unaligned() as i32;
                                                    *pu200 = 0x38;
                                                    *pu200.add(1) = uvar16;
                                                }
                                                FUN_00432a30(iv23, pu200, 0xc);
                                                let q =
                                                    arr!(DAT_007339c8, u8)
                                                        .wrapping_add(
                                                            uvar6.wrapping_mul(0x238) as usize
                                                        )
                                                        .cast::<u32>();
                                                q.write_unaligned(
                                                    q.read_unaligned().wrapping_add(1),
                                                );
                                                bvar3 = 1;
                                            } else {
                                                'l309: {
                                                    let dvar9 = timeGetTime();
                                                    let uvar8b: u32;
                                                    if DAT_007330d4 == 0 {
                                                        bvar11 = ((if iv14 == 2 {
                                                            2u32
                                                        } else {
                                                            0
                                                        }) + 3)
                                                            as u8;
                                                        uvar8b = 1000u32.wrapping_shl(
                                                            (ld(a14, iv18) & 0x1f) as u32,
                                                        );
                                                    } else {
                                                        bvar11 = 0;
                                                        uvar8b =
                                                            (if iv14 == 2 { 37000u32 } else { 0 })
                                                                + 3000;
                                                    }
                                                    if dvar9.wrapping_sub(ld(
                                                        a10,
                                                        u32c.wrapping_mul(0x84),
                                                    )) <= uvar8b
                                                    {
                                                        break 'l309;
                                                    }
                                                    if bvar11 < ld(a14, iv18) {
                                                        st(d39bc, k8e, timeGetTime());
                                                        let mut uv16;
                                                        loop {
                                                            uv16 = FUN_00433340(
                                                                u32c, uvar17, uvar4, uvar5, uvar6,
                                                            );
                                                            if !((uv16 as i32) < 0
                                                                || 0x400 < (uv16 as i32))
                                                            {
                                                                break;
                                                            }
                                                            if uvar17 == uvar10 {
                                                                set_seq(
                                                                    uvar4, uvar5, uvar6, uvar10,
                                                                );
                                                                DAT_005ef21c = uvar10;
                                                                st(
                                                                    d39c0,
                                                                    k8e,
                                                                    ld(d39c0, k8e).wrapping_add(1),
                                                                );
                                                                copy_buf(p20);
                                                                FUN_00433450(u32c);
                                                                *param_1 =
                                                                    ld(arr!(DAT_00739298, u32), 0);
                                                                *param_2 =
                                                                    ld(arr!(DAT_00739298, u32), 1);
                                                                return &raw mut DAT_007392a8;
                                                            }
                                                            uvar17 = uvar17.wrapping_add(1);
                                                            if 0x7f < (uvar17 as i32) {
                                                                uvar17 = 0;
                                                            }
                                                        }
                                                        copy_buf(a08.wrapping_add(
                                                            uv16.wrapping_mul(0x84) as usize,
                                                        ));
                                                        FUN_00433450(uv16);
                                                        set_seq(uvar4, uvar5, uvar6, uvar17);
                                                        DAT_005ef21c = uvar17;
                                                        st(
                                                            d39c0,
                                                            k8e,
                                                            ld(d39c0, k8e).wrapping_add(1),
                                                        );
                                                        *param_1 = ld(arr!(DAT_00739298, u32), 0);
                                                        *param_2 = ld(arr!(DAT_00739298, u32), 1);
                                                        return &raw mut DAT_007392a8;
                                                    }
                                                    *pu200.add(2) = 0;
                                                    if uvar4 == 0 {
                                                        *pu200.add(2) = 2 - (uvar5 == 0) as u32;
                                                    }
                                                    let iv23: i32;
                                                    if iv14 == 2 {
                                                        *pu200 = 0x33;
                                                        iv23 = p20.read_unaligned() as i32;
                                                        *pu200.add(1) =
                                                            (ld(a1c, u32c.wrapping_mul(0x84))
                                                                & 0x7fffffff)
                                                                .wrapping_sub(iv7 as u32);
                                                        *pu200.add(2) = uvar16;
                                                    } else {
                                                        iv23 = p20.read_unaligned() as i32;
                                                        *pu200 = 0x38;
                                                        *pu200.add(1) = uvar16;
                                                    }
                                                    FUN_00432a30(iv23, pu200, 0xc);
                                                    bvar3 = 1;
                                                }
                                            }
                                        }
                                        iv7 = iv7.wrapping_sub(DAT_00525558);
                                        uvar16 = uvar16.wrapping_add(1);
                                        if 0x7f < (uvar16 as i32) {
                                            uvar16 = 0;
                                        }
                                    }
                                    if i310 < 1 {
                                        st(a14, iv18, 0);
                                        st(a10, u32c.wrapping_mul(0x84), 0);
                                    } else if bvar3 != 0 {
                                        st(a14, iv18, ld(a14, iv18).wrapping_add(1));
                                        st(a10, u32c.wrapping_mul(0x84), timeGetTime());
                                    }
                                }
                            } else if u32c == DAT_005251b0 && FUN_00433450(u32c) == 0 {
                                break 'lab;
                            }
                            break 'joined;
                        }
                        let r = FUN_00433450(u32c);
                        if r != 0 {
                            break 'joined;
                        }
                    }
                    break 'lab;
                }
                u32c = u32c.wrapping_add(1);
                if 0x3ff < (u32c as i32) {
                    u32c = 0;
                }
            }
            uvar17 = DAT_00739290;
            ivar12 = ivar12.wrapping_sub(1);
        }
    }
}
