// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0075964b: i32;
    static mut DAT_00758830: i32;
}

#[inline(always)]
fn addr(a: i32) -> *mut u8 {
    a as u32 as usize as *mut u8
}

#[inline(always)]
unsafe fn fill(mut dst: *mut u8, val: u32, n: u32) {
    // SAFETY: the caller supplies a destination the original writes to.
    unsafe {
        let mut w = n >> 2;
        while w != 0 {
            dst.cast::<u32>().write_unaligned(val);
            dst = dst.wrapping_add(4);
            w -= 1;
        }
        let mut b = n & 3;
        while b != 0 {
            *dst = val as u8;
            dst = dst.wrapping_add(1);
            b -= 1;
        }
    }
}

#[inline(always)]
unsafe fn copy(mut dst: *mut u8, mut src: *const u8, n: u32) {
    // SAFETY: the caller supplies source and destination the original uses.
    unsafe {
        let mut w = n >> 2;
        while w != 0 {
            dst.cast::<u32>()
                .write_unaligned(src.cast::<u32>().read_unaligned());
            src = src.wrapping_add(4);
            dst = dst.wrapping_add(4);
            w -= 1;
        }
        let mut b = n & 3;
        while b != 0 {
            *dst = *src;
            src = src.wrapping_add(1);
            dst = dst.wrapping_add(1);
            b -= 1;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
#[allow(unused_assignments)]
pub unsafe extern "C" fn FUN_004d1960(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
) {
    // SAFETY: all pointers are the game's own buffers and globals, read and
    // written exactly where the original does.
    unsafe {
        let stride: i32 = (&raw const DAT_0075964b).cast::<i32>().read_unaligned();
        let mut param_5 = param_5;

        if param_1.read() == param_6 {
            let mut pb11: *mut u8 = addr(param_1.add(4).read());
            let mut local_14: i32 = param_3
                .wrapping_mul(stride)
                .wrapping_add(DAT_00758830)
                .wrapping_add(param_2);
            if 0 < param_5 {
                loop {
                    let tmp = pb11.cast::<i32>().read_unaligned();
                    pb11 = pb11.wrapping_offset(tmp as isize);
                    param_5 = param_5.wrapping_sub(1);
                    if param_5 == 0 {
                        break;
                    }
                }
            }
            if 0 < param_7 {
                let mut local_8 = param_7;
                loop {
                    let mut i10: i32 = 0;
                    let mut pb12: *mut u8 = pb11.wrapping_add(4);
                    loop {
                        let b4: u8 = *pb12;
                        pb11 = pb12.wrapping_add(1);
                        if b4 == 0x80 {
                            break;
                        }
                        if b4 & 0x80 == 0 {
                            if b4 & 0x40 == 0 {
                                let b5 = *pb11;
                                pb12 = pb12.wrapping_add(2);
                                let dst = addr(local_14.wrapping_add(i10));
                                i10 = i10.wrapping_add(b4 as i32);
                                fill(dst, (b5 as u32).wrapping_mul(0x01010101), b4 as u32);
                            } else {
                                let b4 = b4 & 0x3f;
                                let dst = addr(local_14.wrapping_add(i10));
                                i10 = i10.wrapping_add(b4 as i32);
                                fill(dst, 0, b4 as u32);
                                pb12 = pb11;
                            }
                        } else {
                            let n = (b4 & 0x7f) as u32;
                            let dst = addr(local_14.wrapping_add(i10));
                            i10 = i10.wrapping_add(n as i32);
                            pb12 = pb11.wrapping_add(n as usize);
                            copy(dst, pb11, n);
                        }
                    }
                    local_14 = local_14.wrapping_add(stride);
                    local_8 = local_8.wrapping_sub(1);
                    if local_8 == 0 {
                        break;
                    }
                }
                return;
            }
        } else {
            let mut local_10: *mut u8 = addr(param_1.add(4).read());
            let mut local_14: i32 = param_3
                .wrapping_mul(stride)
                .wrapping_add(DAT_00758830)
                .wrapping_add(param_2);
            if 0 < param_5 {
                loop {
                    let tmp = local_10.cast::<i32>().read_unaligned();
                    local_10 = local_10.wrapping_offset(tmp as isize);
                    param_5 = param_5.wrapping_sub(1);
                    if param_5 == 0 {
                        break;
                    }
                }
            }
            if 0 < param_7 {
                let mut local_4 = param_7;
                loop {
                    let mut b3: u8 = 0;
                    let i2: i32 = local_10.cast::<i32>().read_unaligned();
                    let mut i10: i32 = 0;
                    let mut i14: i32 = 4;
                    let mut i8: i32 = 0;
                    let mut i13: i32;
                    let mut b4: u8;
                    let mut b5: u8;
                    let mut u6: u32 = 0;
                    'lab: loop {
                        loop {
                            i13 = i14.wrapping_add(1);
                            b4 = *local_10.wrapping_offset(i14 as isize);
                            i8 = i10;
                            if b3 == 0 {
                                if b4 == 0x80 {
                                    break 'lab;
                                }
                                if b4 & 0x80 == 0 {
                                    if b4 & 0x40 == 0 {
                                        i8 = (b4 as i32).wrapping_add(i10);
                                        if i8 < param_4 {
                                            i13 = i14.wrapping_add(2);
                                        } else {
                                            b3 = 1;
                                            i8 = 0;
                                            b4 = (b4 as i32).wrapping_sub(param_4.wrapping_sub(i10))
                                                as u8;
                                            if b4 == 0 {
                                                i13 = i14.wrapping_add(2);
                                            }
                                        }
                                    } else {
                                        b4 &= 0x3f;
                                        i8 = (b4 as i32).wrapping_add(i10);
                                        if param_4 <= i8 {
                                            b3 = 1;
                                            i8 = 0;
                                            b5 = (b4 as i32).wrapping_sub(param_4.wrapping_sub(i10))
                                                as u8;
                                            b4 = 0;
                                            if b5 != 0 {
                                                b4 = b5 | 0x40;
                                            }
                                        }
                                    }
                                } else {
                                    b4 &= 0x7f;
                                    i8 = (b4 as i32).wrapping_add(i10);
                                    if i8 < param_4 {
                                        i13 = i13.wrapping_add(b4 as i32);
                                    } else {
                                        b3 = 1;
                                        i8 = 0;
                                        i13 = i13.wrapping_add(param_4.wrapping_sub(i10));
                                        b5 = (b4 as i32).wrapping_sub(param_4.wrapping_sub(i10))
                                            as u8;
                                        b4 = 0;
                                        if b5 != 0 {
                                            b4 = b5 | 0x80;
                                        }
                                    }
                                }
                            }
                            i14 = i13;
                            i10 = i8;
                            if !(b3 == 0 || b4 == 0) {
                                break;
                            }
                        }
                        if b4 != 0x80 {
                            if b4 & 0x80 == 0 {
                                if b4 & 0x40 == 0 {
                                    i13 = i14.wrapping_add(1);
                                    let u1 = *local_10.wrapping_offset(i14 as isize);
                                    i10 = (b4 as i32).wrapping_add(i8);
                                    u6 = (u1 as u32).wrapping_mul(0x01010101);
                                    if i10 < param_6 {
                                        fill(addr(local_14.wrapping_add(i8)), u6, b4 as u32);
                                        i14 = i13;
                                        continue 'lab;
                                    }
                                } else {
                                    b4 &= 0x3f;
                                    i10 = (b4 as i32).wrapping_add(i8);
                                    u6 = 0;
                                    if i10 < param_6 {
                                        fill(addr(local_14.wrapping_add(i8)), u6, b4 as u32);
                                        continue 'lab;
                                    }
                                }
                                b4 = param_6.wrapping_sub(i8) as u8;
                                fill(addr(local_14.wrapping_add(i8)), u6, b4 as u32);
                            } else {
                                let u7 = (b4 & 0x7f) as u32;
                                i10 = (u7 as i32).wrapping_add(i8);
                                if i10 < param_6 {
                                    let src = local_10.wrapping_offset(i14 as isize);
                                    i14 = i14.wrapping_add(u7 as i32);
                                    copy(addr(local_14.wrapping_add(i8)), src, u7);
                                    continue 'lab;
                                }
                                b4 = param_6.wrapping_sub(i8) as u8;
                                let src = local_10.wrapping_offset(i14 as isize);
                                copy(addr(local_14.wrapping_add(i8)), src, b4 as u32);
                            }
                        }
                        break;
                    }
                    local_10 = local_10.wrapping_offset(i2 as isize);
                    local_14 = local_14.wrapping_add(stride);
                    local_4 = local_4.wrapping_sub(1);
                    if local_4 == 0 {
                        return;
                    }
                }
            }
        }
    }
}
