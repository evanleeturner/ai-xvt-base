// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1834: u16;
    static mut DAT_009a20ca: u16;
    static mut DAT_009a20cc: u8;
    static mut DAT_009d8c82: u8;
    static mut DAT_009d8c83: u8;
    static mut DAT_009d8c84: u8;
    static mut DAT_009d8c97: u8;
    static mut DAT_009d8c98: u8;
    static mut DAT_009d8c99: u8;
    static mut DAT_009d9096: u8;
    static mut DAT_009d90c2: u8;
    static mut DAT_009d90ee: u8;
    static mut DAT_00524170: u8;
    static mut DAT_00521f2e: u8;
    static mut DAT_00521f1b: i8;
    static mut DAT_0099f970: i32;
    static mut DAT_0099f974: i32;
    static mut DAT_0099f978: i32;
    static mut DAT_0099f97c: u16;
    static mut DAT_0099f97e: u16;
    static mut DAT_0099f980: u16;
    static mut DAT_009d113c: u16;
    static mut DAT_009a73f0: u16;
    static mut DAT_00a07ce8: i32;
    static mut DAT_009d1308: i32;
    static mut DAT_009a1fe8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00458c80(param_1: u16, param_2: i32, param_3: u8);
    fn FUN_00425cb0() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004587c0(param_1: u16) {
    // SAFETY: all globals and callees are the game's own memory and code;
    // the byte tables are indexed with the original's strides and offsets.
    unsafe {
        let u_var6 = DAT_009d113c;
        let a = || DAT_009a1834 as u32;
        let p20ca = (&raw mut DAT_009a20ca) as *const u8;
        let p20cc = (&raw mut DAT_009a20cc) as *const u8;
        let p8c82 = (&raw mut DAT_009d8c82) as *const u8;
        let p8c83 = (&raw mut DAT_009d8c83) as *const u8;
        let p8c84 = (&raw mut DAT_009d8c84) as *const u8;
        let p8c97 = (&raw mut DAT_009d8c97) as *const u8;
        let p8c98 = (&raw mut DAT_009d8c98) as *const u8;
        let p8c99 = (&raw mut DAT_009d8c99) as *const u8;
        let p9096 = (&raw mut DAT_009d9096) as *const u8;
        let p90c2 = (&raw mut DAT_009d90c2) as *const u8;
        let p90ee = (&raw mut DAT_009d90ee) as *const u8;
        let p4170 = (&raw mut DAT_00524170) as *const u8;
        let p1f2e = (&raw mut DAT_00521f2e) as *const u8;
        let p1f1b = (&raw mut DAT_00521f1b) as *const i8;

        let ca = |x: u32| -> u16 {
            p20ca
                .byte_add((x.wrapping_mul(0x93) as usize).wrapping_mul(2))
                .cast::<u16>()
                .read_unaligned()
        };

        if ca(a()) != 0 {
            let off = a().wrapping_mul(0x566) as usize;
            let b_var1 = p4170.add(p8c82.add(off).read() as usize).read();
            if (p1f2e.add((b_var1 as usize) * 0x1a).read() & 0x80) != 0 {
                let c_var2 = p1f1b.add((b_var1 as usize) * 0x1a).read();
                if c_var2 == 8 {
                    let b_var10 = p8c84.add(off).read() & 3;
                    let mut local_18: u32 = 0;
                    let mut local_1c: u32 = 0;
                    let mut local_20: u32 = 0;
                    let mut local_14: u32 = 0;
                    if (p8c84.add(off).read() & 3) == 0 {
                        local_18 = 0x40;
                        local_1c = 0x40;
                    } else if b_var10 == 1 {
                        local_1c = 0x40;
                        local_14 = 0x40;
                    } else if b_var10 == 2 {
                        local_18 = 0x40;
                        local_20 = 0x40;
                    }
                    let i_var11 = (p8c83.add(off).read() as i32).wrapping_sub(1);
                    let s_var3 = p9096.add(off).cast::<i16>().read_unaligned();
                    let s_var4 = p90c2.add(off).cast::<i16>().read_unaligned();
                    let s_var5 = p90ee.add(off).cast::<i16>().read_unaligned();
                    DAT_0099f97c = p8c97.add(off).read() as u16;
                    let mut u_var7: u32 = 0;
                    DAT_0099f97e = p8c98.add(off).read() as u16;
                    let mut local_24: u32 = 0;
                    DAT_0099f980 = p8c99.add(off).read() as u16;
                    if p8c83.add(off).read() != 0 {
                        loop {
                            let mut i_var12: i32 = 0;
                            let cnt = p8c83.add(a().wrapping_mul(0x566) as usize).read();
                            if cnt != 0 {
                                let iv11 = i_var11 as u32;
                                let mut i_var9: i32 = ((s_var5 as i32 as u32)
                                    .wrapping_sub(local_20.wrapping_mul(iv11) >> 1)
                                    .wrapping_sub(local_14.wrapping_mul(iv11) >> 1)
                                    .wrapping_add(local_20.wrapping_mul(local_24)))
                                    as i32;
                                let mut i_var13: i32 = ((s_var3 as i32 as u32)
                                    .wrapping_sub(local_18.wrapping_mul(iv11) >> 1))
                                    as i32;
                                loop {
                                    if (param_1 == 0xffff || param_1 as u32 == u_var7)
                                        && p20cc
                                            .byte_add(a().wrapping_mul(0x126) as usize)
                                            .cast::<u16>()
                                            .read_unaligned()
                                            < ca(a())
                                    {
                                        DAT_0099f974 = local_1c.wrapping_mul(local_24).wrapping_sub(
                                            (local_1c.wrapping_mul(iv11) >> 1)
                                                .wrapping_add(s_var4 as i32 as u32),
                                        )
                                            as i32;
                                        DAT_0099f970 = i_var13;
                                        DAT_0099f978 = i_var9;
                                        FUN_00458c80(DAT_009a1834, 8, b_var1);
                                    }
                                    u_var7 = u_var7.wrapping_add(1);
                                    i_var9 = (i_var9 as u32).wrapping_add(local_14) as i32;
                                    i_var13 = (i_var13 as u32).wrapping_add(local_18) as i32;
                                    i_var12 = i_var12.wrapping_add(1);
                                    if !(i_var12
                                        < p8c83.add(a().wrapping_mul(0x566) as usize).read() as i32)
                                    {
                                        break;
                                    }
                                }
                            }
                            local_24 = local_24.wrapping_add(1);
                            if !(local_24
                                < p8c83.add(a().wrapping_mul(0x566) as usize).read() as u32)
                            {
                                break;
                            }
                        }
                        return;
                    }
                } else {
                    if c_var2 == 9 {
                        DAT_0099f970 = p9096.add(off).cast::<i16>().read_unaligned() as i32;
                        DAT_0099f978 = p90ee.add(off).cast::<i16>().read_unaligned() as i32;
                        DAT_0099f974 = 0u32.wrapping_sub(
                            p90c2.add(off).cast::<i16>().read_unaligned() as i32 as u32,
                        ) as i32;
                        DAT_0099f97c = p8c97.add(off).read() as u16;
                        DAT_0099f97e = p8c98.add(off).read() as u16;
                        DAT_0099f980 = p8c99.add(off).read() as u16;
                        FUN_00458c80(DAT_009a1834, 9, b_var1);
                        return;
                    }
                    if c_var2 != 10 {
                        return;
                    }
                    let s_var3 = p9096.add(off).cast::<i16>().read_unaligned();
                    let mut local_24: u32 = 0;
                    let s_var4 = p90c2.add(off).cast::<i16>().read_unaligned();
                    let s_var5 = p90ee.add(off).cast::<i16>().read_unaligned();
                    DAT_009d113c = DAT_009a73f0;
                    while local_24 < p8c83.add(a().wrapping_mul(0x566) as usize).read() as u32 {
                        let mut i_var12: i32;
                        let mut i_var11: i32;
                        let mut i_var13: i32;
                        let mut i_var9: i32;
                        loop {
                            let mut u_var7 = FUN_00425cb0();
                            i_var12 = ((u_var7 & 0x1ff)
                                .wrapping_sub(0x100)
                                .wrapping_add(s_var3 as i32 as u32))
                                as i32;
                            u_var7 = FUN_00425cb0();
                            i_var11 = ((u_var7 & 0x1ff)
                                .wrapping_sub(0x100)
                                .wrapping_sub(s_var4 as i32 as u32))
                                as i32;
                            u_var7 = FUN_00425cb0();
                            i_var13 = ((u_var7 & 0x1ff)
                                .wrapping_sub(0x100)
                                .wrapping_add(s_var5 as i32 as u32))
                                as i32;
                            i_var9 = DAT_00a07ce8;
                            if DAT_00a07ce8 < DAT_009d1308.wrapping_add(DAT_00a07ce8) {
                                let mut pc_var8 = (DAT_00a07ce8 as u32)
                                    .wrapping_mul(0x23)
                                    .wrapping_add(3)
                                    .wrapping_add(DAT_009a1fe8 as u32)
                                    as *const i8;
                                loop {
                                    let t1 = pc_var8.byte_add(1).cast::<i32>().read_unaligned();
                                    let t2 = pc_var8.byte_add(5).cast::<i32>().read_unaligned();
                                    let t3 = pc_var8.byte_add(9).cast::<i32>().read_unaligned();
                                    if pc_var8.read() != 0
                                        && t1 == i_var12
                                        && t2 == i_var11
                                        && t3 == i_var13
                                    {
                                        break;
                                    }
                                    pc_var8 = pc_var8.byte_add(0x23);
                                    i_var9 = i_var9.wrapping_add(1);
                                    if !(i_var9 < DAT_009d1308.wrapping_add(DAT_00a07ce8)) {
                                        break;
                                    }
                                }
                            }
                            if !(i_var9 < DAT_009d1308) {
                                break;
                            }
                        }
                        DAT_0099f97c = 0;
                        DAT_0099f97e = 0;
                        DAT_0099f980 = 0;
                        DAT_0099f970 = i_var12;
                        DAT_0099f974 = i_var11;
                        DAT_0099f978 = i_var13;
                        let u_var7 = FUN_00425cb0();
                        FUN_00458c80(
                            DAT_009a1834,
                            10,
                            (((u_var7 & 0xffff) % 6) as u8).wrapping_add(b'd'),
                        );
                        local_24 = local_24.wrapping_add(1);
                    }
                    DAT_009a73f0 = DAT_009d113c;
                }
            }
        }
        DAT_009d113c = u_var6;
    }
}
