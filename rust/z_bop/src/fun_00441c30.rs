// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a08ba0: i16;
    static mut DAT_005233d8: u32;
    static mut DAT_005233c8: u8;
    static mut DAT_009a7bac: i16;
    static mut DAT_00a08a00: u16;
    static mut DAT_00a08604: u16;
    static mut DAT_009e9683: u8;
    static mut DAT_009ecc34: i32;
    static mut DAT_00a08374: u16;
    static mut DAT_009e9670: u8;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a806c: u32;
    static mut DAT_009a20ae: u8;
    static mut DAT_009ed232: u8;
    static mut DAT_0051c5ae: u8;
    static mut DAT_0051c5af: u8;

    static mut DAT_00a092ea: u8;
    static mut DAT_00a092e8: u8;
    static mut DAT_00a092d2: u8;
    static mut DAT_00a092d0: u8;
    static mut DAT_00a092de: u8;
    static mut DAT_00a092dc: u8;
    static mut DAT_00a092ba: u8;
    static mut DAT_00a092b8: u8;
    static mut DAT_00a092c6: u8;
    static mut DAT_00a092c4: u8;
    static mut DAT_00a08de4: u8;
    static mut DAT_00a08de6: u8;
    static mut DAT_00a08de8: u8;

    static mut DAT_00a0836a: u16;
    static mut DAT_00a0836e: u16;
    static mut DAT_00a08366: u16;
    static mut DAT_00a0836c: u16;
    static mut DAT_00a08c92: u16;
    static mut DAT_00a08c84: u16;
    static mut DAT_00a08c8c: u16;
    static mut DAT_00a08c80: u16;
    static mut DAT_00a08c90: u16;
    static mut DAT_00a08c82: u16;
    static mut DAT_00a08c8e: u16;
    static mut DAT_00a08c8a: u16;
    static mut DAT_00a08608: u16;
    static mut DAT_00a08606: u16;
    static mut DAT_00a08376: u16;
    static mut DAT_00a083bc: u16;
    static mut DAT_00a08378: u16;
    static mut DAT_00a083b8: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00427150(
        a1: u32,
        a2: u32,
        a3: u32,
        a4: u32,
        a5: u32,
        a6: u32,
        a7: u32,
        a8: u32,
        a9: u32,
    );
    fn FUN_004a9640(a1: i8);
}

