// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a07cec: u16;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a73d8: i8;
    static mut DAT_009a73b0: u16;
    static mut DAT_009a1ff8: u32;
    static mut DAT_00a08144: i32;
    static mut DAT_00a08140: i32;
    static mut DAT_00a08148: i32;
    static mut DAT_00a07cdc: i32;
    static mut DAT_00a07cd8: i32;
    static mut DAT_00a07cd4: i32;
    static mut DAT_009a1fd4: i32;
    static mut DAT_009a1fd8: i32;
    static mut DAT_009a1fd0: i32;
    static mut DAT_00a07ce8: u32;
    static mut DAT_009d1308: i32;
    static mut DAT_00521f2e: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00403540(a: u32, b: u32);
    fn FUN_0041f410(a: i32, b: i32, c: i32) -> u32;
    fn FUN_00433390(a: u32, b: u16, c: u16) -> u8;
    fn FUN_00458e20(a: u32, b: i32);
    fn FUN_004a5490(a: u16, b: u16) -> i32;
}

#[inline(always)]
unsafe fn rd<T>(addr: u32) -> T {
    // SAFETY: the address is inside the game's own memory, read as the original does.
    unsafe { (addr as usize as *const T).read_unaligned() }
}

unsafe fn eligible(sv: i16, u12: u32) -> i32 {
    // SAFETY: all addresses are the game's own records, accessed as the original does.
    unsafe {
        if sv == -1 {
            return 0;
        }
        let rec = (DAT_009a1fe8 as u32).wrapping_add(u12.wrapping_mul(0x23));
        let p: u32 = rd(rec.wrapping_add(0x1f));
        let r3: u8 = rd(rec.wrapping_add(3));
        if p == 0 {
            return 1 - (r3 == 0) as i32;
        }
        if r3 == 0 {
            return 0;
        }
        let q: u32 = rd(p.wrapping_add(0xa9));
        if q == 0 {
            return 1;
        }
        let q5e: u8 = rd(q.wrapping_add(0x5e));
        let q5f: u8 = rd(q.wrapping_add(0x5f));
        if q5e == 0x15 && q5f != 0 {
            return 0;
        }
        let c: i8 = rd(q.wrapping_add(7));
        if c == 1 || c == 3 || c == 4 {
            return 0;
        }
        let q23d: u8 = rd(q.wrapping_add(0x23d));
        let q241: u8 = rd(q.wrapping_add(0x241));
        if q23d == 3 && q241 != 0 {
            FUN_00403540(u12, DAT_009a73b0 as u32);
            let c2: i8 = rd((DAT_009a1fe8 as u32)
                .wrapping_add((DAT_009a73b0 as u32).wrapping_mul(0x23))
                .wrapping_add(2));
            if c2 == 0 || c2 == 1 || c2 == 2 {
                if (DAT_009a1ff8 as i32) >= 0x8001 {
                    return 0;
                }
            } else if (DAT_009a1ff8 as i32) >= 0x10001 {
                return 0;
            }
        }
        let rec2 = (DAT_009a1fe8 as u32).wrapping_add(u12.wrapping_mul(0x23));
        let i: i32 = rd(rec2.wrapping_add(0x1b));
        if i == -1 {
            let p2: u32 = rd(rec2.wrapping_add(0x1f));
            let q2: u32 = rd(p2.wrapping_add(0xa9));
            let a: u32 = rd(q2.wrapping_add(0xcb));
            let b: u32 = rd(q.wrapping_add(0xc3));
            return 1 - (a < b) as i32;
        }
        1
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00461690(
    param_1: u16,
    param_2: u16,
    param_3: i16,
    param_4: u16,
    param_5: u16,
    param_6: i16,
) -> i16 {
    // SAFETY: globals and callees are the game's own; records are read where the original reads them.
    unsafe {
        let mut local_a: i16 = -1;
        let mut local_8: u32 = 0xffff_ffff;
        let mut u9: u32 = DAT_00a07cec as u32;
        if (u9 as i32) < DAT_009a1fe0 {
            loop {
                let u12 = u9 & 0xffff;
                let s11 = u9 as i16;
                let rec = (DAT_009a1fe8 as u32).wrapping_add(u12.wrapping_mul(0x23));
                'body: {
                    let r3: u8 = rd(rec.wrapping_add(3));
                    if r3 == 0 {
                        break 'body;
                    }
                    let mut u8v = FUN_00433390(u9, param_1, param_2) as u16;
                    let u4 = FUN_00433390(u9, param_4, param_5) as u16;
                    if param_3 == 1 {
                        u8v |= u4;
                    } else {
                        u8v &= u4;
                    }
                    if u8v == 0 {
                        break 'body;
                    }
                    let mut cond = DAT_009a73d8 == 0;
                    if !cond {
                        let p: u32 = rd(rec.wrapping_add(0x1f));
                        let q: u32 = rd(p.wrapping_add(0xa9));
                        let s: i16 = rd(q.wrapping_add(0x1e5));
                        cond = s != 0;
                    }
                    if !cond {
                        break 'body;
                    }
                    if eligible(s11, u12) != 0 {
                        let rec = (DAT_009a1fe8 as u32).wrapping_add(u12.wrapping_mul(0x23));
                        let x: i32 = rd(rec.wrapping_add(4));
                        let y: i32 = rd(rec.wrapping_add(8));
                        let z: i32 = rd(rec.wrapping_add(0xc));
                        DAT_009a1ff8 = FUN_0041f410(
                            x.wrapping_sub(DAT_00a08144),
                            y.wrapping_sub(DAT_00a08140),
                            z.wrapping_sub(DAT_00a08148),
                        );
                        if DAT_009a1ff8 < local_8 {
                            if param_6 != 0 {
                                FUN_00458e20(u12, 0);
                                DAT_009a1fd4 = DAT_00a07cdc;
                                DAT_009a1fd8 = DAT_00a07cd8;
                                DAT_009a1fd0 = DAT_00a07cd4;
                                let r = FUN_004a5490(DAT_009a73b0, DAT_009a73b0);
                                if r != 0 {
                                    break 'body;
                                }
                            }
                            local_a = s11;
                            local_8 = DAT_009a1ff8;
                        }
                    }
                }
                let n = (s11 as u16).wrapping_add(1);
                u9 = (u9 & 0xffff_0000) | n as u32;
                if !((n as i32) < DAT_009a1fe0) {
                    break;
                }
            }
        }
        u9 = (u9 & 0xffff_0000) | (DAT_00a07ce8 as u16 as u32);
        if ((DAT_00a07ce8 & 0xffff) as i32)
            < (DAT_009d1308 as u32).wrapping_add(DAT_00a07ce8) as i32
        {
            loop {
                let u12 = u9 & 0xffff;
                let s11 = u9 as i16;
                'body2: {
                    let b1: u8 = rd((DAT_009a1fe8 as u32)
                        .wrapping_add(u12.wrapping_mul(0x23))
                        .wrapping_add(3));
                    if b1 == 0 {
                        break 'body2;
                    }
                    let tbl = (&raw mut DAT_00521f2e).cast::<u8>();
                    if (tbl.add((b1 as usize) * 0x1a).read() & 2) == 0 {
                        break 'body2;
                    }
                    let mut u8v = FUN_00433390(u9, param_1, param_2) as u16;
                    let u4 = FUN_00433390(u9, param_4, param_5) as u16;
                    if param_3 == 1 {
                        u8v |= u4;
                    } else {
                        u8v &= u4;
                    }
                    if u8v == 0 {
                        break 'body2;
                    }
                    if eligible(s11, u12) != 0 {
                        FUN_00458e20(u12, 0);
                        DAT_009a1ff8 = FUN_0041f410(
                            DAT_00a07cdc.wrapping_sub(DAT_00a08144),
                            DAT_00a07cd8.wrapping_sub(DAT_00a08140),
                            DAT_00a07cd4.wrapping_sub(DAT_00a08148),
                        );
                        if DAT_009a1ff8 < local_8 {
                            if param_6 != 0 {
                                DAT_009a1fd4 = DAT_00a07cdc;
                                DAT_009a1fd8 = DAT_00a07cd8;
                                DAT_009a1fd0 = DAT_00a07cd4;
                                let r = FUN_004a5490(DAT_009a73b0, DAT_009a73b0);
                                if r != 0 {
                                    break 'body2;
                                }
                            }
                            local_a = s11;
                            local_8 = DAT_009a1ff8;
                        }
                    }
                }
                let n = (s11 as u16).wrapping_add(1);
                u9 = (u9 & 0xffff_0000) | n as u32;
                if !((n as i32) < (DAT_009d1308 as u32).wrapping_add(DAT_00a07ce8) as i32) {
                    break;
                }
            }
        }
        if 0x10000 < local_8 {
            local_a = -1;
        }
        local_a
    }
}
