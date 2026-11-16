// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: *mut u8;
    static mut DAT_006d4980: i32;
    static mut DAT_0070ad48: [u8; 1];
    static mut DAT_0070ad40: [u8; 1];
    static mut DAT_006d4a72: [u8; 1];
    static mut DAT_006d4aa6: [u8; 1];
    static mut DAT_0085e795: i8;
    static mut DAT_0071b73c: [u8; 1];
    static mut DAT_0071b741: [u8; 1];
    static mut DAT_0071ecf4: i32;
    static mut DAT_00525bf7: [u8; 1];
    static mut DAT_0071f2d8: [u8; 4];
    static mut DAT_0071f2dc: u8;
    static mut DAT_0071f2dd: u8;
    static mut DAT_006d4974: i32;
    static mut DAT_006d4978: i32;
    static mut DAT_006d4970: i32;
    static mut DAT_0071e6c4: i32;
    static mut DAT_00732f38: i32;
    static mut DAT_00732fe8: i32;
    static mut DAT_007092df: i8;
    static mut DAT_0070a17c: i32;
    static mut DAT_007091a8: i32;
    static mut DAT_006ffbe0: i32;
    static mut DAT_00523b9e: u16;
    static mut _DAT_00732950: i16;
    static mut _DAT_00732952: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040c070(a: i32, b: i32, c: i32, d: i32);
    fn FUN_004a1590(a: i32, b: u32, c: i32, d: i32, e: u32, f: u32);
    fn FUN_004440b0(a: u32, b: u32) -> u8;
    fn FUN_004a18c0(a: u32, b: u32, c: u32) -> i32;
    fn FUN_00483410(a: i32, b: i32, c: i32);
    fn FUN_00483540(a: i32, b: i32, c: i32);
    fn FUN_00413d00(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32;
    fn FUN_0040f260(a: u16, b: u16);
    fn FUN_004a1d70(a: i32, b: i32);
    fn FUN_004110e0(a: u32, b: u32, c: u32, d: u32, e: u32, f: u16, g: u32, h: u16, i: u32) -> u32;
    fn FUN_0040a1a0(a: i32, b: u32, c: i32);
    fn FUN_004784d0() -> u32;
}

unsafe fn p(x: *mut u8, o: u32) -> *mut u8 {
    x.wrapping_add(o as usize)
}
unsafe fn r16(x: *const u8) -> u16 {
    // SAFETY: the caller supplies a readable game address.
    unsafe { x.cast::<u16>().read_unaligned() }
}
unsafe fn r32(x: *const u8) -> u32 {
    // SAFETY: the caller supplies a readable game address.
    unsafe { x.cast::<u32>().read_unaligned() }
}
unsafe fn w16(x: *mut u8, v: u16) {
    // SAFETY: the caller supplies a writable game address.
    unsafe { x.cast::<u16>().write_unaligned(v) }
}
unsafe fn w32(x: *mut u8, v: u32) {
    // SAFETY: the caller supplies a writable game address.
    unsafe { x.cast::<u32>().write_unaligned(v) }
}
unsafe fn pt(x: *const u8) -> *mut u8 {
    // SAFETY: the caller supplies a readable game address holding a pointer.
    unsafe { x.cast::<*mut u8>().read_unaligned() }
}
unsafe fn d() -> *mut u8 {
    // SAFETY: the game's own global.
    unsafe { DAT_006d4988 }
}
unsafe fn a(o: u32) -> *mut u8 {
    // SAFETY: reads the game's own global.
    unsafe { p(d(), o) }
}

