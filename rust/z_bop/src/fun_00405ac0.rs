// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: i32;
    static mut DAT_00a08104: i32;
    static mut DAT_009a8e18: i32;
    static mut DAT_009e9640: i32;
    static mut DAT_009ecc34: i32;
    static mut DAT_0051c5a0: [u8; 1];
    static mut DAT_0051c5a2: [u8; 1];
    static mut DAT_0051c59e: [u8; 1];
    static mut DAT_009d8c84: [i8; 1];
    static mut DAT_009d9155: [i8; 1];
    static mut DAT_009e968a: [i8; 1];
    static mut DAT_009e96f9: [i8; 1];
    static mut DAT_009e96f5: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00450650(a: i32, b: i32);
    fn FUN_004065d0(a: i32, b: u32, c: u32) -> u32;
    fn FUN_004269e0(a: i32) -> u32;
    fn FUN_0042e720(a: u32, b: i32);
}

#[inline(always)]
unsafe fn rd<T>(a: i32) -> T {
    // SAFETY: the address is the game's own memory, as in the original.
    unsafe { (a as u32 as usize as *const T).read_unaligned() }
}

#[inline(always)]
unsafe fn wr<T>(a: i32, v: T) {
    // SAFETY: the address is the game's own memory, as in the original.
    unsafe { (a as u32 as usize as *mut T).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00405ac0(param_1: i32, param_2: i32) {
    // SAFETY: all addresses are the game's own memory and functions; reads
    // and writes happen in the original's order.
    unsafe {
        let a0: i32 = (&raw mut DAT_0051c5a0) as usize as i32;
        let a2: i32 = (&raw mut DAT_0051c5a2) as usize as i32;
        let a59e: i32 = (&raw mut DAT_0051c59e) as usize as i32;
        let g8c84: i32 = (&raw mut DAT_009d8c84) as usize as i32;
        let g9155: i32 = (&raw mut DAT_009d9155) as usize as i32;
        let g968a: i32 = (&raw mut DAT_009e968a) as usize as i32;
        let g96f9: i32 = (&raw mut DAT_009e96f9) as usize as i32;
        let g96f5: i32 = (&raw mut DAT_009e96f5) as usize as i32;

        let mut local_16: u32 = 0;
        let mut local_12: u16 = 0;
        let mut u_stack_e: u16 = 0;
        let mut u_stack_c: u8 = 0;
        let mut u_stack_b: u8 = 0;

        let mut i_var9: i32 = DAT_009a1fe8.wrapping_add(param_1.wrapping_mul(0x23));
        let i_var6: i32 = rd::<i32>(i_var9.wrapping_add(0x1b));
        let mut i_var13: i32 = rd::<i32>(i_var9.wrapping_add(0x1f));
        i_var9 = rd::<i32>(i_var13.wrapping_add(0xa9));
        let b_var3: u8 = rd::<u8>(i_var9.wrapping_add(4));
        let u_var7: u16 = b_var3 as u16;
        DAT_00a08104 = i_var9;

        if rd::<i8>(i_var9.wrapping_add(0x29)) == 0 {
            let mut u_var16: u32 = 0;
            i_var13 = i_var9.wrapping_add(param_2);
            let c_var12: i8 = rd::<i8>(i_var13.wrapping_add(0x220));
            if c_var12 == 1 {
                let i_var17: i32 = (u_var7 as i32).wrapping_mul(0x101).wrapping_add(param_2);
                let cur: u8 = rd::<u8>(i_var13.wrapping_add(0x224));
                if cur < rd::<u8>(a0.wrapping_add(i_var17))
                    || rd::<u8>(a2.wrapping_add(i_var17)) < rd::<u8>(i_var13.wrapping_add(0x224))
                {
                    wr::<u8>(
                        i_var13.wrapping_add(0x224),
                        rd::<u8>(a0.wrapping_add(i_var17)),
                    );
                }
                let p = DAT_00a08104.wrapping_add(0x224).wrapping_add(param_2);
                let b_var4: u8 = rd::<u8>(p);
                wr::<u8>(p, b_var4.wrapping_add(1));
                local_12 = b_var4 as u16;
                let p1 = DAT_00a08104.wrapping_add(0x224).wrapping_add(param_2);
                if rd::<u8>(a2.wrapping_add(i_var17)) < rd::<u8>(p1) {
                    wr::<u8>(p1, rd::<u8>(a0.wrapping_add(i_var17)));
                }
                local_16 = (local_16 & 0xffff) | (1u32 << 16);
                u_stack_c = 1;
                u_stack_b = 0;
                u_stack_e = local_12;
            } else if c_var12 == 2 {
                let pbvar14 = i_var13.wrapping_add(0x224);
                i_var13 = (u_var7 as i32).wrapping_mul(0x101).wrapping_add(param_2);
                let pbvar1 = a0.wrapping_add(i_var13);
                if rd::<u8>(pbvar14) < rd::<u8>(a0.wrapping_add(i_var13))
                    || rd::<u8>(a2.wrapping_add(i_var13)) < rd::<u8>(pbvar14)
                {
                    wr::<u8>(pbvar14, rd::<u8>(pbvar1));
                }
                let p = DAT_00a08104.wrapping_add(0x224).wrapping_add(param_2);
                let b_var4: u8 = rd::<u8>(p);
                wr::<u8>(p, b_var4 ^ 1);
                local_12 = b_var4 as u16;
                let p14 = DAT_00a08104.wrapping_add(0x224).wrapping_add(param_2);
                if rd::<u8>(a2.wrapping_add(i_var13)) < rd::<u8>(p14) {
                    wr::<u8>(p14, rd::<u8>(pbvar1));
                }
                u_stack_e = rd::<u8>(a2.wrapping_add(i_var13)) as u16;
                u_stack_c = 2;
                u_stack_b = 0;
                let hi = ((u_stack_e as i32)
                    .wrapping_sub(rd::<u8>(pbvar1) as i32)
                    .wrapping_add(1)
                    / 2) as u16;
                local_16 = (local_16 & 0xffff) | ((hi as u32) << 16);
            } else if c_var12 == 3 {
                let idx = (param_2 as u32).wrapping_add((u_var7 as u32).wrapping_mul(0x101)) as i32;
                local_12 = rd::<u8>(a0.wrapping_add(idx)) as u16;
                u_stack_c = 1;
                u_stack_b = 0;
                u_stack_e = rd::<u8>(a2.wrapping_add(idx)) as u16;
                let hi = (u_stack_e as u32)
                    .wrapping_sub(local_12 as u32)
                    .wrapping_add(1) as u16;
                local_16 = (local_16 & 0xffff) | ((hi as u32) << 16);
            }

            local_16 = (local_16 & 0xffff0000) | (local_12 as u32);
            let mut u_var8: u32 = local_16;
            let mut u_var5: u16 = local_12;
            while u_var5 <= u_stack_e {
                let u_var18: u32 = u_var8 & 0xffff;
                i_var13 = DAT_00a08104.wrapping_add(u_var18.wrapping_mul(4) as i32);
                local_16 = (local_16 & 0xffff) | (u_var8 & 0xffff0000);
                if rd::<i8>(i_var13.wrapping_add(0x370)) != 0
                    && 0 < rd::<i8>(i_var13.wrapping_add(0x371))
                {
                    let idx =
                        (param_2 as u32).wrapping_add((u_var7 as u32).wrapping_mul(0x101)) as i32;
                    local_12 = rd::<u8>(a59e.wrapping_add(idx)) as u16;
                    if 0x3f < rd::<i8>(i_var13.wrapping_add(0x371)) {
                        local_12 = local_12.wrapping_add(1);
                    }
                    let u_var10: u32 = FUN_004065d0(param_1, u_var18, local_12 as u32);
                    if u_var10 != 0xffffffff {
                        let o = DAT_009a1fe8
                            .wrapping_add(0x16)
                            .wrapping_add(param_1.wrapping_mul(0x23));
                        i_var13 = (rd::<u8>(o) as i32).wrapping_mul(0x566);
                        if rd::<i8>(g8c84.wrapping_add(i_var13)) != 0x15
                            && rd::<i8>(g9155.wrapping_add(i_var13)) != 0x15
                        {
                            let q = DAT_00a08104
                                .wrapping_add(0x371)
                                .wrapping_add(u_var18.wrapping_mul(4) as i32);
                            let c_var12: i8;
                            if i_var6 == -1 {
                                c_var12 = rd::<i8>(q).wrapping_sub(1);
                            } else {
                                let u_var11 = FUN_004269e0(5);
                                if (u_var11 as u16) == u_var7 || {
                                    let u_var11b = FUN_004269e0(7);
                                    (u_var11b as u16) == u_var7
                                } {
                                    let q2 = DAT_00a08104
                                        .wrapping_add(0x371)
                                        .wrapping_add(u_var18.wrapping_mul(4) as i32);
                                    c_var12 = rd::<i8>(q2).wrapping_sub(3);
                                } else {
                                    let q2 = DAT_00a08104
                                        .wrapping_add(0x371)
                                        .wrapping_add(u_var18.wrapping_mul(4) as i32);
                                    c_var12 = rd::<i8>(q2).wrapping_sub(4);
                                }
                            }
                            wr::<i8>(
                                DAT_00a08104
                                    .wrapping_add(0x371)
                                    .wrapping_add(u_var18.wrapping_mul(4) as i32),
                                c_var12,
                            );
                        }
                        let hi = (local_16 >> 16) as u16;
                        let call = if hi < 2 {
                            (u_var16 as u16) < 2
                        } else {
                            (u_var16 & 1) == 0
                        };
                        if call {
                            FUN_0042e720(u_var10, i_var6);
                        }
                        let q = DAT_00a08104
                            .wrapping_add(0x371)
                            .wrapping_add(u_var18.wrapping_mul(4) as i32);
                        if rd::<i8>(q) < 0 {
                            wr::<i8>(q, 0);
                        }
                        i_var13 = u_var10.wrapping_sub(DAT_009e9640 as u32).wrapping_mul(10) as i32;
                        if i_var6 == -1 {
                            wr::<i16>(
                                DAT_009a8e18.wrapping_add(4).wrapping_add(i_var13),
                                rd::<i16>(i_var9.wrapping_add(0x42)),
                            );
                            u_var5 = rd::<u16>(i_var9.wrapping_add(0x42));
                            if u_var5 == 0xffff || 0x7fff < u_var5 {
                                wr::<i16>(DAT_009a8e18.wrapping_add(6).wrapping_add(i_var13), 0);
                            } else {
                                let src = (u_var5 as u32)
                                    .wrapping_mul(0x23)
                                    .wrapping_add(DAT_009a1fe8 as u32)
                                    as i32;
                                wr::<u16>(
                                    DAT_009a8e18.wrapping_add(6).wrapping_add(i_var13),
                                    rd::<u16>(src),
                                );
                            }
                        } else {
                            let base =
                                g968a.wrapping_add((i_var6 as u32).wrapping_mul(0x5bd) as i32);
                            wr::<u16>(
                                DAT_009a8e18.wrapping_add(4).wrapping_add(i_var13),
                                rd::<u16>(base),
                            );
                            u_var5 = rd::<u16>(base);
                            if u_var5 == 0xffff {
                                wr::<i16>(DAT_009a8e18.wrapping_add(6).wrapping_add(i_var13), 0);
                            } else {
                                let src = (u_var5 as u32)
                                    .wrapping_mul(0x23)
                                    .wrapping_add(DAT_009a1fe8 as u32)
                                    as i32;
                                wr::<u16>(
                                    DAT_009a8e18.wrapping_add(6).wrapping_add(i_var13),
                                    rd::<u16>(src),
                                );
                            }
                        }
                        u_var16 = (u_var16 as u16).wrapping_add(1) as u32;
                        wr::<i8>(DAT_009a8e18.wrapping_add(i_var13), i_var6 as i8);
                    }
                }
                local_16 = (local_16 & 0xffff0000) | (u_var8 & 0xffff);
                if ((local_16 >> 16) as u16).wrapping_sub(1) == 0 {
                    break;
                }
                u_var5 = (local_16 as u16)
                    .wrapping_add((((u_stack_b as u32) << 8) | (u_stack_c as u32)) as u16);
                local_16 =
                    ((((local_16 >> 16) as u16).wrapping_sub(1) as u32) << 16) | (u_var5 as u32);
                u_var8 = local_16;
            }

            let s_var15: i16 = u_var16 as i16;
            if local_12 == 0x8d || local_12 == 0x8e {
                let p = DAT_00a08104.wrapping_add(0x250);
                wr::<i16>(p, rd::<i16>(p).wrapping_add(s_var15));
                if i_var6 != -1 {
                    let a = g96f9.wrapping_add((i_var6 as u32).wrapping_mul(0x5bd) as i32);
                    let ss: i16 = rd::<i16>(a).wrapping_add(s_var15);
                    wr::<i16>(a, ss);
                }
            } else {
                let p = DAT_00a08104.wrapping_add(0x24c);
                wr::<i16>(p, rd::<i16>(p).wrapping_add(s_var15));
                if i_var6 != -1 {
                    let a = g96f5.wrapping_add((i_var6 as u32).wrapping_mul(0x5bd) as i32);
                    let ss: i16 = rd::<i16>(a).wrapping_add(s_var15);
                    wr::<i16>(a, ss);
                }
            }
            let p = DAT_00a08104
                .wrapping_add(0x226)
                .wrapping_add((param_2 as u32).wrapping_mul(2) as i32);
            let add = (s_var15 as i32).wrapping_mul(0x2f).wrapping_add(2) as i16;
            wr::<i16>(p, rd::<i16>(p).wrapping_add(add));
            let p = DAT_00a08104
                .wrapping_add(0x22a)
                .wrapping_add((param_2 as u32).wrapping_mul(4) as i32);
            let mut ui: u32 = rd::<u32>(p);
            ui = ui.wrapping_add(u_var16.wrapping_mul(0x2f)).wrapping_add(2);
            wr::<u32>(p, ui);
        } else if i_var6 == DAT_009ecc34 {
            FUN_00450650(0x82, DAT_009ecc34);
            return;
        }
    }
}
