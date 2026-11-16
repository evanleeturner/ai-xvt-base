// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009d6940: u8;
    static mut DAT_009d6941: u8;
    static mut DAT_009ec5e2: u16;
    static mut DAT_009e9680: [i8; 0];
    static mut DAT_009ec468: [i8; 0];
    static mut DAT_009d6960: u32;
    static mut DAT_009d6964: u32;
    static mut DAT_009ed22b: u8;
    static mut DAT_009ed22c: u8;
    static mut DAT_009ed22d: u8;
    static mut DAT_009a2000: i16;
    static mut DAT_009a2002: i16;
    static mut DAT_009a20c0: [u8; 0];
    static mut DAT_009d9192: [u8; 0];
    static mut DAT_009a2196: [u8; 0];
    static mut DAT_009d8e25: [u8; 0];
    static mut DAT_009d8e26: [u8; 0];
    static mut DAT_009d8e27: [u8; 0];
    static mut DAT_009d8e28: [u8; 0];
    static mut DAT_009d8e29: [u8; 0];
    static mut DAT_009d8e33: [u8; 0];
    static mut DAT_009d6968: [i32; 0];
    static mut DAT_009a8080: [u8; 0];
    static mut DAT_009a8081: [u8; 0];
    static mut DAT_009a8082: [u8; 0];
    static mut DAT_009a8083: [u8; 0];
    static mut DAT_009a8084: [u8; 0];
    static mut DAT_009a8085: [u8; 0];
    static mut DAT_009a8086: [u8; 0];
    static mut DAT_009a8087: [u8; 0];
    static mut DAT_009a808a: [u8; 0];
    static mut DAT_009a808b: [u8; 0];
    static mut DAT_009a808c: [u8; 0];
    static mut DAT_009a808d: [u8; 0];
    static mut DAT_009a808e: [u8; 0];
    static mut DAT_009a808f: [u8; 0];
    static mut DAT_009a8090: [u8; 0];
    static mut DAT_009a8091: [u8; 0];
    static mut DAT_009a8092: [u8; 0];
    static mut DAT_009a8095: [u8; 0];
    static mut DAT_009a80a7: [u8; 0];
    static mut DAT_009a80a9: [u8; 0];
    static mut DAT_00556354: u16;
    static mut DAT_00556358: u16;
    static mut DAT_009d73a8: [u16; 0];
    static mut DAT_009d7498: [u16; 0];
    static mut DAT_009d73aa: [u16; 0];
    static mut DAT_009d749a: [u16; 0];
    static mut DAT_009d73ac: [u16; 0];
    static mut DAT_009d749c: [u16; 0];
    static mut DAT_009d73ae: [u16; 0];
    static mut DAT_009d749e: [u16; 0];
    static mut DAT_009d736c: [i8; 0];
    static mut DAT_009d738a: [i8; 0];
    static mut DAT_009d738b: [i8; 0];
    static mut DAT_009a2062: i8;
    static mut DAT_009a2063: i8;
    static mut DAT_009d7368: i8;
    static mut DAT_009d736b: i8;
    static mut DAT_009fd458: [u8; 0];
    static mut DAT_009fd462: [u8; 0];
    static mut DAT_009fd4e2: [u8; 0];
    static mut DAT_009fd562: [u8; 0];
    static mut DAT_009d7588: [u32; 0];
    static mut DAT_00a08292: u16;
    static mut DAT_00a07be0: u16;
    static mut DAT_00a07be2: u16;
    static mut DAT_00a080f8: i32;
    static mut DAT_009ecc34: i32;
    static mut DAT_009d7684: u16;
    static mut DAT_00b6a5aa: u8;
    static mut DAT_00b6a606: u8;
    static mut DAT_009ec5ea: i16;
    static mut DAT_009d75c4: [i8; 0];
    static mut DAT_009d7604: [i8; 0];
    static mut DAT_009fe84a: [u8; 0];
    static mut DAT_009fe855: [u8; 0];
    static mut DAT_009fe871: [u8; 0];
    static mut DAT_009fe870: [u8; 0];
    static mut DAT_009fe840: [u8; 0];
    static mut DAT_009fe800: [u8; 0];
    static mut DAT_009e967c: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00435a10(a: u8, b: u8, c: u8) -> u32;
    fn FUN_00431c10(p1: u16, p2: i16, p3: u16, p4: u16, p5: i16, p6: u16) -> i8;
    fn FUN_00450650(a: i32, b: i32);
    fn FUN_00451bf0(a: i32, b: *mut core::ffi::c_void);
    fn FUN_00430420(a: i32);
    fn FUN_0046f720(a: u32) -> *mut core::ffi::c_void;
    fn FUN_00431b50(a: *mut u8, b: i32) -> u32;
}