#[unsafe(no_mangle)]
#[allow(
    non_snake_case,
    unused_assignments,
    unused_variables,
    unused_mut,
    unused_unsafe
)]
pub unsafe extern "C" fn FUN_00410810(
    param_1: u32,
    param_2: u32,
    param_3: i32,
    param_4: u32,
    param_5: u32,
    param_6: u32,
    param_7: u32,
) {
    // SAFETY: all addresses are the game's own memory, accessed as the original does.
    unsafe {
        let unaff_bx: u16 = 0;
        let unaff_ebp: u32 = 0;
        let unaff_esi: u32 = 0;
        let unaff_di: u16 = 0;
        let in_stack_ffffffe0: u16 = 0;
        let mut local_18: u32 = 0;
        let mut local_14: u32 = 0;
        let c_stack_1c: i8;
        let mut iv15: i32;

        let u_var10 = param_1 & 0xffff;
        let i_var17 = u_var10.wrapping_mul(0x23);
        let mut u_var18: u16 = r16(p(pt(a(i_var17.wrapping_add(0x1f))), 0x83));
        let u_var12 = u_var18 as u32;
        let i_var15u = u_var12.wrapping_mul(0x23);
        if (param_2 as u16) == u_var18 {
            return;
        }
        let i_var5 = (param_2 & 0xffff).wrapping_mul(0x23);
        let i_var4: *mut u8 = pt(p(pt(a(i_var5.wrapping_add(0x1f))), 0xa7));
        let idx0 = (*a(i_var15u.wrapping_add(0x16)) as u32).wrapping_mul(0x566);
        let pbvar2: *mut u8 = p(
            i_var4,
            0x1e6u32.wrapping_add(
                *(&raw mut DAT_0070ad48)
                    .cast::<u8>()
                    .wrapping_add(idx0 as usize) as u32,
            ),
        );
        if *pbvar2 == 0 {
            *pbvar2 = 1;
            let i_var14p = a(i_var5);
            let u_var11 = *p(i_var14p, 0x16) as u32;
            local_18 = u_var11.wrapping_mul(0x126);
            let q = (&raw mut DAT_006d4a72)
                .cast::<u8>()
                .wrapping_add(local_18 as usize);
            w16(q, r16(q).wrapping_add(1));
            local_14 = u_var11.wrapping_mul(0x566);
            if *(&raw mut DAT_0070ad40)
                .cast::<u8>()
                .wrapping_add(local_14 as usize)
                == *p(i_var4, 0xb0)
            {
                *(&raw mut DAT_006d4aa6)
                    .cast::<u8>()
                    .wrapping_add(local_18 as usize) = 1;
            }
            let mut cond = false;
            if DAT_0085e795 == 2 {
                local_18 = r16((&raw mut DAT_0071b73c)
                    .cast::<u8>()
                    .wrapping_add(DAT_0071ecf4.wrapping_mul(0x5bd) as usize))
                    as u32;
                if (*(&raw mut DAT_0070ad48)
                    .cast::<u8>()
                    .wrapping_add(local_14 as usize) as u32)
                    == local_18
                {
                    if *p(i_var14p, 2) != 0 {
                        cond = true;
                    }
                }
            }
            if cond {
                let iv14: i32 = 'l: {
                    if (u_var12 as i32) < DAT_006d4980 {
                        let c3 = *a(i_var17.wrapping_add(3)) as i8;
                        if c3 == -0x70 || c3 == -0x6b {
                            break 'l 0x2b;
                        } else if c3 == -0x71 || c3 == -0x6c {
                            break 'l 0x2c;
                        } else if c3 == -0x69 {
                            break 'l 0x2d;
                        } else if c3 == -0x6a {
                            break 'l 0x2e;
                        } else {
                            let c = *a(i_var15u.wrapping_add(2)) as i8;
                            if c == 0 {
                                break 'l 0x19;
                            }
                            if c != 4 {
                                break 'l 0x18;
                            }
                            break 'l 0x1a;
                        }
                    }
                    0x18
                };
                FUN_0040c070(1, iv14, (param_2 & 0xffff) as i32, 0xffff);
            }
        }
        let iv14 = r32(a(i_var15u.wrapping_add(0x1b))) as i32;
        if iv14 != -1 {
            if (*pbvar2 & 0x80) == 0 {
                let u_var8 = *a(i_var5.wrapping_add(0x16)) as u16;
                local_18 = (local_18 & 0xffff0000) | u_var8 as u32;
                local_14 = (*(&raw mut DAT_0070ad40)
                    .cast::<u8>()
                    .wrapping_add((u_var8 as u32).wrapping_mul(0x566) as usize)
                    == *p(i_var4, 0xb0)) as u32;
                FUN_004a1590(
                    3,
                    local_18,
                    iv14,
                    1,
                    local_14,
                    r16((&raw mut DAT_0071b73c)
                        .cast::<u8>()
                        .wrapping_add(iv14.wrapping_mul(0x5bd) as usize))
                        as u32,
                );
                let e = r32(a(i_var15u.wrapping_add(0x1b))) as i32;
                FUN_004a1590(
                    3,
                    *a(i_var5.wrapping_add(0x16)) as u32,
                    -1,
                    1,
                    local_14,
                    r16((&raw mut DAT_0071b73c)
                        .cast::<u8>()
                        .wrapping_add(e.wrapping_mul(0x5bd) as usize)) as u32,
                );
            }
            *pbvar2 |= 0x80;
        }
        'c5c: {
            if (r16(p(i_var4, 0x6a)) as i16) == -1 {
                if DAT_006d4980 <= u_var12 as i32 {
                    break 'c5c;
                }
                let pent = a(i_var15u);
                let c3 = *p(pent, 2) as i8;
                if c3 == 4 || c3 == 5 || c3 == 3 {
                    break 'c5c;
                }
                let mut b6 = true;
                if (r32(p(pent, 0x1b)) as i32) != -1 {
                    let b7 = FUN_004440b0(
                        *p(pt(a(i_var5.wrapping_add(0x1f))), 0x87) as u32,
                        *p(pt(p(pent, 0x1f)), 0x87) as u32,
                    );
                    if b7 == 0 && *a(i_var5.wrapping_add(2)) != 0 {
                        let mut b16: u8 = (*pbvar2 & 0x70) >> 4;
                        if b16 < 5 {
                            let t = *(&raw mut DAT_00525bf7)
                                .cast::<u8>()
                                .wrapping_add(*a(i_var17.wrapping_add(3)) as usize);
                            b16 = b16.wrapping_add((((t == 0) as u32).wrapping_sub(1) & 4) as u8);
                            if 7 < b16 {
                                b16 = 7;
                            }
                            *pbvar2 = (b16 << 4) | (*pbvar2 & 0x8f);
                        }
                        if b16 < 7 {
                            b6 = false;
                        }
                    }
                }
                if !b6 {
                    break 'c5c;
                }
            } else if (r32(a(i_var5.wrapping_add(0x1b))) as i32) == -1 {
                break 'c5c;
            }
            w16(p(i_var4, 0x6a), u_var18);
            iv15 = FUN_004a18c0(
                DAT_0071f2d8[3] as u32,
                DAT_0071f2dc as u32,
                DAT_0071f2dd as u32,
            );
            w16(p(i_var4, 0x6c), iv15 as u16);
        }
        w16(p(i_var4, 0x6e), u_var18);
        *p(i_var4, 0x79) = (*p(i_var4, 0x79)).wrapping_add(1);
        let pent = a(i_var5);
        if *p(pt(p(pent, 0x1f)), 0x90) != 0 {
            FUN_00483410(
                r16(p(pent, 0x12)) as i16 as i32,
                r16(p(pent, 0x10)) as i16 as i32,
                pent as i32,
            );
            FUN_00483540(
                r16(a(i_var5.wrapping_add(0x14))) as i16 as i32,
                0,
                a(i_var5) as i32,
            );
        }
        {
            let p_in = pt(a(i_var5.wrapping_add(0x1f)));
            iv15 = FUN_00413d00(
                r16(p(p_in, 0x91)) as i16 as i32,
                r16(p(p_in, 0x93)) as i16 as i32,
                r16(p(p_in, 0x95)) as i16 as i32,
                (DAT_006d4974 as i16).wrapping_sub(DAT_0071e6c4 as i16) as i32,
                (DAT_006d4978 as i16).wrapping_sub(DAT_00732f38 as i16) as i32,
                (DAT_006d4970 as i16).wrapping_sub(DAT_00732fe8 as i16) as i32,
            );
        }
        if (r32(a(i_var5.wrapping_add(0x1b))) as i32) == -1 {
            local_14 = (local_14 >> 16) << 16;
        } else {
            local_14 = (local_14 & 0xffff0000) | ((-1 < (iv15 as i16)) as u32);
        }
        if DAT_007092df != 0
            && *p(i_var4, 7) == 0
            && *(&raw mut DAT_00525bf7)
                .cast::<u8>()
                .wrapping_add(*a(i_var17.wrapping_add(3)) as usize)
                != 0
        {
            FUN_0040f260(param_2 as u16, param_1 as u16);
        }
        let b16 = *a(i_var17.wrapping_add(3));
        if b16 == 0x98 {
            u_var18 = r16(p(i_var4, 0x1e1));
            if (r32(a(i_var5.wrapping_add(0x1b))) as i32) == -1 {
                let c3 = *a(i_var5.wrapping_add(2)) as i8;
                let s9: u16 = if c3 == 0 || c3 == 1 || c3 == 2 {
                    u_var18.wrapping_add(0xdd4)
                } else {
                    u_var18.wrapping_add(0x1ba8)
                };
                w16(p(i_var4, 0x1e1), s9);
            } else {
                let mut u8v: u16 = 0;
                if *p(i_var4, 0x213) != 0 {
                    loop {
                        let u13 = u8v.wrapping_add(1);
                        *p(i_var4, 0x345u32.wrapping_add((u8v as u32).wrapping_mul(4))) = 0;
                        u8v = u13;
                        if !(u13 < *p(i_var4, 0x213) as u16) {
                            break;
                        }
                    }
                }
                if (r16(p(i_var4, 0x1df)) & 0x10) == 0 {
                    FUN_004a1d70(0x111, r32(a(i_var5.wrapping_add(0x1b))) as i32);
                } else {
                    w16(p(i_var4, 0x1df), r16(p(i_var4, 0x1df)) & 0x3ef);
                    w16(p(i_var4, 0x2a4), 0);
                    w16(p(i_var4, 0x2ba), DAT_00523b9e);
                    if (r32(a(i_var5.wrapping_add(0x1b))) as i32) == DAT_0071ecf4
                        && *(&raw mut DAT_0071b741)
                            .cast::<u8>()
                            .wrapping_add(DAT_0071ecf4.wrapping_mul(0x5bd) as usize)
                            == 0
                    {
                        _DAT_00732950 = 0x5b;
                        _DAT_00732952 = 0x57;
                        FUN_004a1d70(0x56, DAT_0071ecf4);
                    }
                }
            }
            if r16(p(i_var4, 0x1e1)) < u_var18 {
                w16(p(i_var4, 0x1e1), 0xffff);
            }
            c_stack_1c = 1;
        } else {
            let mut cs: i8 = 0;
            if *p(i_var4, 0x23c) == 1
                && r16(p(i_var4, 0x23e)) != 0
                && *(&raw mut DAT_00525bf7)
                    .cast::<u8>()
                    .wrapping_add(b16 as usize)
                    != 0
                && -1 < (iv15 as i16)
            {
                cs = 1;
                FUN_004a1d70(0x167, r32(a(i_var5.wrapping_add(0x1b))) as i32);
            }
            if cs == 0 {
                let u12 = FUN_004110e0(
                    param_2,
                    param_3 as u32,
                    param_1,
                    local_14,
                    unaff_ebp,
                    unaff_di,
                    unaff_esi,
                    unaff_bx,
                    ((u_var18 as u32) << 16) | in_stack_ffffffe0 as u32,
                );
                cs = u12 as i8;
            }
            c_stack_1c = cs;
        }
        w32(
            a(i_var17.wrapping_add(4)),
            (DAT_0070a17c as u32).wrapping_add(DAT_0071e6c4 as u32),
        );
        w32(
            a(i_var17.wrapping_add(8)),
            (DAT_00732f38 as u32).wrapping_add(DAT_007091a8 as u32),
        );
        w32(
            a(i_var17.wrapping_add(0xc)),
            (DAT_00732fe8 as u32).wrapping_add(DAT_006ffbe0 as u32),
        );
        let b16 = *a(i_var17.wrapping_add(3));
        let puvar1 = a(i_var17.wrapping_add(3));
        if *(&raw mut DAT_00525bf7)
            .cast::<u8>()
            .wrapping_add(b16 as usize)
            == 0
        {
            if b16 == 0x8d || b16 == 0x8e {
                *puvar1 = 0x84;
            } else {
                *puvar1 = 0x83;
            }
        } else {
            *puvar1 = 0x81;
        }
        *a(i_var17.wrapping_add(2)) = 0xd;
        *pt(a(i_var17.wrapping_add(0x1f))) = 5;
        *a(i_var17.wrapping_add(0x19)) = 2;
        w16(p(pt(a(i_var17.wrapping_add(0x1f))), 0x81), 0);
        w16(p(pt(a(i_var17.wrapping_add(0x1f))), 0x7f), 0);
        *p(pt(a(i_var17.wrapping_add(0x1f))), 1) = 0;
        w16(
            p(pt(a(i_var17.wrapping_add(0x1f))), 0x79),
            r16(p(pt(a(i_var5.wrapping_add(0x1f))), 0x79)),
        );
        w16(
            a(i_var17.wrapping_add(0x12)),
            r16(a(i_var5.wrapping_add(0x12))),
        );
        w16(
            a(i_var17.wrapping_add(0x10)),
            r16(a(i_var5.wrapping_add(0x10))),
        );
        w16(a(i_var17.wrapping_add(0x14)), 0);
        *p(pt(a(i_var17.wrapping_add(0x1f))), 0x90) = 1;
        {
            let p_in2 = pt(a(i_var17.wrapping_add(0x1f)));
            *p(p_in2, 0x89) = *p(p_in2, 0x90);
        }
        if c_stack_1c != 0 {
            if (r32(a(i_var5.wrapping_add(0x1b))) as i32) == DAT_0071ecf4 {
                FUN_0040a1a0(0x1d, u_var10, DAT_0071ecf4);
                return;
            }
            let c3 = *a(i_var17.wrapping_add(3)) as i8;
            if c3 != -0x7d && c3 != -0x7c {
                let i15 = DAT_0071ecf4;
                let u12 = FUN_004784d0();
                FUN_0040a1a0(((u12 & 3).wrapping_add(0x16)) as i32, u_var10, i15);
                return;
            }
            FUN_0040a1a0(0x1c, u_var10, DAT_0071ecf4);
        }
    }
}
