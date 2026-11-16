// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a07cec: u16;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a73c0: u8;
    static mut DAT_00a08104: i32;
    static mut DAT_009a73b0: u16;
    static mut DAT_009a1ff8: u32;
    static mut DAT_009a73d8: u8;
    static mut DAT_009a73da: u8;
    static mut DAT_009a73db: [u8; 4];
    static mut DAT_009a73df: [u8; 4];
    static mut DAT_009a73e3: [u8; 4];
    static mut DAT_00a07ce8: u16;
    static mut DAT_009d1308: i32;
    static mut DAT_00521f2e: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00433390(param_1: u32, param_2: u16, param_3: u16) -> u8;
    fn FUN_00403540(param_1: u32, param_2: u32);
    fn FUN_00403070(param_1: u32) -> u8;
    fn FUN_0045dba0(param_1: u16, param_2: i16) -> u8;
    fn FUN_0041f410(param_1: i32, param_2: i32, param_3: i32) -> u32;
    fn FUN_00484f80(param_1: u16) -> u32;
}

unsafe fn rd8(a: u32) -> u8 {
    // SAFETY: the address is one the original reads in the game's memory.
    unsafe { (a as usize as *const u8).read() }
}

unsafe fn rd16(a: u32) -> u16 {
    // SAFETY: the address is one the original reads in the game's memory.
    unsafe { (a as usize as *const u16).read_unaligned() }
}

unsafe fn rd32(a: u32) -> u32 {
    // SAFETY: the address is one the original reads in the game's memory.
    unsafe { (a as usize as *const u32).read_unaligned() }
}

