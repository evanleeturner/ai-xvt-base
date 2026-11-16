// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006a884a: [u8; 4];
    static mut DAT_006a8858: i32;
    static mut DAT_006a8866: [u8; 4];
    static mut DAT_006a886c: i32;
    static mut DAT_006a8874: i32;
    static mut DAT_006a887e: [u8; 4];
    static mut DAT_006d4984: u16;
    static mut DAT_0060267c: u32;
    static mut DAT_00602688: *mut f32;
    static mut DAT_00602698: *mut f32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004859c0(param_1: i32, param_2: *mut u32, param_3: *mut f32, param_4: *mut f32) -> u32;
    fn FUN_00485c90(param_1: *mut u32, param_2: *mut f32, param_3: *mut f32) -> u32;
    fn FUN_004853c0(param_1: i32);
}

#[inline(always)]
unsafe fn rd(a: u32) -> u32 {
    // SAFETY: the address is the game's own memory, read as the original does.
    unsafe { (a as usize as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn wr(a: u32, v: u32) {
    // SAFETY: the address is the game's own memory, written as the original does.
    unsafe { (a as usize as *mut u32).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00484fd0(param_1: i32) {
    let p = param_1 as u32;
    // SAFETY: all globals and pointers belong to the game; accesses mirror the original.
    unsafe {
        let i_var7 = DAT_006a886c as u32;
        let i_var1 =
            (DAT_006a8858 as u32).wrapping_add(rd(p.wrapping_add(0xbc)).wrapping_mul(0x18));
        let mut i_var8: u32 = 0;
        let g884a = (&raw mut DAT_006a884a).cast::<u32>().read_unaligned();
        let mut local_10: u32 = rd(p.wrapping_add(0xb8))
            .wrapping_mul(0x70)
            .wrapping_add(g884a);
        wr(p.wrapping_add(0xc0), DAT_006a886c as u32);
        let mut i_var6: u32 = 0;
        wr(p.wrapping_add(0xcc), 0);
        let g8866 = (&raw mut DAT_006a8866).cast::<u32>().read_unaligned();
        let p_var2: u32 = g8866.wrapping_add(i_var7.wrapping_mul(0x1c));
        let mut g887e = (&raw mut DAT_006a887e).cast::<u32>().read_unaligned();
        if 0 < rd(p.wrapping_add(0x90)) as i32 {
            loop {
                i_var8 = i_var8.wrapping_add(4);
                wr(g887e.wrapping_sub(4).wrapping_add(i_var8), 0xffff_ffff);
                i_var6 = i_var6.wrapping_add(1);
                if !((i_var6 as i32) < rd(p.wrapping_add(0x90)) as i32) {
                    break;
                }
            }
        }
        let mut local_4: i32 = 0;
        let mut p_var9: u32 = p_var2;
        if 0 < rd(p.wrapping_add(0xc4)) as i32 {
            loop {
                let i_var7 = rd(local_10)
                    .wrapping_mul(0x40)
                    .wrapping_add(rd(p.wrapping_add(0xa0)));
                let i_var12: u32;
                if rd(local_10.wrapping_add(12)) as i32 == -1 {
                    wr(local_10.wrapping_add(12), DAT_006d4984 as u32);
                    DAT_00602688 = core::ptr::null_mut();
                    DAT_00602698 = core::ptr::null_mut();
                    let mut local_18: u32 =
                        4u32.wrapping_sub((rd(i_var7.wrapping_add(0x1c)) == 0xffff_ffff) as u32);
                    i_var6 = 0;
                    i_var8 = i_var7
                        .wrapping_add(0x10)
                        .wrapping_add(local_18.wrapping_mul(4));
                    let mut i_var10: u32 = 0;
                    let mut i_var11: u32;
                    loop {
                        let i_var3 = rd(i_var8.wrapping_sub(4));
                        DAT_0060267c = 0;
                        local_18 = local_18.wrapping_sub(1);
                        g887e = (&raw mut DAT_006a887e).cast::<u32>().read_unaligned();
                        let i_var4 = rd(g887e.wrapping_add(i_var3.wrapping_mul(4)));
                        i_var11 = i_var10;
                        if i_var4 == 0xffff_ffff {
                            let a = i_var1.wrapping_add(
                                rd((DAT_006a8874 as u32)
                                    .wrapping_add(rd(i_var8.wrapping_sub(0x14)).wrapping_mul(4)))
                                .wrapping_mul(0x18),
                            );
                            let b = i_var1.wrapping_add(
                                rd((DAT_006a8874 as u32).wrapping_add(
                                    rd(i_var7.wrapping_add(i_var6.wrapping_mul(4))).wrapping_mul(4),
                                ))
                                .wrapping_mul(0x18),
                            );
                            let u_var5 = FUN_004859c0(
                                param_1,
                                p_var9 as usize as *mut u32,
                                a as usize as *mut f32,
                                b as usize as *mut f32,
                            );
                            if (u_var5 as i32) < 0 {
                                if DAT_0060267c == 0 {
                                    g887e = (&raw mut DAT_006a887e).cast::<u32>().read_unaligned();
                                    wr(g887e.wrapping_add(i_var3.wrapping_mul(4)), 0xffff_fffe);
                                }
                            } else {
                                wr(
                                    local_10
                                        .wrapping_add(i_var10.wrapping_add(0x14).wrapping_mul(4)),
                                    p_var9,
                                );
                                wr(p_var9.wrapping_add(24), DAT_0060267c);
                                g887e = (&raw mut DAT_006a887e).cast::<u32>().read_unaligned();
                                wr(
                                    g887e.wrapping_add(i_var3.wrapping_mul(4)),
                                    rd(p.wrapping_add(0xcc)),
                                );
                                wr(
                                    p.wrapping_add(0xcc),
                                    rd(p.wrapping_add(0xcc)).wrapping_add(1),
                                );
                                p_var9 = p_var9.wrapping_add(28);
                                i_var11 = i_var10.wrapping_add(1);
                            }
                        } else if i_var4 != 0xffff_fffe {
                            i_var11 = i_var10.wrapping_add(1);
                            wr(
                                local_10.wrapping_add(i_var10.wrapping_add(0x14).wrapping_mul(4)),
                                p_var2.wrapping_add(i_var4.wrapping_mul(7).wrapping_mul(4)),
                            );
                            let idx = rd(g887e.wrapping_add(i_var3.wrapping_mul(4)))
                                .wrapping_mul(7)
                                .wrapping_add(6);
                            let t = rd(p_var2.wrapping_add(idx.wrapping_mul(4)));
                            if t != 0 {
                                DAT_00602698 = DAT_00602688;
                                DAT_00602688 = t as usize as *mut f32;
                            }
                        }
                        i_var6 = local_18;
                        i_var8 = i_var8.wrapping_sub(4);
                        i_var10 = i_var11;
                        if !(0 < local_18 as i32) {
                            break;
                        }
                    }
                    i_var12 = {
                        let mut r = i_var11;
                        if !DAT_00602698.is_null() {
                            let u_var5 = FUN_004859c0(
                                param_1,
                                p_var9 as usize as *mut u32,
                                DAT_00602688,
                                DAT_00602698,
                            );
                            if -1 < u_var5 as i32 {
                                r = i_var11.wrapping_add(1);
                                wr(
                                    local_10
                                        .wrapping_add(i_var11.wrapping_add(0x14).wrapping_mul(4)),
                                    p_var9,
                                );
                                p_var9 = p_var9.wrapping_add(28);
                                wr(
                                    p.wrapping_add(0xcc),
                                    rd(p.wrapping_add(0xcc)).wrapping_add(1),
                                );
                            }
                        }
                        r
                    };
                } else {
                    wr(local_10.wrapping_add(12), DAT_006d4984 as u32);
                    let mut local_18: u32 =
                        4u32.wrapping_sub((rd(i_var7.wrapping_add(0x1c)) == 0xffff_ffff) as u32);
                    i_var6 = 0;
                    i_var8 = i_var7
                        .wrapping_add(0x10)
                        .wrapping_add(local_18.wrapping_mul(4));
                    let mut i_var10: u32 = 0;
                    let mut i_var12b: u32;
                    loop {
                        let i_var3 = rd(i_var8.wrapping_sub(4));
                        local_18 = local_18.wrapping_sub(1);
                        g887e = (&raw mut DAT_006a887e).cast::<u32>().read_unaligned();
                        let i_var4 = rd(g887e.wrapping_add(i_var3.wrapping_mul(4)));
                        i_var12b = i_var10;
                        if i_var4 == 0xffff_ffff {
                            let a = i_var1.wrapping_add(
                                rd((DAT_006a8874 as u32)
                                    .wrapping_add(rd(i_var8.wrapping_sub(0x14)).wrapping_mul(4)))
                                .wrapping_mul(0x18),
                            );
                            let b = i_var1.wrapping_add(
                                rd((DAT_006a8874 as u32).wrapping_add(
                                    rd(i_var7.wrapping_add(i_var6.wrapping_mul(4))).wrapping_mul(4),
                                ))
                                .wrapping_mul(0x18),
                            );
                            let u_var5 = FUN_00485c90(
                                p_var9 as usize as *mut u32,
                                a as usize as *mut f32,
                                b as usize as *mut f32,
                            );
                            if (u_var5 as i32) < 0 {
                                g887e = (&raw mut DAT_006a887e).cast::<u32>().read_unaligned();
                                wr(g887e.wrapping_add(i_var3.wrapping_mul(4)), 0xffff_fffe);
                            } else {
                                wr(
                                    local_10
                                        .wrapping_add(i_var10.wrapping_add(0x14).wrapping_mul(4)),
                                    p_var9,
                                );
                                p_var9 = p_var9.wrapping_add(28);
                                g887e = (&raw mut DAT_006a887e).cast::<u32>().read_unaligned();
                                wr(
                                    g887e.wrapping_add(i_var3.wrapping_mul(4)),
                                    rd(p.wrapping_add(0xcc)),
                                );
                                wr(
                                    p.wrapping_add(0xcc),
                                    rd(p.wrapping_add(0xcc)).wrapping_add(1),
                                );
                                i_var12b = i_var10.wrapping_add(1);
                            }
                        } else if i_var4 != 0xffff_fffe {
                            i_var12b = i_var10.wrapping_add(1);
                            wr(
                                local_10.wrapping_add(i_var10.wrapping_add(0x14).wrapping_mul(4)),
                                p_var2.wrapping_add(i_var4.wrapping_mul(7).wrapping_mul(4)),
                            );
                        }
                        i_var6 = local_18;
                        i_var8 = i_var8.wrapping_sub(4);
                        i_var10 = i_var12b;
                        if !(0 < local_18 as i32) {
                            break;
                        }
                    }
                    i_var12 = i_var12b;
                }
                wr(local_10.wrapping_add(0x19 * 4), i_var12);
                if i_var12 == 0 {
                    wr(local_10.wrapping_add(0x11 * 4), 0);
                    wr(local_10.wrapping_add(0x10 * 4), 0);
                } else {
                    FUN_004853c0(local_10 as i32);
                }
                local_4 = local_4.wrapping_add(1);
                local_10 = local_10.wrapping_add(0x1c * 4);
                if !(local_4 < rd(p.wrapping_add(0xc4)) as i32) {
                    break;
                }
            }
        }
        DAT_006a886c = (DAT_006a886c as u32).wrapping_add(rd(p.wrapping_add(0xcc))) as i32;
    }
}
