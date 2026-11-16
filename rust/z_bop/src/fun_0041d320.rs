// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: u32;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009d8c88: u8;
    static mut DAT_009d8c80: u8;
    static mut DAT_009a20d2: u8;
    static mut DAT_009a2106: u8;
    static mut DAT_00bb7375: i8;
    static mut DAT_009e967c: u8;
    static mut DAT_009e9681: u8;
    static mut DAT_009ecc34: i32;
    static mut DAT_009fd458: u8;
    static mut DAT_0051a44f: u8;
    static mut DAT_009ed228: u8;
    static mut DAT_009ed22c: u8;
    static mut DAT_009ed22d: u8;
    static mut DAT_009a1fd8: i32;
    static mut DAT_00a081e8: i32;
    static mut DAT_009a1fd4: i32;
    static mut DAT_009ec604: i32;
    static mut DAT_009a1fd0: i32;
    static mut DAT_00a08298: i32;
    static mut DAT_009d695f: i8;
    static mut DAT_0051bf8e: u16;
    static mut DAT_00a07be0: u16;
    static mut DAT_00a07be2: u16;
    static mut DAT_009d77fc: i32;
    static mut DAT_009d6828: i32;
    static mut DAT_009cd260: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00430200(a: i32, b: i32, c: u32, d: i32);
    fn FUN_00435640(a: i32, b: u32, c: i32, d: i32, e: u32, f: u32);
    fn FUN_00435a10(a: u8, b: u8, c: u8) -> i32;
    fn FUN_00427bd0(a: i16, b: i16, c: i32);
    fn FUN_00427d30(a: i16, b: i32, c: i32);
    fn FUN_0041bc50(a: u16, b: u16);
    fn FUN_00450650(a: i32, b: i32);
    fn FUN_0041dc30(a: u32, b: u16, c: u32, d: u32, e: u32, f: u16, g: u32, h: u16, i: u32) -> u32;
    fn FUN_0042e4d0(a: u32, b: u32, c: i32);
    fn FUN_00425d10() -> u32;
}

unsafe fn ld<T>(a: u32) -> T {
    // SAFETY: the address is the game's own memory, read as the original does.
    unsafe { (a as usize as *const T).read_unaligned() }
}

unsafe fn st<T>(a: u32, v: T) {
    // SAFETY: the address is the game's own memory, written as the original does.
    unsafe { (a as usize as *mut T).write_unaligned(v) }
}

fn g() -> u32 {
    // SAFETY: the global is the game's own.
    unsafe { DAT_009a1fe8 }
}

fn ent(i: u32) -> u32 {
    // SAFETY: only the address of the game's array is taken.
    let b = unsafe { &raw mut DAT_009e967c } as u32;
    b.wrapping_add(i.wrapping_mul(0x5bd))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_assignments, unused_variables, unused_mut)]
