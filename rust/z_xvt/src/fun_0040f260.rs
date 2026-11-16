// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: *mut u8;
    static mut DAT_006d4980: i32;
    static mut DAT_0071b704: i32;
    static mut DAT_00732904: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004187e0(a: u16, b: u16) -> u32;
    fn FUN_00483410(a: i16, b: i16, c: i32);
    fn FUN_00419100(a: i32, b: i32) -> u32;
    fn FUN_004184f0(a: u16) -> u16;
    fn FUN_00483540(a: i16, b: i32, c: i32);
    fn FUN_0040a1a0(a: u32, b: u32, c: i32) -> u32;
}

unsafe fn rec(off: i32) -> *mut u8 {
    // SAFETY: the game's record table pointer, offset as the original does.
    unsafe { DAT_006d4988.byte_offset(off as isize) }
}

unsafe fn r16(p: *const u8) -> u16 {
    // SAFETY: caller passes a pointer into the game's memory.
    unsafe { p.cast::<u16>().read_unaligned() }
}

unsafe fn r32(p: *const u8) -> i32 {
    // SAFETY: caller passes a pointer into the game's memory.
    unsafe { p.cast::<i32>().read_unaligned() }
}

unsafe fn rp(p: *const u8) -> *mut u8 {
    // SAFETY: caller passes a pointer into the game's memory.
    unsafe { p.cast::<u32>().read_unaligned() as usize as *mut u8 }
}

