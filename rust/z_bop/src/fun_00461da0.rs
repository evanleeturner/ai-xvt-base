// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a73bc: i32;
    static mut DAT_009a73ba: u16;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a73b6: i32;
    static mut DAT_009a73b0: u16;
    static mut DAT_009a1ff8: i32;
    static mut DAT_009a1fe0: i32;
    static mut DAT_00a07cec: u16;
    static mut DAT_00a08104: i32;
    static mut DAT_009d1308: i32;
    static mut DAT_00a07ce8: i32;
    static mut DAT_009d8c88: [u8; 1];
    static mut DAT_009fd458: [i8; 1];
    static mut DAT_009d1320: i8;
    static mut DAT_009a73d8: i8;
    static mut DAT_005242a0: i8;
    static mut s_disableldr1pln_0051a310: [i8; 1];
    static mut s_capfreeldr1pln_0051a348: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00403540(a: u32, b: u32) -> u32;
    fn FUN_00402ec0(a: u32, b: u32, c: i32) -> i32;
    fn FUN_00404450(a: u32) -> u32;
}

unsafe fn rd8(x: i32) -> u8 {
    // SAFETY: the address is the game's own memory, read where the original reads.
    unsafe { (x as u32 as usize as *const u8).read() }
}

unsafe fn rd16(x: i32) -> u16 {
    // SAFETY: the address is the game's own memory, read where the original reads.
    unsafe { (x as u32 as usize as *const u16).read_unaligned() }
}

unsafe fn rd32(x: i32) -> i32 {
    // SAFETY: the address is the game's own memory, read where the original reads.
    unsafe { (x as u32 as usize as *const i32).read_unaligned() }
}

unsafe fn wr8(x: i32, v: u8) {
    // SAFETY: the address is the game's own memory, written where the original writes.
    unsafe { (x as u32 as usize as *mut u8).write(v) }
}

unsafe fn wr16(x: i32, v: u16) {
    // SAFETY: the address is the game's own memory, written where the original writes.
    unsafe { (x as u32 as usize as *mut u16).write_unaligned(v) }
}

unsafe fn sum() -> i32 {
    // SAFETY: reads the game's own globals.
    unsafe { DAT_009d1308.wrapping_add(DAT_00a07ce8) }
}

unsafe fn commit(idx: u16) -> u32 {
    // SAFETY: the globals and the records they point at are the game's own.
    unsafe {
        wr16(DAT_009a73b6.wrapping_add(0x18), idx);
        let u3 = (DAT_009a1fe8 as u32) >> 0x10;
        let us = rd16(((idx as u32).wrapping_mul(0x23) as i32).wrapping_add(DAT_009a1fe8));
        wr16(DAT_009a73b6.wrapping_add(0x1a), us);
        wr8(DAT_009a73b6.wrapping_add(0x1e), 1);
        ((u3 & 0xffff) << 16) | 1
    }
}

