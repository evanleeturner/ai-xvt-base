// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a7394: u16;
    static mut DAT_009d8112: i16;
    static mut DAT_009e9658: i32;
    static mut DAT_009ec5d0: i32;
    static mut DAT_009ec5fe: u16;
    static mut DAT_00a07cec: u16;
    static mut DAT_00a08104: i32;
    static mut DAT_0051c58b: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00425b70(param_1: u32, param_2: u16) -> u32;
    fn FUN_004adb50(param_1: u32);
    fn FUN_004ad880(param_1: u32, param_2: u32, param_3: i32, param_4: u16);
    fn FUN_004ada90(param_1: u32, param_2: u32);
    fn FUN_004ad9e0(param_1: u32, param_2: i32);
}

#[inline(always)]
unsafe fn r16(a: u32) -> u16 {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *const u16).read_unaligned() }
}
#[inline(always)]
unsafe fn r32(a: u32) -> u32 {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *const u32).read_unaligned() }
}
#[inline(always)]
unsafe fn r8(a: u32) -> u8 {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *const u8).read() }
}
#[inline(always)]
unsafe fn w16(a: u32, v: u16) {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *mut u16).write_unaligned(v) }
}
#[inline(always)]
unsafe fn w8(a: u32, v: u8) {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *mut u8).write(v) }
}
#[inline(always)]
unsafe fn g() -> u32 {
    // SAFETY: reads the game's global.
    unsafe { DAT_009a1fe8 as u32 }
}
#[inline(always)]
unsafe fn d() -> u32 {
    // SAFETY: reads the game's global.
    unsafe { DAT_00a08104 as u32 }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_assignments, unused_variables, unused_mut)]