pub unsafe extern "C" fn FUN_0041d320(
    param_1: u32,
    param_2: u16,
    param_3: u32,
    param_4: u32,
    param_5: u32,
    param_6: u32,
    param_7: u16,
) {
    // SAFETY: all addresses and globals are the game's own memory, accessed
    // in the same order as the original.
    unsafe {
        let unaff_bx: u16 = 0;
        let unaff_ebp: u32 = 0;
        let unaff_esi: u32 = 0;
        let unaff_di: u16 = 0;
        let in_stack_0000000a: u16 = 0;
        let mut in_stack_ffffffc8: u32 = 0;

        let d8c88 = &raw mut DAT_009d8c88 as u32;
        let d8c80 = &raw mut DAT_009d8c80 as u32;
        let a20d2 = &raw mut DAT_009a20d2 as u32;
        let a2106 = &raw mut DAT_009a2106 as u32;
        let fd458 = &raw mut DAT_009fd458 as u32;
        let a44f = &raw mut DAT_0051a44f as u32;
        let d228 = &raw mut DAT_009ed228 as u32;
        let e9681 = &raw mut DAT_009e9681 as u32;

        let u_var11 = param_1 & 0xffff;
        let i_var20 = u_var11.wrapping_mul(0x23);
        let u_var4: u16 =
            ld::<u16>(ld::<u32>(g().wrapping_add(0x1f).wrapping_add(i_var20)).wrapping_add(0x85));
        let mut u_var17 = u_var4 as u32;
        let mut i_var13 = u_var17.wrapping_mul(0x23);
        if u_var4 == param_2 {
            return;
        }
        let i_var6 = (param_2 as u32).wrapping_mul(0x23);
        let i_var5: u32 =
            ld::<u32>(ld::<u32>(g().wrapping_add(0x1f).wrapping_add(i_var6)).wrapping_add(0xa9));
        let p2: u32 = (ld::<u8>(d8c88.wrapping_add(
            (ld::<u8>(g().wrapping_add(0x16).wrapping_add(i_var13)) as u32).wrapping_mul(0x566),
        )) as u32)
            .wrapping_add(0x1ec)
            .wrapping_add(i_var5);
        if ld::<u8>(p2) == 0 {
            st::<u8>(p2, 1);
            let i_var14 = g().wrapping_add(i_var6);
            let u_var12 = ld::<u8>(i_var14.wrapping_add(0x16)) as u32;
            let i_var7 = u_var12.wrapping_mul(0x126);
            let sa = a20d2.wrapping_add(i_var7);
            st::<i16>(sa, ld::<i16>(sa).wrapping_add(1));
            let i_var19 = u_var12.wrapping_mul(0x566);
            if (ld::<u8>(d8c80.wrapping_add(i_var19)) as i8)
                == (ld::<u8>(i_var5.wrapping_add(0xb0)) as i8)
            {
                st::<u8>(a2106.wrapping_add(i_var7), 1);
            }
            if DAT_00bb7375 == 2
                && (ld::<u8>(d8c88.wrapping_add(i_var19)) as u16)
                    == ld::<u16>(ent(DAT_009ecc34 as u32))
                && (ld::<u8>(i_var14.wrapping_add(2)) as i8) != 0
            {
                let mut idv: i32 = 0x18;
                if (u_var17 as i32) < DAT_009a1fe0 {
                    let c = ld::<u8>(g().wrapping_add(3).wrapping_add(i_var20)) as i8;
                    if c == -0x70 || c == -0x6b {
                        idv = 0x2b;
                    } else if c == -0x71 || c == -0x6c {
                        idv = 0x2c;
                    } else if c == -0x69 {
                        idv = 0x2d;
                    } else if c == -0x6a {
                        idv = 0x2e;
                    } else {
                        let c2 = ld::<u8>(g().wrapping_add(2).wrapping_add(i_var13)) as i8;
                        if c2 == 0 {
                            idv = 0x19;
                        } else if c2 == 4 {
                            idv = 0x1a;
                        } else {
                            idv = 0x18;
                        }
                    }
                }
                FUN_00430200(1, idv, param_2 as u32, 0xffff);
            }
        }
        let i_var19 = ld::<i32>(g().wrapping_add(0x1b).wrapping_add(i_var13));
        if i_var19 != -1 {
            if (ld::<u8>(p2) & 0x80) == 0 {
                let b8 = ld::<u8>(g().wrapping_add(0x16).wrapping_add(i_var6));
                let u_var12 =
                    ((ld::<u8>(d8c80.wrapping_add((b8 as u32).wrapping_mul(0x566))) as i8)
                        == (ld::<u8>(i_var5.wrapping_add(0xb0)) as i8)) as u32;
                FUN_00435640(
                    3,
                    (g() & 0xffff0000) | b8 as u32,
                    i_var19,
                    1,
                    u_var12,
                    ld::<u16>(ent(i_var19 as u32)) as u32,
                );
                let hi = g() & 0xffff0000;
                let lo = ld::<u8>(g().wrapping_add(0x16).wrapping_add(i_var6)) as u32;
                let e = ld::<i32>(g().wrapping_add(0x1b).wrapping_add(i_var13));
                FUN_00435640(3, hi | lo, -1, 1, u_var12, ld::<u16>(ent(e as u32)) as u32);
            }
            st::<u8>(p2, ld::<u8>(p2) | 0x80);
        }
        'l750: {
            if ld::<i16>(i_var5.wrapping_add(0x6a)) == -1 {
                if DAT_009a1fe0 <= u_var17 as i32 {
                    break 'l750;
                }
                i_var13 = g().wrapping_add(i_var13);
                let c = ld::<u8>(i_var13.wrapping_add(2)) as i8;
                if c == 4 || c == 5 || c == 3 {
                    break 'l750;
                }
                let u_var21 = (in_stack_ffffffc8 >> 0x18) as u8;
                let u_var15 = in_stack_ffffffc8 as u16;
                in_stack_ffffffc8 = ((u_var21 as u32) << 24) | (1u32 << 16) | u_var15 as u32;
                if ld::<i32>(i_var13.wrapping_add(0x1b)) != -1 {
                    let mut b8 = ld::<u8>(ld::<u32>(i_var13.wrapping_add(0x1f)).wrapping_add(0x89));
                    let b3 = ld::<u8>(
                        ld::<u32>(g().wrapping_add(i_var6).wrapping_add(0x1f)).wrapping_add(0x89),
                    );
                    let b18: i32 = if b8 == b3 {
                        0
                    } else {
                        (ld::<u8>(
                            fd458
                                .wrapping_add(b3 as u32)
                                .wrapping_add((b8 as u32).wrapping_mul(0x1e5)),
                        ) == 0) as i32
                    };
                    if b18 == 0 && (ld::<u8>(g().wrapping_add(i_var6).wrapping_add(2)) as i8) != 0 {
                        b8 = (ld::<u8>(p2) & 0x70) >> 4;
                        if b8 < 5 {
                            let t = ((ld::<u8>(
                                a44f.wrapping_add(ld::<u8>(
                                    g().wrapping_add(3).wrapping_add(i_var20),
                                ) as u32),
                            ) == 0) as u32)
                                .wrapping_sub(1)
                                & 4;
                            b8 = b8.wrapping_add(t as u8);
                            if 7 < b8 {
                                b8 = 7;
                            }
                            st::<u8>(p2, (b8 << 4) | (ld::<u8>(p2) & 0x8f));
                        }
                        if b8 < 7 {
                            in_stack_ffffffc8 = ((u_var21 as u32) << 24) | u_var15 as u32;
                        }
                    }
                }
                if (in_stack_ffffffc8 >> 0x10) as u8 != 1 {
                    break 'l750;
                }
            } else if ld::<i32>(g().wrapping_add(0x1b).wrapping_add(i_var6)) == -1 {
                break 'l750;
            }
            st::<u16>(i_var5.wrapping_add(0x6a), u_var4);
            let r = FUN_00435a10(ld::<u8>(d228.wrapping_add(3)), DAT_009ed22c, DAT_009ed22d);
            st::<i16>(i_var5.wrapping_add(0x6c), r as i16);
        }
        st::<u16>(i_var5.wrapping_add(0x6e), u_var4);
        st::<u8>(
            i_var5.wrapping_add(0x79),
            ld::<u8>(i_var5.wrapping_add(0x79)).wrapping_add(1),
        );
        let i_var13 = i_var6.wrapping_add(g());
        if (ld::<u8>(ld::<u32>(i_var13.wrapping_add(0x1f)).wrapping_add(0x92)) as i8) != 0 {
            FUN_00427bd0(
                ld::<i16>(i_var13.wrapping_add(0x12)),
                ld::<i16>(i_var13.wrapping_add(0x10)),
                i_var13 as i32,
            );
            FUN_00427d30(
                ld::<i16>(g().wrapping_add(i_var6).wrapping_add(0x14)),
                0,
                g().wrapping_add(i_var6) as i32,
            );
        }
        let i_var19 = g().wrapping_add(i_var6);
        u_var17 = (ld::<i16>(ld::<u32>(i_var19.wrapping_add(0x1f)).wrapping_add(0x95)) as i32)
            .wrapping_mul((DAT_009a1fd8 as i16).wrapping_sub(DAT_00a081e8 as i16) as i32)
            as u32;
        let t93 = (ld::<i16>(ld::<u32>(i_var19.wrapping_add(0x1f)).wrapping_add(0x93)) as i32)
            .wrapping_mul((DAT_009a1fd4 as i16).wrapping_sub(DAT_009ec604 as i16) as i32)
            as u32;
        let t97 = (ld::<i16>(ld::<u32>(i_var19.wrapping_add(0x1f)).wrapping_add(0x97)) as i32)
            .wrapping_mul((DAT_009a1fd0 as i16).wrapping_sub(DAT_00a08298 as i16) as i32)
            as u32;
        let mut i_var13 = t93.wrapping_add(u_var17).wrapping_add(t97) as i32;
        if 0x3fffffff < i_var13 {
            i_var13 = 0x3fffffff;
        }
        if i_var13 < -0x3fffffff {
            i_var13 = -0x3fff0000;
        }
        let s_stack_1c = (i_var13 >> 0xf) as i16;
        u_var17 &= 0xffff0000;
        if ld::<i32>(i_var19.wrapping_add(0x1b)) != -1 {
            u_var17 = (-1 < s_stack_1c) as u32;
        }
        if DAT_009d695f != 0
            && (ld::<u8>(i_var5.wrapping_add(7)) as i8) == 0
            && ld::<u8>(
                a44f.wrapping_add(ld::<u8>(g().wrapping_add(3).wrapping_add(i_var20)) as u32),
            ) != 0
        {
            FUN_0041bc50(param_2, param_1 as u16);
        }
        let b8 = ld::<u8>(g().wrapping_add(3).wrapping_add(i_var20));
        let c_var9: i8;
        if b8 == 0x98 {
            let u4 = ld::<u16>(i_var5.wrapping_add(0x1e7));
            if ld::<i32>(g().wrapping_add(i_var6).wrapping_add(0x1b)) == -1 {
                let c = ld::<u8>(g().wrapping_add(i_var6).wrapping_add(2)) as i8;
                let s10: u16 = if c == 0 || c == 1 || c == 2 {
                    u4.wrapping_add(0xdd4)
                } else {
                    u4.wrapping_add(0x1ba8)
                };
                st::<u16>(i_var5.wrapping_add(0x1e7), s10);
            } else {
                let mut u15: u16 = 0;
                if ld::<u8>(i_var5.wrapping_add(0x21d)) != 0 {
                    loop {
                        let u16v = u15.wrapping_add(1);
                        st::<u8>(
                            i_var5
                                .wrapping_add(0x371)
                                .wrapping_add((u15 as u32).wrapping_mul(4)),
                            0,
                        );
                        u15 = u16v;
                        if !((u16v as u32) < ld::<u8>(i_var5.wrapping_add(0x21d)) as u32) {
                            break;
                        }
                    }
                }
                if (ld::<u16>(i_var5.wrapping_add(0x1e5)) & 0x10) == 0 {
                    FUN_00450650(
                        0x11b,
                        ld::<i32>(g().wrapping_add(0x1b).wrapping_add(i_var6)),
                    );
                } else {
                    st::<u16>(
                        i_var5.wrapping_add(0x1e5),
                        ld::<u16>(i_var5.wrapping_add(0x1e5)) & 0x3ef,
                    );
                    st::<u16>(i_var5.wrapping_add(0x2b2), 0);
                    st::<u16>(i_var5.wrapping_add(0x2c8), DAT_0051bf8e);
                    if ld::<i32>(g().wrapping_add(0x1b).wrapping_add(i_var6)) == DAT_009ecc34
                        && ld::<u8>(e9681.wrapping_add((DAT_009ecc34 as u32).wrapping_mul(0x5bd)))
                            == 0
                    {
                        DAT_00a07be0 = 0x5c;
                        DAT_00a07be2 = 0x57;
                        FUN_00450650(0x56, DAT_009ecc34);
                    }
                }
            }
            if ld::<u16>(i_var5.wrapping_add(0x1e7)) < u4 {
                st::<u16>(i_var5.wrapping_add(0x1e7), 0xffff);
            }
            c_var9 = 1;
        } else {
            let mut b18: i32 = 0;
            if (ld::<u8>(i_var5.wrapping_add(0x246)) as i8) == 1
                && ld::<i16>(i_var5.wrapping_add(0x248)) != 0
                && ld::<u8>(a44f.wrapping_add(b8 as u32)) != 0
                && -1 < s_stack_1c
            {
                b18 = 1;
                FUN_00450650(
                    0x171,
                    ld::<i32>(g().wrapping_add(0x1b).wrapping_add(i_var6)),
                );
            }
            if b18 != 0 {
                c_var9 = (in_stack_ffffffc8 >> 0x18) as i8;
            } else {
                u_var17 = FUN_0041dc30(
                    ((in_stack_0000000a as u32) << 16) | param_2 as u32,
                    param_3 as u16,
                    param_1,
                    u_var17,
                    unaff_ebp,
                    unaff_di,
                    unaff_esi,
                    unaff_bx,
                    in_stack_ffffffc8,
                );
                c_var9 = u_var17 as i8;
            }
        }
        st::<i32>(
            g().wrapping_add(4).wrapping_add(i_var20),
            (DAT_009d77fc as u32).wrapping_add(DAT_009ec604 as u32) as i32,
        );
        st::<i32>(
            g().wrapping_add(8).wrapping_add(i_var20),
            (DAT_009d6828 as u32).wrapping_add(DAT_00a081e8 as u32) as i32,
        );
        st::<i32>(
            g().wrapping_add(0xc).wrapping_add(i_var20),
            (DAT_009cd260 as u32).wrapping_add(DAT_00a08298 as u32) as i32,
        );
        let b8 = ld::<u8>(g().wrapping_add(3).wrapping_add(i_var20));
        let pu1 = g().wrapping_add(3).wrapping_add(i_var20);
        if ld::<u8>(a44f.wrapping_add(b8 as u32)) == 0 {
            if b8 == 0x8d || b8 == 0x8e {
                st::<u8>(pu1, 0x84);
            } else {
                st::<u8>(pu1, 0x83);
            }
        } else {
            st::<u8>(pu1, 0x81);
        }
        st::<u8>(g().wrapping_add(2).wrapping_add(i_var20), 0xd);
        st::<u8>(ld::<u32>(g().wrapping_add(0x1f).wrapping_add(i_var20)), 5);
        st::<u8>(g().wrapping_add(0x19).wrapping_add(i_var20), 2);
        st::<i16>(
            ld::<u32>(g().wrapping_add(0x1f).wrapping_add(i_var20)).wrapping_add(0x83),
            0,
        );
        st::<i16>(
            ld::<u32>(g().wrapping_add(0x1f).wrapping_add(i_var20)).wrapping_add(0x81),
            0,
        );
        st::<u8>(
            ld::<u32>(g().wrapping_add(0x1f).wrapping_add(i_var20)).wrapping_add(1),
            0,
        );
        let v79 =
            ld::<i16>(ld::<u32>(g().wrapping_add(0x1f).wrapping_add(i_var6)).wrapping_add(0x79));
        st::<i16>(
            ld::<u32>(g().wrapping_add(0x1f).wrapping_add(i_var20)).wrapping_add(0x79),
            v79,
        );
        let v12 = ld::<i16>(g().wrapping_add(0x12).wrapping_add(i_var6));
        st::<i16>(g().wrapping_add(0x12).wrapping_add(i_var20), v12);
        let v10 = ld::<i16>(g().wrapping_add(0x10).wrapping_add(i_var6));
        st::<i16>(g().wrapping_add(0x10).wrapping_add(i_var20), v10);
        st::<i16>(g().wrapping_add(0x14).wrapping_add(i_var20), 0);
        st::<u8>(
            ld::<u32>(g().wrapping_add(0x1f).wrapping_add(i_var20)).wrapping_add(0x92),
            1,
        );
        let i_var13 = ld::<u32>(g().wrapping_add(0x1f).wrapping_add(i_var20));
        st::<u8>(
            i_var13.wrapping_add(0x8b),
            ld::<u8>(i_var13.wrapping_add(0x92)),
        );
        if c_var9 != 0 {
            if ld::<i32>(g().wrapping_add(0x1b).wrapping_add(i_var6)) == DAT_009ecc34 {
                FUN_0042e4d0(0x1d, u_var11, DAT_009ecc34);
                return;
            }
            let c = ld::<u8>(g().wrapping_add(3).wrapping_add(i_var20)) as i8;
            if c != -0x7d && c != -0x7c {
                let i20 = DAT_009ecc34;
                let u = FUN_00425d10();
                FUN_0042e4d0((u & 3).wrapping_add(0x16), u_var11, i20);
                return;
            }
            FUN_0042e4d0(0x1c, u_var11, DAT_009ecc34);
        }
    }
}
