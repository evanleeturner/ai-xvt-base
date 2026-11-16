// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a08104: u32;
    static mut DAT_009a73b0: u16;
    static mut DAT_009a1fe8: u32;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1ff8: u32;
    static mut DAT_009a73c4: i32;
    static mut DAT_009a73c8: i32;
    static mut DAT_009a73cc: i32;
    static mut DAT_00a08144: i32;
    static mut DAT_00a08140: i32;
    static mut DAT_00a08148: i32;
    static mut DAT_009d8a58: i32;
    static mut DAT_009d8a54: i32;
    static mut DAT_009d8b60: i32;
    static mut DAT_009e9640: u16;
    static mut DAT_00a00854: i32;
    static mut DAT_00a07cec: u16;
    static mut DAT_009a8e18: u32;
    static mut DAT_009a73b6: [u8; 4];
    static mut DAT_0051c5aa: [u8; 1];
    static mut DAT_0051c5ac: [u8; 1];
    static mut DAT_0051c5b2: [u8; 1];
    static mut DAT_0051c5b4: [u8; 1];
    static mut DAT_0051c5b6: [u8; 1];
    static mut DAT_0051c5b9: [u8; 1];
    static mut DAT_009d8c88: [u8; 1];
    static mut DAT_009fd458: [u8; 1];
    static mut DAT_009e968a: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004035d0(param_1: u32, param_2: i16, param_3: i16, param_4: i16);
    fn FUN_0041f410(param_1: i32, param_2: i32, param_3: i32) -> u32;
    fn FUN_00403540(param_1: u32, param_2: u32);
    fn FUN_0045f1e0(param_1: u32) -> u32;
    fn FUN_00406290(param_1: u32, param_2: u32, param_3: u32, param_4: i32) -> i32;
    fn FUN_00425cb0() -> u8;
}

#[inline(always)]
unsafe fn r8(a: u32) -> u8 {
    // SAFETY: the address is the game's own memory, read as the original does.
    unsafe { (a as usize as *const u8).read() }
}

#[inline(always)]
unsafe fn r16(a: u32) -> u16 {
    // SAFETY: the address is the game's own memory, read as the original does.
    unsafe { (a as usize as *const u16).read_unaligned() }
}