pub unsafe extern "stdcall" fn FUN_004ace80() {
    // SAFETY: all globals, callees and addresses belong to the game; accessed
    // in the same order and widths as the original.
    unsafe {
        let u_var11: u16 = DAT_009ec5fe;
        let s_var10: i16 = DAT_009d8112;
        let mut local_14: u16 = DAT_00a07cec;
        let mut b_var9: bool = false;
        let mut unaff_ebx: u32 = 0;
        let mut i7: u32 = 0;

        if (DAT_00a07cec as i32) < DAT_009a1fe0 {
            loop {
                if DAT_009ec5d0 != -1 {
                    if b_var9 {
                        DAT_009d8112 = s_var10;
                        DAT_009ec5fe = u_var11;
                        return;
                    }
                    b_var9 = true;
                    local_14 = DAT_009ec5d0 as u16;
                }
                let i8: u32 = (local_14 as u32)
                    .wrapping_mul(8)
                    .wrapping_sub(local_14 as u32)
                    .wrapping_mul(5);
                let mut i14: u32 = g().wrapping_add(i8);
                DAT_009d8112 = s_var10;
                DAT_009ec5fe = u_var11;
                let go: bool;
                if r32(i14.wrapping_add(0x1f)) == 0 {
                    go = true;
                } else {
                    i7 = r32(r32(i14.wrapping_add(0x1f)).wrapping_add(2));
                    if i7 == 0 {
                        go = true;
                    } else {
                        DAT_009ec5fe = u_var11
                            .wrapping_add((DAT_009e9658 as i16).wrapping_sub(i7 as i16) as u16);
                        if DAT_009ec5fe != 0 {
                            DAT_009d8112 = (0xecu32 / (DAT_009ec5fe as u32)) as i16;
                            if DAT_009d8112 == 0 {
                                DAT_009d8112 = 1;
                            }
                            go = true;
                        } else {
                            go = false;
                        }
                    }
                }
                if go && r8(i14.wrapping_add(3)) != 0 && r8(r32(i14.wrapping_add(0x1f))) == 0 {
                    let mut u_stack_12: u16 = 0;
                    i7 = r32(r32(i14.wrapping_add(0x1f)).wrapping_add(0xa9));
                    let s_var3: i16 = r16(i14.wrapping_add(0x12)) as i16;
                    let s_var4: i16 = r16(i14.wrapping_add(0x10)) as i16;
                    let s_var5: i16 = r16(i14.wrapping_add(0x14)) as i16;
                    unaff_ebx = (unaff_ebx & 0xffff0000) | (s_var5 as u16 as u32);
                    DAT_009a7394 = r8(i7.wrapping_add(4)) as u16;
                    if r32(i14.wrapping_add(0x1b)) == 0xffffffff {
                        if r16(i7.wrapping_add(0x1e5)) != 0 {
                            u_stack_12 = r16(i7.wrapping_add(0xbd));
                        }
                    } else if (r8(i7.wrapping_add(0x1e5)) & 0x40) != 0 {
                        u_stack_12 = r16(i7.wrapping_add(0xbd));
                    }
                    DAT_00a08104 = i7 as i32;
                    if r32(i14.wrapping_add(0x1b)) == 0xffffffff
                        && r16(i7.wrapping_add(0x1e5)) != 0
                        && r32(i7.wrapping_add(0x19)) == 0
                    {
                        if r8(i7.wrapping_add(0xa4)) != 0 && r8(i7.wrapping_add(0xa4)) < 4 {
                            let u16_16: u16 = r16(i7.wrapping_add(0x5a))
                                .wrapping_sub(r16(i14.wrapping_add(0x14)));
                            unaff_ebx = 0;
                            let a = (((r16(i7.wrapping_add(0xa0)) as u32)
                                .wrapping_mul(DAT_009ec5fe as u32))
                                as i32
                                / 0xec) as u32;
                            let mut u12 = FUN_00425b70(a, r16(i7.wrapping_add(0xa2)));
                            u12 = FUN_00425b70(u12, r16(d().wrapping_add(0xa5)));
                            let u13: u16 = (u12 as u16).wrapping_mul(2);
                            let mut do_neg = false;
                            let p = g().wrapping_add(0x14).wrapping_add(i8);
                            if r8(d().wrapping_add(0xa4)) == 3 {
                                if r16(i7.wrapping_add(0x5a)) < 0x8000 {
                                    w16(p, r16(p).wrapping_add(u13));
                                } else {
                                    do_neg = true;
                                }
                            } else if u16_16 < 0x8000 {
                                if u13 < u16_16 {
                                    w16(p, r16(p).wrapping_add(u13));
                                } else {
                                    w16(p, r16(i7.wrapping_add(0x5a)));
                                    w8(d().wrapping_add(0xa4), 4);
                                }
                            } else if u13 < u16_16.wrapping_neg() {
                                do_neg = true;
                            } else {
                                w16(p, r16(i7.wrapping_add(0x5a)));
                                w8(d().wrapping_add(0xa4), 4);
                            }
                            if do_neg {
                                let p = g().wrapping_add(0x14).wrapping_add(i8);
                                w16(
                                    p,
                                    r16(p)
                                        .wrapping_add((u12 as u16).wrapping_mul(2).wrapping_neg()),
                                );
                            }
                        }
                        if r8(d().wrapping_add(0x9c)) != 0 {
                            let mut u13: u16 =
                                r16(i7.wrapping_add(0x58)).wrapping_sub(r16(d().wrapping_add(0xd)));
                            if 0x7fff < u13 {
                                u13 = u13.wrapping_neg();
                            }
                            let a = (((r16(d().wrapping_add(0x98)) as u32)
                                .wrapping_mul(DAT_009ec5fe as u32))
                                as i32
                                / 0xec) as u32;
                            let mut u12 = FUN_00425b70(a, r16(d().wrapping_add(0x9a)));
                            u12 = FUN_00425b70(u12, r16(d().wrapping_add(0x9e)));
                            let u16_16: u16 = u12 as u16;
                            unaff_ebx = u12 & 0xffff;
                            FUN_00425b70(unaff_ebx, 0x8000);
                            if r8(d().wrapping_add(0x9c)) == 1 {
                                if u16_16 < u13 || r8(d().wrapping_add(0x9d)) != 0 {
                                    w16(
                                        d().wrapping_add(0xd),
                                        r16(d().wrapping_add(0xd)).wrapping_sub(u16_16),
                                    );
                                    if 0xdfff < r16(d().wrapping_add(0xd)) {
                                        w16(
                                            d().wrapping_add(0xd),
                                            r16(d().wrapping_add(0xd)).wrapping_neg(),
                                        );
                                        let p = g().wrapping_add(0x10).wrapping_add(i8);
                                        w16(p, r16(p).wrapping_add(0x8000));
                                        let p = g().wrapping_add(0x14).wrapping_add(i8);
                                        w16(p, r16(p).wrapping_add(0x8000));
                                        w8(d().wrapping_add(0x9d), 0);
                                        w8(d().wrapping_add(0x9c), 2);
                                    }
                                } else {
                                    w16(d().wrapping_add(0xd), r16(i7.wrapping_add(0x58)));
                                    w8(d().wrapping_add(0x9c), 3);
                                }
                            } else if r8(d().wrapping_add(0x9c)) == 2 {
                                if u16_16 < u13 || r8(d().wrapping_add(0x9d)) != 0 {
                                    w16(
                                        d().wrapping_add(0xd),
                                        r16(d().wrapping_add(0xd)).wrapping_add(u16_16),
                                    );
                                    let u13b = r16(d().wrapping_add(0xd));
                                    if 0x7fff < u13b {
                                        w16(d().wrapping_add(0xd), u13b.wrapping_neg());
                                        let p = g().wrapping_add(0x10).wrapping_add(i8);
                                        w16(p, r16(p).wrapping_add(0x8000));
                                        let p = g().wrapping_add(0x14).wrapping_add(i8);
                                        w16(p, r16(p).wrapping_add(0x8000));
                                        w8(d().wrapping_add(0x9d), 0);
                                        w8(d().wrapping_add(0x9c), 1);
                                    }
                                } else {
                                    w16(d().wrapping_add(0xd), r16(i7.wrapping_add(0x58)));
                                    w8(d().wrapping_add(0x9c), 3);
                                }
                            }
                        }
                        if r8(d().wrapping_add(7)) != 2 && r8(d().wrapping_add(0xab)) != 0 && {
                            let v = r16(i7.wrapping_add(0x5c))
                                .wrapping_sub(r16(g().wrapping_add(0x10).wrapping_add(i8)));
                            // stash in a local through a shadow below
                            v != 0
                        } {
                            let u13: u16 = r16(i7.wrapping_add(0x5c))
                                .wrapping_sub(r16(g().wrapping_add(0x10).wrapping_add(i8)));
                            unaff_ebx = 0;
                            let a = (((r16(d().wrapping_add(0xa7)) as u32)
                                .wrapping_mul(DAT_009ec5fe as u32))
                                as i32
                                / 0xec) as u32;
                            let mut u12 = FUN_00425b70(a, r16(d().wrapping_add(0xa9)));
                            u12 = FUN_00425b70(u12, r16(d().wrapping_add(0xac)));
                            let u16_16: u16 = u12 as u16;
                            let p = g().wrapping_add(0x10).wrapping_add(i8);
                            let to_target;
                            if u13 < 0x8000 {
                                if u16_16 < u13 {
                                    w16(p, r16(p).wrapping_add(u16_16));
                                    to_target = false;
                                } else {
                                    to_target = true;
                                }
                            } else if u13.wrapping_neg() <= u16_16 {
                                to_target = true;
                            } else {
                                w16(p, r16(p).wrapping_sub(u16_16));
                                to_target = false;
                            }
                            if to_target {
                                w16(
                                    g().wrapping_add(0x10).wrapping_add(i8),
                                    r16(i7.wrapping_add(0x5c)),
                                );
                                u12 &= 0xffff0000;
                                w8(d().wrapping_add(0xab), 3);
                            }
                            if r8(d().wrapping_add(0xa4)) == 0 || r8(d().wrapping_add(0xa4)) == 4 {
                                let addr = ((&raw mut DAT_0051c58b) as usize as u32)
                                    .wrapping_add((DAT_009a7394 as u32).wrapping_mul(0x101));
                                u12 = FUN_00425b70(u12, r16(addr));
                                let p = g().wrapping_add(0x14).wrapping_add(i8);
                                if u13 < 0x8000 {
                                    w16(p, r16(p).wrapping_sub(u12 as u16));
                                } else {
                                    w16(p, r16(p).wrapping_add(u12 as u16));
                                }
                            }
                        }
                    }
                    if r32(g().wrapping_add(i8).wrapping_add(0x1b)) == 0xffffffff
                        && r8(d().wrapping_add(0x96)) == 1
                        && r16(d().wrapping_add(0x1e5)) != 0
                        && {
                            unaff_ebx = r32(d().wrapping_add(0x19));
                            unaff_ebx == 0
                        }
                        && (r32(i7.wrapping_add(0x51)) as i32)
                            <= (r32(g().wrapping_add(i8).wrapping_add(0xc)) as i32)
                    {
                        w8(d().wrapping_add(0x96), 0);
                        w16(d().wrapping_add(0xd), 0x4000);
                    }
                    if r32(g().wrapping_add(0x1b).wrapping_add(i8)) == 0xffffffff
                        && r8(d().wrapping_add(0x97)) == 1
                        && r16(d().wrapping_add(0x1e5)) != 0
                        && r32(d().wrapping_add(0x19)) == 0
                    {
                        FUN_004adb50(local_14 as u32);
                    }
                    match r8(d().wrapping_add(7)) {
                        0 => {
                            w16(
                                g().wrapping_add(0x12).wrapping_add(i8),
                                r16(d().wrapping_add(0xd)),
                            );
                            let mut u13: u16 = r16(d().wrapping_add(0xc1));
                            if u13 == 0 || u_stack_12 != 0xffff {
                                let s_var6: i16 = r16(d().wrapping_add(0x92)) as i16;
                                let u15: u16 = (unaff_ebx >> 16) as u16;
                                let s_var17: i16 = (((6 - r8(d().wrapping_add(0x219)) as i32)
                                    - r8(d().wrapping_add(0x23e)) as i32)
                                    - r8(d().wrapping_add(0x21c)) as i32)
                                    as i16;
                                let c_var2: i8 = r8(g().wrapping_add(3).wrapping_add(i8)) as i8;
                                if c_var2 == 7 {
                                    u13 = 0x1000;
                                } else if c_var2 == 5 && 0 < s_var17 {
                                    u13 = 0x3000;
                                } else {
                                    u13 = 0x2000;
                                }
                                let mut u12 = FUN_00425b70(
                                    ((u15 as u32) << 16) | (s_var6 as u16 as u32),
                                    u13,
                                );
                                unaff_ebx = ((u15 as u32) << 16)
                                    | (s_var6.wrapping_add((u12 as i16).wrapping_mul(s_var17))
                                        as u16 as u32);
                                u12 = FUN_00425b70(unaff_ebx, u_stack_12);
                                u13 = u12 as u16;
                                if r16(d().wrapping_add(0xbf)) == 0 {
                                    u13 = u13.wrapping_mul(2);
                                }
                            }
                            let u16_16: u16 = r16(
                                r32(g().wrapping_add(0x1f).wrapping_add(i8)).wrapping_add(0x79)
                            );
                            if u13 < u16_16 {
                                let u12: u32 = (u16_16 as u32).wrapping_sub(u13 as u32);
                                if u12 < 200 {
                                    FUN_004ad880(local_14 as u32, u13 as u32, 1, u_stack_12);
                                } else {
                                    FUN_004ada90(local_14 as u32, u12 / 3);
                                }
                            } else {
                                FUN_004ad880(local_14 as u32, u13 as u32, 1, u_stack_12);
                            }
                        }
                        v @ (1 | 3 | 4 | 6) => {
                            if v != 6 {
                                let u13: u16 =
                                    r16(r32(g().wrapping_add(0x1f).wrapping_add(i8))
                                        .wrapping_add(0x79));
                                if r16(d().wrapping_add(0x92)) < u13 {
                                    unaff_ebx = 0;
                                    FUN_004ada90(
                                        local_14 as u32,
                                        (((u13 as u32)
                                            .wrapping_sub(r16(d().wrapping_add(0x92)) as u32))
                                            as i32
                                            / 3) as u32,
                                    );
                                }
                            }
                            w8(d().wrapping_add(0x96), 0);
                            w8(d().wrapping_add(0x97), 0);
                            w8(d().wrapping_add(0xa4), 0);
                            w8(d().wrapping_add(0x9c), 0);
                        }
                        2 => {
                            if r16(r32(g().wrapping_add(0x1f).wrapping_add(i8)).wrapping_add(0x79))
                                != 0
                            {
                                FUN_004ada90(local_14 as u32, 0x14);
                            }
                        }
                        5 => {
                            if r16(d().wrapping_add(0x1e5)) == 0 {
                                if r16(
                                    r32(g().wrapping_add(0x1f).wrapping_add(i8)).wrapping_add(0x79)
                                ) != 0
                                {
                                    FUN_004ada90(local_14 as u32, 0x14);
                                }
                                if r16(
                                    r32(g().wrapping_add(0x1f).wrapping_add(i8)).wrapping_add(0x79)
                                ) == 0
                                {
                                    w8(d().wrapping_add(7), 0);
                                }
                            } else if r16(i7.wrapping_add(100)) == 0 {
                                if r32(i7.wrapping_add(0x60)) == 0 {
                                    FUN_004ad9e0(local_14 as u32, 500);
                                } else {
                                    FUN_004ad9e0(local_14 as u32, 200);
                                }
                            } else {
                                FUN_004ad9e0(local_14 as u32, 0x32);
                            }
                        }
                        _ => {}
                    }
                    w16(
                        d().wrapping_add(0xf),
                        r16(g().wrapping_add(0x10).wrapping_add(i8)),
                    );
                    i14 = g().wrapping_add(i8);
                    if (r16(i14.wrapping_add(0x12)) as i16) != s_var3
                        || (r16(i14.wrapping_add(0x10)) as i16) != s_var4
                        || (r16(i14.wrapping_add(0x14)) as i16) != s_var5
                    {
                        w8(r32(i14.wrapping_add(0x1f)).wrapping_add(0x8b), 1);
                        w8(
                            r32(g().wrapping_add(0x1f).wrapping_add(i8)).wrapping_add(0x92),
                            1,
                        );
                    }
                    if r16(d().wrapping_add(0x66)) != 0xffff {
                        let i14b: u32 = (r16(d().wrapping_add(0x66)) as u32).wrapping_mul(0x23);
                        if r32(g().wrapping_add(i14b).wrapping_add(0x1f)) != 0 {
                            w16(
                                g().wrapping_add(i14b).wrapping_add(0x12),
                                r16(g().wrapping_add(0x12).wrapping_add(i8)),
                            );
                            w16(
                                g().wrapping_add(0x10).wrapping_add(i14b),
                                r16(g().wrapping_add(0x10).wrapping_add(i8)),
                            );
                            w16(
                                g().wrapping_add(0x14).wrapping_add(i14b),
                                r16(g().wrapping_add(0x14).wrapping_add(i8)),
                            );
                            unaff_ebx =
                                r32(r32(g().wrapping_add(0x1f).wrapping_add(i14b))
                                    .wrapping_add(0xa9));
                            w16(unaff_ebx.wrapping_add(0xd), r16(d().wrapping_add(0xd)));
                            w8(
                                r32(g().wrapping_add(0x1f).wrapping_add(i14b)).wrapping_add(0x8b),
                                1,
                            );
                            w8(
                                r32(g().wrapping_add(0x1f).wrapping_add(i14b)).wrapping_add(0x92),
                                1,
                            );
                        }
                    }
                }
                local_14 = local_14.wrapping_add(1);
                if !((local_14 as i32) < DAT_009a1fe0) {
                    break;
                }
            }
        }
        DAT_009d8112 = s_var10;
        DAT_009ec5fe = u_var11;
    }
}
