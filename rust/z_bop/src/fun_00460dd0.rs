// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a73b0: u16;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1ff8: u32;
    static mut DAT_00a07cec: u16;
    static mut DAT_00a07ce8: i32;
    static mut DAT_009d1308: i32;
    static mut DAT_00a08104: i32;
    static mut DAT_00a08294: i32;
    static mut DAT_00a8f750: i32;
    static mut DAT_009a1fd0: i32;
    static mut DAT_009a1fd4: i32;
    static mut DAT_009a1fd8: i32;
    static mut DAT_00a08140: i32;
    static mut DAT_00a08144: i32;
    static mut DAT_00a08148: i32;
    static mut DAT_00a07cd4: i32;
    static mut DAT_00a07cd8: i32;
    static mut DAT_00a07cdc: i32;
    static mut DAT_00522498: i32;
    static mut DAT_0099f930: [u8; 0];
    static mut DAT_00521f2e: [u8; 0];
    static mut DAT_00521f1c: [u8; 0];
    static mut DAT_009d8c88: [u8; 0];
    static mut DAT_009fd458: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0041f410(param_1: i32, param_2: i32, param_3: i32) -> u32;
    fn FUN_004a5490(param_1: u16, param_2: u16) -> i32;
    fn FUN_00433390(param_1: u32, param_2: u16, param_3: u16) -> u8;
    fn FUN_00403540(param_1: u32, param_2: u32);
    fn FUN_00458e20(param_1: u32, param_2: i32);
}

unsafe fn r8(a: u32) -> u8 {
    // SAFETY: the address is the game's own memory.
    unsafe { (a as *const u8).read() }
}

unsafe fn r16(a: u32) -> u16 {
    // SAFETY: the address is the game's own memory.
    unsafe { (a as *const u16).read_unaligned() }
}

unsafe fn r32(a: u32) -> u32 {
    // SAFETY: the address is the game's own memory.
    unsafe { (a as *const u32).read_unaligned() }
}