unsafe fn cmp_str(a: *const i8, b: *const i8, n: i32) -> bool {
    let mut cnt = n;
    let mut p = a;
    let mut q = b;
    let mut eq = false;
    // SAFETY: both pointers address the game's own NUL-padded strings.
    unsafe {
        loop {
            if cnt == 0 {
                break;
            }
            cnt -= 1;
            eq = *p == *q;
            p = p.add(1);
            q = q.add(1);
            if !eq {
                break;
            }
        }
    }
    eq
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00461da0() -> u32 {
    // SAFETY: all globals, callees and pointed-at records are the game's own
    // memory, accessed in the original's order.
    unsafe {
        let i13 = DAT_009a73bc;
        let mut u3: u32 = DAT_009a73ba as u32;
        let mut i6: i32 = rd32(
            DAT_009a1fe8
                .wrapping_add(0x1b)
                .wrapping_add(u3.wrapping_mul(0x23) as i32),
        );
        let u8v: u32 = ((((DAT_009a1fe8 as u32) >> 8) << 8)
            | rd8(DAT_009a73bc.wrapping_add(0x5e)) as u32)
            & 0xffff00ff;
        DAT_009a73d8 = (u8v == 0) as i8;
        let pc1: *const i8 = (&raw mut DAT_009d1320)
            .cast::<i8>()
            .add(rd8(DAT_009a73b6.wrapping_add(0xb)) as usize * 0x55)
            as *const i8;
        let mut i5: i32 = 0xf;
        let mut p14: *const i8 = pc1;
        let mut p15: *const i8 = (&raw mut s_disableldr1pln_0051a310).cast::<i8>() as *const i8;
        loop {
            if i5 == 0 {
                break;
            }
            i5 -= 1;
            DAT_009a73d8 = (*p14 == *p15) as i8;
            p14 = p14.add(1);
            p15 = p15.add(1);
            if DAT_009a73d8 == 0 {
                break;
            }
        }
        let s7 = u8v as i16;
        if s7 != 0xc && s7 != 0x17 && i6 == -1 {
            return 0xffff0000;
        }
        let mut u10: u16 = rd16(DAT_009a73b6.wrapping_add(0x2b));
        if u10 != 0xffff && u10 != 0xfb {
            let mut go59 = false;
            let mut i9: i32 = 0;
            if u10 == 0xffff {
                i5 = 0;
            } else {
                i5 = (u10 as u32).wrapping_mul(0x23) as i32;
                i9 = DAT_009a1fe8.wrapping_add(i5);
                let t1 = rd32(i9.wrapping_add(0x1f));
                if t1 == 0 {
                    let b = (rd8(i9.wrapping_add(3)) == 0) as i32;
                    i5 = 1 - b;
                } else if rd8(i9.wrapping_add(3)) == 0 {
                    i5 = 0;
                } else {
                    i9 = rd32(t1.wrapping_add(0xa9));
                    if i9 == 0 {
                        i5 = 1;
                    } else if rd8(i9.wrapping_add(0x5e)) == 0x15 && rd8(i9.wrapping_add(0x5f)) != 0
                    {
                        i5 = 0;
                    } else {
                        let c2 = rd8(i9.wrapping_add(7));
                        if c2 == 1 || c2 == 3 || c2 == 4 {
                            i5 = 0;
                        } else if rd8(i9.wrapping_add(0x23d)) == 3
                            && rd8(i9.wrapping_add(0x241)) != 0
                        {
                            FUN_00403540(u10 as u32, DAT_009a73b0 as u32);
                            let c = rd8((DAT_009a73b0 as u32)
                                .wrapping_mul(0x23)
                                .wrapping_add(2)
                                .wrapping_add(DAT_009a1fe8 as u32)
                                as i32);
                            if c == 0 || c == 1 || c == 2 {
                                if DAT_009a1ff8 < 0x8001 {
                                    go59 = true;
                                } else {
                                    i5 = 0;
                                }
                            } else if DAT_009a1ff8 < 0x10001 {
                                go59 = true;
                            } else {
                                i5 = 0;
                            }
                        } else {
                            go59 = true;
                        }
                    }
                }
                if go59 {
                    i5 = DAT_009a1fe8.wrapping_add(i5);
                    let t2 = rd32(i5.wrapping_add(0x1b));
                    if t2 == -1 {
                        let t1 = rd32(i5.wrapping_add(0x1f));
                        let t2 = rd32(t1.wrapping_add(0xa9));
                        let u1 = rd32(t2.wrapping_add(0xcb)) as u32;
                        let u2 = rd32(i9.wrapping_add(0xc3)) as u32;
                        i5 = 1 - (u1 < u2) as i32;
                    } else {
                        i5 = 1;
                    }
                }
            }
            if i5 != 0 {
                if i6 != -1 {
                    u3 = FUN_00403540(DAT_009a73b0 as u32, u10 as u32);
                    if 0x50000 < DAT_009a1ff8 {
                        return u3 & 0xffff0000;
                    }
                }
                return commit(u10);
            }
            wr16(DAT_009a73b6.wrapping_add(0x2b), 0xffff);
        }
        u10 = 0xffff;
        i5 = DAT_009a1fe8.wrapping_add((u3.wrapping_mul(0x23)) as i32);
        i6 = rd32(i5.wrapping_add(0x1b));
        let mut u12: u16 = DAT_00a07cec;
        'ret: {
            if i6 == -1 {
                u10 = rd16(i13.wrapping_add(0x42));
                u3 = 0xffffffff;
                if u10 == 0xffff {
                    break 'ret;
                }
            } else {
                while (u12 as i32) < DAT_009a1fe0 {
                    let i13b = ((u12 as u32).wrapping_mul(0x23) as i32).wrapping_add(DAT_009a1fe8);
                    if rd8(i13b.wrapping_add(3)) != 0 {
                        let t1 = rd32(i13b.wrapping_add(0x1f));
                        let i9 = rd32(t1.wrapping_add(0xa9));
                        let ss = rd16(i9.wrapping_add(0x1e5)) as i16;
                        if DAT_009a73d8 == 0 || ss != 0 {
                            if rd16(DAT_00a08104.wrapping_add(0x36e)) != u12 {
                                let t2 = rd32(
                                    ((rd16(i9.wrapping_add(0x6a)) as u32).wrapping_mul(0x23)
                                        as i32)
                                        .wrapping_add(0x1b)
                                        .wrapping_add(DAT_009a1fe8),
                                );
                                if t2 == i6 {
                                    let t1 = rd32(i5.wrapping_add(0x1f));
                                    u3 = rd8(t1.wrapping_add(0x89)) as u32;
                                    let idx = rd8(i13b.wrapping_add(0x16)) as usize * 0x566;
                                    let base = (&raw mut DAT_009d8c88).cast::<u8>();
                                    let v = base.add(idx).read();
                                    let b16 = if v as u32 == u3 {
                                        false
                                    } else {
                                        let i2 = v as usize + u3 as usize * 0x1e5;
                                        (&raw mut DAT_009fd458).cast::<i8>().add(i2).read() == 0
                                    };
                                    if b16 {
                                        u10 = u12;
                                    }
                                }
                            }
                        }
                    }
                    u12 = u12.wrapping_add(1);
                }
                if u10 == 0xffff {
                    loop {
                        u3 = 0;
                        if !(0 < sum()) {
                            break;
                        }
                    }
                    break 'ret;
                }
            }
            i6 = ((u10 as u32).wrapping_mul(0x23) as i32).wrapping_add(DAT_009a1fe8);
            u3 = rd32(i6.wrapping_add(0x1f)) as u32;
            if u3 == 0 {
                let c2 = rd8(i6.wrapping_add(0x16)) as i8;
                u12 = u10.wrapping_add(1);
                if sum() <= u12 as i32 {
                    u12 = DAT_00a07ce8 as u16;
                }
                u3 = (DAT_00a07ce8 as u32) & 0xffff;
                u10 = DAT_00a07ce8 as u16;
                if (u3 as i32) < sum() {
                    loop {
                        let ss = rd16(
                            ((u12 as u32).wrapping_mul(0x23) as i32)
                                .wrapping_add(0x17)
                                .wrapping_add(DAT_009a1fe8),
                        ) as i16;
                        if (DAT_009a73d8 == 0 || ss != 0)
                            && rd16(DAT_00a08104.wrapping_add(0x36e)) != u12
                        {
                            i6 =
                                ((u12 as u32).wrapping_mul(0x23) as i32).wrapping_add(DAT_009a1fe8);
                            if rd8(i6.wrapping_add(3)) != 0
                                && rd8(i6.wrapping_add(0x16)) as i8 == c2
                            {
                                i6 = FUN_00402ec0(DAT_009a73b0 as u32, u12 as u32, 1);
                                if i6 != 0 {
                                    let u4 = FUN_00404450(u12 as u32);
                                    if (u4 as i16) != 0 {
                                        return commit(u12);
                                    }
                                }
                            }
                            u12 = u12.wrapping_add(1);
                            if sum() <= u12 as i32 {
                                u12 = DAT_00a07ce8 as u16;
                            }
                        }
                        u10 = u10.wrapping_add(1);
                        if sum() <= u10 as i32 {
                            return (sum() as u32) & 0xffff0000;
                        }
                    }
                }
            } else {
                let t1 = rd32((u3 as i32).wrapping_add(0xa9));
                if t1 != 0 {
                    let c2 = rd8(i6.wrapping_add(0x16)) as i8;
                    u10 = u10.wrapping_add(rd8(DAT_00a08104.wrapping_add(0xb0)) as u16);
                    if DAT_009a1fe0 <= u10 as i32 {
                        u10 = DAT_00a07cec;
                    }
                    u3 = DAT_00a07cec as u32;
                    u12 = DAT_00a07cec;
                    let mut ss: i16 = 0;
                    if (u3 as i32) < DAT_009a1fe0 {
                        loop {
                            let i13b =
                                ((u10 as u32).wrapping_mul(0x23) as i32).wrapping_add(DAT_009a1fe8);
                            let t2 = rd32(i13b.wrapping_add(0x1f));
                            i6 = rd32(t2.wrapping_add(0xa9));
                            if i6 != 0 {
                                ss = rd16(i6.wrapping_add(0x1e5)) as i16;
                            }
                            if i6 == 0
                                || (DAT_009a73d8 != 0 && ss == 0)
                                || rd16(DAT_00a08104.wrapping_add(0x36e)) == u12
                            {
                                u10 = u10.wrapping_add(1);
                                if DAT_009a1fe0 <= u10 as i32 {
                                    u10 = DAT_00a07cec;
                                }
                            } else {
                                if rd8(i13b.wrapping_add(3)) != 0
                                    && rd8(i13b.wrapping_add(0x16)) as i8 == c2
                                {
                                    i6 = FUN_00402ec0(DAT_009a73b0 as u32, u10 as u32, 1);
                                    if i6 != 0 {
                                        let mut b16 = cmp_str(
                                            pc1,
                                            (&raw mut s_capfreeldr1pln_0051a348).cast::<i8>()
                                                as *const i8,
                                            0xf,
                                        );
                                        if !b16 {
                                            b16 = cmp_str(
                                                pc1,
                                                (&raw mut s_disableldr1pln_0051a310).cast::<i8>()
                                                    as *const i8,
                                                0xf,
                                            );
                                            if !b16 {
                                                b16 = cmp_str(
                                                    pc1,
                                                    (&raw mut DAT_005242a0) as *const i8,
                                                    0xd,
                                                );
                                                if !b16 {
                                                    return commit(u10);
                                                }
                                            }
                                        }
                                        let u4 = FUN_00404450(u10 as u32);
                                        if (u4 as i16) != 0 {
                                            return commit(u10);
                                        }
                                    }
                                }
                                u10 = u10.wrapping_add(1);
                                if DAT_009a1fe0 <= u10 as i32 {
                                    u10 = DAT_00a07cec;
                                }
                            }
                            u12 = u12.wrapping_add(1);
                            u3 = u12 as u32;
                            if !((u3 as i32) < DAT_009a1fe0) {
                                break;
                            }
                        }
                    }
                }
            }
        }
        u3 & 0xffff0000
    }
}
