// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009ec5f4: u16;
    static mut DAT_009ec5fe: u16;
    static mut DAT_00a07cec: u16;
    static mut DAT_00a07ce8: u32;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1fe8: u32;
    static mut DAT_009a1ff8: u32;
    static mut DAT_009a8c00: u32;
    static mut DAT_009d1308: i32;
    static mut DAT_009d6954: u8;
    static mut DAT_009d8110: u16;
    static mut DAT_009ecc34: i32;
    static mut DAT_009ed22d: u8;
    static mut DAT_00a08104: u32;
    static mut DAT_009d8c88: u8;
    static mut DAT_009d8c89: u8;
    static mut DAT_009fd458: u8;
    static mut DAT_0051c5aa: u8;
    static mut DAT_00a00870: i32;
    static mut DAT_00a00874: i32;
    static mut DAT_009e9681: u8;
    static mut DAT_009e968a: u8;
    static mut DAT_009e9696: u8;
    static mut DAT_009e9697: u8;
    static mut DAT_009e9698: u8;
    static mut DAT_009e9699: u8;
    static mut DAT_009e9c21: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004269e0(param_1: u32) -> u32;
    fn FUN_004034e0(param_1: u32, param_2: u32);
    fn FUN_004836e0(param_1: u32, param_2: u32, param_3: u32);
    fn FUN_00482640(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32) -> u64;
    fn FUN_00450650(param_1: i32, param_2: i32);
    fn FUN_00451930() -> i16;
    fn FUN_0042edc0(param_1: i32, param_2: i32);
    fn FUN_004a8cc0(param_1: u32, param_2: u32);
    fn FUN_00405820(param_1: u32) -> i32;
    fn FUN_004adc40(param_1: u32) -> i32;
    fn FUN_004ae2a0(param_1: u32, param_2: i32) -> i32;
    fn FUN_00405790(param_1: u32, param_2: u32, param_3: i32);
    fn FUN_0042e4d0(param_1: i32, param_2: i32, param_3: i32);
    fn FUN_00405ac0(param_1: u32, param_2: u32);
    fn FUN_004a7900(param_1: u32, param_2: u16, param_3: u16);
    fn FUN_00446c60(param_1: u16);
}

unsafe fn r8(a: u32) -> u8 {
    // SAFETY: the address is the game's own memory, read as the original does.
    unsafe { (a as usize as *const u8).read() }
}
unsafe fn s8(a: u32) -> i8 {
    // SAFETY: the address is the game's own memory, read as the original does.
    unsafe { (a as usize as *const i8).read() }
}
unsafe fn r16(a: u32) -> u16 {
    // SAFETY: the address is the game's own memory, read as the original does.
    unsafe { (a as usize as *const u16).read_unaligned() }
}
unsafe fn r32(a: u32) -> u32 {
    // SAFETY: the address is the game's own memory, read as the original does.
    unsafe { (a as usize as *const u32).read_unaligned() }
}
unsafe fn w8(a: u32, v: u8) {
    // SAFETY: the address is the game's own memory, written as the original does.
    unsafe { (a as usize as *mut u8).write(v) }
}
unsafe fn w16(a: u32, v: u16) {
    // SAFETY: the address is the game's own memory, written as the original does.
    unsafe { (a as usize as *mut u16).write_unaligned(v) }
}
unsafe fn w32(a: u32, v: u32) {
    // SAFETY: the address is the game's own memory, written as the original does.
    unsafe { (a as usize as *mut u32).write_unaligned(v) }
}

fn m16(b: bool) -> u16 {
    if b { 0xffff } else { 0 }
}