unsafe fn bump(id: u16) {
    // SAFETY: the globals and the structure behind DAT_00a08104 are the game's own.
    unsafe {
        let mut i: i32 = 0;
        let base = DAT_00a08104 as u32;
        if r8(base.wrapping_add(0x21d)) != 0 {
            let mut p = base.wrapping_add(0x3b2);
            loop {
                if r16(p) == id {
                    DAT_009a1ff8 = DAT_009a1ff8.wrapping_add(0x8000);
                }
                p = p.wrapping_add(4);
                i = i.wrapping_add(1);
                if !(i < r8((DAT_00a08104 as u32).wrapping_add(0x21d)) as i32) {
                    break;
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00460dd0(
    param_1: u16,
    param_2: u16,
    param_3: i16,
    param_4: u16,
    param_5: u16,
    param_6: i32,
) -> u16 {
    // SAFETY: all globals, callees and addresses belong to the game; reads
    // and writes happen in the original's order.
    unsafe {
        let mut unaff_edi: u32 = 0;
        let mut local_a: u16 = 0xffff;
        let mut local_4: u32 = 0x10000;
        let mut b14: bool = r8((DAT_009a73b0 as u32)
            .wrapping_mul(0x23)
            .wrapping_add(3)
            .wrapping_add(DAT_009a1fe8 as u32))
            == 0x36;
        if b14 {
            local_4 = (DAT_00522498 as u32).wrapping_add(0x10000);
        }
        let mut u10: u16 = DAT_00a07cec;
        if (u10 as i32) < DAT_009a1fe0 {
            loop {
                let flag = (&raw const DAT_0099f930)
                    .cast::<u8>()
                    .add((u10 as u32).wrapping_mul(2).wrapping_add(param_6 as u32) as usize)
                    .read();
                if flag != 0 {
                    unaff_edi = (u10 as u32).wrapping_mul(0x23);
                    let i6 = (DAT_009a1fe8 as u32).wrapping_add(unaff_edi);
                    let t0 = r32(i6.wrapping_add(4)) as i32;
                    let t1 = r32(i6.wrapping_add(8)) as i32;
                    let t2 = r32(i6.wrapping_add(0xc)) as i32;
                    DAT_009a1ff8 = FUN_0041f410(
                        t0.wrapping_sub(DAT_00a08144),
                        t1.wrapping_sub(DAT_00a08140),
                        t2.wrapping_sub(DAT_00a08148),
                    );
                    if b14 {
                        bump(u10);
                    }
                    if DAT_009a1ff8 < local_4 {
                        let mut b13 = true;
                        if !b14 {
                            let i6 = (DAT_009a1fe8 as u32).wrapping_add(unaff_edi);
                            DAT_00a8f750 = DAT_00a8f750.wrapping_add(1);
                            DAT_009a1fd4 = r32(i6.wrapping_add(4)) as i32;
                            DAT_009a1fd8 = r32(i6.wrapping_add(8)) as i32;
                            DAT_009a1fd0 = r32(i6.wrapping_add(0xc)) as i32;
                            let r = FUN_004a5490(DAT_009a73b0, DAT_009a73b0);
                            b13 = r == 0;
                        }
                        if b13 {
                            local_4 = DAT_009a1ff8;
                            local_a = u10;
                        }
                    }
                }
                u10 = u10.wrapping_add(1);
                if !((u10 as i32) < DAT_009a1fe0) {
                    break;
                }
            }
        }
        let mut u4: u32 = (unaff_edi & 0xffff0000) | (DAT_00a07ce8 as u16 as u32);
        if ((DAT_00a07ce8 as u32 & 0xffff) as i32) < DAT_009d1308.wrapping_add(DAT_00a07ce8) {
            loop {
                let u8_: u32 = u4 & 0xffff;
                let b1 = r8(u8_
                    .wrapping_mul(0x23)
                    .wrapping_add(3)
                    .wrapping_add(DAT_009a1fe8 as u32));
                let u10: u16 = u4 as u16;
                if b1 != 0
                    && ((&raw const DAT_00521f2e)
                        .cast::<u8>()
                        .add((b1 as usize).wrapping_mul(0x1a))
                        .read()
                        & 2)
                        != 0
                {
                    let mut u11: u16 = FUN_00433390(u4, param_1, param_2) as u16;
                    let u3: u16 = FUN_00433390(u4, param_4, param_5) as u16;
                    if param_3 == 1 {
                        u11 |= u3;
                    } else {
                        u11 &= u3;
                    }
                    if u11 != 0 {
                        let mut i6: u32;
                        if u10 == 0xffff {
                            i6 = 0;
                        } else {
                            let rec = (DAT_009a1fe8 as u32).wrapping_add(u8_.wrapping_mul(0x23));
                            let t0 = r32(rec.wrapping_add(0x1f));
                            if t0 == 0 {
                                let b13 = r8(rec.wrapping_add(3)) == 0;
                                i6 = 1 - (b13 as u32);
                            } else if r8(rec.wrapping_add(3)) == 0 {
                                i6 = 0;
                            } else {
                                i6 = r32(t0.wrapping_add(0xa9));
                                if i6 == 0 {
                                    i6 = 1;
                                } else if r8(i6.wrapping_add(0x5e)) == 0x15
                                    && r8(i6.wrapping_add(0x5f)) != 0
                                {
                                    i6 = 0;
                                } else {
                                    let c2 = r8(i6.wrapping_add(7));
                                    if c2 == 1 || c2 == 3 || c2 == 4 {
                                        i6 = 0;
                                    } else {
                                        let mut do_l = false;
                                        if r8(i6.wrapping_add(0x23d)) == 3
                                            && r8(i6.wrapping_add(0x241)) != 0
                                        {
                                            FUN_00403540(u8_, DAT_009a73b0 as u32);
                                            let c = r8((DAT_009a73b0 as u32)
                                                .wrapping_mul(0x23)
                                                .wrapping_add(2)
                                                .wrapping_add(DAT_009a1fe8 as u32));
                                            if c == 0 || c == 1 || c == 2 {
                                                if (DAT_009a1ff8 as i32) < 0x8001 {
                                                    do_l = true;
                                                } else {
                                                    i6 = 0;
                                                }
                                            } else if (DAT_009a1ff8 as i32) < 0x10001 {
                                                do_l = true;
                                            } else {
                                                i6 = 0;
                                            }
                                        } else {
                                            do_l = true;
                                        }
                                        if do_l {
                                            let i5 = (DAT_009a1fe8 as u32)
                                                .wrapping_add(u8_.wrapping_mul(0x23));
                                            let t1 = r32(i5.wrapping_add(0x1b)) as i32;
                                            if t1 == -1 {
                                                let mut t2 = r32(i5.wrapping_add(0x1f));
                                                t2 = r32(t2.wrapping_add(0xa9));
                                                let a = r32(t2.wrapping_add(0xcb));
                                                let b = r32(i6.wrapping_add(0xc3));
                                                let b13 = a < b;
                                                i6 = 1 - (b13 as u32);
                                            } else {
                                                i6 = 1;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if i6 != 0 {
                            FUN_00458e20(u8_, 0);
                            DAT_009a1ff8 = FUN_0041f410(
                                DAT_00a07cdc.wrapping_sub(DAT_00a08144),
                                DAT_00a07cd8.wrapping_sub(DAT_00a08140),
                                DAT_00a07cd4.wrapping_sub(DAT_00a08148),
                            );
                            if b14 {
                                bump(u10);
                            }
                            if DAT_009a1ff8 < local_4 {
                                let mut b13 = true;
                                if !b14 {
                                    DAT_00a8f750 = DAT_00a8f750.wrapping_add(1);
                                    DAT_009a1fd4 = DAT_00a07cdc;
                                    DAT_009a1fd8 = DAT_00a07cd8;
                                    DAT_009a1fd0 = DAT_00a07cd4;
                                    let r = FUN_004a5490(DAT_009a73b0, DAT_009a73b0);
                                    b13 = r == 0;
                                }
                                if b13 {
                                    local_4 = DAT_009a1ff8;
                                    local_a = u10;
                                }
                            }
                        }
                    }
                }
                u4 = (u4 & 0xffff0000) | (u10.wrapping_add(1) as u32);
                if !((u10.wrapping_add(1) as i32) < DAT_009d1308.wrapping_add(DAT_00a07ce8)) {
                    break;
                }
            }
        }
        let i6 = DAT_00a08104;
        if !b14 && 0x10000 < local_4 {
            local_a = 0xffff;
        }
        if local_a != 0xffff && DAT_00a08294 == 0xe {
            let mut u10: u16 = DAT_00a07cec;
            if (u10 as i32) < DAT_009a1fe0 {
                loop {
                    let r7 = (DAT_009a1fe8 as u32).wrapping_add((u10 as u32).wrapping_mul(0x23));
                    let i5 = r32((DAT_009a73b0 as u32)
                        .wrapping_mul(0x23)
                        .wrapping_add(0x1f)
                        .wrapping_add(DAT_009a1fe8 as u32));
                    let u4 = r8(i5.wrapping_add(0x89)) as u32;
                    let d = (&raw const DAT_009d8c88)
                        .cast::<u8>()
                        .add((r8(r7.wrapping_add(0x16)) as usize).wrapping_mul(0x566))
                        .read();
                    if u4 == d as u32 {
                        b14 = false;
                    } else {
                        b14 = (&raw const DAT_009fd458)
                            .cast::<u8>()
                            .add((d as usize).wrapping_add((u4 as usize).wrapping_mul(0x1e5)))
                            .read()
                            == 0;
                    }
                    let s79 = r16(i5.wrapping_add(0x79)) as i16;
                    let mut cond = false;
                    if !b14 {
                        if s79 == 0 {
                            cond = true;
                        } else {
                            let c2 = r8(r7.wrapping_add(2));
                            if c2 != 0 && c2 != 1 && c2 != 2 {
                                cond = true;
                            }
                        }
                    }
                    if cond {
                        let t0 = r32(r7.wrapping_add(4)) as i32;
                        let t1 = r32(r7.wrapping_add(8)) as i32;
                        let t2 = r32(r7.wrapping_add(0xc)) as i32;
                        let i5b = FUN_0041f410(
                            t0.wrapping_sub(DAT_00a08144),
                            t1.wrapping_sub(DAT_00a08140),
                            t2.wrapping_sub(DAT_00a08148),
                        );
                        let idx = r8((DAT_009a1fe8 as u32)
                            .wrapping_add(3)
                            .wrapping_add((u10 as u32).wrapping_mul(0x23)))
                            as usize;
                        let v = (&raw const DAT_00521f1c)
                            .cast::<u8>()
                            .add(idx.wrapping_mul(0x1a))
                            .cast::<u32>()
                            .read_unaligned();
                        DAT_009a1ff8 = i5b.wrapping_sub(v);
                        if DAT_009a1ff8 <= local_4 {
                            let r5 = (local_a as u32)
                                .wrapping_mul(0x23)
                                .wrapping_add(DAT_009a1fe8 as u32);
                            DAT_009a1fd4 = r32(r5.wrapping_add(4)) as i32;
                            DAT_009a1fd8 = r32(r5.wrapping_add(8)) as i32;
                            DAT_009a1fd0 = r32(r5.wrapping_add(0xc)) as i32;
                            let r = FUN_004a5490(u10, u10);
                            if r != 0 {
                                DAT_00a08104 = i6;
                                return 0xffff;
                            }
                        }
                    }
                    u10 = u10.wrapping_add(1);
                    if !((u10 as i32) < DAT_009a1fe0) {
                        break;
                    }
                }
            }
        }
        DAT_00a08104 = i6;
        local_a
    }
}
