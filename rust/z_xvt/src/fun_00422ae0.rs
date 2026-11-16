// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0070ad42: u8;
    static mut DAT_0070ad43: u8;
    static mut DAT_0070ad44: u8;
    static mut DAT_0070ad4d: u8;
    static mut DAT_0070ad4e: u8;
    static mut DAT_005247e8: u8;
    static mut DAT_0052687b: u8;
    static mut DAT_0052687a: u8;
    static mut DAT_0052686b: u8;
    static mut DAT_006d4a66: u8;
    static mut DAT_0070b156: u8;
    static mut DAT_0070b182: u8;
    static mut DAT_0070b1ae: u8;
    static mut DAT_0052b157: u8;
    static mut DAT_0052b159: u8;
    static mut DAT_0052b15b: u8;
    static mut DAT_0052b15d: u8;
    static mut DAT_0052a7b0: u8;
    static mut DAT_0052a948: u8;
    static mut DAT_0052aae0: u8;
    static mut DAT_006d4988: i32;
    static mut DAT_00732a2c: i32;
    static mut DAT_00732a28: i32;
    static mut DAT_00732a24: i32;
    static mut DAT_0070ab18: i32;
    static mut DAT_0070ab14: i32;
    static mut DAT_0070ac20: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00449580(a: u32, b: u32, c: u32) -> i64;
    fn FUN_0042a230(a: u32, b: i32, c: i32, d: u16, e: i16, f: i16);
}

