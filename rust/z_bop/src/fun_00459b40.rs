// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009d8c82: u8;
    static mut DAT_009d8c83: u8;
    static mut DAT_009d8c84: u8;
    static mut DAT_009d8c8d: u8;
    static mut DAT_009d8c8e: u8;
    static mut DAT_00524170: u8;
    static mut DAT_00521f2e: u8;
    static mut DAT_00521f2f: u8;
    static mut DAT_00521f1b: u8;
    static mut DAT_0051c659: u8;
    static mut DAT_0051c65b: u8;
    static mut DAT_0051c65d: u8;
    static mut DAT_0051c65f: u8;
    static mut DAT_00527b08: u8;
    static mut DAT_00527ca0: u8;
    static mut DAT_00527970: u8;
    static mut DAT_009d9096: u8;
    static mut DAT_009d90c2: u8;
    static mut DAT_009d90ee: u8;
    static mut DAT_009d8a58: i32;
    static mut DAT_009d8a54: i32;
    static mut DAT_009d8b60: i32;
    static mut DAT_00a07cd4: i32;
    static mut DAT_00a07cd8: i32;
    static mut DAT_00a07cdc: i32;
    static mut DAT_009a1fe8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ae0b0(param_1: u32) -> i64;
    fn FUN_00458e20(param_1: u32, param_2: u32);
    fn FUN_004035d0(param_1: u32, param_2: i16, param_3: i16, param_4: i16);
}

unsafe fn rb(p: *mut u8, off: isize) -> u8 {
    // SAFETY: caller guarantees the game's table is readable at this offset.
    unsafe { p.offset(off).read() }
}