fn p<T>(b: *mut [T; 0], i: u32) -> *mut T {
    b.cast::<T>().wrapping_offset(i as i32 as isize)
}

macro_rules! at {
    ($n:ident, $i:expr) => {
        p(&raw mut $n, ($i) as u32)
    };
}

fn sel(x: u8) -> u8 {
    if x & 2 == 0 { 4 } else { 2 }
}

#[unsafe(no_mangle)]
#[allow(
    non_snake_case,
    unused_assignments,
    unused_mut,
    unused_variables,
    unused_unsafe
)]
pub unsafe extern "stdcall" fn FUN_00430a00() {
    // SAFETY: all globals and callees are the game's own memory and code; the
    // indices and pointer walks reproduce the original's accesses exactly.
    unsafe {
        let base = (&raw mut DAT_009e9680).cast::<i8>();
        let end = (&raw mut DAT_009ec468).cast::<i8>();
        if DAT_009d6941 != 0 {
            return;
        }
        if DAT_009ec5e2 == 0 || DAT_009d6940 != 0 {
            DAT_009d6960 = 0;
            let mut pc = base;
            loop {
                if *pc != 0 {
                    DAT_009d6960 = DAT_009d6960.wrapping_add(1);
                }
                pc = pc.wrapping_add(0x5bd);
                if !(pc < end) {
                    break;
                }
            }
            if DAT_009d6964 < DAT_009d6960 {
                DAT_009d6964 = DAT_009d6960;
            }
            let u_var12 = FUN_00435a10(DAT_009ed22b, DAT_009ed22c, DAT_009ed22d);
            let mut u_var10: u16 = 0;
            if 0 < DAT_009a2000 {
                loop {
                    let mut u_stack_c: u32 = 0;
                    let i_var1 = (u_var10 as u32)
                        .wrapping_add((u_var10 as u32).wrapping_mul(0x92))
                        .wrapping_mul(2);
                    loop {
                        let mut i_cond = false;
                        if *at!(DAT_009a20c0, i_var1.wrapping_add(3)) == 0 {
                            let u_idx = (u_var10 as u32)
                                .wrapping_mul(0x500)
                                .wrapping_add((u_var10 as u32).wrapping_mul(0x66));
                            let t = at!(DAT_009d9192, u_idx).cast::<i32>().read_unaligned();
                            if t == -1 {
                                i_cond = true;
                            }
                        }
                        if i_cond {
                            let mut u_var19: u16 = 0;
                            loop {
                                let u_var9 = u_var19.wrapping_add(1);
                                *at!(
                                    DAT_009a2196,
                                    u_stack_c
                                        .wrapping_add(i_var1)
                                        .wrapping_add((u_var19 as u32).wrapping_mul(8))
                                ) = 0;
                                u_var19 = u_var9;
                                if !(u_var9 < 10) {
                                    break;
                                }
                            }
                        } else {
                            let mut u_var15: u32 = 0;
                            let i_var14 = u_stack_c
                                .wrapping_mul(0x4e)
                                .wrapping_add((u_var10 as u32).wrapping_mul(0x500))
                                .wrapping_add((u_var10 as u32).wrapping_mul(0x66));
                            loop {
                                let i_var16 = i_var1.wrapping_add(u_var15.wrapping_mul(8));
                                let tgt = at!(DAT_009a2196, u_stack_c.wrapping_add(i_var16));
                                if *at!(DAT_009d8e29, i_var14.wrapping_add(u_var15)) == 0 {
                                    *tgt = 0;
                                } else if *tgt == 4 {
                                    let b5 = *at!(DAT_009d8e26, i_var14);
                                    if b5 == 0 || b5 == 10 {
                                        *tgt = 0;
                                    } else {
                                        let c8 = FUN_00431c10(
                                            b5 as u16,
                                            1,
                                            u_var10,
                                            *at!(DAT_009d8e27, i_var14) as u16,
                                            0,
                                            u_var15 as u16,
                                        );
                                        let mut u_var13 = (c8 as u8) as u32;
                                        if (u_var13 as i16) == 1 {
                                            u_var13 = 8;
                                            let d33 = *at!(DAT_009d8e33, i_var14);
                                            if d33 != 0 {
                                                let c =
                                                    ((d33 as u32).wrapping_mul(5) < u_var12) as u16;
                                                u_var13 = (c.wrapping_neg() & 0xfffa)
                                                    .wrapping_add(8)
                                                    as u32;
                                            }
                                        }
                                        if (u_var13 as i16) == 8
                                            && (*at!(DAT_009d8e25, i_var14) != 2
                                                || (*at!(DAT_009d8e27, i_var14) != 0x12
                                                    && *at!(DAT_009d8e27, i_var14) != 0x13))
                                        {
                                            u_var13 = 1;
                                            let q = at!(DAT_009d6968, u_var15);
                                            let add = ((*at!(DAT_009d8e28, i_var14) as i8 as i32)
                                                .wrapping_mul(0xfa))
                                                as u32;
                                            *q = (*q as u32).wrapping_add(add) as i32;
                                        }
                                        *tgt = u_var13 as u8;
                                    }
                                }
                                let u_var19 = (u_var15 as u16).wrapping_add(1);
                                u_var15 = u_var19 as u32;
                                if !(u_var19 < 10) {
                                    break;
                                }
                            }
                        }
                        let u_var19 = (u_stack_c as u16).wrapping_add(1);
                        u_stack_c = u_var19 as u32;
                        if !(u_var19 < 8) {
                            break;
                        }
                    }
                    u_var10 = u_var10.wrapping_add(1);
                    if !((u_var10 as i32) < (DAT_009a2000 as i32)) {
                        break;
                    }
                }
            }
            let mut u_stack_22: u16 = 0;
            let mut b_stack_24: u8 = 0;
            let mut s_stack_1c: i16 = 0;
            loop {
                let mut u_stack_20: u16 = 0;
                let u_var15 = u_stack_22 as u32;
                let i_var1 = u_var15.wrapping_mul(3);
                loop {
                    let mut b_stack_1e: u8 = (u_stack_20 != 1) as u8;
                    let mut u_var13 = u_stack_20 as u32;
                    let mut i_var14 = u_var15.wrapping_mul(7).wrapping_add(u_var13);
                    let mut b5 = *at!(DAT_009a8080, i_var14.wrapping_mul(0x2a));
                    let mut b7 = *at!(DAT_009a808b, i_var14.wrapping_mul(0x2a));
                    let b2 = *at!(DAT_009a808f, i_var14.wrapping_mul(0x2a));
                    i_var14 = i_var14.wrapping_mul(0x2a);
                    let mut b23 = *at!(DAT_009a8084, i_var14);
                    let mut b25: u8 = 0;
                    let mut i_var16 = i_var1.wrapping_add(u_var13);
                    let mut u_var10: u16 = 0;
                    loop {
                        let u_var19 = u_var10.wrapping_add(1);
                        let i_var17 = (u_var10 as u32).wrapping_add(i_var16.wrapping_mul(4));
                        *at!(DAT_009d73a8, i_var17) = 0;
                        *at!(DAT_009d7498, i_var17) = 0;
                        u_var10 = u_var19;
                        if !(u_var19 < 4) {
                            break;
                        }
                    }
                    let c8: i8;
                    if (b5 == 10 || b5 == 0)
                        && (b23 == 10 || b23 == 0)
                        && (b7 == 10 || b7 == 0)
                        && (b2 == 10 || b2 == 0)
                    {
                        c8 = 0;
                    } else {
                        u_var10 = 10;
                        b25 = 1;
                        if b23 == 0x27 && *at!(DAT_009a8085, i_var14) == 0x0c {
                            u_var10 = *at!(DAT_009a8086, i_var14) as u16;
                        }
                        b5 = FUN_00431c10(
                            b5 as u16,
                            *at!(DAT_009a8081, i_var14) as i16,
                            *at!(DAT_009a8082, i_var14) as u16,
                            *at!(DAT_009a8083, i_var14) as u16,
                            0,
                            u_var10,
                        ) as u8;
                        let us = DAT_00556354;
                        *at!(DAT_009d73a8, i_var16.wrapping_mul(4)) = DAT_00556358;
                        *at!(DAT_009d7498, i_var16.wrapping_mul(4)) = us;
                        let mut ed7 = false;
                        if b23 == 0x27 {
                            if (b5 & 1) == 0 {
                                ed7 = true;
                            } else {
                                b23 = 1;
                            }
                        } else {
                            let b6 = FUN_00431c10(
                                b23 as u16,
                                *at!(DAT_009a8085, i_var14) as i16,
                                *at!(DAT_009a8086, i_var14) as u16,
                                *at!(DAT_009a8087, i_var14) as u16,
                                0,
                                u_var10,
                            ) as u8;
                            let us = DAT_00556354;
                            *at!(DAT_009d73aa, i_var16.wrapping_mul(4)) = DAT_00556358;
                            *at!(DAT_009d749a, i_var16.wrapping_mul(4)) = us;
                            if *at!(DAT_009a808a, i_var14) == 1 {
                                if ((b6 | b5) & 1) == 0 {
                                    b5 = b6 & b5;
                                    ed7 = true;
                                } else {
                                    b23 = 1;
                                }
                            } else {
                                b23 = 1;
                                if (b6 & b5 & 1) == 0 {
                                    b5 = b6 | b5;
                                    ed7 = true;
                                }
                            }
                        }
                        if ed7 {
                            b23 = sel(b5);
                        }
                        u_var10 = 10;
                        if b2 == 0x27 && *at!(DAT_009a8090, i_var14) == 0x0c {
                            u_var10 = *at!(DAT_009a8091, i_var14) as u16;
                        }
                        b5 = FUN_00431c10(
                            b7 as u16,
                            *at!(DAT_009a808c, i_var14) as i16,
                            *at!(DAT_009a808d, i_var14) as u16,
                            *at!(DAT_009a808e, i_var14) as u16,
                            0,
                            u_var10,
                        ) as u8;
                        let us = DAT_00556354;
                        *at!(DAT_009d73ac, i_var16.wrapping_mul(4)) = DAT_00556358;
                        *at!(DAT_009d749c, i_var16.wrapping_mul(4)) = us;
                        if b2 == 0x27 {
                            if (b5 & 1) == 0 {
                                b23 = sel(b5);
                            } else {
                                b23 = 1;
                            }
                        } else {
                            b7 = FUN_00431c10(
                                b2 as u16,
                                *at!(DAT_009a8090, i_var14) as i16,
                                *at!(DAT_009a8091, i_var14) as u16,
                                *at!(DAT_009a8092, i_var14) as u16,
                                0,
                                u_var10,
                            ) as u8;
                            let us = DAT_00556354;
                            *at!(DAT_009d73ae, i_var16.wrapping_mul(4)) = DAT_00556358;
                            *at!(DAT_009d749e, i_var16.wrapping_mul(4)) = us;
                            if *at!(DAT_009a8095, i_var14) == 1 {
                                if ((b7 | b5) & 1) == 0 {
                                    b7 = b7 & b5;
                                    b_stack_24 = sel(b7);
                                } else {
                                    b_stack_24 = 1;
                                }
                            } else if (b7 & b5 & 1) == 0 {
                                b7 = b7 | b5;
                                b_stack_24 = sel(b7);
                            } else {
                                b_stack_24 = 1;
                            }
                        }
                        if *at!(DAT_009a80a7, i_var14) == 1 {
                            if ((b_stack_24 | b23) & 1) == 0 {
                                c8 = sel(b_stack_24 & b23) as i8;
                            } else {
                                c8 = 1;
                            }
                        } else if (b_stack_24 & b23 & 1) == 0 {
                            c8 = sel(b_stack_24 | b23) as i8;
                        } else {
                            c8 = 1;
                        }
                    }
                    let mut joined = false;
                    if u_stack_20 == 1 {
                        if c8 == 1 {
                            b_stack_1e = 1;
                            joined = true;
                        }
                    } else if c8 == 1 {
                        joined = true;
                    } else if c8 == 2 {
                        b_stack_1e = 1;
                    } else if c8 == 4 {
                        b_stack_1e = 0;
                    }
                    if joined {
                        let c3 = *at!(DAT_009d736c, i_var16);
                        b25 = 1;
                        if c3 != 1 {
                            b25 = 1;
                            let q = at!(DAT_009d6968, u_var15);
                            let add = ((*at!(DAT_009a80a9, i_var14) as i8 as i32)
                                .wrapping_mul(0xfa)) as u32;
                            *q = (*q as u32).wrapping_add(add) as i32;
                        }
                    }
                    *at!(DAT_009d736c, i_var16) = c8;
                    let mut u_var10: u16 = 0;
                    if 0 < DAT_009a2000 {
                        loop {
                            let mut u_var20: u32 = 0;
                            let u_var18 = u_var10 as u32;
                            loop {
                                let iv14 = u_var20
                                    .wrapping_mul(0x4e)
                                    .wrapping_add(u_var18.wrapping_mul(0x566));
                                i_var14 = iv14;
                                if *at!(DAT_009d8e29, iv14.wrapping_add(u_var15)) != 0
                                    && (*at!(DAT_009d8e25, iv14) as u32) == u_var13
                                {
                                    let idx = u_var20
                                        .wrapping_add(u_var15.wrapping_mul(8))
                                        .wrapping_add(u_var18.wrapping_mul(0x126));
                                    if u_stack_20 == 1 {
                                        if *at!(DAT_009a2196, idx) == 1 {
                                            b_stack_1e = 1;
                                            b25 = 1;
                                        }
                                    } else {
                                        let cv = *at!(DAT_009a2196, idx) as i8;
                                        if cv == 1 {
                                            b25 = 1;
                                        } else if cv == 2 {
                                            b_stack_1e = 1;
                                        } else if cv == 4 && b_stack_1e != 1 {
                                            b_stack_1e = 0;
                                        }
                                    }
                                }
                                let u_var19 = (u_var20 as u16).wrapping_add(1);
                                u_var20 = u_var19 as u32;
                                if !(u_var19 < 8) {
                                    break;
                                }
                            }
                            u_var10 = u_var10.wrapping_add(1);
                            if !((u_var10 as i32) < (DAT_009a2000 as i32)) {
                                break;
                            }
                        }
                    }
                    if b_stack_1e == 1 && b25 == 0 {
                        b_stack_1e = 0;
                    }
                    if DAT_009a2062 == 2
                        && u_stack_20 == 1
                        && b_stack_1e == 1
                        && *at!(DAT_009d738a, i_var1) == 1
                    {
                        b_stack_1e = 0;
                    }
                    if u_stack_20 == 0 && DAT_009a2062 == 2 && DAT_009a2063 != 0 {
                        b_stack_1e = 0;
                    }
                    if (((*at!(DAT_009d738a, i_var16) != 0) as u8) != b_stack_1e)
                        && b_stack_1e != 0
                        && *at!(DAT_009d738a, i_var16) != 1
                        && DAT_009a2063 == 0
                    {
                        *at!(DAT_009d738a, i_var16) = b_stack_1e as i8;
                        if b_stack_1e == 1 {
                            let mut iv16: i32 = 10;
                            let mut iv14: i32 = 0;
                            let mut pc24 = at!(DAT_009d738a, u_var13);
                            let mut pc11 = pc24;
                            loop {
                                if *pc11 == 1 {
                                    iv14 = iv14.wrapping_add(1);
                                }
                                s_stack_1c = iv14 as i16;
                                pc11 = pc11.wrapping_add(3);
                                iv16 = iv16.wrapping_sub(1);
                                if iv16 == 0 {
                                    break;
                                }
                            }
                            if u_stack_20 == 0 {
                                iv16 = 0;
                                if DAT_009a2062 == 4 {
                                    u_var13 = 0;
                                    let mut iv17: i32 = 0;
                                    loop {
                                        b25 = 0;
                                        if u_var15 != u_var13 {
                                            b25 = (*at!(
                                                DAT_009fd458,
                                                u_var15.wrapping_add(iv17 as u32)
                                            ) == 0)
                                                as u8;
                                        }
                                        if b25 != 0 && *pc24 == 1 {
                                            iv16 = iv16.wrapping_add(1);
                                        }
                                        pc24 = pc24.wrapping_add(3);
                                        iv17 = iv17.wrapping_add(0x1e5);
                                        u_var13 = u_var13.wrapping_add(1);
                                        if !(iv17 < 0x12f2) {
                                            break;
                                        }
                                    }
                                }
                                if iv16 == 0 {
                                    let q = at!(DAT_009d6968, u_var15);
                                    *q = (*q as u32).wrapping_add(10000) as i32;
                                }
                                *at!(DAT_009d7588, u_var15) = u_var12;
                                DAT_009d7368 = 1;
                            } else if u_stack_20 == 2 {
                                DAT_009d736b = 1;
                                let q = at!(DAT_009d6968, u_var15);
                                *q = (*q as u32).wrapping_add(0x9c4) as i32;
                            }
                            if u_stack_20 == 0 {
                                let mut pu21: u32 = 0;
                                let mut pc11 = base;
                                let mut pu_stack_10: u32 = 0xffff_ffff;
                                loop {
                                    if *pc11 != 0 {
                                        let us =
                                            pc11.wrapping_sub(4).cast::<u16>().read_unaligned();
                                        if us == u_stack_22 {
                                            pu_stack_10 = pu21;
                                        }
                                    }
                                    pc11 = pc11.wrapping_add(0x5bd);
                                    pu21 = pu21.wrapping_add(1);
                                    if !(pc11 < end) {
                                        break;
                                    }
                                }
                                iv16 = 0;
                                pc11 = base;
                                loop {
                                    let c8b = DAT_009a2062;
                                    if *pc11 != 0 {
                                        let us =
                                            pc11.wrapping_sub(4).cast::<u16>().read_unaligned();
                                        if us == u_stack_22 {
                                            pc11.wrapping_add(0x71)
                                                .cast::<i32>()
                                                .write_unaligned(iv14);
                                            DAT_00a08292 =
                                                pc11.wrapping_sub(6).cast::<u16>().read_unaligned();
                                            let mut ok = true;
                                            if c8b == 4 && *at!(DAT_009d738b, i_var1) == 1 {
                                                ok = false;
                                            }
                                            if DAT_00b6a5aa == 0
                                                && DAT_00b6a606 == 1
                                                && *at!(DAT_009d738b, i_var1) == 1
                                            {
                                                ok = false;
                                            }
                                            if ok {
                                                FUN_00450650(0xc1, iv16);
                                                if DAT_009a2062 == 2 && 1 < DAT_00a080f8 {
                                                    DAT_00a07be0 = ((s_stack_1c as i32)
                                                        .wrapping_add(0x126))
                                                        as u16;
                                                    FUN_00450650(0x131, iv16);
                                                }
                                                if iv16 == DAT_009ecc34 {
                                                    let mut u_stack_18: u32 = 0;
                                                    let iv17 = u_var15.wrapping_mul(0x1e5);
                                                    loop {
                                                        let iv22 = u_stack_18.wrapping_mul(0x40);
                                                        let ix = iv17.wrapping_add(iv22);
                                                        if *at!(DAT_009fd462, ix) != 0 {
                                                            FUN_00451bf0(
                                                                0,
                                                                at!(DAT_009fd462, ix).cast(),
                                                            );
                                                            DAT_009d7684 = 0x6f;
                                                            if (u_stack_18 as i16) != 0 {
                                                                DAT_009d7684 = 0;
                                                            }
                                                            let bb = *at!(DAT_009fd462, ix);
                                                            let a = if bb < 0x31 || 0x36 < bb {
                                                                0xc4
                                                            } else {
                                                                0xcf
                                                            };
                                                            FUN_00450650(a, iv16);
                                                        }
                                                        let uv =
                                                            (u_stack_18 as u16).wrapping_add(1);
                                                        u_stack_18 = uv as u32;
                                                        if !(uv < 2) {
                                                            break;
                                                        }
                                                    }
                                                    if DAT_009a2062 == 4 {
                                                        FUN_00430420(1);
                                                    } else {
                                                        FUN_00430420(0);
                                                    }
                                                }
                                            } else {
                                                let a = if u_stack_22 == 0 { 0xd0 } else { 0xd1 };
                                                FUN_00450650(a, iv16);
                                            }
                                        } else {
                                            DAT_00a07be2 =
                                                ((s_stack_1c as i32).wrapping_add(0x126)) as u16;
                                            if (DAT_009a2062 == 2 || DAT_009a2062 == 1)
                                                && pu_stack_10 != 0xffff_ffff
                                            {
                                                FUN_00451bf0(0, FUN_0046f720(pu_stack_10));
                                                FUN_00450650(0x132, iv16);
                                            }
                                        }
                                    }
                                    pc11 = pc11.wrapping_add(0x5bd);
                                    iv16 = iv16.wrapping_add(1);
                                    if !(pc11 < end) {
                                        break;
                                    }
                                }
                            } else if u_stack_20 == 1 {
                                iv14 = 0;
                                pc11 = base;
                                loop {
                                    let mut i_cond = false;
                                    if *pc11 != 0 {
                                        let us =
                                            pc11.wrapping_sub(4).cast::<u16>().read_unaligned();
                                        if us == u_stack_22
                                            && *at!(DAT_009d738a, i_var1) != 2
                                            && iv14 == DAT_009ecc34
                                        {
                                            i_cond = true;
                                        }
                                    }
                                    if i_cond {
                                        let mut ok = true;
                                        if DAT_009a2062 == 4 && *at!(DAT_009d738a, i_var1) == 1 {
                                            ok = false;
                                        }
                                        if ok {
                                            DAT_00a08292 =
                                                pc11.wrapping_sub(6).cast::<u16>().read_unaligned();
                                            FUN_00450650(0xce, iv14);
                                            let mut u_stack_18: u32 = 0;
                                            let iv16b = u_var15.wrapping_mul(0x1e5);
                                            loop {
                                                let iv17 = u_stack_18.wrapping_mul(0x40);
                                                let ix = iv16b.wrapping_add(iv17);
                                                if *at!(DAT_009fd4e2, ix) != 0 {
                                                    FUN_00451bf0(0, at!(DAT_009fd4e2, ix).cast());
                                                    DAT_009d7684 = 0x71;
                                                    if (u_stack_18 as i16) != 0 {
                                                        DAT_009d7684 = 0;
                                                    }
                                                    let bb = *at!(DAT_009fd4e2, ix);
                                                    let a = if bb < 0x31 || 0x36 < bb {
                                                        0xc4
                                                    } else {
                                                        0xcf
                                                    };
                                                    FUN_00450650(a, iv14);
                                                }
                                                let uv = (u_stack_18 as u16).wrapping_add(1);
                                                u_stack_18 = uv as u32;
                                                if !(uv < 2) {
                                                    break;
                                                }
                                            }
                                            if DAT_009a2062 == 4 {
                                                FUN_00430420(7);
                                            } else {
                                                FUN_00430420(6);
                                            }
                                        } else {
                                            let a = if u_stack_22 == 0 { 0xd2 } else { 0xd3 };
                                            FUN_00450650(a, iv14);
                                        }
                                    }
                                    pc11 = pc11.wrapping_add(0x5bd);
                                    iv14 = iv14.wrapping_add(1);
                                    if !(pc11 < end) {
                                        break;
                                    }
                                }
                            } else {
                                iv14 = 0;
                                pc11 = base;
                                loop {
                                    let mut i_cond = false;
                                    if *pc11 != 0 {
                                        let us =
                                            pc11.wrapping_sub(4).cast::<u16>().read_unaligned();
                                        if us == u_stack_22 && iv14 == DAT_009ecc34 {
                                            i_cond = true;
                                        }
                                    }
                                    if i_cond {
                                        let mut u_stack_18: u32 = 0;
                                        let iv16b = u_var15.wrapping_mul(0x1e5);
                                        loop {
                                            let iv17 = u_stack_18.wrapping_mul(0x40);
                                            let ix = iv16b.wrapping_add(iv17);
                                            if *at!(DAT_009fd562, ix) != 0 {
                                                FUN_00451bf0(0, at!(DAT_009fd562, ix).cast());
                                                DAT_009d7684 = 0x70;
                                                if (u_stack_18 as i16) != 0 {
                                                    DAT_009d7684 = 0;
                                                }
                                                DAT_00a08292 = pc11
                                                    .wrapping_sub(6)
                                                    .cast::<u16>()
                                                    .read_unaligned();
                                                let bb = *at!(DAT_009fd562, ix);
                                                let a = if bb < 0x31 || 0x36 < bb {
                                                    0xc4
                                                } else {
                                                    0xcf
                                                };
                                                FUN_00450650(a, iv14);
                                            }
                                            let uv = (u_stack_18 as u16).wrapping_add(1);
                                            u_stack_18 = uv as u32;
                                            if !(uv < 2) {
                                                break;
                                            }
                                        }
                                    }
                                    pc11 = pc11.wrapping_add(0x5bd);
                                    iv14 = iv14.wrapping_add(1);
                                    if !(pc11 < end) {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    u_stack_20 = u_stack_20.wrapping_add(1);
                    if !(u_stack_20 < 3) {
                        break;
                    }
                }
                u_stack_22 = u_stack_22.wrapping_add(1);
                if !(u_stack_22 < 10) {
                    break;
                }
            }
            DAT_009ec5e2 = 0xec;
        }
        if DAT_009ec5ea < 1 {
            let mut u_var10: u16 = 0;
            if 0 < DAT_009a2002 {
                loop {
                    let u_var12 = u_var10 as u32;
                    if *at!(DAT_009d75c4, u_var12) == 0 {
                        let i_var1 = u_var12.wrapping_mul(0x72);
                        let u_var15 = FUN_00431b50(at!(DAT_009fe84a, i_var1), 0);
                        let u_var13 = FUN_00431b50(at!(DAT_009fe855, i_var1), 0);
                        let u_var19: u16 = if *at!(DAT_009fe871, i_var1) == 1 {
                            (u_var15 as u16) | (u_var13 as u16)
                        } else {
                            (u_var15 as u16) & (u_var13 as u16)
                        };
                        if (u_var19 & 1) != 0 {
                            *at!(DAT_009d75c4, u_var12) = 1;
                            let c8 = *at!(DAT_009fe870, i_var1) as i8;
                            *at!(DAT_009d7604, u_var12) = c8;
                            if c8 == 0 {
                                let us2 = at!(DAT_009e967c, DAT_009ecc34.wrapping_mul(0x5bd))
                                    .cast::<u16>()
                                    .read_unaligned();
                                if *at!(DAT_009fe840, i_var1.wrapping_add(us2 as u32)) != 0 {
                                    FUN_00451bf0(0, at!(DAT_009fe800, i_var1).cast());
                                    if 0xf < u_var10 {
                                        DAT_009d7684 = 0;
                                    } else {
                                        DAT_009d7684 = u_var10.wrapping_add(0x5f);
                                    }
                                    FUN_00450650(0xcf, DAT_009ecc34);
                                }
                            }
                        }
                    } else if *at!(DAT_009d7604, u_var12) != 0 {
                        let c8 = (*at!(DAT_009d7604, u_var12)).wrapping_sub(1);
                        *at!(DAT_009d7604, u_var12) = c8;
                        if c8 == 0 {
                            let us2 = at!(DAT_009e967c, DAT_009ecc34.wrapping_mul(0x5bd))
                                .cast::<u16>()
                                .read_unaligned();
                            let ix = u_var12.wrapping_mul(0x72).wrapping_add(us2 as u32);
                            if *at!(DAT_009fe840, ix) != 0 {
                                FUN_00451bf0(
                                    0,
                                    at!(DAT_009fe800, u_var12.wrapping_mul(0x72)).cast(),
                                );
                                if u_var10 < 0x10 {
                                    DAT_009d7684 = u_var10.wrapping_add(0x5f);
                                } else {
                                    DAT_009d7684 = 0;
                                }
                                FUN_00450650(0xcf, DAT_009ecc34);
                            }
                        }
                    }
                    u_var10 = u_var10.wrapping_add(1);
                    if !((u_var10 as i32) < (DAT_009a2002 as i32)) {
                        break;
                    }
                }
            }
            DAT_009ec5ea = 0x49c;
        }
    }
}
