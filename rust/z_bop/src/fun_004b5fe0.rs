// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b13: [u8; 4];
    static mut DAT_00aa6cfc: i32;
}

#[inline(always)]
unsafe fn rb(a: u32) -> u8 {
    // SAFETY: address inside the game's own data stream.
    unsafe { (a as usize as *const u8).read() }
}

#[inline(always)]
unsafe fn rd(a: u32) -> u32 {
    // SAFETY: address inside the game's own data stream.
    unsafe { (a as usize as *const u32).read_unaligned() }
}

unsafe fn fill(dst: u32, dw: u32, by: u32, v: u32) {
    let mut p = dst as usize as *mut u8;
    let mut n = dw;
    // SAFETY: destination is the game's own surface memory.
    unsafe {
        while n != 0 {
            (p as *mut u32).write_unaligned(v);
            p = p.wrapping_add(4);
            n -= 1;
        }
        let mut m = by;
        while m != 0 {
            p.write(v as u8);
            p = p.wrapping_add(1);
            m -= 1;
        }
    }
}

unsafe fn copy(dst: u32, src: u32, dw: u32, by: u32) {
    let mut d = dst as usize as *mut u8;
    let mut s = src as usize as *const u8;
    let mut n = dw;
    // SAFETY: source and destination are the game's own memory.
    unsafe {
        while n != 0 {
            (d as *mut u32).write_unaligned((s as *const u32).read_unaligned());
            d = d.wrapping_add(4);
            s = s.wrapping_add(4);
            n -= 1;
        }
        let mut m = by;
        while m != 0 {
            d.write(s.read());
            d = d.wrapping_add(1);
            s = s.wrapping_add(1);
            m -= 1;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b5fe0(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
) {
    let mut param_5 = param_5;
    // SAFETY: all pointers and globals are the game's own memory, accessed
    // where the original accesses them.
    unsafe {
        let first = param_1.read_unaligned();
        let stride0 = u32::from_ne_bytes(DAT_00aa7b13);
        let mut local_14: u32 = stride0
            .wrapping_mul(param_3 as u32)
            .wrapping_add(param_2 as u32)
            .wrapping_add(DAT_00aa6cfc as u32);
        let mut p11: u32 = param_1.byte_add(16).read_unaligned() as u32;

        if first == param_6 {
            if 0 < param_5 {
                loop {
                    p11 = p11.wrapping_add(rd(p11));
                    param_5 = param_5.wrapping_sub(1);
                    if param_5 == 0 {
                        break;
                    }
                }
            }
            if 0 < param_7 {
                let mut local_8 = param_7;
                loop {
                    let mut i10: u32 = 0;
                    let mut p12 = p11.wrapping_add(4);
                    loop {
                        let b5 = rb(p12);
                        p11 = p12.wrapping_add(1);
                        if b5 == 0x80 {
                            break;
                        }
                        if b5 & 0x80 == 0 {
                            if b5 & 0x40 == 0 {
                                let b1 = rb(p11);
                                p12 = p12.wrapping_add(2);
                                let d = local_14.wrapping_add(i10);
                                i10 = i10.wrapping_add(b5 as u32);
                                fill(
                                    d,
                                    (b5 >> 2) as u32,
                                    (b5 & 3) as u32,
                                    (b1 as u32).wrapping_mul(0x01010101),
                                );
                            } else {
                                let c = b5 & 0x3f;
                                let d = local_14.wrapping_add(i10);
                                i10 = i10.wrapping_add(c as u32);
                                fill(d, (c >> 2) as u32, (c & 3) as u32, 0);
                                p12 = p11;
                            }
                        } else {
                            let u7 = (b5 & 0x7f) as u32;
                            let d = local_14.wrapping_add(i10);
                            i10 = i10.wrapping_add(u7);
                            p12 = p11.wrapping_add(u7);
                            copy(d, p11, u7 >> 2, (b5 & 3) as u32);
                        }
                    }
                    local_14 = local_14.wrapping_add(u32::from_ne_bytes(DAT_00aa7b13));
                    local_8 = local_8.wrapping_sub(1);
                    if local_8 == 0 {
                        break;
                    }
                }
                return;
            }
        } else {
            let mut local_c: u32 = p11;
            if 0 < param_5 {
                loop {
                    local_c = local_c.wrapping_add(rd(local_c));
                    param_5 = param_5.wrapping_sub(1);
                    if param_5 == 0 {
                        break;
                    }
                }
            }
            if 0 < param_7 {
                let mut local_4 = param_7;
                loop {
                    let mut b4: u8 = 0;
                    let mut i3: u32 = rd(local_c);
                    let mut i10: i32 = 0;
                    let mut i14: u32 = 4;
                    'row: {
                        'inner: loop {
                            let mut i13 = i14.wrapping_add(1);
                            let mut b5: u8 = rb(local_c.wrapping_add(i14));
                            let mut u7: u32 = b5 as u32;
                            let mut i8: i32 = i10;
                            if b4 == 0 {
                                if b5 == 0x80 {
                                    break 'row;
                                }
                                if b5 & 0x80 == 0 {
                                    if b5 & 0x40 == 0 {
                                        i8 = (b5 as i32).wrapping_add(i10);
                                        if i8 < param_4 {
                                            i13 = i14.wrapping_add(2);
                                        } else {
                                            b4 = 1;
                                            i8 = 0;
                                            b5 = b5.wrapping_sub(
                                                (param_4 as u8).wrapping_sub(i10 as u8),
                                            );
                                            u7 = b5 as u32;
                                            if b5 == 0 {
                                                i13 = i14.wrapping_add(2);
                                            }
                                        }
                                    } else {
                                        u7 &= 0xffffff3f;
                                        i8 = ((b5 & 0x3f) as i32).wrapping_add(i10);
                                        if param_4 <= i8 {
                                            b4 = 1;
                                            i8 = 0;
                                            b5 = (u7 as u8).wrapping_sub(
                                                (param_4 as u8).wrapping_sub(i10 as u8),
                                            );
                                            u7 = b5 as u32;
                                            if b5 != 0 {
                                                u7 = (b5 | 0x40) as u32;
                                            }
                                        }
                                    }
                                } else {
                                    u7 &= 0xffffff7f;
                                    i8 = ((b5 & 0x7f) as i32).wrapping_add(i10);
                                    if i8 < param_4 {
                                        i13 = i13.wrapping_add((b5 & 0x7f) as u32);
                                    } else {
                                        b4 = 1;
                                        i8 = 0;
                                        let diff = (param_4 as u32).wrapping_sub(i10 as u32);
                                        i13 = i13.wrapping_add(diff);
                                        b5 = (u7 as u8).wrapping_sub(diff as u8);
                                        u7 = b5 as u32;
                                        if b5 != 0 {
                                            u7 = (b5 | 0x80) as u32;
                                        }
                                    }
                                }
                            }
                            i14 = i13;
                            i10 = i8;
                            if b4 != 0 {
                                b5 = u7 as u8;
                                if b5 != 0 {
                                    if b5 != 0x80 {
                                        if u7 & 0x80 == 0 {
                                            let v: u32;
                                            if u7 & 0x40 == 0 {
                                                let i13b = i14.wrapping_add(1);
                                                let u2 = rb(local_c.wrapping_add(i14));
                                                i10 = (u7 as i32).wrapping_add(i8);
                                                if i10 < param_6 {
                                                    fill(
                                                        local_14.wrapping_add(i8 as u32),
                                                        u7 >> 2,
                                                        u7 & 3,
                                                        (u2 as u32).wrapping_mul(0x01010101),
                                                    );
                                                    i14 = i13b;
                                                    continue 'inner;
                                                }
                                                v = (u2 as u32).wrapping_mul(0x01010101);
                                            } else {
                                                b5 &= 0x3f;
                                                i10 = (b5 as i32).wrapping_add(i8);
                                                if i10 < param_6 {
                                                    fill(
                                                        local_14.wrapping_add(i8 as u32),
                                                        (b5 >> 2) as u32,
                                                        (b5 & 3) as u32,
                                                        0,
                                                    );
                                                    continue 'inner;
                                                }
                                                v = 0;
                                            }
                                            let n = (param_6 as u8).wrapping_sub(i8 as u8);
                                            fill(
                                                local_14.wrapping_add(i8 as u32),
                                                (n >> 2) as u32,
                                                (n & 3) as u32,
                                                v,
                                            );
                                        } else {
                                            u7 = (b5 & 0x7f) as u32;
                                            i10 = (u7 as i32).wrapping_add(i8);
                                            if i10 < param_6 {
                                                let src = local_c.wrapping_add(i14);
                                                i14 = i14.wrapping_add(u7);
                                                copy(
                                                    local_14.wrapping_add(i8 as u32),
                                                    src,
                                                    ((b5 & 0x7f) >> 2) as u32,
                                                    u7 & 3,
                                                );
                                                continue 'inner;
                                            }
                                            let n = (param_6 as u8).wrapping_sub(i8 as u8);
                                            copy(
                                                local_14.wrapping_add(i8 as u32),
                                                local_c.wrapping_add(i14),
                                                (n >> 2) as u32,
                                                (n & 3) as u32,
                                            );
                                        }
                                    }
                                    break 'inner;
                                }
                            }
                        }
                    }
                    local_c = local_c.wrapping_add(i3);
                    i3 = u32::from_ne_bytes(DAT_00aa7b13);
                    local_14 = local_14.wrapping_add(i3);
                    local_4 = local_4.wrapping_sub(1);
                    if local_4 == 0 {
                        return;
                    }
                }
            }
        }
    }
}