fn concat22(h: u16, l: u32) -> u32 {
    ((h as u32) << 16) | (l as u16 as u32)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00441c30() {
    // SAFETY: all globals and callees are the game's own memory and
    // functions; pointer reads happen at the offsets the original uses.
    unsafe {
        let mut u_var12: u16 = 0;
        loop {
            let s_var3: i16 = (&raw mut DAT_00a08ba0).add(u_var12 as usize).read();
            let s_var5: i16 = DAT_005233d8 as i16;
            let u_var9: u16 = (DAT_005233d8 >> 0x10) as u16;
            let mut u_var13: u16 = DAT_00a08a00;
            let mut u_var14: u16 = DAT_00a08604;
            let idx = (DAT_009ecc34 as u32).wrapping_mul(0x5bd) as usize;
            let mut kind = 0;
            match u_var12 {
                0 => {
                    if s_var3 != 0 {
                        if (&raw mut DAT_009e9683).add(idx).read() != 0 {
                            kind = 5;
                        } else {
                            kind = 1;
                        }
                    }
                }
                1 => {
                    if s_var3 != 0 {
                        if (&raw mut DAT_009e9683).add(idx).read() != 0 {
                            kind = 5;
                        } else {
                            kind = 2;
                        }
                    }
                }
                3 => {
                    if s_var3 != 0 && (&raw mut DAT_009e9683).add(idx).read() == 0 {
                        kind = 3;
                    }
                }
                4 => {
                    if s_var3 != 0 {
                        if (&raw mut DAT_009e9683).add(idx).read() != 0 {
                            kind = 5;
                        } else {
                            kind = 4;
                        }
                    }
                }
                5 => {
                    if s_var3 != 0 {
                        kind = 4;
                    }
                }
                6 => {
                    if s_var3 != 0 && (&raw mut DAT_009e9683).add(idx).read() != 0 {
                        kind = 5;
                    }
                }
                _ => {}
            }
            if kind != 0 {
                let u_var6 = concat22(
                    u_var9,
                    (s_var5 as i32 as u32).wrapping_mul(DAT_009a7bac as i32 as u32),
                );
                let i_var4: i32 = (DAT_00a08374 as u32).wrapping_mul(0xc) as i32;
                let (b1, b2, g11, g7): (*mut u8, *mut u8, u16, u16) = match kind {
                    1 => {
                        u_var13 = DAT_00a08366;
                        u_var14 = DAT_00a0836c;
                        (
                            &raw mut DAT_00a092e8,
                            &raw mut DAT_00a092ea,
                            DAT_00a0836a,
                            DAT_00a0836e,
                        )
                    }
                    2 => {
                        u_var13 = DAT_00a08c8c;
                        u_var14 = DAT_00a08c80;
                        (
                            &raw mut DAT_00a092d0,
                            &raw mut DAT_00a092d2,
                            DAT_00a08c92,
                            DAT_00a08c84,
                        )
                    }
                    3 => {
                        u_var13 = DAT_00a08c8e;
                        u_var14 = DAT_00a08c8a;
                        (
                            &raw mut DAT_00a092dc,
                            &raw mut DAT_00a092de,
                            DAT_00a08c90,
                            DAT_00a08c82,
                        )
                    }
                    4 => (
                        &raw mut DAT_00a092b8,
                        &raw mut DAT_00a092ba,
                        DAT_00a08608,
                        DAT_00a08606,
                    ),
                    _ => {
                        u_var13 = DAT_00a08378;
                        u_var14 = DAT_00a083b8;
                        (
                            &raw mut DAT_00a092c4,
                            &raw mut DAT_00a092c6,
                            DAT_00a08376,
                            DAT_00a083bc,
                        )
                    }
                };
                let u_var8 = concat22(
                    u_var9,
                    b2.byte_add(i_var4 as usize).cast::<u16>().read_unaligned() as u32,
                );
                let u_var10 = concat22(
                    ((i_var4 as u32) >> 0x10) as u16,
                    b1.byte_add(i_var4 as usize).cast::<u16>().read_unaligned() as u32,
                );
                let u_var11 = concat22(
                    (((DAT_00a08374 as u32).wrapping_mul(3)) >> 0x10) as u16,
                    g11 as u32,
                );
                let u_var7 = concat22(u_var9, g7 as u32);
                FUN_00427150(
                    DAT_009a806c,
                    DAT_005233c8 as u32,
                    u_var7,
                    u_var11,
                    u_var10,
                    u_var8,
                    u_var13 as u32,
                    u_var14 as u32,
                    u_var6,
                );
            }
            u_var12 = u_var12.wrapping_add(1);
            if 7 < u_var12 {
                break;
            }
        }
        if DAT_00a08374 == 0x90 {
            let idx = (DAT_009ecc34 as u32).wrapping_mul(0x5bd) as usize;
            let i_tmp1: i32 = (&raw mut DAT_009e9670)
                .byte_add(idx)
                .cast::<i32>()
                .read_unaligned();
            let a2 = (i_tmp1 as u32)
                .wrapping_mul(0x23)
                .wrapping_add(0x1f)
                .wrapping_add(DAT_009a1fe8 as u32);
            let i_tmp2: i32 = (a2 as usize as *const i32).read_unaligned();
            let a3 = (i_tmp2 as u32).wrapping_add(0xa9);
            let i_var4: i32 = (a3 as usize as *const i32).read_unaligned();
            let base = i_var4 as u32;
            if ((base.wrapping_add(0x1e1) as usize as *const u8).read() & 8) != 0 {
                let b_var1 = (base.wrapping_add(4) as usize as *const u8).read();
                FUN_004a9640(0);
                let mut u_var12: u16 = 0;
                let i = (b_var1 as u32).wrapping_mul(0x101) as usize;
                let b1 = (&raw mut DAT_0051c5af).add(i).read();
                let b2 = (&raw mut DAT_0051c5ae).add(i).read();
                let sum = (b1 as u16).wrapping_add(b2 as u16);
                if sum != 0 {
                    loop {
                        let i_var4: u32 = (u_var12 as u32)
                            .wrapping_add(DAT_00a08374 as u32)
                            .wrapping_mul(0xc);
                        let w = (&raw mut DAT_00a08de8)
                            .byte_add(i_var4 as usize)
                            .cast::<i16>()
                            .read_unaligned();
                        if w != 0 {
                            let u_var6 = concat22(
                                (DAT_005233d8 >> 0x10) as u16,
                                (DAT_005233d8 as i16 as i32 as u32)
                                    .wrapping_mul(DAT_009a7bac as i32 as u32),
                            );
                            let u_var8 = ((((u_var6 >> 8) & 0xffffff) << 8) | DAT_009a20ae as u32)
                                & 0xffff00ff;
                            let u_var9 = (u_var8 >> 0x10) as u16;
                            let k = (DAT_009ed232 as u32).wrapping_add(1);
                            FUN_00427150(
                                DAT_009a806c,
                                DAT_005233c8 as u32,
                                concat22(u_var9, (u_var12 as u32).wrapping_mul(k).wrapping_add(2)),
                                2,
                                concat22(
                                    (i_var4 >> 0x10) as u16,
                                    (&raw mut DAT_00a08de4)
                                        .byte_add(i_var4 as usize)
                                        .cast::<u16>()
                                        .read_unaligned()
                                        as u32,
                                ),
                                concat22(
                                    u_var9,
                                    (&raw mut DAT_00a08de6)
                                        .byte_add(i_var4 as usize)
                                        .cast::<u16>()
                                        .read_unaligned()
                                        as u32,
                                ),
                                (w as i32 as u32).wrapping_mul(k),
                                u_var8 as u16 as u32,
                                u_var6,
                            );
                        }
                        u_var12 = u_var12.wrapping_add(1);
                        if u_var12 >= sum {
                            break;
                        }
                    }
                }
            }
        }
    }
}