unsafe fn valid(idx: u32, id: u16, edi: &mut u32) -> i32 {
    // SAFETY: the globals and the pointers read are the game's own memory,
    // read in the original's order.
    unsafe {
        if id == 0xffff {
            return 0;
        }
        let base = (DAT_009a1fe8 as u32).wrapping_add(idx.wrapping_mul(0x23));
        let ta = rd32(base.wrapping_add(0x1f));
        if ta == 0 {
            let b = rd8(base.wrapping_add(3)) == 0;
            return 1 - (b as i32);
        }
        if rd8(base.wrapping_add(3)) == 0 {
            return 0;
        }
        let p = rd32(ta.wrapping_add(0xa9));
        *edi = p;
        if p == 0 {
            return 1;
        }
        if rd8(p.wrapping_add(0x5e)) == 0x15 && rd8(p.wrapping_add(0x5f)) != 0 {
            return 0;
        }
        let c = rd8(p.wrapping_add(7));
        if c == 1 || c == 3 || c == 4 {
            return 0;
        }
        if rd8(p.wrapping_add(0x23d)) == 3 && rd8(p.wrapping_add(0x241)) != 0 {
            FUN_00403540(idx, DAT_009a73b0 as u32);
            let c2 = rd8((DAT_009a73b0 as u32)
                .wrapping_mul(0x23)
                .wrapping_add(2)
                .wrapping_add(DAT_009a1fe8 as u32));
            if c2 == 0 || c2 == 1 || c2 == 2 {
                if (DAT_009a1ff8 as i32) >= 0x8001 {
                    return 0;
                }
            } else if (DAT_009a1ff8 as i32) >= 0x10001 {
                return 0;
            }
        }
        let base2 = (DAT_009a1fe8 as u32).wrapping_add(idx.wrapping_mul(0x23));
        let ta2 = rd32(base2.wrapping_add(0x1b));
        if ta2 == 0xffff_ffff {
            let tb0 = rd32(base2.wrapping_add(0x1f));
            let tb = rd32(tb0.wrapping_add(0xa9));
            let tc = rd32(tb.wrapping_add(0xcb));
            let td = rd32(p.wrapping_add(0xc3));
            return 1 - ((tc < td) as i32);
        }
        1
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0045c720(
    param_1: u16,
    param_2: u16,
    param_3: i16,
    param_4: u16,
    param_5: u16,
) -> u32 {
    // SAFETY: the globals, callees and pointers are the game's own memory,
    // read and written where the original does.
    unsafe {
        let mut local_c: i32 = 0;
        let mut edi: u32 = 0;
        let mut dummy: u32 = 0;
        let mut u11: u32 = DAT_00a07cec as u32;

        if (DAT_00a07cec as i32) < DAT_009a1fe0 {
            loop {
                let u10 = u11 & 0xffff;
                let base = (DAT_009a1fe8 as u32).wrapping_add(u10.wrapping_mul(0x23));
                let s8 = u11 as u16;
                'body: {
                    if rd8(base.wrapping_add(3)) != 0
                        && rd8(base.wrapping_add(0x16)) != DAT_009a73c0
                    {
                        let mut v13 = FUN_00433390(u11, param_1, param_2) as u16;
                        let v9 = FUN_00433390(u11, param_4, param_5) as u16;
                        if param_3 == 1 {
                            v13 |= v9;
                        } else {
                            v13 &= v9;
                        }
                        edi = (edi & 0xffff0000) | (v13 as u32);
                        if v13 != 0 && rd16((DAT_00a08104 as u32).wrapping_add(0x36e)) != s8 {
                            let r = valid(u10, s8, &mut edi);
                            if r != 0 {
                                let i5 = rd32(
                                    (DAT_009a1fe8 as u32)
                                        .wrapping_add(0x1f)
                                        .wrapping_add(u10.wrapping_mul(0x23)),
                                );
                                let i6 = rd32(i5.wrapping_add(0xa9));
                                let cond = DAT_009a73d8 == 0
                                    || (rd16(i6.wrapping_add(0x1e5)) != 0
                                        && (DAT_009a73d8 == 0
                                            || rd8(i6.wrapping_add(0x1eb)) == 0
                                            || {
                                                let t = rd32(
                                                    (DAT_009a73b0 as u32)
                                                        .wrapping_mul(0x23)
                                                        .wrapping_add(0x1f)
                                                        .wrapping_add(DAT_009a1fe8 as u32),
                                                );
                                                rd8(t.wrapping_add(0x89))
                                                    != rd8(i5.wrapping_add(0x89))
                                            }));
                                if cond {
                                    if (DAT_009a73da & 4) != 0 {
                                        let b = FUN_00403070(u11);
                                        if (b as u16) == 0 {
                                            break 'body;
                                        }
                                    }
                                    local_c = local_c.wrapping_add(1);
                                }
                            }
                        }
                    }
                }
                let n = s8.wrapping_add(1);
                u11 = (u11 & 0xffff0000) | (n as u32);
                if !((n as i32) < DAT_009a1fe0) {
                    break;
                }
            }
        }

        u11 = DAT_00a07ce8 as u32;
        if (DAT_00a07ce8 as i32) < DAT_009d1308.wrapping_add(DAT_00a07ce8 as i32) {
            loop {
                edi = u11 & 0xffff;
                let b1 = rd8(edi
                    .wrapping_mul(0x23)
                    .wrapping_add(3)
                    .wrapping_add(DAT_009a1fe8 as u32));
                let s8 = u11 as u16;
                'body: {
                    if b1 != 0
                        && ((&raw const DAT_00521f2e)
                            .cast::<u8>()
                            .add((b1 as usize).wrapping_mul(0x1a))
                            .read()
                            & 2)
                            != 0
                    {
                        let mut v13 = FUN_00433390(u11, param_1, param_2) as u16;
                        let v9 = FUN_00433390(u11, param_4, param_5) as u16;
                        if param_3 == 1 {
                            v13 |= v9;
                        } else {
                            v13 &= v9;
                        }
                        if v13 != 0 && rd16((DAT_00a08104 as u32).wrapping_add(0x36e)) != s8 {
                            let r = valid(edi, s8, &mut dummy);
                            if r != 0 {
                                if (DAT_009a73da & 4) != 0 {
                                    let b = FUN_00403070(u11);
                                    if (b as u16) == 0 {
                                        break 'body;
                                    }
                                }
                                local_c = local_c.wrapping_add(1);
                            }
                        }
                    }
                }
                let n = s8.wrapping_add(1);
                u11 = (u11 & 0xffff0000) | (n as u32);
                if !((n as i32) < DAT_009d1308.wrapping_add(DAT_00a07ce8 as i32)) {
                    break;
                }
            }
        }

        if local_c == 0 {
            return 0xffff;
        }

        u11 = (edi & 0xffff0000) | (DAT_00a07cec as u32);
        let mut local_4: u32 = 0xffff_ffff;
        let mut local_e: u16 = 0xffff;

        if (DAT_00a07cec as i32) < DAT_009a1fe0 {
            loop {
                let u10 = u11 & 0xffff;
                let i5 = u10.wrapping_mul(0x23);
                let id = u11 as u16;
                'body: {
                    if rd8((DAT_009a1fe8 as u32).wrapping_add(i5).wrapping_add(3)) != 0
                        && rd8((DAT_009a1fe8 as u32).wrapping_add(i5).wrapping_add(0x16))
                            != DAT_009a73c0
                    {
                        let mut v9 = FUN_00433390(u11, param_1, param_2) as u16;
                        let v4 = FUN_00433390(u11, param_4, param_5) as u16;
                        if param_3 == 1 {
                            v9 |= v4;
                        } else {
                            v9 &= v4;
                        }
                        if v9 != 0 && rd16((DAT_00a08104 as u32).wrapping_add(0x36e)) != id {
                            let r = valid(u10, id, &mut dummy);
                            if r != 0 {
                                let i6 =
                                    rd32((DAT_009a1fe8 as u32).wrapping_add(0x1f).wrapping_add(i5));
                                let i7 = rd32(i6.wrapping_add(0xa9));
                                let cond = DAT_009a73d8 == 0
                                    || (rd16(i7.wrapping_add(0x1e5)) != 0
                                        && (DAT_009a73d8 == 0
                                            || rd8(i7.wrapping_add(0x1eb)) == 0
                                            || {
                                                let t = rd32(
                                                    (DAT_009a73b0 as u32)
                                                        .wrapping_mul(0x23)
                                                        .wrapping_add(0x1f)
                                                        .wrapping_add(DAT_009a1fe8 as u32),
                                                );
                                                rd8(t.wrapping_add(0x89))
                                                    != rd8(i6.wrapping_add(0x89))
                                            }));
                                if cond {
                                    if (DAT_009a73da & 4) != 0 {
                                        let b = FUN_00403070(u11);
                                        if (b as u16) == 0 {
                                            break 'body;
                                        }
                                    }
                                    if (DAT_009a73da & 1) != 0 {
                                        let b = FUN_0045dba0(id, local_c as i16);
                                        if (b as u16) == 0 {
                                            break 'body;
                                        }
                                    }
                                    if (DAT_009a73da & 0x20) == 0 {
                                        FUN_00403540(DAT_009a73b0 as u32, u10);
                                    } else {
                                        let ib = i5.wrapping_add(DAT_009a1fe8 as u32);
                                        let tc = (&raw const DAT_009a73db)
                                            .cast::<u32>()
                                            .read_unaligned();
                                        let td = (&raw const DAT_009a73df)
                                            .cast::<u32>()
                                            .read_unaligned();
                                        let te = (&raw const DAT_009a73e3)
                                            .cast::<u32>()
                                            .read_unaligned();
                                        DAT_009a1ff8 = FUN_0041f410(
                                            (rd32(ib.wrapping_add(4))).wrapping_sub(tc) as i32,
                                            (rd32(ib.wrapping_add(8))).wrapping_sub(td) as i32,
                                            (rd32(ib.wrapping_add(0xc))).wrapping_sub(te) as i32,
                                        );
                                    }
                                    if 0x4000 < (DAT_009a1ff8 as i32) {
                                        let r2 = FUN_00484f80(id);
                                        if (r2 as u8) == 1 {
                                            break 'body;
                                        }
                                    }
                                    if DAT_009a1ff8 < local_4 {
                                        local_4 = DAT_009a1ff8;
                                        local_e = id;
                                    }
                                }
                            }
                        }
                    }
                }
                let n = id.wrapping_add(1);
                u11 = (u11 & 0xffff0000) | (n as u32);
                if !((n as i32) < DAT_009a1fe0) {
                    break;
                }
            }
        }

        let mut u10: u32 = (u11 & 0xffff0000) | (DAT_00a07ce8 as u32);
        u11 = DAT_00a07ce8 as u32;
        if (u11 as i32) < DAT_009d1308.wrapping_add(DAT_00a07ce8 as i32) {
            loop {
                u11 = u10 & 0xffff;
                let b1 = rd8(u11
                    .wrapping_mul(0x23)
                    .wrapping_add(3)
                    .wrapping_add(DAT_009a1fe8 as u32));
                let id = u10 as u16;
                'body: {
                    if b1 != 0
                        && ((&raw const DAT_00521f2e)
                            .cast::<u8>()
                            .add((b1 as usize).wrapping_mul(0x1a))
                            .read()
                            & 2)
                            != 0
                    {
                        let mut v9 = FUN_00433390(u10, param_1, param_2) as u16;
                        let v4 = FUN_00433390(u10, param_4, param_5) as u16;
                        if param_3 == 1 {
                            v9 |= v4;
                        } else {
                            v9 &= v4;
                        }
                        if v9 != 0 && rd16((DAT_00a08104 as u32).wrapping_add(0x36e)) != id {
                            let r = valid(u11, id, &mut dummy);
                            if r != 0 {
                                if (DAT_009a73da & 4) != 0 {
                                    let b = FUN_00403070(u10);
                                    if (b as u16) == 0 {
                                        break 'body;
                                    }
                                }
                                if (DAT_009a73da & 1) != 0 {
                                    let b = FUN_0045dba0(id, local_c as i16);
                                    if (b as u16) == 0 {
                                        break 'body;
                                    }
                                }
                                FUN_00403540(DAT_009a73b0 as u32, u11);
                                if DAT_009a1ff8 < local_4 {
                                    local_4 = DAT_009a1ff8;
                                    local_e = id;
                                }
                            }
                        }
                    }
                }
                let n = id.wrapping_add(1);
                u10 = (u10 & 0xffff0000) | (n as u32);
                u11 = (DAT_009d1308 as u32).wrapping_add(DAT_00a07ce8 as u32);
                if !((n as i32) < (u11 as i32)) {
                    break;
                }
            }
        }
        (u11 & 0xffff0000) | (local_e as u32)
    }
}
