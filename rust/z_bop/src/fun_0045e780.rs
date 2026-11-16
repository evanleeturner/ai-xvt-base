// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a08104: *mut u16;
    static mut DAT_009a73b6: *mut u16;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1ff8: u32;
    static mut DAT_009a73b0: u16;
    static mut DAT_009a73d0: u16;
    static mut DAT_009a8070: u16;
    static mut DAT_00a080f0: u16;
    static mut DAT_009a8c00: u32;
    static mut DAT_009e9640: u16;
    static mut DAT_00a00854: i32;
    static mut DAT_00a07ce8: i32;
    static mut DAT_00a08294: u16;
    static mut DAT_009d1320: [u8; 0];
    static mut DAT_00524290: [u32; 0];
    static mut DAT_0052429c: [u8; 0];
    static mut DAT_0051a44f: [u8; 0];
    static mut DAT_0051a194: [u32; 0];
    static mut DAT_0051c5aa: [u8; 0];
    static mut DAT_0051a310: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00403540(a: u32, b: u32);
    fn FUN_004034e0(a: u32, b: u32);
    fn FUN_00484f80(a: u32) -> u32;
    fn FUN_00425b70(a: u32, b: u32) -> u32;
    fn FUN_0045f1e0(a: u32) -> u32;
    fn FUN_00406030(a: u32, b: u32);
}

