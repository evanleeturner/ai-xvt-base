// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00521f1c: u8;
    static mut DAT_00521f20: u8;
    static mut DAT_00521f1b: u8;
    static mut DAT_00527640: u8;
    static mut DAT_0051c633: u8;
    static mut DAT_0051c635: u8;
    static mut DAT_0051c637: u8;
    static mut DAT_0051c639: u8;
    static mut DAT_0051c63b: u8;
    static mut DAT_0051c63d: u8;
    static mut DAT_0051c63f: u8;
    static mut DAT_0051c641: u8;
    static mut DAT_0051c645: u8;
    static mut DAT_0051c649: u8;
    static mut DAT_0051c64d: u8;
    static mut DAT_0051c651: u8;
    static mut DAT_0051c655: u8;
    static mut DAT_0051c659: u8;
    static mut DAT_0051c65b: u8;
    static mut DAT_0051c65d: u8;
    static mut DAT_0051c65f: u8;
    static mut DAT_0051c59e: u8;
    static mut DAT_0051c5a0: u8;
    static mut DAT_0051c5a2: u8;
    static mut DAT_0051c5a4: u8;
    static mut DAT_0051c5a6: u8;
    static mut DAT_0051c5a8: u8;
    static mut DAT_0051c5aa: u8;
    static mut DAT_0051c5ac: u8;
    static mut DAT_0051c5ae: u8;
    static mut DAT_0051c5b2: u8;
    static mut DAT_0051c5b4: u8;
    static mut DAT_0051c5b6: u8;
    static mut DAT_0051c5b8: u8;
    static mut DAT_0051c5b9: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004adf10(a: u32) -> i64;
    fn FUN_004ae0e0(a: u32) -> i64;
    fn FUN_004ae120(a: u32) -> i64;
    fn FUN_004ae160(a: u32) -> i64;
    fn FUN_004ae020(a: u32) -> i64;
    fn FUN_004ae0b0(a: u32) -> i64;
    fn FUN_004adc40(a: u32) -> i32;
    fn FUN_004af250(a: u32, b: u32) -> u32;
    fn FUN_004ae2a0(a: u32, b: u32) -> i32;
    fn FUN_004af4c0(a: u32, b: u32, c: u32, d: *mut u32, e: *mut i32, f: *mut i32, g: *mut i32);
    fn FUN_004af2d0(a: u32, b: u32, c: u32) -> u32;
}

unsafe fn at(base: *mut u8, off: u32) -> *mut u8 {
    // SAFETY: the offset stays inside the game's own table.
    unsafe { base.add(off as usize) }
}

unsafe fn st16(p: *mut u8, v: u16) {
    // SAFETY: p points into the game's own table.
    unsafe { p.cast::<u16>().write_unaligned(v) }
}