unsafe fn rd<T>(base: *mut u8, off: i32) -> T {
    // SAFETY: the caller passes the address of one of the game's own tables
    // and an offset the original code uses.
    unsafe {
        base.wrapping_byte_offset(off as isize)
            .cast::<T>()
            .read_unaligned()
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00422ae0(param_1: u32, param_2: u16, param_3: u16) {
    // SAFETY: all globals and callees are the game's own; table reads happen
    // at the offsets the original computes.
    unsafe {
        let uvar10: u32 = param_1 & 0xffff;
        let ivar7: i32 = uvar10.wrapping_mul(0x566) as i32;
        let idx = rd::<u8>(&raw mut DAT_0070ad42, ivar7) as i32;
        let uvar3: u32 = rd::<u8>(&raw mut DAT_005247e8, idx) as u32;
        let k: i32 = uvar3.wrapping_mul(0x16) as i32;
        let mut bvar4: u8 = rd::<u8>(&raw mut DAT_0052687b, k);
        let lvar12: i64 = FUN_00449580(uvar3.wrapping_mul(5), uvar3.wrapping_mul(0xb), uvar3);
        let w = rd::<u16>(&raw mut DAT_006d4a66, uvar10.wrapping_mul(0x126) as i32);
        let ivar8: i32 = uvar10
            .wrapping_mul(0x2b3)
            .wrapping_add((w as i32).wrapping_sub(0x8000) as u32)
            .wrapping_mul(2) as i32;
        DAT_00732a2c = ((rd::<i16>(&raw mut DAT_0070b156, ivar8) as i32 as u32) << 8) as i32;
        DAT_00732a28 =
            (rd::<i16>(&raw mut DAT_0070b182, ivar8) as i32 as u32).wrapping_mul(0xffffff00) as i32;
        DAT_00732a24 = ((rd::<i16>(&raw mut DAT_0070b1ae, ivar8) as i32 as u32) << 8) as i32;

        if (rd::<u8>(&raw mut DAT_0052687a, k) & 0x80) == 0 {
            let iv8: i32 = (bvar4 as u32).wrapping_mul(0xef) as i32;
            let s1: i16 = rd::<i16>(&raw mut DAT_0052b15d, iv8);
            let s159: i16 = rd::<i16>(&raw mut DAT_0052b159, iv8);
            let s15b: i16 = rd::<i16>(&raw mut DAT_0052b15b, iv8);
            let s6: i16 = (rd::<u8>(&raw mut DAT_0070ad4e, ivar7) as i32).wrapping_add(1) as i16;
            let iv7: i32 = (param_2 as u32)
                .wrapping_add((rd::<u8>(&raw mut DAT_0070ad4d, ivar7) as u32).wrapping_mul(6))
                as i32;
            let o2: i32 = iv7.wrapping_mul(2);
            let mut u2: u16 = (s159 as i32 as u32)
                .wrapping_mul(rd::<i16>(&raw mut DAT_0052a7b0, o2) as i32 as u32)
                .wrapping_mul(s6 as i32 as u32) as u16;
            let mut s11: i16 = (s1 as i32 as u32)
                .wrapping_mul(rd::<i16>(&raw mut DAT_0052a948, o2) as i32 as u32)
                .wrapping_mul(s6 as i32 as u32) as i16;
            let mut s9: i16 = (s15b as i32 as u32)
                .wrapping_mul(rd::<i16>(&raw mut DAT_0052aae0, o2) as i32 as u32)
                .wrapping_mul(s6 as i32 as u32) as i16;
            if s6 == 1 {
                u2 = (u2 as u32).wrapping_add((((s159 as i32) / 2) as u32).wrapping_mul(rd::<i16>(
                    &raw mut DAT_0052a7b0,
                    o2,
                )
                    as i32
                    as u32)) as u16;
                s9 = (s9 as i32 as u32).wrapping_add(
                    (((s15b as i32) / 2) as u32)
                        .wrapping_mul(rd::<i16>(&raw mut DAT_0052aae0, o2) as i32 as u32),
                ) as i16;
                let q: i32 =
                    (s1 as i32).wrapping_add((((s1 as i32) >> 0x1f) as u32 & 3) as i32) >> 2;
                s11 = (s11 as i32 as u32).wrapping_add(
                    ((q as i16) as i32 as u32)
                        .wrapping_mul(rd::<i16>(&raw mut DAT_0052a948, o2) as i32 as u32),
                ) as i16;
            }
            if param_3 == 0xffff {
                DAT_0070ab18 = 0;
                DAT_0070ab14 = 0;
                DAT_0070ac20 = 0;
            } else {
                let t: i32 = (param_3 as u32)
                    .wrapping_mul(0x23)
                    .wrapping_add(DAT_006d4988 as u32) as i32;
                FUN_0042a230(u2 as u32, t, t, u2, s9, s11);
            }
            let s157: i16 = rd::<i16>(&raw mut DAT_0052b157, iv8);
            if s157 != 0 {
                bvar4 = s157 as u8;
                let sh = (bvar4 & 0x1f) as u32;
                DAT_0070ab18 = (DAT_0070ab18 as u32).wrapping_shl(sh) as i32;
                DAT_0070ab14 = (DAT_0070ab14 as u32).wrapping_shl(sh) as i32;
                DAT_0070ac20 = (DAT_0070ac20 as u32).wrapping_shl(sh) as i32;
            }
            DAT_00732a2c = (DAT_00732a2c as u32).wrapping_add(DAT_0070ab18 as u32) as i32;
            DAT_00732a28 = (DAT_00732a28 as u32).wrapping_add(DAT_0070ab14 as u32) as i32;
            DAT_00732a24 = (DAT_00732a24 as u32).wrapping_add(DAT_0070ac20 as u32) as i32;
        } else if rd::<u8>(&raw mut DAT_0052686b, k) == 9 {
            DAT_00732a2c = ((rd::<i16>(&raw mut DAT_0070b156, ivar7) as i32 as u32) << 8) as i32;
            DAT_00732a28 = (rd::<i16>(&raw mut DAT_0070b182, ivar7) as i32 as u32)
                .wrapping_mul(0xffffff00) as i32;
            DAT_00732a24 = ((rd::<i16>(&raw mut DAT_0070b1ae, ivar7) as i32 as u32) << 8) as i32;
        } else if rd::<u8>(&raw mut DAT_0052686b, k) == 8 {
            let mut s1: i16 = 0;
            let mut s6: i16 = 0;
            let mut local_10: i16 = 0;
            let mut local_16: i16 = 0;
            let b4: u8 = rd::<u8>(&raw mut DAT_0070ad44, ivar7) & 3;
            if b4 == 0 {
                s6 = 0x40;
                s1 = 0x40;
            } else if b4 == 1 {
                s1 = 0x40;
                local_16 = 0x40;
            } else if b4 == 2 {
                s6 = 0x40;
                local_10 = 0x40;
            }
            let u5: u16 = rd::<u8>(&raw mut DAT_0070ad43, ivar7) as u16;
            let iv8: i32 = (u5.wrapping_sub(1) as i16) as i32;
            let mut u2: u16 = 0;
            let mut local_12: i16 = 0;
            if rd::<u8>(&raw mut DAT_0070ad43, ivar7) != 0 {
                loop {
                    let mut s9: i16 = 0;
                    if rd::<u8>(&raw mut DAT_0070ad43, ivar7) != 0 {
                        loop {
                            if param_2 == u2 {
                                let h6: u16 = ((s6 as i32).wrapping_mul(iv8) / 2) as i16 as u16;
                                let a: u16 = (rd::<i16>(&raw mut DAT_0070b156, ivar7) as u16)
                                    .wrapping_sub(h6);
                                DAT_00732a2c = (((s6 as i32).wrapping_mul(s9 as i32) as u32)
                                    .wrapping_add(a as i16 as i32 as u32))
                                .wrapping_mul(0x100)
                                    as i32;
                                let h1: u16 = ((s1 as i32).wrapping_mul(iv8) / 2) as i16 as u16;
                                let t: i16 = (rd::<i16>(&raw mut DAT_0070b182, ivar7) as u16)
                                    .wrapping_add(h1)
                                    as i16;
                                let neg: i16 = (t as i32).wrapping_neg() as i16;
                                DAT_00732a28 = (((s1 as i32).wrapping_mul(local_12 as i32) as u32)
                                    .wrapping_add(neg as i32 as u32))
                                .wrapping_mul(0x100)
                                    as i32;
                                let h10: u16 =
                                    ((local_10 as i32).wrapping_mul(iv8) / 2) as i16 as u16;
                                let h16: u16 =
                                    ((local_16 as i32).wrapping_mul(iv8) / 2) as i16 as u16;
                                let c: i16 = (rd::<i16>(&raw mut DAT_0070b1ae, ivar7) as u16)
                                    .wrapping_sub(h10)
                                    .wrapping_sub(h16)
                                    as i16;
                                DAT_00732a24 = (((local_16 as i32).wrapping_mul(s9 as i32) as u32)
                                    .wrapping_add(
                                        (local_10 as i32).wrapping_mul(local_12 as i32) as u32
                                    )
                                    .wrapping_add(c as i32 as u32))
                                .wrapping_mul(0x100)
                                .wrapping_add(lvar12 as i16 as i32 as u32)
                                    as i32;
                                return;
                            }
                            u2 = u2.wrapping_add(1);
                            s9 = s9.wrapping_add(1);
                            if !((s9 as i32) < (u5 as i32)) {
                                break;
                            }
                        }
                    }
                    local_12 = local_12.wrapping_add(1);
                    if !((local_12 as i32) < (u5 as i32)) {
                        break;
                    }
                }
            }
        }
        DAT_00732a24 = (DAT_00732a24 as u32).wrapping_add(lvar12 as i16 as i32 as u32) as i32;
    }
}