unsafe fn w16(p: *mut u8, v: u16) {
    // SAFETY: caller passes a pointer into the game's memory.
    unsafe { p.cast::<u16>().write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040f260(param_1: u16, param_2: u16) {
    // SAFETY: all pointers are the game's own object records and globals,
    // read and written at the same offsets and in the same order as the original.
    unsafe {
        let i_var4 = (param_1 as u32).wrapping_mul(0x23) as i32;
        let c_var2 = *rec(i_var4 + 2);
        if c_var2 > 2 {
            return;
        }
        let mut p_obj_a = rp(rec(i_var4 + 0x1f));
        let i_var10 = rp(p_obj_a.byte_add(0xa7));
        w16(i_var10.byte_add(0x70), param_2);
        let u_var16 = param_2 as u32;
        p_obj_a = rp(rec(i_var4 + 0x1f));
        let mut s_var15 = r16(p_obj_a.byte_add(0x79)) as i16;
        let i_var5 = u_var16.wrapping_mul(0x23) as i32;
        let sa = r16(rec(i_var5 + 0x10)) as i16;
        let sb = r16(rec(i_var4 + 0x10)) as i16;
        let mut u_var6 = (sa as i32 as u32).wrapping_sub(sb as i32 as u32) as u16;
        if 0x7fff < u_var6 {
            u_var6 = u_var6.wrapping_neg();
        }
        if 0x4000 < u_var6 {
            u_var6 = 0x8000u32.wrapping_sub(u_var6 as u32) as u16;
        }
        if (u_var16 as i32) < DAT_006d4980 {
            let p_obj_b = rp(rec(i_var5 + 0x1f));
            let u_var3 = r16(p_obj_b.byte_add(0x79));
            if u_var6 < 0x4000 {
                let u_var8 = FUN_004187e0(u_var3, u_var6);
                s_var15 = s_var15.wrapping_sub(u_var8 as i16);
            } else {
                let u_var8 = FUN_004187e0(u_var3, u_var6);
                s_var15 = s_var15.wrapping_add(u_var8 as i16);
            }
        }
        if s_var15 < 0 {
            s_var15 = s_var15.wrapping_neg();
        }
        let mut i_var19 = rec(i_var4);
        p_obj_a = rp(i_var19.byte_add(0x1f));
        let mut i_var11: *mut u8 = core::ptr::null_mut();
        if !p_obj_a.is_null() {
            i_var11 = rp(rec(0x1f + i_var5));
        }
        let i_var17: i32;
        let local_10: i32;
        let mut i_var14: i32;
        let i_var11i: i32;
        if p_obj_a.is_null() || i_var11.is_null() {
            i_var11 = rec(i_var5);
            i_var17 = r32(i_var19.byte_add(4)).wrapping_sub(r32(i_var11.byte_add(4)));
            local_10 = r32(i_var19.byte_add(8)).wrapping_sub(r32(i_var11.byte_add(8)));
            i_var14 = r32(i_var19.byte_add(0xc));
            i_var11i = r32(i_var11.byte_add(0xc));
        } else {
            i_var17 = r32(p_obj_a.byte_add(6)).wrapping_sub(r32(i_var11.byte_add(6)));
            local_10 = r32(p_obj_a.byte_add(10)).wrapping_sub(r32(i_var11.byte_add(10)));
            i_var14 = r32(p_obj_a.byte_add(0xe));
            i_var11i = r32(i_var11.byte_add(0xe));
        }
        i_var14 = i_var14.wrapping_sub(i_var11i);
        let dist = i_var14
            .wrapping_mul(i_var14)
            .wrapping_add(local_10.wrapping_mul(local_10))
            .wrapping_add(i_var17.wrapping_mul(i_var17));
        let mut local_26: u16;
        let mut local_28: u16;
        let i_var18: i32;
        let local_18: i32;
        if dist < 0x33 {
            local_26 = 0;
            local_28 = 100;
            i_var18 = i_var17;
            local_18 = local_10;
        } else {
            i_var18 = i_var17.wrapping_mul(s_var15 as i32).wrapping_mul(1000);
            local_26 = (i_var18 / dist) as u16;
            local_18 = local_10.wrapping_mul(s_var15 as i32).wrapping_mul(1000);
            local_28 = (local_18 / dist) as u16;
        }
        p_obj_a = rp(i_var19.byte_add(0x1f));
        if *p_obj_a.byte_add(0x89) != 0 {
            let sa = r16(i_var19.byte_add(0x12)) as i16;
            let sb = r16(i_var19.byte_add(0x10)) as i16;
            FUN_00483410(sa, sb, i_var19 as i32);
        }
        i_var19 = rec(i_var4);
        p_obj_a = rp(i_var19.byte_add(0x1f));
        u_var6 = r16(p_obj_a.byte_add(0x8a));
        local_26 = local_26.wrapping_add(u_var6);
        if ((local_26 ^ u_var6) & 0x8000) != 0 {
            local_26 = u_var6;
        }
        p_obj_a = rp(i_var19.byte_add(0x1f));
        u_var6 = r16(p_obj_a.byte_add(0x8c));
        local_28 = local_28.wrapping_add(u_var6);
        if ((u_var6 ^ local_28) & 0x8000) != 0 {
            local_28 = u_var6;
        }
        let mut local_2a = r16(i_var19.byte_add(0x10));
        let u_var9 = FUN_00419100(local_26 as i16 as i32, local_28 as i16 as i32);
        w16(i_var19.byte_add(0x10), u_var9 as i16 as u16);
        if DAT_0071b704 <= u_var16 as i32 && (u_var16 as i32) < DAT_00732904 {
            let p_obj_b = rp(rec(0x1f + i_var5));
            let s_var13 = r16(p_obj_b.byte_add(0x79)) as i16;
            let puvar1 = rec(0x10 + i_var4);
            let mut s_var12 = s_var13.wrapping_mul(8);
            u_var6 = r16(puvar1);
            if u_var6 < local_2a {
                s_var12 = s_var13.wrapping_mul(-8);
            }
            u_var6 = u_var6.wrapping_add(s_var12 as u16);
            w16(puvar1, u_var6);
        }
        if (u_var16 as i32) < DAT_006d4980 {
            let i_var11 = rec(i_var5);
            let p_obj_b = rp(i_var11.byte_add(0x1f));
            let i_var19 = rp(p_obj_b.byte_add(0xa7));
            let u_var7 = FUN_004184f0(local_26);
            w16(i_var11.byte_add(0x12), u_var7);
            let us_t = r16(rec(0x12 + i_var5));
            w16(i_var19.byte_add(0xd), us_t);
            u_var6 = (s_var15 as i32 as u32).wrapping_mul(100) as u16;
            if 0x7fff < u_var6 {
                u_var6 = 0x7fff;
            }
            let ua = r16(rec(i_var5 + 0x10));
            if local_2a < ua {
                u_var6 = u_var6.wrapping_neg();
            }
            let p_obj_b = rp(rec(i_var5 + 0x1f));
            w16(p_obj_b.byte_add(0x77), u_var6);
            let i_var19 = rec(i_var5);
            let sa = r16(i_var19.byte_add(0x12)) as i16;
            let sb = r16(i_var19.byte_add(0x10)) as i16;
            FUN_00483410(sa, sb, i_var19 as i32);
            let sa = r16(rec(i_var5 + 0x14)) as i16;
            FUN_00483540(sa, 0, rec(i_var5) as i32);
        }
        if (u_var16 as i32) < DAT_006d4980 {
            let p_obj_b = rp(rec(0x1f + i_var5));
            let p_t = rp(p_obj_b.byte_add(0xa7));
            w16(p_t.byte_add(0x70), param_1);
            let i_var19i = i_var14
                .wrapping_mul(i_var14)
                .wrapping_add(local_10.wrapping_mul(local_10))
                .wrapping_add(i_var17.wrapping_mul(i_var17));
            let mut s_var12: i16;
            let s_var13: i16;
            if i_var19i < 0x33 {
                s_var12 = 0;
                s_var13 = 100;
            } else {
                s_var12 =
                    (i_var18.wrapping_mul(s_var15 as i32).wrapping_mul(1000) / i_var19i) as i16;
                s_var13 =
                    (local_18.wrapping_mul(s_var15 as i32).wrapping_mul(1000) / i_var19i) as i16;
            }
            let i_var19 = rec(i_var5);
            let p_obj_b = rp(i_var19.byte_add(0x1f));
            if *p_obj_b.byte_add(0x89) != 0 {
                let sa = r16(i_var19.byte_add(0x12)) as i16;
                let sb = r16(i_var19.byte_add(0x10)) as i16;
                FUN_00483410(sa, sb, i_var19 as i32);
            }
            let i_var19 = rec(i_var5);
            let p_obj_b = rp(i_var19.byte_add(0x1f));
            s_var12 = s_var12.wrapping_add(r16(p_obj_b.byte_add(0x8a)) as i16);
            local_2a = r16(i_var19.byte_add(0x10));
            let p_obj_b = rp(i_var19.byte_add(0x1f));
            let arg2 = s_var13.wrapping_add(r16(p_obj_b.byte_add(0x8c)) as i16);
            let u_var9 = FUN_00419100(s_var12 as i32, arg2 as i32);
            w16(i_var19.byte_add(0x10), u_var9 as i16 as u16);
            let u_var7 = FUN_004184f0(s_var12 as u16);
            w16(rec(0x12 + i_var4), u_var7);
            let us_t = r16(rec(0x12 + i_var4));
            w16(i_var10.byte_add(0xd), us_t);
        }
        u_var6 = (s_var15 as i32 as u32).wrapping_mul(100) as u16;
        if 0x7fff < u_var6 {
            u_var6 = 0x7fff;
        }
        let ua = r16(rec(i_var4 + 0x10));
        if local_2a < ua {
            u_var6 = u_var6.wrapping_neg();
        }
        p_obj_a = rp(rec(i_var4 + 0x1f));
        w16(p_obj_a.byte_add(0x77), u_var6);
        let i_var19 = rec(i_var4);
        let sa = r16(i_var19.byte_add(0x12)) as i16;
        let sb = r16(i_var19.byte_add(0x10)) as i16;
        FUN_00483410(sa, sb, i_var19 as i32);
        let sa = r16(rec(i_var4 + 0x14)) as i16;
        FUN_00483540(sa, 0, rec(i_var4) as i32);
        if (u_var16 as i32) < DAT_00732904 {
            let i_var19 = rec(i_var5);
            let sa = r16(i_var19.byte_add(0x12)) as i16;
            let sb = r16(i_var19.byte_add(0x10)) as i16;
            FUN_00483410(sa, sb, i_var19 as i32);
            let sa = r16(rec(i_var5 + 0x14)) as i16;
            FUN_00483540(sa, 0, rec(i_var5) as i32);
        }
        let i_t2 = r32(rec(0x1b + i_var4));
        FUN_0040a1a0(0x1f, param_1 as u32, i_t2);
        if (u_var16 as i32) < DAT_006d4980 {
            let i_t2 = r32(rec(0x1b + i_var4));
            FUN_0040a1a0(0x20, u_var16, i_t2);
        }
    }
}
