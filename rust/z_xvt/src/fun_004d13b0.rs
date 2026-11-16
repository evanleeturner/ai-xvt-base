// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0075964b: i32;
    static mut DAT_00758830: i32;
}

unsafe fn stride() -> u32 {
    // SAFETY: the game's global, read unaligned.
    unsafe { (&raw const DAT_0075964b).read_unaligned() as u32 }
}

unsafe fn wr(base: u32, idx: u32, v: i32) {
    // SAFETY: caller's surface memory, written where the original does.
    unsafe {
        (base.wrapping_add(idx.wrapping_mul(2)) as *mut u16).write_unaligned(v as u16);
    }
}

unsafe fn pal(p: *mut i32, b: u8) -> i32 {
    // SAFETY: palette table in the caller's structure.
    unsafe { p.add(b as usize + 5).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d13b0(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
) {
    // SAFETY: all pointers are the game's own data; reads and writes mirror the original.
    unsafe {
        let mut param_5 = param_5;
        if param_1.read_unaligned() == param_6 {
            let mut pb8: *const u8 = param_1.add(4).read_unaligned() as u32 as *const u8;
            let mut local_c: u32 = (param_3 as u32)
                .wrapping_mul(stride())
                .wrapping_add((param_2 as u32).wrapping_mul(2))
                .wrapping_add(DAT_00758830 as u32);
            if 0 < param_5 {
                loop {
                    let d = (pb8 as *const i32).read_unaligned();
                    pb8 = (pb8 as u32).wrapping_add(d as u32) as *const u8;
                    param_5 = param_5.wrapping_sub(1);
                    if param_5 == 0 {
                        break;
                    }
                }
            }
            if 0 < param_7 {
                let mut local_24 = param_7;
                loop {
                    let mut i10: i32 = 0;
                    let mut pb9: *const u8 = pb8.add(4);
                    loop {
                        let mut b4 = pb9.read();
                        pb8 = pb9.add(1);
                        if b4 == 0x80 {
                            break;
                        }
                        if b4 & 0x80 == 0 {
                            if b4 & 0x40 == 0 {
                                pb9 = pb9.add(2);
                                let u5 = b4 as u32;
                                let i12 = pal(param_1, pb8.read());
                                let mut idx = i10 as u32;
                                let mut u7 = u5;
                                while u7 != 0 {
                                    wr(local_c, idx, i12);
                                    idx = idx.wrapping_add(1);
                                    u7 = u7.wrapping_sub(1);
                                }
                                i10 = i10.wrapping_add(u5 as i32);
                            } else {
                                i10 = i10.wrapping_add((b4 & 0x3f) as i32);
                                pb9 = pb8;
                            }
                        } else {
                            let mut i12: i32 = 0;
                            b4 &= 0x7f;
                            if b4 != 0 {
                                loop {
                                    let i13 = i10.wrapping_add(i12);
                                    let p = pb8.add(i12 as u32 as usize);
                                    i12 = i12.wrapping_add(1);
                                    wr(local_c, i13 as u32, pal(param_1, p.read()));
                                    if !(i12 < b4 as i32) {
                                        break;
                                    }
                                }
                            }
                            i10 = i10.wrapping_add(b4 as i32);
                            pb9 = pb8.add(b4 as usize);
                        }
                    }
                    local_c = local_c.wrapping_add(stride() & 0xfffffffe);
                    local_24 = local_24.wrapping_sub(1);
                    if local_24 == 0 {
                        break;
                    }
                }
                return;
            }
        } else {
            let mut local_10: *const u8 = param_1.add(4).read_unaligned() as u32 as *const u8;
            let mut local_c: u32 = (param_3 as u32)
                .wrapping_mul(stride())
                .wrapping_add((param_2 as u32).wrapping_mul(2))
                .wrapping_add(DAT_00758830 as u32);
            if 0 < param_5 {
                loop {
                    let d = (local_10 as *const i32).read_unaligned();
                    local_10 = (local_10 as u32).wrapping_add(d as u32) as *const u8;
                    param_5 = param_5.wrapping_sub(1);
                    if param_5 == 0 {
                        break;
                    }
                }
            }
            if 0 < param_7 {
                let mut local_1c = param_7;
                loop {
                    let mut i12: i32 = 0;
                    let mut b1: bool = false;
                    let i10: i32 = (local_10 as *const i32).read_unaligned();
                    let mut i13: i32 = 4;
                    let mut b4: u8 = 0;
                    let mut u7: u32;
                    'row: {
                        loop {
                            'f: loop {
                                loop {
                                    let mut i11 = i13.wrapping_add(1);
                                    b4 = local_10.add(i13 as u32 as usize).read();
                                    u7 = b4 as u32;
                                    if !b1 {
                                        if b4 == 0x80 {
                                            break 'row;
                                        }
                                        if b4 & 0x80 == 0 {
                                            let c2 = i12 as i8;
                                            if b4 & 0x40 == 0 {
                                                i12 = (b4 as i32).wrapping_add(i12);
                                                if i12 < param_4 {
                                                    i11 = i13.wrapping_add(2);
                                                } else {
                                                    b1 = true;
                                                    i12 = 0;
                                                    b4 = b4.wrapping_sub(
                                                        (param_4 as i8).wrapping_sub(c2) as u8,
                                                    );
                                                    u7 = b4 as u32;
                                                    if b4 == 0 {
                                                        i11 = i13.wrapping_add(2);
                                                    }
                                                }
                                            } else {
                                                u7 &= 0xffffff3f;
                                                i12 = ((b4 & 0x3f) as i32).wrapping_add(i12);
                                                if param_4 <= i12 {
                                                    b1 = true;
                                                    i12 = 0;
                                                    b4 = (u7 as i8).wrapping_sub(
                                                        (param_4 as i8).wrapping_sub(c2),
                                                    )
                                                        as u8;
                                                    u7 = b4 as u32;
                                                    if b4 != 0 {
                                                        u7 = (b4 | 0x40) as u32;
                                                    }
                                                }
                                            }
                                        } else {
                                            u7 &= 0xffffff7f;
                                            let t = ((b4 & 0x7f) as i32).wrapping_add(i12);
                                            if t < param_4 {
                                                i11 = i11.wrapping_add((b4 & 0x7f) as i32);
                                                i12 = t;
                                            } else {
                                                b1 = true;
                                                let d = param_4.wrapping_sub(i12);
                                                i11 = i11.wrapping_add(d);
                                                b4 = (u7 as i8).wrapping_sub(d as i8) as u8;
                                                u7 = b4 as u32;
                                                i12 = 0;
                                                if b4 != 0 {
                                                    u7 = (b4 | 0x80) as u32;
                                                }
                                            }
                                        }
                                    }
                                    i13 = i11;
                                    if b1 {
                                        b4 = u7 as u8;
                                        if b4 != 0 {
                                            break;
                                        }
                                    }
                                }
                                // i11 is the offset after the opcode here
                                let i11 = i13;
                                if b4 == 0x80 {
                                    break 'row;
                                }
                                if u7 & 0x80 != 0 {
                                    let u5 = u7 & 0x7f;
                                    if param_6 <= (u5.wrapping_add(i12 as u32)) as i32 {
                                        let n = (param_6 as u8).wrapping_sub(i12 as u8);
                                        if n != 0 {
                                            let mut k: i32 = 0;
                                            loop {
                                                let k1 = k.wrapping_add(1);
                                                let src = local_10
                                                    .add((k as u32).wrapping_add(i11 as u32)
                                                        as usize)
                                                    .read();
                                                wr(
                                                    local_c,
                                                    (k as u32).wrapping_add(i12 as u32),
                                                    pal(param_1, src),
                                                );
                                                k = k1;
                                                if !(k1 < n as i32) {
                                                    break;
                                                }
                                            }
                                        }
                                        break 'row;
                                    }
                                    let mut k: i32 = 0;
                                    if u7 & 0x7f != 0 {
                                        loop {
                                            let dst = (i12 as u32).wrapping_add(k as u32);
                                            let s = (i11 as u32).wrapping_add(k as u32);
                                            k = k.wrapping_add(1);
                                            let src = local_10.add(s as usize).read();
                                            wr(local_c, dst, pal(param_1, src));
                                            if !(k < u5 as i32) {
                                                break;
                                            }
                                        }
                                    }
                                    i12 = i12.wrapping_add(u5 as i32);
                                    i13 = i11.wrapping_add(u5 as i32);
                                    continue 'f;
                                }
                                if u7 & 0x40 != 0 {
                                    break 'f;
                                }
                                i13 = i11.wrapping_add(1);
                                if param_6 <= (u7.wrapping_add(i12 as u32)) as i32 {
                                    u7 = (param_6 as u8).wrapping_sub(i12 as u8) as u32;
                                    let v = pal(param_1, local_10.add(i11 as u32 as usize).read());
                                    let mut idx = i12 as u32;
                                    while u7 != 0 {
                                        wr(local_c, idx, v);
                                        idx = idx.wrapping_add(1);
                                        u7 = u7.wrapping_sub(1);
                                    }
                                    break 'row;
                                }
                                let v = pal(param_1, local_10.add(i11 as u32 as usize).read());
                                let mut idx = i12 as u32;
                                let mut u5 = u7;
                                while u5 != 0 {
                                    wr(local_c, idx, v);
                                    idx = idx.wrapping_add(1);
                                    u5 = u5.wrapping_sub(1);
                                }
                                i12 = i12.wrapping_add(u7 as i32);
                            }
                            i12 = i12.wrapping_add((b4 & 0x3f) as i32);
                            if !(i12 < param_6) {
                                break;
                            }
                        }
                    }
                    local_10 = (local_10 as u32).wrapping_add(i10 as u32) as *const u8;
                    local_c = local_c.wrapping_add(stride() & 0xfffffffe);
                    local_1c = local_1c.wrapping_sub(1);
                    if local_1c == 0 {
                        break;
                    }
                }
            }
        }
    }
}
