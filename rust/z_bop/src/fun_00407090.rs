// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: u32;
    static mut DAT_00a082d8: u16;
    static mut DAT_009e9640: u32;
    static mut DAT_00a00854: u32;
    static mut DAT_009a8e18: u32;
    static mut DAT_009a20b0: u32;
    static mut DAT_009cd26c: u32;
    static mut DAT_009fd438: u32;
    static mut DAT_009a8c00: u32;
    static mut _DAT_00a07cec: i32;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009ecc34: i32;
    static mut DAT_0051a44f: [u8; 0];
    static mut DAT_0051a194: [u32; 0];
    static mut DAT_0051a2fc: [u8; 0];
    static mut DAT_009d8c84: [u8; 0];
    static mut DAT_009d9155: [u8; 0];
    static mut DAT_009d8c88: [u8; 0];
    static mut DAT_009fd458: [u8; 0];
    static mut DAT_009e967c: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00459750(a: i32) -> u16;
    fn FUN_004269e0(a: u32);
    fn FUN_00405860(a: i32) -> i16;
    fn FUN_004ae120(a: u32) -> i64;
    fn FUN_004ae080(a: u32) -> i64;
    fn FUN_00427bd0(a: i16, b: i16, c: i32);
    fn FUN_004464c0(a: i32);
    fn FUN_004034e0(a: u32, b: u32);
    fn FUN_00430610(a: i32, b: i8, c: i8, d: i8, e: i32);
    fn FUN_0042e720(a: u32, b: i32);
}

