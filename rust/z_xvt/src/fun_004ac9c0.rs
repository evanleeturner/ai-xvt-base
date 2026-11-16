// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00656c80: u8;
    static mut DAT_0071ecf4: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_0071b730: u8;
    static mut DAT_0071b74a: u8;
    static mut DAT_0071b757: u8;
    static mut DAT_0071b758: u8;
    static mut DAT_0071bb37: u8;
    static mut DAT_0052b0ac: u8;
    static mut DAT_00655184: u16;
    static mut DAT_00655ab0: u8;
    static mut DAT_00655ab2: u8;
    static mut DAT_00655ab4: u8;
    static mut DAT_00655ab6: u8;
    static mut DAT_00655ad4: u8;
    static mut DAT_00655ad6: u8;
    static mut DAT_00655ad8: u8;
    static mut DAT_00655ada: u8;
    static mut DAT_00655b04: u8;
    static mut DAT_00655b06: u8;
    static mut DAT_00656c90: u8;
    static mut DAT_00656c96: u8;
    static mut DAT_0052552c: i32;
    static mut DAT_00525518: i32;
    static mut DAT_006da538: i32;
    static mut DAT_006da9ec: i32;
    static mut DAT_006db594: i32;
    static mut DAT_0071f2f0: u32;
    static mut DAT_0070a184: unsafe extern "C" fn(u32, i32, i32, i32, i32);
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004783e0(a: u16, b: i32) -> u32;
    fn FUN_00461f20(a: i32, b: i32, c: i32, d: i32);
    fn FUN_0048ee20(a: i8);
    fn FUN_0048ed00(a: i32, b: i32, c: i16, d: i16);
    fn FUN_0048ece0(a: i16, b: u16);
    fn FUN_0048eda0(a: i32);
    fn FUN_0048ed70(a: i32);
    fn FUN_0040ccc0(a: u16, b: u32, c: i32);
    fn FUN_00410060(a: u16, b: u16, c: u32) -> u32;
}

unsafe fn r8(p: *const u8, off: u32) -> u8 {
    // SAFETY: caller supplies a valid game address.
    unsafe { p.byte_add(off as usize).read() }
}
unsafe fn r16(p: *const u8, off: u32) -> u16 {
    // SAFETY: caller supplies a valid game address.
    unsafe { p.byte_add(off as usize).cast::<u16>().read_unaligned() }
}
unsafe fn r32(p: *const u8, off: u32) -> u32 {
    // SAFETY: caller supplies a valid game address.
    unsafe { p.byte_add(off as usize).cast::<u32>().read_unaligned() }
}
unsafe fn w16(p: *mut u8, off: u32, v: u16) {
    // SAFETY: caller supplies a valid game address.
    unsafe { p.byte_add(off as usize).cast::<u16>().write_unaligned(v) }
}

unsafe fn ecf4() -> u32 {
    // SAFETY: game global.
    unsafe { (DAT_0071ecf4 as u32).wrapping_mul(0x5bd) }
}