unsafe fn rs(p: *mut u8, off: isize) -> i16 {
    // SAFETY: caller guarantees the game's table is readable at this offset.
    unsafe { p.offset(off).cast::<i16>().read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00459b40(param_1: u32, param_2: u16, param_3: u16) {
    // SAFETY: all globals and callees are the game's own memory; table
    // reads happen at the offsets the original computes.
    unsafe {
        let u_var9: u32 = param_1 & 0xffff;
        let mut i_var1: i32 = u_var9.wrapping_mul(0x566) as i32;
        let u_var4: u32 = rb(
            &raw mut DAT_00524170,
            rb(&raw mut DAT_009d8c82, i_var1 as isize) as isize,
        ) as u32;
        let u_var2: u8 = rb(&raw mut DAT_00521f2f, u_var4.wrapping_mul(0x1a) as isize);
        let l_var14: i64 = FUN_004ae0b0(u_var4);
        FUN_00458e20(0x8000, u_var9);
        if rb(&raw mut DAT_00521f2e, u_var4.wrapping_mul(0x1a) as isize) & 0x80 == 0 {
            let u_var3: u16 = (((u_var2 as u32) << 8) | u_var2 as u32) as u16;
            let s65f = rs(&raw mut DAT_0051c65f, u_var3 as isize);
            let s65d = rs(&raw mut DAT_0051c65d, u_var3 as isize);
            let s65b = rs(&raw mut DAT_0051c65b, u_var3 as isize);
            let s_var8: i16 = s65f;
            let s_var10: i16 =
                (rb(&raw mut DAT_009d8c8e, i_var1 as isize) as i32).wrapping_add(1) as i16;
            i_var1 = (param_2 as u32)
                .wrapping_add((rb(&raw mut DAT_009d8c8d, i_var1 as isize) as u32).wrapping_mul(6))
                as i32;
            let s_a = rs(&raw mut DAT_00527b08, (i_var1 as isize) * 2);
            i_var1 = i_var1.wrapping_mul(2);
            let s_b = rs(&raw mut DAT_00527ca0, i_var1 as isize);
            let s_c = rs(&raw mut DAT_00527970, i_var1 as isize);
            let m10 = s_var10 as i32 as u32;
            let mut s_var13: i16 = m10
                .wrapping_mul(s_a as i32 as u32)
                .wrapping_mul(s_var8 as i32 as u32) as i16;
            let mut s_var11: i16 = m10
                .wrapping_mul(s_b as i32 as u32)
                .wrapping_mul(s65d as i32 as u32) as i16;
            let mut s_var7: i16 = m10
                .wrapping_mul(s_c as i32 as u32)
                .wrapping_mul(s65b as i32 as u32) as i16;
            if s_var10 == 1 {
                s_var7 = (s_var7 as i32 as u32)
                    .wrapping_add((((s65b as i32) / 2) as u32).wrapping_mul(s_c as i32 as u32))
                    as i16;
                s_var11 = (s_var11 as i32 as u32)
                    .wrapping_add((((s65d as i32) / 2) as u32).wrapping_mul(s_b as i32 as u32))
                    as i16;
                let t: i32 = s_var8 as i32;
                let q: i16 = (t.wrapping_add((t >> 0x1f) & 3) >> 2) as i16;
                s_var13 = (s_var13 as i32 as u32)
                    .wrapping_add((q as i32 as u32).wrapping_mul(s_a as i32 as u32))
                    as i16;
            }
            if param_3 == 0xffff {
                FUN_00458e20(0x8000, u_var9);
                DAT_009d8a58 = 0;
                DAT_009d8a54 = 0;
                DAT_009d8b60 = 0;
            } else {
                FUN_004035d0(
                    (param_3 as u32)
                        .wrapping_mul(0x23)
                        .wrapping_add(DAT_009a1fe8),
                    s_var7,
                    s_var11,
                    s_var13,
                );
            }
            let s659 = rs(&raw mut DAT_0051c659, u_var3 as isize);
            if s659 != 0 {
                let b5 = ((s659 as u8) & 0x1f) as u32;
                DAT_009d8a58 = (DAT_009d8a58 as u32).wrapping_shl(b5) as i32;
                DAT_009d8a54 = (DAT_009d8a54 as u32).wrapping_shl(b5) as i32;
                DAT_009d8b60 = (DAT_009d8b60 as u32).wrapping_shl(b5) as i32;
            }
            DAT_00a07cd4 = DAT_00a07cd4.wrapping_add(DAT_009d8b60);
            DAT_00a07cdc = DAT_00a07cdc.wrapping_add(DAT_009d8a58);
            DAT_00a07cd8 = DAT_00a07cd8.wrapping_add(DAT_009d8a54);
        } else if rb(&raw mut DAT_00521f1b, u_var4.wrapping_mul(0x1a) as isize) == 9 {
            DAT_00a07cdc = ((rs(&raw mut DAT_009d9096, i_var1 as isize) as i32 as u32) << 8) as i32;
            DAT_00a07cd8 = (rs(&raw mut DAT_009d90c2, i_var1 as isize) as i32 as u32)
                .wrapping_mul(0xffffff00) as i32;
            DAT_00a07cd4 = ((rs(&raw mut DAT_009d90ee, i_var1 as isize) as i32 as u32) << 8) as i32;
        } else if rb(&raw mut DAT_00521f1b, u_var4.wrapping_mul(0x1a) as isize) == 8 {
            let mut s_var8: i16 = 0;
            let mut s_var7: i16 = 0;
            let mut local_8: i16 = 0;
            let mut local_10: i16 = 0;
            let b_var5: u8 = rb(&raw mut DAT_009d8c84, i_var1 as isize) & 3;
            if b_var5 == 0 {
                s_var7 = 0x40;
                s_var8 = 0x40;
            } else if b_var5 == 1 {
                s_var8 = 0x40;
                local_10 = 0x40;
            } else if b_var5 == 2 {
                s_var7 = 0x40;
                local_8 = 0x40;
            }
            let u_var6: u16 = rb(&raw mut DAT_009d8c83, i_var1 as isize) as u16;
            let i_var12: i32 = (u_var6 as i32).wrapping_sub(1) as i16 as i32;
            let mut u_var3: u16 = 0;
            let mut local_12: i16 = 0;
            if rb(&raw mut DAT_009d8c83, i_var1 as isize) != 0 {
                loop {
                    let mut s_var10: i16 = 0;
                    if rb(&raw mut DAT_009d8c83, i_var1 as isize) != 0 {
                        loop {
                            if u_var3 == param_2 {
                                let a = rs(&raw mut DAT_009d9096, i_var1 as isize);
                                let h = (i_var12.wrapping_mul(s_var7 as i32) / 2) as i16;
                                let inner = a.wrapping_sub(h);
                                DAT_00a07cdc = ((s_var10 as i32).wrapping_mul(s_var7 as i32) as u32)
                                    .wrapping_add(inner as i32 as u32)
                                    .wrapping_mul(0x100)
                                    as i32;

                                let a2 = rs(&raw mut DAT_009d90c2, i_var1 as isize);
                                let h2 = (i_var12.wrapping_mul(s_var8 as i32) / 2) as i16;
                                let sum = a2.wrapping_add(h2);
                                let neg = sum.wrapping_neg();
                                DAT_00a07cd8 = ((local_12 as i32).wrapping_mul(s_var8 as i32)
                                    as u32)
                                    .wrapping_add(neg as i32 as u32)
                                    .wrapping_mul(0x100)
                                    as i32;

                                let a3 = rs(&raw mut DAT_009d90ee, i_var1 as isize);
                                let h3 = (i_var12.wrapping_mul(local_8 as i32) / 2) as i16;
                                let h4 = (i_var12.wrapping_mul(local_10 as i32) / 2) as i16;
                                let inner3 = a3.wrapping_sub(h3).wrapping_sub(h4);
                                let p1 = (local_12 as i32).wrapping_mul(local_8 as i32) as u32;
                                let p2 = (s_var10 as i32).wrapping_mul(local_10 as i32) as u32;
                                DAT_00a07cd4 = (l_var14 as i16 as i32 as u32).wrapping_add(
                                    p1.wrapping_add(p2)
                                        .wrapping_add(inner3 as i32 as u32)
                                        .wrapping_mul(0x100),
                                ) as i32;
                                return;
                            }
                            u_var3 = u_var3.wrapping_add(1);
                            s_var10 = s_var10.wrapping_add(1);
                            if !((s_var10 as i32) < (u_var6 as i32)) {
                                break;
                            }
                        }
                    }
                    local_12 = local_12.wrapping_add(1);
                    if !((local_12 as i32) < (u_var6 as i32)) {
                        break;
                    }
                }
            }
        }
        DAT_00a07cd4 = (DAT_00a07cd4 as u32).wrapping_add(l_var14 as i16 as i32 as u32) as i32;
    }
}
