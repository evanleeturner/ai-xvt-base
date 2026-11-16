// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009e9640: i32;
    static mut DAT_00a00854: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0046a700(a: u16, b: u16) -> u32;
    fn FUN_00427bd0(a: i16, b: i16, c: i32);
    fn FUN_0046abf0(a: i32, b: i32) -> u32;
    fn FUN_0046a460(a: u16) -> u16;
    fn FUN_00427d30(a: i16, b: i32, c: i32);
    fn FUN_0042e4d0(a: i32, b: u32, c: i32);
}

unsafe fn base() -> *mut u8 {
    // SAFETY: reads the game's global base pointer.
    unsafe { (DAT_009a1fe8 as u32) as usize as *mut u8 }
}

fn off(p: *mut u8, o: i32) -> *mut u8 {
    p.wrapping_offset(o as isize)
}

unsafe fn rp(p: *mut u8) -> *mut u8 {
    // SAFETY: caller passes a valid address holding a 4-byte pointer.
    unsafe { (p.cast::<u32>().read_unaligned()) as usize as *mut u8 }
}

unsafe fn r32(p: *mut u8) -> i32 {
    // SAFETY: caller passes a valid address.
    unsafe { p.cast::<i32>().read_unaligned() }
}

unsafe fn r16(p: *mut u8) -> i16 {
    // SAFETY: caller passes a valid address.
    unsafe { p.cast::<i16>().read_unaligned() }
}

unsafe fn ru16(p: *mut u8) -> u16 {
    // SAFETY: caller passes a valid address.
    unsafe { p.cast::<u16>().read_unaligned() }
}

unsafe fn w16(p: *mut u8, v: u16) {
    // SAFETY: caller passes a valid address.
    unsafe { p.cast::<u16>().write_unaligned(v) }
}