unsafe fn slot_ok() -> bool {
    // SAFETY: game globals and tables.
    unsafe {
        let iv = r32((&raw mut DAT_0071b730) as *const u8, ecf4()) as i32;
        if iv == -1 {
            return false;
        }
        let c = r8(
            core::ptr::null::<u8>(),
            (iv as u32)
                .wrapping_mul(0x23)
                .wrapping_add(3)
                .wrapping_add(DAT_006d4988 as u32),
        ) as i8;
        c == 1 || c == 2 || c == 3 || c == 0x0e || c == 4
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004ac9c0() {
    // SAFETY: all globals, tables and callees are the game's own memory.
    unsafe {
        DAT_00656c80 = 0;
        let rec = (&raw mut DAT_0071b730) as *const u8;
        let slot = r32(rec, ecf4());
        let mut tmp = slot
            .wrapping_mul(0x23)
            .wrapping_add(0x1f)
            .wrapping_add(DAT_006d4988 as u32);
        tmp = r32(core::ptr::null::<u8>(), tmp);
        tmp = tmp.wrapping_add(0xa7);
        tmp = r32(core::ptr::null::<u8>(), tmp);
        let p6 = tmp as usize as *const u8;

        let mut u_stack_20: u16 = 0;
        let b_var2 = r8(p6, 4);
        let mut local_26: u32 = (r8(p6, 0x213) as u32) << 0x10;
        let mut local_22: i16 = 0;
        let mut u_stack_1e: u16 = 0;
        let mut local_1c: u16 = 0;
        let mut u_stack_1a: u16 = 0;

        if r8(p6, 0x213) != 0 {
            loop {
                let u_var8 = u_stack_20 as u32;
                let mut u_var13: u32 = ((r8(
                    (&raw mut DAT_0052b0ac) as *const u8,
                    (b_var2 as u32).wrapping_mul(0xef),
                ) as u32)
                    < u_var8) as u32;
                let mut i_var12: u32 = (DAT_00655184 as u32).wrapping_add(u_var8);
                let s_var4 = r16(
                    (&raw mut DAT_00655ad4) as *const u8,
                    i_var12.wrapping_mul(0xc),
                ) as i16;
                let i_var1 = i_var12.wrapping_mul(0xc);
                let mut u_var16 = r16((&raw mut DAT_00655ad6) as *const u8, i_var1);
                if (s_var4 as i32 as u32).wrapping_add(u_var16 as u32) != 0 || DAT_00655184 != 0 {
                    let u_var10 = r16((&raw mut DAT_00655ad8) as *const u8, i_var1);
                    local_1c =
                        (r8(p6, 0x345u32.wrapping_add(u_var8.wrapping_mul(4))) as i8 as i16) as u16;
                    let w = r16(p6, 0x1db);
                    if r8((&raw mut DAT_0071bb37) as *const u8, ecf4()) == 0
                        && (w & 2) != 0
                        && (w & 4) != 0
                    {
                        if (local_1c as i16) < 1 || (r8(p6, 0x1df) & 0x10) == 0 {
                            u_stack_1e = 0;
                            local_26 = (local_26 >> 16) << 16;
                            local_22 = 0;
                        } else {
                            let u_var15 = local_1c.wrapping_add(1);
                            if (u_var15 as i16) < 0x41 {
                                local_26 = (local_26 >> 16) << 16;
                                local_22 = 1;
                                local_1c = u_var15;
                            } else {
                                local_26 = (local_26 & 0xffff0000) | 1;
                                local_22 = 2;
                                local_1c = local_1c.wrapping_sub(0x3f);
                            }
                            u_stack_1e = local_1c / 6;
                            local_1c = u_var15;
                            if 10 < u_stack_1e {
                                u_stack_1e = 10;
                            }
                        }
                        let a96 = (&raw mut DAT_00656c96) as *mut u8;
                        if r16(a96, i_var12.wrapping_mul(2)) != u_stack_1e {
                            w16(a96, i_var12.wrapping_mul(2), u_stack_1e);
                            let mut s_var9: i16 = 3;
                            if DAT_0052552c != 0x13 {
                                s_var9 =
                                    ((0u16.wrapping_sub((DAT_0052552c == 0x112) as u16)) & 0xfffe)
                                        .wrapping_add(6) as i16;
                            }
                            let s_var5 = r16((&raw mut DAT_00655ada) as *const u8, i_var1) as i16;
                            if s_var5 != 0 {
                                s_var9 = s_var9.wrapping_neg();
                            }
                            if DAT_00655184 == 0 {
                                let mut u_var11: u32 = 0;
                                loop {
                                    let mut u_var14 =
                                        (u_stack_20 as u32) << 16 | (local_22 as u16 as u32);
                                    if u_stack_1e <= u_var11 as u16 {
                                        u_var14 = local_26;
                                    }
                                    let u_var15 = (u_var11 as u16).wrapping_add(1);
                                    let idx = (u_var14 & 0xffff).wrapping_add(u_var10 as u32);
                                    let v = (&raw mut DAT_0071f2f0).add(idx as usize).read();
                                    DAT_0070a184(
                                        v,
                                        u_var11
                                            .wrapping_mul(s_var9 as i32 as u32)
                                            .wrapping_add(s_var4 as i32 as u32)
                                            as i32,
                                        u_var16 as u32 as i32,
                                        0xfd,
                                        (s_var5 != 0) as i32,
                                    );
                                    u_var11 = u_var15 as u32;
                                    if u_var15 >= 10 {
                                        break;
                                    }
                                }
                            } else if r16((&raw mut DAT_00655ad8) as *const u8, i_var1) != 0 {
                                let u_var11 = FUN_004783e0(local_1c, 0x7f);
                                FUN_00461f20(
                                    DAT_006da9ec,
                                    DAT_006da538,
                                    DAT_006db594,
                                    (DAT_00525518 as u32).wrapping_mul(DAT_006da538 as u32) as i32,
                                );
                                FUN_0048ee20(2);
                                FUN_0048ed00(0, 0, DAT_006da538 as i16, DAT_006db594 as i16);
                                FUN_0048ece0(s_var4, u_var16);
                                FUN_0048eda0(0);
                                FUN_0048ed70(0x4a);
                                let off =
                                    (DAT_00655184 as u32).wrapping_add(u_var8).wrapping_mul(0xc);
                                FUN_0040ccc0(
                                    ((u_var11 & 0xffff) / 0x28f) as u16,
                                    r16((&raw mut DAT_00655ad8) as *const u8, off) as u32,
                                    1,
                                );
                                FUN_00461f20(0, 0x140, 200, 0);
                            }
                        }
                    }
                    if (local_1c as i16) < 1 || (r8(p6, 0x1df) & 0x10) == 0 {
                        u_stack_1a = 0;
                        u_var16 = 0;
                    } else if r8((&raw mut DAT_0071b758) as *const u8, ecf4()) != 0
                        || (r8((&raw mut DAT_0071b757) as *const u8, ecf4()) as u32) != u_var13
                    {
                        u_stack_1a = 1;
                        u_var16 = 0;
                    } else {
                        match r8(p6, 0x216u32.wrapping_add(u_var13)) {
                            0 | 4 => {
                                u_stack_1a = 0;
                            }
                            1 => {
                                let b = r8(p6, 0x21au32.wrapping_add(u_var13)) as u32;
                                u_stack_1a =
                                    ((0u16.wrapping_sub((b == u_var8) as u16)) & 2).wrapping_add(1);
                            }
                            2 => {
                                let u_var11 = r8(p6, 0x21au32.wrapping_add(u_var13)) as u32;
                                if u_var11 == u_var8
                                    || (3 < (local_26 >> 16)
                                        && u_var11.wrapping_sub(u_var8) == 0xfffffffe)
                                {
                                    u_stack_1a = 3;
                                } else {
                                    u_stack_1a = 1;
                                }
                            }
                            3 => {
                                u_stack_1a = 3;
                            }
                            _ => {}
                        }
                        u_var16 = u_stack_1a;
                        if u_stack_1a == 3
                            && r16(p6, 0x21cu32.wrapping_add(u_var13.wrapping_mul(2))) != 0
                        {
                            u_stack_1a = 5;
                            u_var16 = 2;
                        }
                    }
                    if local_22 == 2 {
                        u_stack_1a = u_stack_1a.wrapping_add(1);
                    }
                    let b7 = slot_ok();
                    let w = r16(p6, 0x1db);
                    if b7
                        && r8((&raw mut DAT_0071bb37) as *const u8, ecf4()) == 0
                        && (w & 2) != 0
                        && (w & 4) != 0
                    {
                        i_var12 = (DAT_00655184 as u32).wrapping_add(u_var8).wrapping_mul(0xc);
                        let s =
                            (r16((&raw mut DAT_00655b04) as *const u8, i_var12) as u32)
                                .wrapping_add(
                                    r16((&raw mut DAT_00655b06) as *const u8, i_var12) as u32
                                );
                        if s != 0 {
                            i_var12 = u_var8.wrapping_add(7).wrapping_add(DAT_00655184 as u32);
                            let a90 = (&raw mut DAT_00656c90) as *mut u8;
                            if r16(a90, i_var12.wrapping_mul(2)) != u_stack_1a {
                                w16(a90, i_var12.wrapping_mul(2), u_stack_1a);
                                i_var12 = i_var12.wrapping_mul(0xc);
                                let idx = (r16((&raw mut DAT_00655ab4) as *const u8, i_var12)
                                    as u32)
                                    .wrapping_add(u_stack_1a as u32);
                                let v = (&raw mut DAT_0071f2f0).add(idx as usize).read();
                                DAT_0070a184(
                                    v,
                                    r16((&raw mut DAT_00655ab0) as *const u8, i_var12) as i16
                                        as i32,
                                    r16((&raw mut DAT_00655ab2) as *const u8, i_var12) as i16
                                        as i32,
                                    r16((&raw mut DAT_00655ab6) as *const u8, i_var12) as i16
                                        as i32,
                                    0,
                                );
                            }
                        }
                    }
                    i_var12 = (DAT_00655184 as u32)
                        .wrapping_add(0x25)
                        .wrapping_add(u_var8);
                    let a90 = (&raw mut DAT_00656c90) as *mut u8;
                    if r16(a90, i_var12.wrapping_mul(2)) != u_var16 {
                        w16(a90, i_var12.wrapping_mul(2), u_var16);
                        i_var12 = i_var12.wrapping_mul(0xc);
                        let idx = (r16((&raw mut DAT_00655ab4) as *const u8, i_var12) as u32)
                            .wrapping_add(u_var16 as u32);
                        let v = (&raw mut DAT_0071f2f0).add(idx as usize).read();
                        DAT_0070a184(
                            v,
                            r16((&raw mut DAT_00655ab0) as *const u8, i_var12) as i16 as i32,
                            r16((&raw mut DAT_00655ab2) as *const u8, i_var12) as i16 as i32,
                            r16((&raw mut DAT_00655ab6) as *const u8, i_var12) as i16 as i32,
                            0,
                        );
                    }
                    let w = r16(p6, 0x1df);
                    let u_var10 = w & 4;
                    if u_var10 == 0 || u_var16 != 3 {
                        if u_var10 == 0 {
                            u_var16 = 0;
                        } else if u_var16 == 2 {
                            u_var16 = 1;
                        }
                    } else {
                        u_var16 = r16((&raw mut DAT_0071b74a) as *const u8, ecf4());
                        let t = r32(rec, ecf4());
                        if u_var16 == 0xffff || {
                            u_var13 = FUN_00410060(
                                t as u16,
                                u_var16,
                                (u_stack_1e as u32) << 16 | (u_stack_20 as u32),
                            );
                            (u_var13 as u16 as i16) == 0
                        } {
                            u_var16 = 1;
                        } else {
                            DAT_00656c80 = 1;
                            u_var16 = 2;
                        }
                    }
                    if u_var16 == 3 {
                        u_var16 = 0;
                    }
                    let b7 = slot_ok();
                    if b7 {
                        i_var12 = (DAT_00655184 as u32)
                            .wrapping_add(0x29)
                            .wrapping_add(u_var8);
                        if r16(a90, i_var12.wrapping_mul(2)) != u_var16 {
                            w16(a90, i_var12.wrapping_mul(2), u_var16);
                            i_var12 = i_var12.wrapping_mul(0xc);
                            let idx = (r16((&raw mut DAT_00655ab4) as *const u8, i_var12) as u32)
                                .wrapping_add(u_var16 as u32);
                            let v = (&raw mut DAT_0071f2f0).add(idx as usize).read();
                            DAT_0070a184(
                                v,
                                r16((&raw mut DAT_00655ab0) as *const u8, i_var12) as i16 as i32,
                                r16((&raw mut DAT_00655ab2) as *const u8, i_var12) as i16 as i32,
                                r16((&raw mut DAT_00655ab6) as *const u8, i_var12) as i16 as i32,
                                0,
                            );
                        }
                    }
                }
                u_stack_20 = u_stack_20.wrapping_add(1);
                if u_stack_20 >= (local_26 >> 16) as u16 {
                    break;
                }
            }
        }
    }
}