#[inline(always)]
unsafe fn r32(a: u32) -> u32 {
    // SAFETY: the address is the game's own memory, read as the original does.
    unsafe { (a as usize as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn w8(a: u32, v: u8) {
    // SAFETY: the address is the game's own memory, written as the original does.
    unsafe { (a as usize as *mut u8).write(v) }
}

#[inline(always)]
unsafe fn w16(a: u32, v: u16) {
    // SAFETY: the address is the game's own memory, written as the original does.
    unsafe { (a as usize as *mut u16).write_unaligned(v) }
}

#[inline(always)]
unsafe fn ent(i: u32) -> u32 {
    // SAFETY: reads a game global by value.
    unsafe { i.wrapping_mul(0x23).wrapping_add(DAT_009a1fe8) }
}

#[inline(always)]
unsafe fn rec(u: u16) -> u32 {
    // SAFETY: reads game globals by value.
    unsafe { DAT_009a8e18.wrapping_add((u.wrapping_sub(DAT_009e9640) as u32).wrapping_mul(10)) }
}

#[inline(always)]
unsafe fn arr(base: *mut u8, idx: u32) -> *mut u8 {
    base.wrapping_add(idx as usize)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0045f380() -> u32 {
    // SAFETY: all globals, tables and callees are the game's own; every
    // address is read and written exactly where the original does.
    unsafe {
        if r8(DAT_00a08104.wrapping_add(7)) == 3 {
            return DAT_00a08104 & 0xffff0000;
        }
        if r16(DAT_00a08104.wrapping_add(0x1e5)) == 0 {
            return DAT_00a08104 & 0xffff0000;
        }
        if r16(DAT_00a08104.wrapping_add(0x1e7)) != 0 {
            return DAT_00a08104 & 0xffff0000;
        }
        if r8(ent(DAT_009a73b0 as u32).wrapping_add(2)) == 0 {
            return DAT_009a1fe8 & 0xffff0000;
        }
        let mut local_1a: u16 = 0;
        let mut u_var8: u32 = DAT_00a08104;
        if r8(DAT_00a08104.wrapping_add(0x232)) != 0 {
            loop {
                let i_var4 = (local_1a as u32)
                    .wrapping_add((r8(DAT_00a08104.wrapping_add(4)) as u32).wrapping_mul(0x101));
                let mut u_stack_18: u16 = *arr((&raw mut DAT_0051c5aa).cast::<u8>(), i_var4) as u16;
                let lim0 =
                    (*arr((&raw mut DAT_0051c5ac).cast::<u8>(), i_var4) as u16).wrapping_add(1);
                if u_stack_18 < lim0 {
                    loop {
                        u_var8 = u_stack_18 as u32;
                        let idx9 = u_var8.wrapping_mul(8).wrapping_add(
                            (r8(DAT_00a08104.wrapping_add(4)) as u32).wrapping_mul(0x101),
                        );
                        let b9 = *arr((&raw mut DAT_0051c5b9).cast::<u8>(), idx9) as u32;
                        if r8(b9.wrapping_add(0x2d8).wrapping_add(DAT_00a08104)) == 0 {
                            let p4 = DAT_00a08104.wrapping_add(u_var8.wrapping_mul(4));
                            if (r8(p4.wrapping_add(0x370)) == 0x90
                                || r8(p4.wrapping_add(0x370)) == 0x95)
                                && r8(p4.wrapping_add(0x372)) != 0
                            {
                                if r8(p4.wrapping_add(0x373)) == 0 {
                                    w16(p4.wrapping_add(0x3b2), 0xffff);
                                    let puvar5 = p4.wrapping_add(0x3b2);
                                    DAT_00a08144 = DAT_009a73c4;
                                    DAT_00a08140 = DAT_009a73c8;
                                    DAT_00a08148 = DAT_009a73cc;
                                    let i_var6 = (r8(DAT_00a08104.wrapping_add(4)) as u32)
                                        .wrapping_mul(0x101);
                                    let i_var4b = i_var6.wrapping_add(u_var8.wrapping_mul(8));
                                    let s1 = (arr((&raw mut DAT_0051c5b2).cast::<u8>(), i_var4b)
                                        as *const i16)
                                        .read_unaligned();
                                    let s2 = (arr((&raw mut DAT_0051c5b4).cast::<u8>(), i_var4b)
                                        as *const i16)
                                        .read_unaligned();
                                    let s3 = (arr(
                                        (&raw mut DAT_0051c5b6).cast::<u8>(),
                                        u_var8.wrapping_mul(8).wrapping_add(i_var6),
                                    ) as *const i16)
                                        .read_unaligned();
                                    FUN_004035d0(ent(DAT_009a73b0 as u32), s1, s2, s3);
                                    if r8(ent(DAT_009a73b0 as u32).wrapping_add(3)) == 0x35 {
                                        DAT_009d8a58 = DAT_009d8a58.wrapping_mul(2);
                                        DAT_009d8a54 = DAT_009d8a54.wrapping_mul(2);
                                        DAT_009d8b60 = DAT_009d8b60.wrapping_mul(2);
                                    }
                                    DAT_00a08144 = DAT_00a08144.wrapping_add(DAT_009d8a58);
                                    DAT_00a08140 = DAT_00a08140.wrapping_add(DAT_009d8a54);
                                    DAT_00a08148 = DAT_00a08148.wrapping_add(DAT_009d8b60);
                                    let mut local_10: u32 = 0x40000;
                                    let mut local_16: u16 = 0xffff;
                                    let mut u_var12: u16 = DAT_009e9640;
                                    if (u_var12 as i32) < DAT_00a00854 {
                                        loop {
                                            let i4 = ent(u_var12 as u32);
                                            if r8(i4.wrapping_add(3)) != 0 {
                                                let i6 = rec(u_var12);
                                                if r8(i6.wrapping_add(1)) != 0
                                                    && r16(i6.wrapping_add(4)) == DAT_009a73b0
                                                {
                                                    let mut s_var13: i16 = 0;
                                                    let mut u_var9: u16 = DAT_009e9640;
                                                    while (u_var9 as i32) < DAT_00a00854 {
                                                        if r8(ent(u_var9 as u32).wrapping_add(3))
                                                            != 0
                                                            && u_var9 != u_var12
                                                        {
                                                            let i6b = rec(u_var9);
                                                            if 4 < r8(i6b.wrapping_add(1))
                                                                && r16(i6b.wrapping_add(4))
                                                                    == u_var12
                                                            {
                                                                s_var13 = s_var13.wrapping_add(1);
                                                            }
                                                        }
                                                        u_var9 = u_var9.wrapping_add(1);
                                                    }
                                                    if s_var13 == 0 {
                                                        let ta = r32(i4.wrapping_add(4)) as i32;
                                                        let tb = r32(i4.wrapping_add(8)) as i32;
                                                        let tc = r32(i4.wrapping_add(0xc)) as i32;
                                                        DAT_009a1ff8 = FUN_0041f410(
                                                            ta.wrapping_sub(DAT_00a08144),
                                                            tb.wrapping_sub(DAT_00a08140),
                                                            tc.wrapping_sub(DAT_00a08148),
                                                        );
                                                        if 0x4000 < (DAT_009a1ff8 as i32)
                                                            && DAT_009a1ff8 < local_10
                                                        {
                                                            local_16 = u_var12;
                                                            local_10 = DAT_009a1ff8;
                                                        }
                                                    }
                                                }
                                            }
                                            u_var12 = u_var12.wrapping_add(1);
                                            if !((u_var12 as i32) < DAT_00a00854) {
                                                break;
                                            }
                                        }
                                    }
                                    if local_16 == 0xffff {
                                        local_10 = 0x40000;
                                        local_16 = 0xffff;
                                        u_var12 = DAT_00a07cec;
                                        if (u_var12 as i32) < DAT_009a1fe0 {
                                            loop {
                                                let u_var7 = u_var12 as u32;
                                                let i_var4c = ent(u_var7);
                                                if r8(i_var4c.wrapping_add(3)) != 0 {
                                                    let tp1 = r32(i_var4c.wrapping_add(0x1f));
                                                    let i_var6c = r32(tp1.wrapping_add(0xa9));
                                                    let mut go = false;
                                                    if ((r16(i_var6c.wrapping_add(0x42))
                                                        == DAT_009a73b0
                                                        && (r8(i_var6c.wrapping_add(0x5e))
                                                            == 0x0c
                                                            || r8(i_var6c.wrapping_add(0x5e))
                                                                == 0x17))
                                                        || r16(i_var6c.wrapping_add(0x6a))
                                                            == u_var12)
                                                        || r16(i_var6c.wrapping_add(0x6e))
                                                            == u_var12
                                                    {
                                                        go = true;
                                                    } else {
                                                        let ta =
                                                            r32(i_var4c.wrapping_add(0x1b)) as i32;
                                                        if ta != -1 {
                                                            let tp = r32((DAT_009a73b0 as u32)
                                                                .wrapping_mul(0x23)
                                                                .wrapping_add(0x1f)
                                                                .wrapping_add(DAT_009a1fe8));
                                                            let u_var11 =
                                                                r8(tp.wrapping_add(0x89)) as u32;
                                                            let c88 = *arr(
                                                                (&raw mut DAT_009d8c88)
                                                                    .cast::<u8>(),
                                                                (r8(i_var4c.wrapping_add(0x16))
                                                                    as u32)
                                                                    .wrapping_mul(0x566),
                                                            )
                                                                as u32;
                                                            let b14 = if u_var11 == c88 {
                                                                false
                                                            } else {
                                                                *arr(
                                                                    (&raw mut DAT_009fd458)
                                                                        .cast::<u8>(),
                                                                    c88.wrapping_add(
                                                                        u_var11.wrapping_mul(0x1e5),
                                                                    ),
                                                                ) == 0
                                                            };
                                                            if b14 {
                                                                let ta2 =
                                                                    r32(i_var4c.wrapping_add(0x1b));
                                                                let t16 = (arr(
                                                                    (&raw mut DAT_009e968a)
                                                                        .cast::<u8>(),
                                                                    ta2.wrapping_mul(0x5bd),
                                                                )
                                                                    as *const u16)
                                                                    .read_unaligned();
                                                                if t16 == DAT_009a73b0 {
                                                                    go = true;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    if go {
                                                        let res: i32 = 'r: {
                                                            if u_var12 == 0xffff {
                                                                break 'r 0;
                                                            }
                                                            let p = ent(u_var7);
                                                            let tp1 = r32(p.wrapping_add(0x1f));
                                                            if tp1 == 0 {
                                                                let b = (r8(p.wrapping_add(3)) == 0)
                                                                    as i32;
                                                                break 'r 1i32.wrapping_sub(b);
                                                            }
                                                            if r8(p.wrapping_add(3)) == 0 {
                                                                break 'r 0;
                                                            }
                                                            let tp1 = r32(p.wrapping_add(0x1f));
                                                            let q = r32(tp1.wrapping_add(0xa9));
                                                            if q == 0 {
                                                                break 'r 1;
                                                            }
                                                            if r8(q.wrapping_add(0x5e)) == 0x15
                                                                && r8(q.wrapping_add(0x5f)) != 0
                                                            {
                                                                break 'r 0;
                                                            }
                                                            let c1 = r8(q.wrapping_add(7));
                                                            if c1 == 1 || c1 == 3 || c1 == 4 {
                                                                break 'r 0;
                                                            }
                                                            if r8(q.wrapping_add(0x23d)) == 3
                                                                && r8(q.wrapping_add(0x241)) != 0
                                                            {
                                                                FUN_00403540(
                                                                    u_var7,
                                                                    DAT_009a73b0 as u32,
                                                                );
                                                                let c =
                                                                    r8(ent(DAT_009a73b0 as u32)
                                                                        .wrapping_add(2));
                                                                if c == 0 || c == 1 || c == 2 {
                                                                    if (DAT_009a1ff8 as i32)
                                                                        >= 0x8001
                                                                    {
                                                                        break 'r 0;
                                                                    }
                                                                } else if (DAT_009a1ff8 as i32)
                                                                    >= 0x10001
                                                                {
                                                                    break 'r 0;
                                                                }
                                                            }
                                                            let i6 = ent(u_var7);
                                                            let ta =
                                                                r32(i6.wrapping_add(0x1b)) as i32;
                                                            if ta == -1 {
                                                                let tp1 =
                                                                    r32(i6.wrapping_add(0x1f));
                                                                let tb =
                                                                    r32(tp1.wrapping_add(0xa9));
                                                                let tp1 =
                                                                    r32(tb.wrapping_add(0xcb));
                                                                let u_var11 =
                                                                    r32(q.wrapping_add(0xc3));
                                                                let b14 = (tp1 < u_var11) as i32;
                                                                break 'r 1i32.wrapping_sub(b14);
                                                            }
                                                            1
                                                        };
                                                        if res != 0 {
                                                            let mut u_var9: u16 = 0;
                                                            let mut u_var10: u16 = DAT_009e9640;
                                                            while (u_var10 as i32) < DAT_00a00854 {
                                                                if r8(ent(u_var10 as u32)
                                                                    .wrapping_add(3))
                                                                    != 0
                                                                    && u_var10 != u_var12
                                                                {
                                                                    let i4 = rec(u_var10);
                                                                    if r8(i4.wrapping_add(1)) != 0
                                                                        && r16(i4.wrapping_add(4))
                                                                            == u_var12
                                                                    {
                                                                        u_var9 =
                                                                            u_var9.wrapping_add(1);
                                                                    }
                                                                }
                                                                u_var10 = u_var10.wrapping_add(1);
                                                            }
                                                            if u_var9 < 2 {
                                                                let i4 = ent(u_var7);
                                                                let ta =
                                                                    r32(i4.wrapping_add(4)) as i32;
                                                                let tb =
                                                                    r32(i4.wrapping_add(8)) as i32;
                                                                let tc = r32(i4.wrapping_add(0xc))
                                                                    as i32;
                                                                DAT_009a1ff8 = FUN_0041f410(
                                                                    ta.wrapping_sub(DAT_00a08144),
                                                                    tb.wrapping_sub(DAT_00a08140),
                                                                    tc.wrapping_sub(DAT_00a08148),
                                                                );
                                                                if DAT_009a1ff8 < local_10 {
                                                                    local_16 = u_var12;
                                                                    local_10 = DAT_009a1ff8;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                u_var12 = u_var12.wrapping_add(1);
                                                if !((u_var12 as i32) < DAT_009a1fe0) {
                                                    break;
                                                }
                                            }
                                        }
                                    } else {
                                        w16(puvar5, local_16);
                                    }
                                    if local_16 != 0xffff {
                                        w16(puvar5, local_16);
                                    }
                                    if r16(puvar5) != 0xffff {
                                        let t = u32::from_ne_bytes(DAT_009a73b6);
                                        let u_var2 = r16(t.wrapping_add(0x18));
                                        let t = u32::from_ne_bytes(DAT_009a73b6);
                                        w16(t.wrapping_add(0x18), r16(puvar5));
                                        let t = u32::from_ne_bytes(DAT_009a73b6);
                                        let u_var7 = FUN_0045f1e0(
                                            ((t >> 0x10) << 0x10)
                                                | (r16(t.wrapping_add(0x18)) as u32),
                                        );
                                        let t = u32::from_ne_bytes(DAT_009a73b6);
                                        w16(t.wrapping_add(0x1c), u_var7 as u16);
                                        let i_var4d = FUN_00406290(
                                            DAT_009a73b0 as u32,
                                            u_var8,
                                            r8(DAT_00a08104
                                                .wrapping_add(0x370)
                                                .wrapping_add(u_var8.wrapping_mul(4)))
                                                as u32,
                                            0xffff,
                                        );
                                        if i_var4d != -1 {
                                            let b3 = FUN_00425cb0();
                                            w8(
                                                DAT_009a8e18.wrapping_add(1).wrapping_add(
                                                    (i_var4d as u32).wrapping_mul(10),
                                                ),
                                                (b3 & 3).wrapping_add(3),
                                            );
                                            w8(
                                                DAT_00a08104
                                                    .wrapping_add(0x373)
                                                    .wrapping_add(u_var8.wrapping_mul(4)),
                                                0x14,
                                            );
                                        }
                                        let t = u32::from_ne_bytes(DAT_009a73b6);
                                        w16(t.wrapping_add(0x18), u_var2);
                                    }
                                } else {
                                    w8(
                                        p4.wrapping_add(0x373),
                                        r8(p4.wrapping_add(0x373)).wrapping_sub(1),
                                    );
                                }
                            }
                        }
                        u_stack_18 = u_stack_18.wrapping_add(1);
                        let idxl = (local_1a as u32).wrapping_add(
                            (r8(DAT_00a08104.wrapping_add(4)) as u32).wrapping_mul(0x101),
                        );
                        let lim = (*arr((&raw mut DAT_0051c5ac).cast::<u8>(), idxl) as u16)
                            .wrapping_add(1);
                        if !(u_stack_18 < lim) {
                            break;
                        }
                    }
                }
                local_1a = local_1a.wrapping_add(1);
                u_var8 = local_1a as u32;
                if !(u_var8 < r8(DAT_00a08104.wrapping_add(0x232)) as u32) {
                    break;
                }
            }
        }
        u_var8 & 0xffff0000
    }
}