unsafe fn st32(p: *mut u8, v: i32) {
    // SAFETY: p points into the game's own table.
    unsafe { p.cast::<i32>().write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0049d860(param_1: u8, param_2: u8) {
    // SAFETY: all globals and callees are the game's own; every write goes
    // where the original writes it.
    unsafe {
        let d1c = &raw mut DAT_00521f1c;
        let d20 = &raw mut DAT_00521f20;
        let d1b = &raw mut DAT_00521f1b;
        let d640 = &raw mut DAT_00527640;
        let c633 = &raw mut DAT_0051c633;
        let c635 = &raw mut DAT_0051c635;
        let c637 = &raw mut DAT_0051c637;
        let c639 = &raw mut DAT_0051c639;
        let c63b = &raw mut DAT_0051c63b;
        let c63d = &raw mut DAT_0051c63d;
        let c63f = &raw mut DAT_0051c63f;
        let c641 = &raw mut DAT_0051c641;
        let c645 = &raw mut DAT_0051c645;
        let c649 = &raw mut DAT_0051c649;
        let c64d = &raw mut DAT_0051c64d;
        let c651 = &raw mut DAT_0051c651;
        let c655 = &raw mut DAT_0051c655;
        let c659 = &raw mut DAT_0051c659;
        let c65b = &raw mut DAT_0051c65b;
        let c65d = &raw mut DAT_0051c65d;
        let c65f = &raw mut DAT_0051c65f;
        let c59e = &raw mut DAT_0051c59e;
        let c5a0 = &raw mut DAT_0051c5a0;
        let c5a2 = &raw mut DAT_0051c5a2;
        let c5a4 = &raw mut DAT_0051c5a4;
        let c5a6 = &raw mut DAT_0051c5a6;
        let c5a8 = &raw mut DAT_0051c5a8;
        let c5aa = &raw mut DAT_0051c5aa;
        let c5ac = &raw mut DAT_0051c5ac;
        let c5ae = &raw mut DAT_0051c5ae;
        let c5b2 = &raw mut DAT_0051c5b2;
        let c5b4 = &raw mut DAT_0051c5b4;
        let c5b6 = &raw mut DAT_0051c5b6;
        let c5b8 = &raw mut DAT_0051c5b8;
        let c5b9 = &raw mut DAT_0051c5b9;

        let u_stack_2c: u16 = param_2 as u16;
        let s2c: u32 = u_stack_2c as u32;
        let mut local_20: i32 = (param_2 as u32).wrapping_mul(0x1a) as i32;
        let mut lvar = FUN_004adf10(param_2 as u32);
        st32(at(d1c, local_20 as u32), lvar as i32);
        st32(at(d20, local_20 as u32), (lvar as i32) >> 1);
        if param_1 == 0xff {
            return;
        }
        lvar = FUN_004ae0e0(s2c);
        let mut u_var16: u32 = lvar as u32;
        lvar = FUN_004ae120(s2c);
        let mut u_var15: u32 = lvar as u32;
        lvar = FUN_004ae160(s2c);
        let mut s_var12: i16 = 0;
        let mut u_var7: u32 = lvar as u32;
        while 0x280 < u_var16 || 0x280 < u_var15 || 0x280 < u_var7 {
            u_var16 >>= 1;
            u_var15 >>= 1;
            s_var12 = s_var12.wrapping_add(1);
            u_var7 >>= 1;
        }
        let i2: u32 = (param_1 as u32).wrapping_mul(0x101);
        st16(at(c659, i2), s_var12 as u16);
        st16(at(c65b, i2), u_var16 as u16);
        st16(at(c65f, i2), u_var15 as u16);
        st16(at(c65d, i2), u_var7 as u16);
        let mut tmps: i16 = at(c63d, i2).cast::<i16>().read_unaligned();
        if tmps == 0 {
            lvar = FUN_004ae020(s2c);
            st16(at(c63d, i2), lvar as u16);
            lvar = FUN_004ae020(s2c);
            st16(at(c63f, i2), lvar as u16);
        }
        tmps = at(c639, i2).cast::<i16>().read_unaligned();
        if tmps == 0 {
            lvar = FUN_004ae0b0(s2c);
            st16(at(c639, i2), lvar as u16);
            lvar = FUN_004ae0b0(s2c);
            st16(at(c63b, i2), lvar as u16);
        }
        let local_c: i32 = FUN_004adc40(s2c);
        s_var12 = 0;
        let mut local_14: u32 = 0;
        let mut local_18: i32 = 0;
        let mut local_28: i32 = 0;
        let mut local_24: i32 = 0;
        let mut local_10: u32;
        let mut local_1c: u32;
        let mut u_stack_32: u16;
        if 0 < local_c {
            loop {
                local_10 = s_var12 as u16 as u32;
                local_1c = FUN_004af250(s2c, local_10);
                if local_1c != 0 {
                    let i_var8 = FUN_004ae2a0(s2c, local_10);
                    u_stack_32 = 0;
                    if 0 < local_1c as i32 {
                        loop {
                            let mut handled = false;
                            FUN_004af4c0(
                                s2c,
                                local_10,
                                u_stack_32 as u32,
                                &raw mut local_14,
                                &raw mut local_18,
                                &raw mut local_28,
                                &raw mut local_24,
                            );
                            let u_var5 = local_24 as u16;
                            let u_var10 = local_28 as u16;
                            match local_14 {
                                0x19 => {
                                    st32(at(c641, i2), local_18);
                                    st32(at(c645, i2), local_24);
                                    st32(at(c649, i2), local_28);
                                    handled = true;
                                }
                                0x1a => {
                                    st32(at(c64d, i2), local_18);
                                    st32(at(c651, i2), local_24);
                                    st32(at(c655, i2), local_28);
                                    handled = true;
                                }
                                0x1b => {
                                    st16(at(c63b, i2), u_var5);
                                    st16(at(c637, i2), u_var10);
                                    handled = true;
                                }
                                0x1c => {
                                    st16(at(c639, i2), u_var5);
                                    st16(at(c637, i2), u_var10);
                                    handled = true;
                                }
                                0x1d => {
                                    st16(at(c63f, i2), u_var5);
                                    st16(at(c637, i2), u_var10);
                                    handled = true;
                                }
                                0x1e => {
                                    st16(at(c63d, i2), u_var5);
                                    st16(at(c637, i2), u_var10);
                                    handled = true;
                                }
                                0x1f => {
                                    st16(at(c635, i2), u_var5);
                                    st16(at(c633, i2), u_var10);
                                    handled = true;
                                }
                                _ => {}
                            }
                            if !handled {
                                u_var16 = 0;
                                let mut c_var4: u8 = (local_14 as u8).wrapping_sub(0x78);
                                let mut u_var11: u16;
                                loop {
                                    u_var11 = u_var16 as u16;
                                    if *at(c59e, i2.wrapping_add(u_var16)) == c_var4 {
                                        break;
                                    }
                                    u_var11 = u_var11.wrapping_add(1);
                                    u_var16 = u_var11 as u32;
                                    if !(u_var11 < 2) {
                                        break;
                                    }
                                }
                                if 1 < u_var11 {
                                    u_var11 = 0;
                                    loop {
                                        if *at(c5a8, i2.wrapping_add(u_var11 as u32)) == c_var4 {
                                            break;
                                        }
                                        u_var11 = u_var11.wrapping_add(1);
                                        if !(u_var11 < 2) {
                                            break;
                                        }
                                    }
                                    if 1 < u_var11 {
                                        let t = *at(d640, local_14);
                                        if t == 1 {
                                            u_var16 = 0;
                                            loop {
                                                if *at(c59e, i2.wrapping_add(u_var16)) == 0 {
                                                    break;
                                                }
                                                u_var11 = (u_var16 as i16).wrapping_add(1) as u16;
                                                u_var16 = u_var11 as u32;
                                                if !(u_var11 < 2) {
                                                    break;
                                                }
                                            }
                                            if (u_var16 as u16) < 2 {
                                                *at(c59e, i2.wrapping_add(u_var16)) = c_var4;
                                                let i_var1 = u_var16.wrapping_add(i2);
                                                let cond =
                                                    if i_var8 == 4 || i_var8 == 0x15 || i_var8 == 5
                                                    {
                                                        true
                                                    } else {
                                                        c_var4 = *at(d1b, local_20 as u32);
                                                        c_var4 == 3 || c_var4 == 5 || c_var4 == 4
                                                    };
                                                if cond {
                                                    *at(c5a6, i_var1) = 2;
                                                } else if local_14 == 5 || local_14 == 0x10 {
                                                    *at(c5a6, i_var1) = 1;
                                                } else {
                                                    *at(c5a6, i_var1) = 0;
                                                }
                                            }
                                        } else if t == 2 {
                                            u_var16 = 0;
                                            loop {
                                                if *at(c5a8, i2.wrapping_add(u_var16)) == 0 {
                                                    break;
                                                }
                                                u_var11 = (u_var16 as i16).wrapping_add(1) as u16;
                                                u_var16 = u_var11 as u32;
                                                if !(u_var11 < 2) {
                                                    break;
                                                }
                                            }
                                            if (u_var16 as u16) < 2 {
                                                *at(c5a8, i2.wrapping_add(u_var16)) = c_var4;
                                            }
                                        }
                                    }
                                }
                            }
                            u_stack_32 = u_stack_32.wrapping_add(1);
                            if !((u_stack_32 as u32 as i32) < local_1c as i32) {
                                break;
                            }
                        }
                    }
                }
                s_var12 = s_var12.wrapping_add(1);
                if !((s_var12 as u16 as u32 as i32) < local_c) {
                    break;
                }
            }
        }

        let mut u_var11: u16 = 0;
        let mut b_var13: u8 = 0;
        let mut b_var14: u8;
        loop {
            b_var14 = b_var13;
            if b_var13 == 0x10 {
                let i_var8 = (u_var11 as u32).wrapping_add(i2);
                *at(c59e, i_var8) = 0;
                *at(c5a6, i_var8) = 0;
            } else {
                let local_4: u32 = (u_var11 as u32).wrapping_add(i2);
                s_var12 = 0;
                let c_var4: u8 = *at(c59e, local_4);
                if 0 < local_c {
                    loop {
                        let u_stack_2e: u8 = s_var12 as u8;
                        let local_8: u32 = s_var12 as u16 as u32;
                        local_1c = FUN_004af250(s2c, local_8);
                        if local_1c != 0 {
                            let mut u_var6: u16 = 0xff;
                            local_20 = FUN_004ae2a0(s2c, local_8);
                            u_stack_32 = 0;
                            if 0 < local_1c as i32 {
                                local_10 = c_var4.wrapping_add(0x78) as u32;
                                loop {
                                    FUN_004af4c0(
                                        s2c,
                                        local_8,
                                        u_stack_32 as u32,
                                        &raw mut local_14,
                                        &raw mut local_18,
                                        &raw mut local_28,
                                        &raw mut local_24,
                                    );
                                    if local_10 == local_14 {
                                        if u_var6 == 0xff {
                                            if param_2 == 0x35 {
                                                local_18 >>= 1;
                                                local_28 >>= 1;
                                                local_24 >>= 1;
                                            }
                                            let off =
                                                (b_var14 as u32).wrapping_mul(8).wrapping_add(i2);
                                            st16(at(c5b2, off), local_18 as u16);
                                            st16(at(c5b6, off), local_28 as u16);
                                            st16(at(c5b4, off), local_24 as u16);
                                            *at(c5b9, off) = u_stack_2e;
                                            *at(c5b8, off) = 0xff;
                                            if local_20 == 4 || local_20 == 0x15 {
                                                u_var6 = b_var14 as u16;
                                            }
                                            b_var14 = b_var14.wrapping_add(1);
                                            if b_var14 == 0x10 {
                                                break;
                                            }
                                        } else {
                                            let u_var9 =
                                                FUN_004af2d0(s2c, local_8, u_stack_32 as u32);
                                            *at(
                                                c5b8,
                                                (u_var6 as u32).wrapping_mul(8).wrapping_add(i2),
                                            ) = u_var9 as u8;
                                            u_var6 = 0xff;
                                        }
                                    }
                                    u_stack_32 = u_stack_32.wrapping_add(1);
                                    if !((u_stack_32 as u32 as i32) < local_1c as i32) {
                                        break;
                                    }
                                }
                            }
                            if b_var14 == 0x10 {
                                break;
                            }
                        }
                        s_var12 = s_var12.wrapping_add(1);
                        if !((s_var12 as u16 as u32 as i32) < local_c) {
                            break;
                        }
                    }
                }
                if b_var13 != b_var14 {
                    *at(c5a0, local_4) = b_var13;
                    *at(c5a2, local_4) = b_var14.wrapping_sub(1);
                    *at(c5a4, local_4) = b_var14.wrapping_sub(b_var13);
                }
            }
            b_var13 = b_var14;
            u_var11 = u_var11.wrapping_add(1);
            if !(u_var11 < 2) {
                break;
            }
        }

        u_var11 = 0;
        loop {
            b_var14 = b_var13;
            if b_var13 == 0x10 {
                *at(c5a8, i2.wrapping_add(u_var11 as u32)) = 0;
            } else {
                local_20 = (u_var11 as u32).wrapping_add(i2) as i32;
                s_var12 = 0;
                let c_var4: u8 = *at(c5a8, local_20 as u32);
                'blk: {
                    if 0 < local_c {
                        loop {
                            let u_stack_2e: u8 = s_var12 as u8;
                            let u_var16b: u32 = s_var12 as u16 as u32;
                            local_1c = FUN_004af250(s2c, u_var16b);
                            if local_1c != 0 {
                                let mut u_var15b: u32 = 0;
                                FUN_004ae2a0(s2c, u_var16b);
                                if 0 < local_1c as i32 {
                                    local_10 = c_var4.wrapping_add(0x78) as u32;
                                    loop {
                                        FUN_004af4c0(
                                            s2c,
                                            u_var16b,
                                            u_var15b,
                                            &raw mut local_14,
                                            &raw mut local_18,
                                            &raw mut local_28,
                                            &raw mut local_24,
                                        );
                                        if local_10 == local_14 {
                                            if param_2 == 0x35 {
                                                local_18 >>= 1;
                                                local_28 >>= 1;
                                                local_24 >>= 1;
                                            }
                                            let u7 = b_var14 as u32;
                                            b_var14 = b_var14.wrapping_add(1);
                                            let off = u7.wrapping_mul(8).wrapping_add(i2);
                                            st16(at(c5b2, off), local_18 as u16);
                                            st16(at(c5b6, off), local_28 as u16);
                                            st16(at(c5b4, off), local_24 as u16);
                                            *at(c5b9, off) = u_stack_2e;
                                            if b_var14 == 0x10 {
                                                break 'blk;
                                            }
                                        }
                                        u_var15b = (u_var15b as i16).wrapping_add(1) as u16 as u32;
                                        if !((u_var15b as i32) < local_1c as i32) {
                                            break;
                                        }
                                    }
                                }
                                if b_var14 == 0x10 {
                                    break;
                                }
                            }
                            s_var12 = s_var12.wrapping_add(1);
                            if !((s_var12 as u16 as u32 as i32) < local_c) {
                                break;
                            }
                        }
                    }
                }
                if b_var13 != b_var14 {
                    *at(c5aa, local_20 as u32) = b_var13;
                    *at(c5ac, local_20 as u32) = b_var14.wrapping_sub(1);
                    *at(c5ae, local_20 as u32) = b_var14.wrapping_sub(b_var13);
                }
            }
            b_var13 = b_var14;
            u_var11 = u_var11.wrapping_add(1);
            if !(u_var11 < 2) {
                break;
            }
        }
    }
}