unsafe fn rd8(a: u32) -> u8 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *const u8).read() }
}
unsafe fn rd16(a: u32) -> u16 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *const u16).read_unaligned() }
}
unsafe fn rd32(a: u32) -> u32 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *const u32).read_unaligned() }
}
unsafe fn wr8(a: u32, v: u8) {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *mut u8).write(v) }
}
unsafe fn wr16(a: u32, v: u16) {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *mut u16).write_unaligned(v) }
}
unsafe fn g() -> u32 {
    // SAFETY: the game's global pointer.
    unsafe { DAT_00a08104 as usize as u32 }
}
unsafe fn s() -> u32 {
    // SAFETY: the game's global pointer.
    unsafe { DAT_009a73b6 as usize as u32 }
}
unsafe fn fe8() -> u32 {
    // SAFETY: the game's global.
    unsafe { DAT_009a1fe8 as u32 }
}
fn ad(a: u32, b: u32) -> u32 {
    a.wrapping_add(b)
}
unsafe fn tab8(p: *mut u8, i: u32) -> u8 {
    // SAFETY: the game's own table.
    unsafe { p.add(i as usize).read() }
}
unsafe fn tab32(p: *mut u32, i: u32) -> u32 {
    // SAFETY: the game's own table.
    unsafe { p.add(i as usize).read() }
}
unsafe fn eq15(idx: u16) -> bool {
    // SAFETY: the game's own tables.
    unsafe {
        let a = (&raw mut DAT_009d1320)
            .cast::<u8>()
            .add(idx as usize * 0x55);
        let b = (&raw mut DAT_0051a310).cast::<u8>();
        let mut r = idx == 0;
        let mut i = 0usize;
        while i < 15 {
            r = a.add(i).read() == b.add(i).read();
            i += 1;
            if !r {
                break;
            }
        }
        r
    }
}
unsafe fn cmp15(idx: u16) -> u32 {
    // SAFETY: the game's own tables.
    unsafe {
        let a = (&raw mut DAT_009d1320)
            .cast::<u8>()
            .add(idx as usize * 0x55);
        let b = (&raw mut DAT_0051a310).cast::<u8>();
        let mut lt = false;
        let mut eq = true;
        let mut i = 0usize;
        while i < 15 {
            let x = a.add(i).read();
            let y = b.add(i).read();
            lt = x < y;
            eq = x == y;
            i += 1;
            if !eq {
                break;
            }
        }
        let mut r = 0u32;
        if !eq {
            r = 1u32
                .wrapping_sub(lt as u32)
                .wrapping_sub((lt as u32 != 0) as u32);
        }
        r
    }
}
unsafe fn special(b16: u8, c4: i8) -> bool {
    // SAFETY: the game's global.
    unsafe { DAT_00a08294 == 0xe && (b16 == 0x98 || b16 == 0x99) && c4 == 2 }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0045e780() -> u32 {
    // SAFETY: all addresses are the game's own memory, accessed where the
    // original accesses them.
    unsafe {
        if rd16(ad(g(), 0x1e5)) == 0 {
            return g() & 0xffff0000;
        }
        let mut u_var18: u16 = DAT_009a73b6.add(0xc).read();
        let mut i_var9: i32 = 0;
        let mut u_var1: u8 = 0;
        let mut local_c: u32 = 0;
        'l: {
            if u_var18 == 0xffff {
                break 'l;
            }
            i_var9 = (u_var18 as u32).wrapping_mul(0x23) as i32;
            let mut i_var17 = ad(fe8(), i_var9 as u32);
            let b26: bool;
            if rd32(ad(i_var17, 0x1f)) == 0 {
                b26 = rd8(ad(i_var17, 3)) == 0;
            } else {
                if rd8(ad(i_var17, 3)) == 0 {
                    i_var9 = 0;
                    break 'l;
                }
                i_var17 = rd32(ad(rd32(ad(i_var17, 0x1f)), 0xa9));
                if i_var17 == 0 {
                    i_var9 = 1;
                    break 'l;
                }
                if rd8(ad(i_var17, 0x5e)) == 0x15 && rd8(ad(i_var17, 0x5f)) != 0 {
                    i_var9 = 0;
                    break 'l;
                }
                let mut c4 = rd8(ad(i_var17, 7)) as i8;
                if c4 == 1 || c4 == 3 || c4 == 4 {
                    i_var9 = 0;
                    break 'l;
                }
                if rd8(ad(i_var17, 0x23d)) == 3 && rd8(ad(i_var17, 0x241)) != 0 {
                    FUN_00403540(u_var18 as u32, DAT_009a73b0 as u32);
                    c4 = rd8(ad(ad((DAT_009a73b0 as u32).wrapping_mul(0x23), 2), fe8())) as i8;
                    if c4 == 0 || c4 == 1 || c4 == 2 {
                        if 0x8000 < DAT_009a1ff8 {
                            i_var9 = 0;
                            break 'l;
                        }
                    } else if 0x10000 < DAT_009a1ff8 {
                        i_var9 = 0;
                        break 'l;
                    }
                }
                i_var9 = ad(fe8(), i_var9 as u32) as i32;
                if rd32(ad(i_var9 as u32, 0x1b)) != 0xffffffff {
                    i_var9 = 1;
                    break 'l;
                }
                b26 = rd32(ad(
                    rd32(rd32(ad(i_var9 as u32, 0x1f)).wrapping_add(0xa9)),
                    0xcb,
                )) < rd32(ad(i_var17, 0xc3));
            }
            i_var9 = 1u32.wrapping_sub(b26 as u32) as i32;
        }
        if i_var9 == 0 {
            local_c = g();
            u_var18 = 0;
            if rd8(ad(g(), 0x21b)) != 0 {
                loop {
                    wr8(ad(ad(u_var18 as u32, 0x220), g()), 0);
                    local_c = u_var18.wrapping_add(1) as u32;
                    u_var18 = u_var18.wrapping_add(1);
                    if !(local_c < rd8(ad(g(), 0x21b)) as u32) {
                        break;
                    }
                }
            }
        } else {
            let mut u_var14: u32 = u_var18 as u32;
            let mut u_var19: u32 =
                tab32((&raw mut DAT_00524290).cast::<u32>(), DAT_009a73d0 as u32);
            if (u_var14 as i32) < DAT_00a07ce8 {
                let i9b = ad(fe8(), u_var14.wrapping_mul(0x23));
                if rd32(ad(i9b, 0x1f)) != 0 {
                    if 0x19 < rd16(ad(rd32(ad(i9b, 0x1f)), 0x79)) {
                        let mut u_var7 = rd16(ad(
                            ad((DAT_009a73b0 as u32).wrapping_mul(0x23), 0x10),
                            fe8(),
                        ))
                        .wrapping_sub(rd16(ad(i9b, 0x10)));
                        if 0x7fff < u_var7 {
                            u_var7 = (-(u_var7 as i32)) as u16;
                        }
                        if u_var7 < 0x2000 {
                            u_var19 = u_var19.wrapping_sub(0x4000);
                        } else if u_var7 < 0x5000 {
                            u_var19 = u_var19.wrapping_sub(0x2000);
                        }
                    }
                    if rd8(ad(i9b, 2)) == 5 || rd8(ad(i9b, 2)) == 4 {
                        u_var19 = u_var19.wrapping_add(0x6000);
                    }
                }
            }
            FUN_004034e0(DAT_009a73b0 as u32, u_var14);
            let mut u_stack_1c: u16 = DAT_009a8070.wrapping_sub(rd16(ad(
                ad((DAT_009a73b0 as u32).wrapping_mul(0x23), 0x10),
                fe8(),
            )));
            if 0x7fff < u_stack_1c {
                u_stack_1c = (-(u_stack_1c as i32)) as u16;
            }
            let mut local_1a: u16 = DAT_00a080f0.wrapping_sub(rd16(ad(g(), 0xd)));
            if 0x7fff < local_1a {
                local_1a = (-(local_1a as i32)) as u16;
            }
            let b_var16: u8;
            if u_stack_1c < 0x800 && local_1a < 0x800 && DAT_009a8c00 < u_var19 {
                let mut t = 3u8;
                if 0x1fff < (DAT_009a8c00 as i32) {
                    t = (((DAT_009a8c00 as i32) < 0x4000) as u8).wrapping_add(1);
                }
                b_var16 = t;
                u_var1 = tab8((&raw mut DAT_0052429c).cast::<u8>(), DAT_009a73d0 as u32);
            } else {
                b_var16 = 0;
            }
            let b_var2 = rd8(ad(g(), 0x21b));
            let b_var3 = rd8(ad(s(), 0xb));
            let mut u_var12: u16 = b_var3 as u16;
            let mut u_var7: u16 = 0;
            if b_var2 != 0 {
                loop {
                    let mut b_var11: u8 = 0;
                    'blk: {
                        if b_var16 == 0 {
                            break 'blk;
                        }
                        b_var11 = b_var16;
                        if (rd8(ad(ad(u_var7 as u32, 0x21e), g())) as i8) == -0x73 {
                            let e = eq15(u_var12);
                            if !e || (DAT_009a73b6.add(0xf).read() as u8 as i8) != 1 {
                                b_var11 = 0;
                                break 'blk;
                            }
                        } else if (DAT_009a73b6.add(0xf).read() as u8 as i8) != 0 {
                            let r = cmp15(u_var12);
                            b_var11 =
                                ((((r == 0) as u32).wrapping_sub(1)) & (b_var16 as u32)) as u8;
                        }
                    }
                    let u_var8 = u_var7.wrapping_add(1);
                    wr8(ad(ad(u_var7 as u32, 0x220), g()), b_var11);
                    wr8(ad(ad(u_var7 as u32, 0x222), g()), u_var1);
                    u_var7 = u_var8;
                    if !((u_var8 as u32) < b_var2 as u32) {
                        break;
                    }
                }
            }
            local_c = DAT_009a1fe0 as u32;
            let mut ps_var6: u32 = 0;
            let mut cond = false;
            if (u_var14 as i32) < DAT_009a1fe0 {
                local_c = FUN_00484f80(u_var18 as u32);
                ps_var6 = g();
                cond = (local_c as u8 as i8) == 0;
            }
            if cond {
                i_var9 = u_var14.wrapping_mul(0x23) as i32;
                let mut i_var17 = ad(fe8(), i_var9 as u32);
                let mut c4 = rd8(ad(i_var17, 2)) as i8;
                let local_22: u16;
                let local_20: u16;
                let local_10: u32;
                if c4 == 4 || c4 == 5 || c4 == 3 || (c4 == 1 && DAT_00a08294 == 0xe) {
                    c4 = 2;
                    local_20 = 6;
                    local_22 = 0x10;
                    local_10 = ((0u32.wrapping_sub((DAT_009a73d0 == 2) as u32)) & 0x9f12)
                        .wrapping_add(0x31b5a);
                } else {
                    local_10 = 0x18dad;
                    local_22 = 2;
                    c4 = 1;
                    local_20 = 1;
                }
                local_c = 0;
                let i_var24 = rd32(ad(rd32(ad(i_var17, 0x1f)), 0xa9));
                let e = eq15(u_var12);
                let mut ps_var15 = rd32(ad(i_var24, 0x211));
                if e {
                    u_var19 = rd32(ad(i_var24, 0xcb)) / 10;
                    if u_var19 < rd32(ad(i_var24, 0xc3)) {
                        ps_var15 = ps_var15.wrapping_add(u_var19);
                    }
                    u_var19 = 0;
                    if rd8(ad(g(), 0x232)) != 0 {
                        loop {
                            let b16 = rd8(ad(ad(u_var19, 0x233), g()));
                            let mut hit = (tab8((&raw mut DAT_0051a44f).cast::<u8>(), b16 as u32)
                                as i32)
                                == c4 as i32;
                            if !hit {
                                hit = special(b16, c4);
                            }
                            if hit {
                                u_var14 = tab32((&raw mut DAT_0051a194).cast::<u32>(), b16 as u32);
                                let c5 = rd8(ad(i_var17, 2)) as i8;
                                if c5 == 4 || c5 == 5 {
                                    u_var14 >>= 4;
                                }
                                if c5 == 3 {
                                    u_var14 >>= 2;
                                }
                                local_c = local_c.wrapping_add(u_var14);
                            }
                            u_var19 = (u_var19 as u16).wrapping_add(1) as u32;
                            if !(u_var19 < rd8(ad(g(), 0x232)) as u32) {
                                break;
                            }
                        }
                    }
                } else {
                    ps_var15 = ps_var15.wrapping_add(
                        rd32(ad(i_var24, 0xcb)).wrapping_sub(rd32(ad(i_var24, 0xc3))),
                    );
                }
                u_var7 = DAT_009e9640;
                let mut ps_var10: u32 = DAT_009e9640 as u32;
                while (ps_var10 as i32) < DAT_00a00854 {
                    let i24 = ad((u_var7 as u32).wrapping_mul(0x23), fe8());
                    if 0x88 < rd8(ad(i24, 3)) {
                        let i13 = rd32(ad(rd32(ad(i24, 0x1f)), 0xa5));
                        if rd8(ad(i13, 1)) != 0 && rd16(ad(i13, 4)) == u_var18 {
                            u_var19 = tab32(
                                (&raw mut DAT_0051a194).cast::<u32>(),
                                rd8(ad(i24, 3)) as u32,
                            );
                            let c5 = rd8(ad(i_var17, 2)) as i8;
                            if c5 == 4 || c5 == 5 {
                                u_var19 >>= 4;
                            }
                            if c5 == 3 {
                                u_var19 >>= 2;
                            }
                            local_c = local_c.wrapping_add(u_var19);
                        }
                    }
                    u_var7 = u_var7.wrapping_add(1);
                    ps_var10 = u_var7 as u32;
                }
                if local_c < ps_var15 {
                    u_var7 = 0;
                    u_var12 = DAT_009e9640;
                    local_c = DAT_00a00854 as u32;
                    ps_var10 = DAT_009e9640 as u32;
                    while (ps_var10 as i32) < DAT_00a00854 {
                        i_var17 = ad((u_var12 as u32).wrapping_mul(0x23), fe8());
                        let b16 = rd8(ad(i_var17, 3));
                        let mut u_var8 = u_var7;
                        if b16 != 0 {
                            let mut skip = false;
                            if (tab8((&raw mut DAT_0051a44f).cast::<u8>(), b16 as u32) as i32)
                                != c4 as i32
                            {
                                if !special(b16, c4) {
                                    skip = true;
                                }
                            }
                            if !skip {
                                i_var17 = rd32(ad(rd32(ad(i_var17, 0x1f)), 0xa5));
                                if rd8(ad(i_var17, 1)) != 0 && rd16(ad(i_var17, 4)) == u_var18 {
                                    u_var8 = u_var7.wrapping_add(1);
                                    if special(b16, c4) {
                                        u_var8 = u_var7.wrapping_add(8);
                                    }
                                }
                            }
                        }
                        u_var12 = u_var12.wrapping_add(1);
                        local_c = u_var12 as u32;
                        u_var7 = u_var8;
                        ps_var10 = local_c;
                    }
                    if u_var7 < local_22 {
                        local_c = s();
                        if (DAT_009a73b6.add(0x1a).read() as u8 as i8) == 0x17 {
                            local_c = g();
                            if rd16(ad(g(), 0x1e7)) == 0 {
                                if rd32(ad(g(), 0x1d)) == 0 {
                                    local_c = local_20 as u32;
                                    if (rd8(ad(g(), 0x78)) as u32) < local_c {
                                        if u_stack_1c < 0x300
                                            && local_1a < 0x300
                                            && DAT_009a8c00 < local_10
                                        {
                                            u_var7 = ((DAT_009a73d0 as u32)
                                                .wrapping_add(1)
                                                .wrapping_mul(0x1d8))
                                                as u16;
                                            if DAT_009a73d0 == 2 {
                                                wr16(
                                                    ad(g(), 0x23b),
                                                    rd16(ad(g(), 0x23b)).wrapping_add(rd32(ad(
                                                        s(),
                                                        0xe,
                                                    ))
                                                        as u16),
                                                );
                                                if rd32(ad(g(), 199)) <= rd32(ad(g(), 0xc3)) {
                                                    u_var7 = 0x2c4;
                                                }
                                            } else {
                                                u_var19 = FUN_00425b70(rd32(ad(s(), 0xe)), 0xc000);
                                                wr16(
                                                    ad(ps_var6, 0x23b),
                                                    rd16(ad(ps_var6, 0x23b))
                                                        .wrapping_add(u_var19 as u16),
                                                );
                                            }
                                            local_c = g();
                                            if (u_var7 as i32)
                                                <= (rd16(ad(g(), 0x23b)) as i16 as i32)
                                                && {
                                                    u_var19 = 0;
                                                    rd8(ad(g(), 0x232)) != 0
                                                }
                                            {
                                                loop {
                                                    ps_var6 = g();
                                                    let u_stack_14 = ps_var15 as u16;
                                                    if rd16(ad(
                                                        ad(g(), u_var19.wrapping_mul(2)),
                                                        0x237,
                                                    )) == 0
                                                    {
                                                        let mut b16 =
                                                            rd8(ad(ad(g(), u_var19), 0x233));
                                                        let mut skip = false;
                                                        if (tab8(
                                                            (&raw mut DAT_0051a44f).cast::<u8>(),
                                                            b16 as u32,
                                                        )
                                                            as i32)
                                                            != c4 as i32
                                                        {
                                                            let b26: bool;
                                                            if special(b16, c4) {
                                                                if u_var18 == 0xffff || {
                                                                    u_var7 = rd16(ad(
                                                                        rd32(ad(
                                                                            rd32(ad(
                                                                                ad(
                                                                                    ad(fe8(), 0x1f),
                                                                                    i_var9 as u32,
                                                                                ),
                                                                                0,
                                                                            )),
                                                                            0xa9,
                                                                        )),
                                                                        0x1e7,
                                                                    ));
                                                                    u_var7 == 0
                                                                } {
                                                                    b26 = true;
                                                                } else {
                                                                    b26 = u_var7 < 0x49c;
                                                                }
                                                            } else {
                                                                b26 = false;
                                                            }
                                                            if !b26 {
                                                                skip = true;
                                                            }
                                                        }
                                                        if !skip {
                                                            if DAT_009a73d0 == 2
                                                                && rd32(ad(g(), 199))
                                                                    <= rd32(ad(g(), 0xc3))
                                                            {
                                                                wr8(ad(ad(g(), u_var19), 0x235), 3);
                                                            } else {
                                                                let idx = tab8(
                                                                    (&raw mut DAT_0051c5aa)
                                                                        .cast::<u8>(),
                                                                    ad(
                                                                        u_var19,
                                                                        (rd8(ad(g(), 4)) as u32)
                                                                            .wrapping_mul(0x101),
                                                                    ),
                                                                )
                                                                    as u32;
                                                                b16 = rd8(ad(
                                                                    ad(g(), idx.wrapping_mul(4)),
                                                                    0x376,
                                                                ));
                                                                let b2 = rd8(ad(
                                                                    ad(g(), idx.wrapping_mul(4)),
                                                                    0x372,
                                                                ));
                                                                wr8(ad(ad(g(), u_var19), 0x235), 1);
                                                                if b2 < b16 {
                                                                    wr8(
                                                                        ad(
                                                                            ad(ps_var6, u_var19),
                                                                            0x235,
                                                                        ),
                                                                        0x81,
                                                                    );
                                                                }
                                                            }
                                                            u_var14 = FUN_0045f1e0(
                                                                ((u_stack_14 as u32) << 16)
                                                                    | u_var18 as u32,
                                                            );
                                                            DAT_009a73b6
                                                                .add(0xe)
                                                                .write(u_var14 as u16);
                                                            FUN_00406030(
                                                                DAT_009a73b0 as u32,
                                                                u_var19,
                                                            );
                                                            wr8(
                                                                ad(g(), 0x78),
                                                                rd8(ad(g(), 0x78)).wrapping_add(1),
                                                            );
                                                            let c5 = rd8(ad(
                                                                ad(fe8(), 2),
                                                                i_var9 as u32,
                                                            ))
                                                                as i8;
                                                            if c5 == 0 || c5 == 1 {
                                                                wr16(ad(g(), 0x23b), 0);
                                                            }
                                                        }
                                                    }
                                                    u_var19 =
                                                        (u_var19 as u16).wrapping_add(1) as u32;
                                                    if rd8(ad(g(), 0x232)) as u32 <= u_var19 {
                                                        return 0;
                                                    }
                                                }
                                            }
                                        } else {
                                            wr16(
                                                ad(g(), 0x23b),
                                                rd16(ad(g(), 0x23b))
                                                    .wrapping_sub(rd32(ad(s(), 0xe)) as u16),
                                            );
                                            local_c = ad(g(), 0x23b);
                                            if (rd16(local_c) as i16) < 0 {
                                                wr16(local_c, 0);
                                                return local_c & 0xffff0000;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        local_c & 0xffff0000
    }
}