macro_rules! h {
    ($o:expr) => {
        DAT_00a08104.wrapping_add($o)
    };
}

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_assignments, unused_variables, unused_mut)]
pub unsafe extern "C" fn FUN_00404710(param_1: i32) {
    // SAFETY: all globals and callees are the game's own; every pointer is
    // dereferenced where and how the original does.
    unsafe {
        let d8c88 = (&raw mut DAT_009d8c88) as usize as u32;
        let d8c89 = (&raw mut DAT_009d8c89) as usize as u32;
        let dfd458 = (&raw mut DAT_009fd458) as usize as u32;
        let d5aa = (&raw mut DAT_0051c5aa) as usize as u32;
        let d870 = (&raw mut DAT_00a00870) as usize as u32;
        let d874 = (&raw mut DAT_00a00874) as usize as u32;
        let d9681 = (&raw mut DAT_009e9681) as usize as u32;
        let d968a = (&raw mut DAT_009e968a) as usize as u32;
        let d9696 = (&raw mut DAT_009e9696) as usize as u32;
        let d9697 = (&raw mut DAT_009e9697) as usize as u32;
        let d9698 = (&raw mut DAT_009e9698) as usize as u32;
        let d9699 = (&raw mut DAT_009e9699) as usize as u32;
        let d9c21 = (&raw mut DAT_009e9c21) as usize as u32;

        let mut c_var2: i8;
        let mut b_var3: u8;
        let mut u_var4: u16;
        let mut i_var5: u32;
        let mut u_var6: u32;
        let mut u_var7: u32;
        let mut i_var8: u32;
        let mut s_var10: i16 = 0;
        let mut i_var11: u32;
        let mut i_var12: u32 = 0;
        let mut pc_var13: u32;
        let mut u_var14: u32;
        let mut s_var15: i16;
        let mut u_var16: u32;
        let mut u_var18: u16;
        let mut i_var19: i32;
        let mut b_var20: bool;
        let b_var21: bool;
        let mut local_e: u16;
        let mut b_stack_c: u16 = 0;

        b_var21 = DAT_009ec5f4 == 0;
        if b_var21 {
            DAT_009ec5f4 = 0xec;
        }
        u_var16 = DAT_00a07cec as u32;
        while (u_var16 as i32) < DAT_009a1fe0 {
            i_var5 = u_var16.wrapping_mul(0x23).wrapping_add(DAT_009a1fe8);
            if r8(i_var5.wrapping_add(3)) != 0 {
                pc_var13 = r32(i_var5.wrapping_add(0x1f));
                if r8(pc_var13) == 0 {
                    let base = r32(pc_var13.wrapping_add(0xa9));
                    u_var18 = 0;
                    loop {
                        u_var4 = u_var18.wrapping_add(1);
                        w32(
                            base.wrapping_add(0x15)
                                .wrapping_add((u_var18 as u32).wrapping_mul(4)),
                            0,
                        );
                        u_var18 = u_var4;
                        if !(u_var4 < 5) {
                            break;
                        }
                    }
                }
            }
            u_var16 = (u_var16 as u16).wrapping_add(1) as u32;
        }

        local_e = DAT_00a07cec;
        while (local_e as i32) < DAT_009a1fe0 {
            'cont: {
                u_var6 = local_e as u32;
                i_var5 = u_var6.wrapping_mul(0x23);
                i_var11 = DAT_009a1fe8.wrapping_add(i_var5);
                u_var16 = r8(i_var11.wrapping_add(3)) as u32;
                if r8(i_var11.wrapping_add(3)) != 0 {
                    pc_var13 = r32(i_var11.wrapping_add(0x1f));
                    if r8(pc_var13) == 0 {
                        'l5184: {
                            if r32(i_var11.wrapping_add(0x1b)) as i32 == -1 {
                                if !b_var21 {
                                    break 'cont;
                                }
                                u_var6 = r32(pc_var13.wrapping_add(0xa9));
                                DAT_00a08104 = u_var6;
                                if r8(i_var11.wrapping_add(2)) == 0 {
                                    w8(u_var6.wrapping_add(0x219), 2);
                                    w8(h!(0x21c), 2);
                                    if (r8(h!(0x1e3)) & 1) != 0 {
                                        u_var16 = r8(DAT_009a1fe8
                                            .wrapping_add(0x16)
                                            .wrapping_add(i_var5))
                                            as u32;
                                        u_var18 = r8(d8c89.wrapping_add(u_var16.wrapping_mul(0x566)))
                                            as u16;
                                        u_var16 = u_var18 as u32;
                                        if r8(u_var6.wrapping_add(0x5e)) == 0x20 {
                                            if u_var18 == 5 {
                                                w8(h!(0x219), 0);
                                            } else if u_var18 == 4 || u_var18 == 3 {
                                                w8(h!(0x219), 1);
                                            } else {
                                                w8(h!(0x219), 2);
                                            }
                                            w8(h!(0x21c), 3);
                                        }
                                        let n = FUN_00405820(local_e as u32);
                                        if (r32(h!(0x211)) as i32) < n {
                                            if r8(h!(0x21c)) == 2 {
                                                w8(h!(0x21c), 4);
                                            }
                                            if (r32(h!(0x211)) as i32) < 1 {
                                                if r8(h!(0x219)) == 2 {
                                                    w8(h!(0x219), 4);
                                                }
                                                if u_var18 < 2 {
                                                    b_stack_c = 0xfa;
                                                } else {
                                                    u_var18 = (m16(u_var18 < 3) & 0xdae4)
                                                        .wrapping_add(10000);
                                                    b_stack_c = u_var18;
                                                }
                                            } else if 1 < u_var18 {
                                                if u_var18 == 5 {
                                                    u_var4 = 1;
                                                } else {
                                                    u_var4 = 3;
                                                    if u_var18 != 4 {
                                                        u_var4 = (m16(u_var18 == 3) & 0xfff8)
                                                            .wrapping_add(0xf);
                                                    }
                                                }
                                                u_var18 =
                                                    m16((DAT_009ed22d as u16 & u_var4) == u_var4)
                                                        & 100;
                                                b_stack_c = u_var18;
                                            } else {
                                                b_stack_c = 0;
                                            }
                                            s_var10 = 0;
                                            u_var16 = 0;
                                            if r8(h!(0x21d)) != 0 {
                                                loop {
                                                    c_var2 = s8(h!(0x371)
                                                        .wrapping_add(u_var16.wrapping_mul(4)));
                                                    if 0 < c_var2 {
                                                        s_var10 =
                                                            s_var10.wrapping_add(c_var2 as i16);
                                                    }
                                                    u_var18 = (u_var16 as u16).wrapping_add(1);
                                                    u_var16 = u_var18 as u32;
                                                    if !(u_var18 < r8(h!(0x21d)) as u16) {
                                                        break;
                                                    }
                                                }
                                            }
                                            u_var6 = 0;
                                            u_var16 = (0u32.wrapping_sub(
                                                (r8(DAT_009a1fe8
                                                    .wrapping_add(3)
                                                    .wrapping_add(i_var5))
                                                    == 0xc)
                                                    as u32,
                                            ) & 0x1c)
                                                .wrapping_add(4);
                                            u_var18 = 0;
                                            while s_var10 != 0 && u_var18 < b_stack_c {
                                                pc_var13 =
                                                    h!(0x371).wrapping_add(u_var6.wrapping_mul(4));
                                                c_var2 = s8(pc_var13);
                                                if 0 < c_var2 {
                                                    s_var10 = s_var10.wrapping_sub(1);
                                                    w8(pc_var13, c_var2.wrapping_sub(1) as u8);
                                                    w32(
                                                        h!(0x211),
                                                        r32(h!(0x211)).wrapping_add(u_var16),
                                                    );
                                                    if n <= r32(h!(0x211)) as i32 {
                                                        s_var10 = 0;
                                                    }
                                                }
                                                u_var4 = (u_var6 as u16).wrapping_add(1);
                                                u_var6 = u_var4 as u32;
                                                if (r8(h!(0x21d)) as u16) <= u_var4 {
                                                    u_var6 = 0;
                                                }
                                                u_var18 = u_var18.wrapping_add(1);
                                            }
                                        }
                                    }
                                    pc_var13 = h!(0x21c);
                                    if r8(pc_var13) == 2 {
                                        s_var10 = 0;
                                        u_var18 = 0;
                                        u_var16 &= 0xffff0000;
                                        if r8(h!(0x21d)) != 0 {
                                            loop {
                                                i_var11 = DAT_00a08104.wrapping_add(
                                                    (u_var16 & 0xffff).wrapping_mul(4),
                                                );
                                                if r8(i_var11.wrapping_add(0x370)) != 0 {
                                                    s_var10 = s_var10
                                                        .wrapping_add(
                                                            s8(i_var11.wrapping_add(0x371)) as i16,
                                                        );
                                                    u_var18 = u_var18.wrapping_add(1);
                                                }
                                                u_var4 = (u_var16 as u16).wrapping_add(1);
                                                u_var16 = (u_var16 & 0xffff0000) | u_var4 as u32;
                                                if !(u_var4 < r8(h!(0x21d)) as u16) {
                                                    break;
                                                }
                                            }
                                        }
                                        if u_var18 != 0 {
                                            s_var10 =
                                                ((s_var10 as i32) / (u_var18 as u32 as i32)) as i16;
                                            if s_var10 < 0x20 {
                                                s_var10 = 0x14;
                                                w8(pc_var13, 4);
                                                break 'l5184;
                                            }
                                            w8(pc_var13, ((s_var10 < 0x60) as u8).wrapping_add(2));
                                        }
                                    }
                                    s_var10 = 0x14;
                                } else {
                                    w8(u_var6.wrapping_add(0x219), 2);
                                    s_var10 = 0;
                                    w8(h!(0x21c), 2);
                                    if DAT_009d6954 == 2
                                        && r8(DAT_009a1fe8.wrapping_add(i_var5).wrapping_add(2))
                                            == 4
                                    {
                                        if r32(h!(0xc3)) != 0 {
                                            b_var3 = r8(DAT_009a1fe8
                                                .wrapping_add(i_var5)
                                                .wrapping_add(3));
                                            if b_var3 == 0x33
                                                || b_var3 == 0x34
                                                || b_var3 == 0x35
                                                || b_var3 == 0x36
                                            {
                                                s_var10 = 0;
                                                let n: i32;
                                                if b_var3 < 0x49 {
                                                    n = r32(d870.wrapping_add(
                                                        (b_var3 as u32)
                                                            .wrapping_mul(0x65)
                                                            .wrapping_mul(4),
                                                    ))
                                                        as i32;
                                                } else {
                                                    n = FUN_004adc40(b_var3 as u32);
                                                }
                                                i_var19 = 0;
                                                if 0 < n {
                                                    loop {
                                                        b_var3 = r8(DAT_009a1fe8
                                                            .wrapping_add(3)
                                                            .wrapping_add(i_var5));
                                                        u_var6 = b_var3 as u32;
                                                        let v8: i32;
                                                        if b_var3 < 0x49 {
                                                            if i_var19 < 0 {
                                                                v8 = 0;
                                                            } else {
                                                                let mut k = i_var19;
                                                                let cnt = r32(d870.wrapping_add(
                                                                    u_var6
                                                                        .wrapping_mul(0x65)
                                                                        .wrapping_mul(4),
                                                                ))
                                                                    as i32;
                                                                if cnt <= i_var19 {
                                                                    k = cnt.wrapping_sub(1);
                                                                }
                                                                v8 = r32(d874.wrapping_add(
                                                                    u_var6
                                                                        .wrapping_mul(0x65)
                                                                        .wrapping_add(k as u32)
                                                                        .wrapping_mul(4),
                                                                ))
                                                                    as i32;
                                                            }
                                                        } else {
                                                            v8 = FUN_004ae2a0(u_var6, i_var19);
                                                        }
                                                        i_var8 = v8 as u32;
                                                        if v8 == 8
                                                            && r8(h!(0x33c)
                                                                .wrapping_add(i_var19 as u32))
                                                                != 0
                                                        {
                                                            s_var10 = s_var10.wrapping_add(1);
                                                        }
                                                        i_var19 = i_var19.wrapping_add(1);
                                                        if !(i_var19 < n) {
                                                            break;
                                                        }
                                                    }
                                                }
                                            } else {
                                                s_var10 = 1;
                                            }
                                        }
                                        w8(h!(0x219), 3);
                                    }
                                    s_var10 = s_var10.wrapping_mul(5);
                                }
                            } else {
                                DAT_00a08104 = r32(pc_var13.wrapping_add(0xa9));
                                i_var19 = r32(i_var11.wrapping_add(0x1b)) as i32;
                                i_var8 = (i_var19 as u32).wrapping_mul(0x5bd);
                                if r8(d9698.wrapping_add(i_var8)) != 0 {
                                    u_var7 = FUN_004269e0(r8(i_var11.wrapping_add(3)) as u32);
                                    u_var18 = r16(d968a.wrapping_add(i_var8));
                                    let mut zero = u_var18 == 0xffff;
                                    if !zero {
                                        let t = r8(d5aa
                                            .wrapping_add(r8(d9697.wrapping_add(i_var8)) as u32)
                                            .wrapping_add((u_var7 & 0xffff).wrapping_mul(0x101)))
                                            as u32;
                                        let a =
                                            r8(h!(0x376).wrapping_add(t.wrapping_mul(4))) as u16;
                                        let b =
                                            r8(h!(0x372).wrapping_add(t.wrapping_mul(4))) as u16;
                                        zero = a.wrapping_add(b) == 0;
                                    }
                                    if zero {
                                        w8(d9696.wrapping_add(i_var8), 0);
                                        w16(h!(0x23b), 0);
                                    } else {
                                        u_var7 = u_var18 as u32;
                                        i_var11 = u_var7.wrapping_mul(0x23);
                                        c_var2 =
                                            s8(DAT_009a1fe8.wrapping_add(2).wrapping_add(i_var11));
                                        if c_var2 == 4 || c_var2 == 5 {
                                            FUN_004836e0(
                                                u_var6,
                                                u_var18 as u32,
                                                r16(d9699.wrapping_add(i_var8)) as u32,
                                            );
                                        } else {
                                            FUN_004034e0(u_var6, u_var7);
                                        }
                                        u_var14 = 0x18dad;
                                        if (u_var7 as i32) < DAT_009a1fe0 && {
                                            c_var2 = s8(DAT_009a1fe8
                                                .wrapping_add(2)
                                                .wrapping_add(i_var11));
                                            i_var12 = i_var11;
                                            c_var2 == 4 || c_var2 == 5 || c_var2 == 3
                                        } {
                                            u_var14 = 0x3ba6c;
                                        }
                                        if DAT_009a8c00 < u_var14
                                            && (FUN_00482640(
                                                i_var12 as i32,
                                                u_var14 as i32,
                                                u_var18 as i32,
                                                0,
                                                i_var19,
                                            )
                                                as i16)
                                                != 0
                                        {
                                            w16(
                                                h!(0x23b),
                                                r16(h!(0x23b)).wrapping_add(DAT_009ec5fe),
                                            );
                                            if (u_var7 as i32) < DAT_009a1fe0 {
                                                i_var11 = r32(r32(DAT_009a1fe8
                                                    .wrapping_add(0x1f)
                                                    .wrapping_add(i_var11))
                                                .wrapping_add(0xa9));
                                                c_var2 = s8(i_var11.wrapping_add(0x246));
                                                if c_var2 == 1
                                                    && r16(i_var11.wrapping_add(0x248)) != 0
                                                {
                                                    w16(
                                                        h!(0x23b),
                                                        r16(h!(0x23b))
                                                            .wrapping_sub(DAT_009ec5fe >> 1),
                                                    );
                                                }
                                            }
                                            u_var7 = FUN_004269e0(0xc);
                                            u_var14 = FUN_004269e0(r8(DAT_009a1fe8
                                                .wrapping_add(3)
                                                .wrapping_add(i_var5))
                                                as u32);
                                            let thr = ((m16((u_var14 as i16) == (u_var7 as i16))
                                                & 0xfe9e)
                                                .wrapping_add(0x2c4))
                                                as u32;
                                            if ((r16(h!(0x23b)) as i16) as i32) < (thr as i32) {
                                                w8(d9696.wrapping_add(i_var8), 1);
                                            } else {
                                                w8(d9696.wrapping_add(i_var8), 2);
                                            }
                                        } else {
                                            if 0 < (r16(h!(0x23b)) as i16) {
                                                u_var4 = r16(h!(0x23b))
                                                    .wrapping_sub(DAT_009ec5fe >> 1)
                                                    .wrapping_sub(DAT_009ec5fe);
                                                w16(h!(0x23b), u_var4);
                                                if (r16(h!(0x23b)) as i16) < 0 {
                                                    w16(h!(0x23b), 0);
                                                }
                                            }
                                            w8(d9696.wrapping_add(i_var8), 0);
                                        }
                                    }
                                }
                                u_var18 = 0xffff;
                                if (r8(h!(0x1e6)) & 1) == 0
                                    || r8(h!(0x241)) == 0
                                    || r8(h!(0x23d)) == 0
                                    || r8(d9681.wrapping_add(i_var8)) != 0
                                {
                                    if i_var19 == DAT_009ecc34 && {
                                        DAT_009d8110 = 0xffff;
                                        (r8(h!(0x1e4)) & 1) != 0
                                    } {
                                        FUN_0042edc0(0, i_var19);
                                    }
                                } else {
                                    if r32(d9c21.wrapping_add(i_var8)) == 0 {
                                        w32(d9c21.wrapping_add(i_var8), 0x3b);
                                        s_var10 = (r16(h!(0x23f)) as i16).wrapping_add(-0x7d);
                                        if s_var10 < 0 {
                                            s_var10 = 0;
                                        }
                                        w16(h!(0x23f), s_var10 as u16);
                                        if s_var10 == 0 && r8(h!(0x241)) != 0 {
                                            w8(h!(0x241), 0);
                                            w16(h!(0x242), 0);
                                            if i_var19 == DAT_009ecc34 {
                                                FUN_00450650(
                                                    (r8(h!(0x23d)) as i32).wrapping_add(0xf2),
                                                    DAT_009ecc34,
                                                );
                                            }
                                        }
                                    }
                                    u_var4 = r16(d968a.wrapping_add(i_var8));
                                    if u_var4 != 0xffff && (u_var4 as i32) < DAT_009a1fe0 {
                                        i_var11 = r32((u_var4 as u32)
                                            .wrapping_mul(0x23)
                                            .wrapping_add(0x1f)
                                            .wrapping_add(DAT_009a1fe8));
                                        i_var8 = r32(i_var11.wrapping_add(0xa9));
                                        c_var2 = s8(i_var8.wrapping_add(7));
                                        if c_var2 == 0
                                            && (FUN_00482640(
                                                i_var11 as i32,
                                                i_var8 as i32,
                                                u_var4 as i32,
                                                0,
                                                i_var19,
                                            )
                                                as i16)
                                                != 0
                                            && DAT_009a1ff8 < 0x20000
                                        {
                                            u_var18 = u_var4;
                                        }
                                    }
                                    if u_var18 != 0xffff {
                                        i_var11 = r32(r32((u_var18 as u32)
                                            .wrapping_mul(0x23)
                                            .wrapping_add(0x1f)
                                            .wrapping_add(DAT_009a1fe8))
                                        .wrapping_add(0xa9));
                                        b_var3 = r8(h!(0x23d));
                                        if b_var3 == 1 || b_var3 == 2 {
                                            if r8(i_var11.wrapping_add(0x246)) == 1
                                                && r16(i_var11.wrapping_add(0x248)) != 0
                                            {
                                                if i_var19 == DAT_009ecc34 && {
                                                    s_var10 = FUN_00451930();
                                                    s_var10 != 0x100
                                                } {
                                                    FUN_00450650(0x100, i_var19);
                                                }
                                                u_var18 = 0xffff;
                                            } else {
                                                let p = i_var11
                                                    .wrapping_add(0x15)
                                                    .wrapping_add((b_var3 as u32).wrapping_mul(4));
                                                w32(p, r32(p).wrapping_add(r16(h!(0x242)) as u32));
                                            }
                                        }
                                    }
                                    if i_var19 == DAT_009ecc34 {
                                        DAT_009d8110 = u_var18;
                                        FUN_0042edc0(1, i_var19);
                                    }
                                }
                                s_var10 = 0x14;
                                u_var18 = DAT_00a07cec;
                                while (u_var18 as i32) < DAT_009a1fe0 {
                                    i_var11 = (u_var18 as u32)
                                        .wrapping_mul(0x23)
                                        .wrapping_add(DAT_009a1fe8);
                                    c_var2 = s8(i_var11.wrapping_add(3));
                                    if c_var2 != 0
                                        && (r8(i_var11.wrapping_add(2)) == 4
                                            || (r8(i_var11.wrapping_add(2)) == 5
                                                && (c_var2 == 0x45 || c_var2 == 0x5b)))
                                    {
                                        u_var7 = r8(DAT_009a1fe8
                                            .wrapping_add(0x16)
                                            .wrapping_add(i_var5))
                                            as u32;
                                        u_var14 =
                                            r8(r32(i_var11.wrapping_add(0x1f)).wrapping_add(0x89))
                                                as u32;
                                        let own = r8(d8c88.wrapping_add(u_var7.wrapping_mul(0x566)))
                                            as u32;
                                        if own == u_var14 {
                                            b_var20 = false;
                                        } else {
                                            b_var20 = r8(dfd458
                                                .wrapping_add(own)
                                                .wrapping_add(u_var14.wrapping_mul(0x1e5)))
                                                == 0;
                                        }
                                        if b_var20 {
                                            FUN_004a8cc0(u_var6, u_var18 as u32);
                                        }
                                    }
                                    u_var18 = u_var18.wrapping_add(1);
                                }
                            }
                        }
                        if b_var21 {
                            DAT_00a08104 =
                                r32(r32(i_var5.wrapping_add(DAT_009a1fe8).wrapping_add(0x1f))
                                    .wrapping_add(0xa9));
                            c_var2 = s8(i_var5.wrapping_add(DAT_009a1fe8).wrapping_add(3));
                            if c_var2 == 2 {
                                s_var10 = ((s_var10 as i32 as u32).wrapping_mul(2)) as i16;
                            }
                            if (r8(h!(0x1e5)) & 1) != 0 && s_var10 != 0 && {
                                s_var10 = (((r8(h!(0x219)) as i32).wrapping_sub(2) as u32)
                                    .wrapping_mul(s_var10 as i32 as u32))
                                    as i16;
                                s_var10 != 0
                            } {
                                if r8(h!(0x21a)) == 0 {
                                    u_var6 = 0;
                                } else {
                                    if r8(h!(0x21a)) != 2 {
                                        s_var10 = ((s_var10 as i32) / 2) as i16;
                                        FUN_00405790(local_e as u32, 0, s_var10 as i32);
                                    }
                                    u_var6 = 1;
                                }
                                FUN_00405790(local_e as u32, u_var6, s_var10 as i32);
                            }
                            if (r8(h!(0x1e5)) & 0x10) != 0 && {
                                u_var6 = 0;
                                r8(h!(0x21d)) != 0
                            } {
                                loop {
                                    let i19 = u_var6.wrapping_mul(4);
                                    i_var11 = DAT_00a08104.wrapping_add(u_var6.wrapping_mul(4));
                                    if r8(i_var11.wrapping_add(0x370)) != 0
                                        && r8(i_var11.wrapping_add(0x370)) != 2
                                    {
                                        s_var15 = r8(h!(0x21c)) as i16;
                                        s_var10 = s_var15.wrapping_add(-2);
                                        if r16(h!(0xbf)) == 0 {
                                            s_var10 = s_var15.wrapping_add(-6);
                                        }
                                        c_var2 =
                                            s8(DAT_009a1fe8.wrapping_add(3).wrapping_add(i_var5));
                                        if c_var2 == 5 || c_var2 == 7 {
                                            s_var10 =
                                                ((s_var10 as i32 as u32).wrapping_mul(3)) as i16;
                                        } else {
                                            s_var10 =
                                                ((s_var10 as i32 as u32).wrapping_mul(2)) as i16;
                                        }
                                        w8(
                                            i_var11.wrapping_add(0x371),
                                            r8(i_var11.wrapping_add(0x371))
                                                .wrapping_add(s_var10 as u8),
                                        );
                                        if s_var10 < 0
                                            && s8(i19
                                                .wrapping_add(0x371)
                                                .wrapping_add(DAT_00a08104))
                                                < 0
                                        {
                                            w8(
                                                i19.wrapping_add(0x371).wrapping_add(DAT_00a08104),
                                                0,
                                            );
                                        }
                                        if 0 < s_var10
                                            && s8(i19
                                                .wrapping_add(0x371)
                                                .wrapping_add(DAT_00a08104))
                                                < 0
                                        {
                                            w8(
                                                i19.wrapping_add(0x371).wrapping_add(DAT_00a08104),
                                                0x7f,
                                            );
                                        }
                                    }
                                    u_var6 = (u_var6 as u16).wrapping_add(1) as u32;
                                    if !(u_var6 < r8(h!(0x21d)) as u32) {
                                        break;
                                    }
                                }
                            }
                            if r16(h!(0xbf)) == 0 {
                                b_var20 = false;
                                u_var6 = 0;
                                if r8(h!(0x21d)) != 0 {
                                    loop {
                                        if 0 < s8(h!(0x371).wrapping_add(u_var6.wrapping_mul(4))) {
                                            b_var20 = true;
                                        }
                                        u_var18 = (u_var6 as u16).wrapping_add(1);
                                        u_var6 = u_var18 as u32;
                                        if !(u_var18 < r8(h!(0x21d)) as u16) {
                                            break;
                                        }
                                    }
                                }
                                if !b_var20 {
                                    w16(h!(0xbf), 0xffff);
                                    FUN_00450650(0x11d, DAT_009ecc34);
                                    FUN_0042e4d0(0x5e, -1, DAT_009ecc34);
                                }
                            }
                            if (r8(h!(0x1e6)) & 1) != 0 {
                                s_var10 = (((r8(h!(0x23e)) as i32).wrapping_sub(2) as u32)
                                    .wrapping_mul(0x7d)
                                    .wrapping_add(r16(h!(0x23f)) as u32))
                                    as i16;
                                if s_var10 < 0 {
                                    s_var10 = 0;
                                }
                                if 9999 < s_var10 {
                                    s_var10 = 9999;
                                }
                                w16(h!(0x23f), s_var10 as u16);
                                if s_var10 == 0 && r8(h!(0x241)) != 0 {
                                    w8(h!(0x241), 0);
                                    w16(h!(0x242), 0);
                                    if r32(DAT_009a1fe8.wrapping_add(0x1b).wrapping_add(i_var5))
                                        as i32
                                        == DAT_009ecc34
                                    {
                                        FUN_00450650(
                                            (r8(h!(0x23d)) as i32).wrapping_add(0xf2),
                                            DAT_009ecc34,
                                        );
                                    }
                                }
                            }
                            if r16(h!(0x248)) != 0
                                && {
                                    w16(h!(0x248), r16(h!(0x248)).wrapping_sub(1));
                                    r8(h!(0x246)) == 1
                                }
                                && r16(h!(0x248)) == 0
                                && {
                                    i_var5 =
                                        r32(DAT_009a1fe8.wrapping_add(0x1b).wrapping_add(i_var5));
                                    i_var5 as i32 != -1
                                }
                            {
                                FUN_00450650(0x170, i_var5 as i32);
                            }
                        }
                    }
                }
            }
            local_e = local_e.wrapping_add(1);
        }

        local_e = DAT_00a07cec;
        while (local_e as i32) < DAT_009a1fe0 {
            i_var5 = (local_e as u32).wrapping_mul(0x23);
            i_var11 = DAT_009a1fe8.wrapping_add(i_var5);
            if r8(i_var11.wrapping_add(3)) != 0
                && {
                    pc_var13 = r32(i_var11.wrapping_add(0x1f));
                    r8(pc_var13) == 0
                }
                && {
                    DAT_00a08104 = r32(pc_var13.wrapping_add(0xa9));
                    r16(h!(0x1e7)) == 0
                }
            {
                if r32(h!(0x1d)) == 0 && {
                    u_var16 = 0;
                    r8(h!(0x21b)) != 0
                } {
                    loop {
                        s_var10 = r16(h!(0x226).wrapping_add(u_var16.wrapping_mul(2))) as i16;
                        if s_var10 != 0 {
                            s_var10 = s_var10.wrapping_sub(DAT_009ec5fe as i16);
                            if s_var10 < 0 {
                                s_var10 = 0;
                            }
                            w16(
                                h!(0x226).wrapping_add(u_var16.wrapping_mul(2)),
                                s_var10 as u16,
                            );
                        }
                        if r32(DAT_009a1fe8.wrapping_add(0x1b).wrapping_add(i_var5)) as i32 == -1
                            && s_var10 < (DAT_009ec5fe as i16)
                            && r8(u_var16.wrapping_add(0x220).wrapping_add(DAT_00a08104)) != 0
                        {
                            if (r8(h!(0x1e5)) & 0x10) != 0 && r8(h!(7)) == 0 {
                                FUN_00405ac0(local_e as u32, u_var16);
                            }
                            pc_var13 = u_var16.wrapping_add(0x222).wrapping_add(DAT_00a08104);
                            w8(pc_var13, r8(pc_var13).wrapping_sub(1));
                            w16(
                                h!(0x226).wrapping_add(u_var16.wrapping_mul(2)),
                                r16(h!(0x226).wrapping_add(u_var16.wrapping_mul(2)))
                                    .wrapping_add(DAT_009ec5fe.wrapping_mul(2)),
                            );
                            w32(
                                h!(0x22a).wrapping_add(u_var16.wrapping_mul(4)),
                                r32(h!(0x22a).wrapping_add(u_var16.wrapping_mul(4)))
                                    .wrapping_add((DAT_009ec5fe as u32).wrapping_mul(2)),
                            );
                            if r8(u_var16.wrapping_add(DAT_00a08104).wrapping_add(0x222)) == 0 {
                                w8(u_var16.wrapping_add(DAT_00a08104).wrapping_add(0x220), 0);
                            }
                        }
                        u_var16 = (u_var16 as u16).wrapping_add(1) as u32;
                        if !(u_var16 < r8(h!(0x21b)) as u32) {
                            break;
                        }
                    }
                }
                u_var16 = 0;
                if r8(h!(0x21d)) != 0 {
                    loop {
                        if r8(h!(0x370).wrapping_add(u_var16.wrapping_mul(4))) == 2 && {
                            u_var18 = r16(DAT_00a08104
                                .wrapping_add(u_var16.wrapping_mul(4))
                                .wrapping_add(0x3b2));
                            u_var18 != 0xffff
                        } {
                            FUN_004a7900(local_e as u32, u_var16 as u16, u_var18);
                        }
                        u_var16 = (u_var16 as u16).wrapping_add(1) as u32;
                        if !(u_var16 < r8(h!(0x21d)) as u32) {
                            break;
                        }
                    }
                }
                u_var16 = 0;
                if r8(h!(0x232)) != 0 {
                    loop {
                        pc_var13 = h!(0x237).wrapping_add(u_var16.wrapping_mul(2));
                        s_var10 = r16(pc_var13) as i16;
                        if s_var10 != 0 {
                            s_var10 = s_var10.wrapping_sub(DAT_009ec5fe as i16);
                            if s_var10 < 0 {
                                s_var10 = 0;
                            }
                            w16(pc_var13, s_var10 as u16);
                        }
                        u_var16 = (u_var16 as u16).wrapping_add(1) as u32;
                        if !(u_var16 < r8(h!(0x232)) as u32) {
                            break;
                        }
                    }
                }
            }
            local_e = local_e.wrapping_add(1);
        }

        u_var16 = DAT_00a07ce8 & 0xffff;
        while (u_var16 as i32) < DAT_009d1308.wrapping_add(DAT_00a07ce8 as i32) {
            i_var5 = u_var16.wrapping_mul(0x23).wrapping_add(DAT_009a1fe8);
            if r8(i_var5.wrapping_add(3)) != 0 && r8(i_var5.wrapping_add(2)) == 8 {
                FUN_00446c60(u_var16 as u16);
            }
            u_var18 = (u_var16 as u16).wrapping_add(1);
            u_var16 = u_var18 as u32;
        }
    }
}