unsafe fn rb(a: u32) -> u8 {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *const u8).read() }
}
unsafe fn rw(a: u32) -> u16 {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *const u16).read_unaligned() }
}
unsafe fn rd(a: u32) -> u32 {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *const u32).read_unaligned() }
}
unsafe fn wb(a: u32, v: u8) {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *mut u8).write(v) }
}
unsafe fn ww(a: u32, v: u16) {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *mut u16).write_unaligned(v) }
}
unsafe fn wd(a: u32, v: u32) {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as usize as *mut u32).write_unaligned(v) }
}
unsafe fn ab(base: *mut u8, i: isize) -> u8 {
    // SAFETY: base is one of the game's tables; the index is the original's.
    unsafe { base.offset(i).read() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00407090(param_1: u32, param_2: i32) -> u32 {
    // SAFETY: all globals, tables, callees and addresses are the game's own;
    // reads and writes happen in the original's order.
    unsafe {
        let i5: u32 = param_1.wrapping_mul(0x23);
        let mut i3: u32 = rd(DAT_009a1fe8.wrapping_add(0x1b).wrapping_add(i5));
        let local_c: u8;
        let mut u15: u32;
        let mut u9: u32;
        let mut u_stack8: u16 = 0;
        let mut local_sub: u32;
        if i3 as i32 == -1 {
            local_c = 7;
            u15 = FUN_00459750(7) as u32;
            u9 = 0;
        } else {
            local_c = 6;
            u15 = (DAT_00a082d8 as u32).wrapping_add(i3.wrapping_mul(0xc));
            u9 = u15.wrapping_add(0xc);
            if ab((&raw mut DAT_0051a44f).cast::<u8>(), param_2 as isize) != 0 {
                u15 = u15.wrapping_add(8);
            }
            let mut u8_ = u15 & 0xffff;
            while u8_ < u9 {
                if rb((u15 & 0xffff)
                    .wrapping_mul(0x23)
                    .wrapping_add(3)
                    .wrapping_add(DAT_009a1fe8))
                    == 0
                {
                    i3 = (u15 & 0xffff).wrapping_mul(0x23);
                    local_sub = rd(DAT_009a1fe8.wrapping_add(0x1f).wrapping_add(i3));
                    ww(local_sub.wrapping_add(0x85), 0);
                    wb(local_sub.wrapping_add(1), 0);
                    break;
                }
                let u14 = u15.wrapping_add(1) as u16;
                u15 = u14 as u32;
                u8_ = u14 as u32;
            }
            if u9 <= (u15 & 0xffff) {
                u15 = (DAT_00a082d8 as u32).wrapping_add(0x60);
                u9 = (DAT_00a082d8 as u32).wrapping_add(0x80);
                u8_ = u15 & 0xffff;
                while u8_ < u9 {
                    if rb((u15 & 0xffff)
                        .wrapping_mul(0x23)
                        .wrapping_add(3)
                        .wrapping_add(DAT_009a1fe8))
                        == 0
                    {
                        i3 = (u15 & 0xffff).wrapping_mul(0x23);
                        local_sub = rd(DAT_009a1fe8.wrapping_add(0x1f).wrapping_add(i3));
                        ww(local_sub.wrapping_add(0x85), 0);
                        wb(local_sub.wrapping_add(1), 0);
                        break;
                    }
                    let u14 = u15.wrapping_add(1) as u16;
                    u15 = u14 as u32;
                    u8_ = u14 as u32;
                }
            }
            if u9 <= (u15 & 0xffff) {
                return 0xffffffff;
            }
        }
        if u15 as u16 as i16 == -1 {
            return 0xffffffff;
        }
        let i16_: u32 = i5.wrapping_add(DAT_009a1fe8);
        let b2: u8 = rb(i16_.wrapping_add(3));
        u9 = u15 & 0xffff;
        let i3: u32 = u9.wrapping_mul(0x23);
        local_sub = rd(DAT_009a1fe8.wrapping_add(0x1f).wrapping_add(i3));
        wb(local_sub, 1);
        wb(DAT_009a1fe8.wrapping_add(2).wrapping_add(i3), local_c);
        wb(DAT_009a1fe8.wrapping_add(3).wrapping_add(i3), param_2 as u8);
        ww(local_sub.wrapping_add(0x83), 1);
        ww(local_sub.wrapping_add(0x85), param_1 as u16);
        wb(local_sub.wrapping_add(0x87), b2);
        FUN_004269e0(((u_stack8 as u32) << 16) | (b2 as u32));
        let local_src: u32 = rd(i16_.wrapping_add(0x1f));
        wb(
            local_sub.wrapping_add(0x88),
            rb(local_src.wrapping_add(0x88)),
        );
        wb(
            local_sub.wrapping_add(0x89),
            rb(local_src.wrapping_add(0x89)),
        );
        let s = rw(i16_.wrapping_add(0x12)) as i16;
        let s = 0xFFFF8000u32.wrapping_sub(s as i32 as u32) as i16;
        ww(DAT_009a1fe8.wrapping_add(0x12).wrapping_add(i3), s as u16);
        let u = rw(i16_.wrapping_add(0x14));
        ww(DAT_009a1fe8.wrapping_add(0x14).wrapping_add(i3), u);
        let s = rw(i16_.wrapping_add(0x10)) as i16;
        let s = (s as i32 as u32).wrapping_add(0xFFFF8000u32) as i16;
        ww(DAT_009a1fe8.wrapping_add(0x10).wrapping_add(i3), s as u16);
        let off2: isize = param_2.wrapping_mul(2).wrapping_add(10) as isize;
        let tv: u16 = (&raw mut DAT_0051a2fc)
            .cast::<u8>()
            .offset(off2)
            .cast::<u16>()
            .read_unaligned();
        ww(local_sub.wrapping_add(0x79), tv >> 1);
        let s = (rw(local_src.wrapping_add(0x79)) as i16).wrapping_add(tv as i16);
        ww(
            DAT_009a8e18
                .wrapping_add(8)
                .wrapping_add(u9.wrapping_sub(DAT_009e9640).wrapping_mul(10)),
            s as u16,
        );
        let tbl: u32 = (&raw mut DAT_0051a194)
            .cast::<u32>()
            .offset(param_2 as isize)
            .read();
        let lu = (rw(local_src.wrapping_add(0x79)) as u32).wrapping_add(tbl);
        wd(local_sub.wrapping_add(0x7d), lu);
        let p11 = local_sub.wrapping_add(0x7d);
        let lu = rd(p11);
        let tbl2: u32 = (&raw mut DAT_0051a194)
            .cast::<u32>()
            .offset(param_2 as isize)
            .read();
        if lu < tbl2 {
            wd(p11, tbl2);
        }
        let s7 = FUN_00405860(param_2);
        ww(local_sub.wrapping_add(0x81), s7 as u16);
        wb(local_sub.wrapping_add(0x92), 1);
        let i12 = rd(DAT_009a1fe8.wrapping_add(0x1f).wrapping_add(i3));
        wb(i12.wrapping_add(0x8b), rb(i12.wrapping_add(0x92)));
        let mut u4 = rd(i16_.wrapping_add(4));
        let i12 = DAT_009a1fe8.wrapping_add(i3);
        wd(i12.wrapping_add(4), u4);
        local_sub = rd(i12.wrapping_add(0x1f));
        wd(local_sub.wrapping_add(6), u4);
        u4 = rd(i16_.wrapping_add(8));
        wd(i12.wrapping_add(8), u4);
        local_sub = rd(i12.wrapping_add(0x1f));
        wd(local_sub.wrapping_add(10), u4);
        u4 = rd(i16_.wrapping_add(0xc));
        wd(i12.wrapping_add(0xc), u4);
        local_sub = rd(i12.wrapping_add(0x1f));
        wd(local_sub.wrapping_add(0xe), u4);
        let l18 = FUN_004ae120(rb(i12.wrapping_add(3)) as u32);
        let l19 = FUN_004ae080(b2 as u32);
        let u8v: u32 = (l18 as i16).wrapping_add(l19 as i16) as u16 as u32;
        let s16a = rw(i12.wrapping_add(0x12)) as i16;
        let s7b = rw(i12.wrapping_add(0x10)) as i16;
        FUN_00427bd0(s16a, s7b, i12 as i32);
        local_sub = rd(i12.wrapping_add(0x1f));
        let s = rw(local_sub.wrapping_add(0x8c)) as i16;
        let l = (s as i32 as i64).wrapping_mul(u8v as i32 as i64);
        DAT_009a20b0 = ((l as u32) >> 15) | (((l as u64 >> 32) as i32 as u32) << 17);
        local_sub = rd(i12.wrapping_add(0x1f));
        let s = rw(local_sub.wrapping_add(0x8e)) as i16;
        let l = (s as i32 as i64).wrapping_mul(u8v as i32 as i64);
        DAT_009cd26c = ((l as u32) >> 15) | (((l as u64 >> 32) as i32 as u32) << 17);
        local_sub = rd(i12.wrapping_add(0x1f));
        let s = rw(local_sub.wrapping_add(0x90)) as i16;
        let l = (s as i32 as i64).wrapping_mul(u8v as i32 as i64);
        DAT_009fd438 = ((l as u32) >> 15) | (((l as u64 >> 32) as i32 as u32) << 17);
        FUN_004464c0(i12 as i32);
        let i12: u32 = ((u15 as u16).wrapping_sub(DAT_009e9640 as u16) as u32).wrapping_mul(10);
        local_sub = rd(DAT_009a1fe8.wrapping_add(0x1f).wrapping_add(i3));
        wd(local_sub.wrapping_add(0xa5), DAT_009a8e18.wrapping_add(i12));
        wb(DAT_009a8e18.wrapping_add(1).wrapping_add(i12), 0);
        ww(DAT_009a8e18.wrapping_add(4).wrapping_add(i12), 0xffff);
        ww(DAT_009a8e18.wrapping_add(6).wrapping_add(i12), 0);
        ww(DAT_009a8e18.wrapping_add(2).wrapping_add(i12), 0xffff);
        wb(
            DAT_009a8e18.wrapping_add(i12),
            rb(DAT_009a1fe8.wrapping_add(0x1b).wrapping_add(i5)),
        );
        u15 = rb(DAT_009a1fe8.wrapping_add(i5).wrapping_add(0x16)) as u32;
        local_sub = rd(DAT_009a1fe8.wrapping_add(i5).wrapping_add(0x1f));
        let mut i16v: u32 = rd(local_sub.wrapping_add(0xa9));
        let ix = u15.wrapping_mul(0x566) as isize;
        if ab((&raw mut DAT_009d8c84).cast::<u8>(), ix) != 0x15
            && ab((&raw mut DAT_009d9155).cast::<u8>(), ix) != 0x15
        {
            let a = i16v.wrapping_add(0x247);
            wb(a, rb(a).wrapping_sub(1));
        }
        ww(i16v.wrapping_add(0x24a), 0x1d8);
        'l40785a: {
            if param_2 != 0x9b {
                break 'l40785a;
            }
            let mut local_6: u16 = 0xffff;
            u_stack8 = 0xffff;
            let mut local_20: u32 = 0xffffffff;
            let mut local_38: u32 = 0xffffffff;
            if DAT_009e9640 < DAT_00a00854 {
                let mut local_28: u32 = DAT_009e9640.wrapping_mul(0x23);
                u15 = DAT_009e9640;
                loop {
                    let bv = rb(local_28.wrapping_add(DAT_009a1fe8).wrapping_add(3));
                    local_sub = rd(local_28.wrapping_add(DAT_009a1fe8).wrapping_add(0x1f));
                    if bv != 0
                        && rb(local_sub) == 1
                        && ab((&raw mut DAT_0051a44f).cast::<u8>(), bv as isize) != 0
                        && rw(DAT_009a8e18
                            .wrapping_add(4)
                            .wrapping_add(u15.wrapping_sub(DAT_009e9640).wrapping_mul(10)))
                            as u32
                            == param_1
                    {
                        i16v = 0;
                        if DAT_009e9640 < DAT_00a00854 {
                            let mut p13: u32 = DAT_009a8e18.wrapping_add(4);
                            let mut i10: u32 = DAT_00a00854.wrapping_sub(DAT_009e9640);
                            loop {
                                if bv == 0x9b && rw(p13) as u32 == u15 {
                                    i16v = i16v.wrapping_add(1);
                                }
                                p13 = p13.wrapping_add(10);
                                i10 = i10.wrapping_sub(1);
                                if i10 == 0 {
                                    break;
                                }
                            }
                        }
                        FUN_004034e0(param_1, u15);
                        if i16v == 0 {
                            if DAT_009a8c00 < local_20 {
                                local_20 = DAT_009a8c00;
                                local_6 = u15 as u16;
                            }
                        } else if DAT_009a8c00 < local_38 {
                            local_38 = DAT_009a8c00;
                            u_stack8 = u15 as u16;
                        }
                    }
                    u15 = u15.wrapping_add(1);
                    local_28 = local_28.wrapping_add(0x23);
                    if !(u15 < DAT_00a00854) {
                        break;
                    }
                }
            }
            if local_6 == 0xffff {
                let mut u14: u16 = _DAT_00a07cec as u16;
                if ((_DAT_00a07cec as u32 & 0xffff) as i32) < DAT_009a1fe0 {
                    loop {
                        let i16x = (u14 as u32).wrapping_mul(0x23).wrapping_add(DAT_009a1fe8);
                        if rb(i16x.wrapping_add(3)) != 0 {
                            local_sub = rd(i16x.wrapping_add(0x1f));
                            let i10 = rd(local_sub.wrapping_add(0xa9));
                            if rb(i10.wrapping_add(7)) == 0 {
                                let mut b6 = false;
                                local_sub = rd(i16x.wrapping_add(0x1b));
                                if local_sub as i32 == -1 {
                                    let lu = rw(i10.wrapping_add(0x42));
                                    if lu as u32 == param_1 {
                                        b6 = true;
                                    }
                                } else {
                                    let lu = (&raw mut DAT_009e967c)
                                        .cast::<u8>()
                                        .offset(local_sub.wrapping_mul(0x5bd) as isize)
                                        .cast::<u16>()
                                        .read_unaligned();
                                    u15 = lu as u32;
                                    let mut b17 = false;
                                    let idx = ab(
                                        (&raw mut DAT_009d8c88).cast::<u8>(),
                                        (rb((param_1 & 0xffff)
                                            .wrapping_mul(0x23)
                                            .wrapping_add(0x16)
                                            .wrapping_add(DAT_009a1fe8))
                                            as u32)
                                            .wrapping_mul(0x566)
                                            as isize,
                                    ) as u32;
                                    if idx != u15 {
                                        b17 = ab(
                                            (&raw mut DAT_009fd458).cast::<u8>(),
                                            idx.wrapping_add(u15.wrapping_mul(0x1e5)) as isize,
                                        ) == 0;
                                    }
                                    if b17 {
                                        b6 = true;
                                    }
                                }
                                if b6 {
                                    FUN_004034e0(param_1, u14 as u32);
                                    if DAT_009a8c00 < local_20 && (DAT_009a8c00 as i32) < 0x8000 {
                                        local_20 = DAT_009a8c00;
                                        local_6 = u14;
                                    }
                                }
                            }
                        }
                        u14 = u14.wrapping_add(1);
                        if !((u14 as i32) < DAT_009a1fe0) {
                            break;
                        }
                    }
                }
            }
            let chosen: u16 = if local_6 != 0xffff {
                local_6
            } else if u_stack8 != 0xffff {
                u_stack8
            } else {
                break 'l40785a;
            };
            ww(DAT_009a8e18.wrapping_add(4).wrapping_add(i12), chosen);
            wb(DAT_009a8e18.wrapping_add(1).wrapping_add(i12), 6);
            let lu = rw((chosen as u32)
                .wrapping_mul(0x23)
                .wrapping_add(DAT_009a1fe8));
            ww(DAT_009a8e18.wrapping_add(6).wrapping_add(i12), lu);
        }
        u15 = rw(DAT_009a8e18.wrapping_add(4).wrapping_add(i12)) as u32;
        if _DAT_00a07cec <= u15 as i32 && (u15 as i32) < DAT_009a1fe0 {
            local_sub = rd(DAT_009a1fe8.wrapping_add(0x1f).wrapping_add(i3));
            let p = local_sub.wrapping_add(0x81);
            let v = rw(p);
            ww(p, v >> 1);
        }
        let u14 = rw(DAT_009a8e18.wrapping_add(4).wrapping_add(i12));
        if u14 != 0xffff {
            local_sub = rd((u14 as u32)
                .wrapping_mul(0x23)
                .wrapping_add(0x1b)
                .wrapping_add(DAT_009a1fe8));
            if local_sub as i32 == DAT_009ecc34 {
                FUN_00430610(0x25, 0, 0, 0, -1);
            }
        }
        let i3b = rd(DAT_009a1fe8.wrapping_add(0x1b).wrapping_add(i5)) as i32;
        if i3b != -1 {
            FUN_0042e720(u9, i3b);
        }
        u9
    }
}