fn ai(p: *mut u8) -> i32 {
    p as usize as i32
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0041bc50(param_1: u16, param_2: u16) {
    // SAFETY: all pointers are into the game's own object tables, accessed
    // at the offsets the original uses; callees and globals are the game's.
    unsafe {
        let i_var3: i32 = (param_1 as u32).wrapping_mul(0x23) as i32;
        let c_var1: i8 = *off(base(), i_var3 + 2) as i8;
        if !(c_var1 == 0 || c_var1 == 1 || c_var1 == 2) {
            return;
        }
        let mut in_a = rp(off(base(), i_var3 + 0x1f));
        let i_var9 = rp(off(in_a, 0xa9));
        w16(off(i_var9, 0x70), param_2);
        let u_var14: u32 = param_2 as u32;
        in_a = rp(off(base(), i_var3 + 0x1f));
        let mut s_var12: i16 = r16(off(in_a, 0x79));
        let i_var4: i32 = u_var14.wrapping_mul(0x23) as i32;
        let mut u_var5: u16 = ((r16(off(base(), i_var4 + 0x10)) as i32 as u32).wrapping_sub(r16(
            off(base(), i_var3 + 0x10),
        )
            as i32
            as u32)) as u16;
        if 0x7fff < u_var5 {
            u_var5 = 0u32.wrapping_sub(u_var5 as u32) as u16;
        }
        if 0x4000 < u_var5 {
            u_var5 = 0x8000u32.wrapping_sub(u_var5 as u32) as u16;
        }
        if (u_var14 as i32) < DAT_009a1fe0 {
            let in_b = rp(off(base(), i_var4 + 0x1f));
            let u_var2: u16 = ru16(off(in_b, 0x79));
            if u_var5 < 0x4000 {
                let u_var7 = FUN_0046a700(u_var2, u_var5);
                s_var12 = (s_var12 as i32 as u32).wrapping_sub(u_var7 as i16 as i32 as u32) as i16;
            } else {
                let u_var7 = FUN_0046a700(u_var2, u_var5);
                s_var12 = (s_var12 as i32 as u32).wrapping_add(u_var7 as i16 as i32 as u32) as i16;
            }
        }
        if s_var12 < 0 {
            s_var12 = (0u32.wrapping_sub(s_var12 as i32 as u32)) as i16;
        }
        let p_a = off(base(), i_var3);
        let p_b = off(base(), i_var4);
        let mut i_var13 = p_a;
        in_a = rp(off(p_a, 0x1f));
        let mut in_b = rp(off(p_b, 0x1f));
        let i_var15: i32;
        let local_10: i32;
        let mut i_var18: i32;
        let mut i_var10: i32;
        if in_a.is_null() || in_b.is_null() {
            i_var15 = r32(off(p_a, 4)).wrapping_sub(r32(off(p_b, 4)));
            local_10 = r32(off(p_a, 8)).wrapping_sub(r32(off(p_b, 8)));
            i_var18 = r32(off(p_a, 0xc));
            i_var10 = r32(off(p_b, 0xc));
        } else {
            i_var15 = r32(off(in_a, 6)).wrapping_sub(r32(off(in_b, 6)));
            local_10 = r32(off(in_a, 10)).wrapping_sub(r32(off(in_b, 10)));
            i_var18 = r32(off(in_a, 0xe));
            i_var10 = r32(off(in_b, 0xe));
        }
        i_var18 = i_var18.wrapping_sub(i_var10);
        i_var10 = i_var18
            .wrapping_mul(i_var18)
            .wrapping_add(local_10.wrapping_mul(local_10))
            .wrapping_add(i_var15.wrapping_mul(i_var15));
        let mut local_26: u16;
        let mut local_28: u16;
        let i_var16: i32;
        let local_18: i32;
        if i_var10 < 0x33 {
            local_26 = 0;
            local_28 = 100;
            i_var16 = i_var15;
            local_18 = local_10;
        } else {
            i_var16 = i_var15.wrapping_mul(s_var12 as i32).wrapping_mul(1000);
            local_26 = (i_var16 / i_var10) as u16;
            local_18 = local_10.wrapping_mul(s_var12 as i32).wrapping_mul(1000);
            local_28 = (local_18 / i_var10) as u16;
        }
        in_a = rp(off(i_var13, 0x1f));
        if *off(in_a, 0x8b) != 0 {
            FUN_00427bd0(
                r16(off(i_var13, 0x12)),
                r16(off(i_var13, 0x10)),
                ai(i_var13),
            );
        }
        i_var13 = off(base(), i_var3);
        in_a = rp(off(i_var13, 0x1f));
        u_var5 = ru16(off(in_a, 0x8c));
        local_26 = local_26.wrapping_add(u_var5);
        if ((u_var5 ^ local_26) & 0x8000) != 0 {
            local_26 = u_var5;
        }
        u_var5 = ru16(off(in_a, 0x8e));
        local_28 = local_28.wrapping_add(u_var5);
        if ((u_var5 ^ local_28) & 0x8000) != 0 {
            local_28 = u_var5;
        }
        let mut local_2a: u16 = ru16(off(i_var13, 0x10));
        let u_var8 = FUN_0046abf0(local_26 as i16 as i32, local_28 as i16 as i32);
        w16(off(i_var13, 0x10), u_var8 as u16);
        if DAT_009e9640 <= u_var14 as i32 && (u_var14 as i32) < DAT_00a00854 {
            u_var5 = ru16(off(base(), 0x10 + i_var3));
            in_b = rp(off(base(), 0x1f + i_var4));
            let s_var17: i16 = r16(off(in_b, 0x79));
            let mut s_var11: i16 = (s_var17 as i32 as u32).wrapping_mul(8) as i16;
            if u_var5 < local_2a {
                s_var11 = (s_var17 as i32 as u32).wrapping_mul(-8i32 as u32) as i16;
            }
            w16(
                off(base(), 0x10 + i_var3),
                (u_var5 as u32).wrapping_add(s_var11 as i32 as u32) as u16,
            );
        }
        if (u_var14 as i32) < DAT_009a1fe0 {
            i_var13 = off(base(), i_var4);
            in_b = rp(off(i_var13, 0x1f));
            let aux = rp(off(in_b, 0xa9));
            let u_var6 = FUN_0046a460(local_26);
            w16(off(i_var13, 0x12), u_var6);
            w16(off(aux, 0xd), ru16(off(base(), 0x12 + i_var4)));
            u_var5 = (s_var12 as i32 as u32).wrapping_mul(100) as u16;
            if 0x7fff < u_var5 {
                u_var5 = 0x7fff;
            }
            if local_2a < ru16(off(base(), i_var4 + 0x10)) {
                u_var5 = 0u32.wrapping_sub(u_var5 as u32) as u16;
            }
            in_b = rp(off(base(), i_var4 + 0x1f));
            w16(off(in_b, 0x77), u_var5);
            i_var13 = off(base(), i_var4);
            FUN_00427bd0(
                r16(off(i_var13, 0x12)),
                r16(off(i_var13, 0x10)),
                ai(i_var13),
            );
            FUN_00427d30(r16(off(base(), i_var4 + 0x14)), 0, ai(off(base(), i_var4)));
        }
        if (u_var14 as i32) < DAT_009a1fe0 {
            in_b = rp(off(base(), 0x1f + i_var4));
            let aux = rp(off(in_b, 0xa9));
            w16(off(aux, 0x70), param_1);
            i_var10 = i_var18
                .wrapping_mul(i_var18)
                .wrapping_add(local_10.wrapping_mul(local_10))
                .wrapping_add(i_var15.wrapping_mul(i_var15));
            let mut s_var17: i16;
            let s_var11: i16;
            if i_var10 < 0x33 {
                s_var17 = 0;
                s_var11 = 100;
            } else {
                s_var17 =
                    (i_var16.wrapping_mul(s_var12 as i32).wrapping_mul(1000) / i_var10) as i16;
                s_var11 =
                    (local_18.wrapping_mul(s_var12 as i32).wrapping_mul(1000) / i_var10) as i16;
            }
            let mut p_b2 = off(base(), i_var4);
            in_b = rp(off(p_b2, 0x1f));
            if *off(in_b, 0x8b) != 0 {
                FUN_00427bd0(r16(off(p_b2, 0x12)), r16(off(p_b2, 0x10)), ai(p_b2));
            }
            p_b2 = off(base(), i_var4);
            in_b = rp(off(p_b2, 0x1f));
            s_var17 =
                (s_var17 as i32 as u32).wrapping_add(r16(off(in_b, 0x8c)) as i32 as u32) as i16;
            local_2a = ru16(off(p_b2, 0x10));
            let u_var8 = FUN_0046abf0(
                s_var17 as i32,
                (s_var11 as i32 as u32).wrapping_add(r16(off(in_b, 0x8e)) as i32 as u32) as i16
                    as i32,
            );
            w16(off(p_b2, 0x10), u_var8 as u16);
            let u_var6 = FUN_0046a460(s_var17 as u16);
            w16(off(base(), 0x12 + i_var3), u_var6);
            w16(off(i_var9, 0xd), ru16(off(base(), 0x12 + i_var3)));
        }
        u_var5 = (s_var12 as i32 as u32).wrapping_mul(100) as u16;
        if 0x7fff < u_var5 {
            u_var5 = 0x7fff;
        }
        if local_2a < ru16(off(base(), i_var3 + 0x10)) {
            u_var5 = 0u32.wrapping_sub(u_var5 as u32) as u16;
        }
        in_a = rp(off(base(), i_var3 + 0x1f));
        w16(off(in_a, 0x77), u_var5);
        i_var13 = off(base(), i_var3);
        FUN_00427bd0(
            r16(off(i_var13, 0x12)),
            r16(off(i_var13, 0x10)),
            ai(i_var13),
        );
        FUN_00427d30(r16(off(base(), i_var3 + 0x14)), 0, ai(off(base(), i_var3)));
        if (u_var14 as i32) < DAT_00a00854 {
            i_var13 = off(base(), i_var4);
            FUN_00427bd0(
                r16(off(i_var13, 0x12)),
                r16(off(i_var13, 0x10)),
                ai(i_var13),
            );
            FUN_00427d30(r16(off(base(), i_var4 + 0x14)), 0, ai(off(base(), i_var4)));
        }
        let t0 = r32(off(base(), 0x1b + i_var3));
        FUN_0042e4d0(0x1f, param_1 as u32, t0);
        if (u_var14 as i32) < DAT_009a1fe0 {
            let t0 = r32(off(base(), 0x1b + i_var3));
            FUN_0042e4d0(0x20, u_var14, t0);
        }
    }
}
