// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758830: i32;
    static mut DAT_0075964b: [u8; 4];
}

#[inline(always)]
unsafe fn fill_run(mut dst: *mut u8, n: u8, fill: u32, b: u8) -> *mut u8 {
    // SAFETY: the caller supplies a destination buffer large enough for the run.
    unsafe {
        let mut c = (n >> 2) as u32;
        while c != 0 {
            dst.cast::<u32>().write_unaligned(fill);
            dst = dst.add(4);
            c -= 1;
        }
        let mut c = (n & 3) as u32;
        while c != 0 {
            *dst = b;
            dst = dst.add(1);
            c -= 1;
        }
        dst
    }
}

#[inline(always)]
unsafe fn copy_run(mut dst: *mut u8, mut src: *const u8, n: u8) {
    // SAFETY: the caller supplies valid source and destination buffers.
    unsafe {
        let mut c = (n >> 2) as u32;
        while c != 0 {
            dst.cast::<u32>()
                .write_unaligned(src.cast::<u32>().read_unaligned());
            src = src.add(4);
            dst = dst.add(4);
            c -= 1;
        }
        let mut c = (n & 3) as u32;
        while c != 0 {
            *dst = *src;
            src = src.add(1);
            dst = dst.add(1);
            c -= 1;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d1080(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
) {
    let mut param_5 = param_5;
    // SAFETY: the globals and the RLE sprite data and the destination surface
    // are the game's own memory, accessed exactly as the original does.
    unsafe {
        if param_1.read() == param_6 {
            let mut p11 = param_1.add(4).read() as usize as *const u8;
            let stride = (&raw mut DAT_0075964b).cast::<u32>().read_unaligned();
            let mut local_14: u32 = (param_3 as u32)
                .wrapping_mul(stride)
                .wrapping_add(DAT_00758830 as u32)
                .wrapping_add(param_2 as u32);
            if 0 < param_5 {
                loop {
                    let tmp = p11.cast::<i32>().read_unaligned();
                    p11 = p11.offset(tmp as isize);
                    param_5 -= 1;
                    if param_5 == 0 {
                        break;
                    }
                }
            }
            if 0 < param_7 {
                let mut local_8 = param_7;
                loop {
                    let mut i13: i32 = 0;
                    let mut p12 = p11.add(4);
                    loop {
                        let b9 = *p12;
                        p11 = p12.add(1);
                        if b9 == 0x80 {
                            break;
                        }
                        if b9 & 0x80 == 0 {
                            if b9 & 0x40 == 0 {
                                let b10 = *p11;
                                p12 = p12.add(2);
                                let dst = local_14.wrapping_add(i13 as u32) as usize as *mut u8;
                                i13 = i13.wrapping_add(b9 as i32);
                                let fill = (b10 as u32).wrapping_mul(0x01010101);
                                p11 = fill_run(dst, b9, fill, b10);
                            } else {
                                i13 = i13.wrapping_add((b9 & 0x3f) as i32);
                                p12 = p11;
                            }
                        } else {
                            let u5 = (b9 & 0x7f) as u32;
                            let dst = local_14.wrapping_add(i13 as u32) as usize as *mut u8;
                            i13 = i13.wrapping_add(u5 as i32);
                            p12 = p11.add(u5 as usize);
                            copy_run(dst, p11, b9 & 0x7f);
                            p11 = p11.add(u5 as usize);
                        }
                    }
                    let stride = (&raw mut DAT_0075964b).cast::<u32>().read_unaligned();
                    local_14 = local_14.wrapping_add(stride);
                    local_8 -= 1;
                    if local_8 == 0 {
                        break;
                    }
                }
                return;
            }
        } else {
            let mut p14 = param_1.add(4).read() as usize as *const u8;
            let stride = (&raw mut DAT_0075964b).cast::<u32>().read_unaligned();
            let mut local_14: u32 = (param_3 as u32)
                .wrapping_mul(stride)
                .wrapping_add(DAT_00758830 as u32)
                .wrapping_add(param_2 as u32);
            if 0 < param_5 {
                loop {
                    let tmp = p14.cast::<i32>().read_unaligned();
                    p14 = p14.offset(tmp as isize);
                    param_5 -= 1;
                    if param_5 == 0 {
                        break;
                    }
                }
            }
            if 0 < param_7 {
                let mut local_4 = param_7;
                loop {
                    let mut i7: i32 = 4;
                    let i2: i32 = p14.cast::<i32>().read_unaligned();
                    let mut b3: i32 = 0;
                    let mut i13: i32 = 0;
                    let mut b9: u8;
                    let mut i6: i32;
                    'end: {
                        'inner: loop {
                            b9 = *p14.offset(i7 as isize);
                            let mut i8 = i7.wrapping_add(1);
                            i6 = i13;
                            if b3 == 0 {
                                if b9 == 0x80 {
                                    break 'end;
                                }
                                if b9 & 0x80 == 0 {
                                    if b9 & 0x40 == 0 {
                                        i6 = (b9 as i32).wrapping_add(i13);
                                        if i6 < param_4 {
                                            i8 = i7.wrapping_add(2);
                                        } else {
                                            b3 = 1;
                                            i6 = 0;
                                            b9 = b9.wrapping_sub(
                                                (param_4 as u8).wrapping_sub(i13 as u8),
                                            );
                                            if b9 == 0 {
                                                i8 = i7.wrapping_add(2);
                                            }
                                        }
                                    } else {
                                        b9 &= 0x3f;
                                        i6 = (b9 as i32).wrapping_add(i13);
                                        if param_4 <= i6 {
                                            b3 = 1;
                                            i6 = 0;
                                            let b10 = b9.wrapping_sub(
                                                (param_4 as u8).wrapping_sub(i13 as u8),
                                            );
                                            b9 = 0;
                                            if b10 != 0 {
                                                b9 = b10 | 0x40;
                                            }
                                        }
                                    }
                                } else {
                                    b9 &= 0x7f;
                                    i6 = (b9 as i32).wrapping_add(i13);
                                    if i6 < param_4 {
                                        i8 = i8.wrapping_add(b9 as i32);
                                    } else {
                                        b3 = 1;
                                        i6 = 0;
                                        i8 = i8.wrapping_add(param_4.wrapping_sub(i13));
                                        let b10 = b9.wrapping_sub(param_4.wrapping_sub(i13) as u8);
                                        b9 = 0;
                                        if b10 != 0 {
                                            b9 = b10 | 0x80;
                                        }
                                    }
                                }
                            }
                            i7 = i8;
                            i13 = i6;
                            if b3 == 0 || b9 == 0 {
                                continue 'inner;
                            }
                            if b9 == 0x80 {
                                break 'end;
                            }
                            if b9 & 0x80 == 0 {
                                if b9 & 0x40 == 0 {
                                    let u1 = *p14.offset(i7 as isize);
                                    i7 = i7.wrapping_add(1);
                                    i13 = i6.wrapping_add(b9 as i32);
                                    let fill = (u1 as u32).wrapping_mul(0x01010101);
                                    if param_6 <= i13 {
                                        b9 = (param_6 as u8).wrapping_sub(i6 as u8);
                                        let dst =
                                            local_14.wrapping_add(i6 as u32) as usize as *mut u8;
                                        fill_run(dst, b9, fill, u1);
                                        break 'end;
                                    }
                                    let dst = local_14.wrapping_add(i6 as u32) as usize as *mut u8;
                                    fill_run(dst, b9, fill, u1);
                                    continue 'inner;
                                }
                                i13 = i6.wrapping_add((b9 & 0x3f) as i32);
                                if param_6 <= i13 {
                                    break 'end;
                                }
                                continue 'inner;
                            }
                            let u4 = (b9 & 0x7f) as u32;
                            i13 = (u4 as i32).wrapping_add(i6);
                            if i13 < param_6 {
                                let src = p14.offset(i7 as isize);
                                i7 = i7.wrapping_add(u4 as i32);
                                let dst = local_14.wrapping_add(i6 as u32) as usize as *mut u8;
                                copy_run(dst, src, b9 & 0x7f);
                                continue 'inner;
                            }
                            b9 = (param_6 as u8).wrapping_sub(i6 as u8);
                            let src = p14.offset(i7 as isize);
                            let dst = local_14.wrapping_add(i6 as u32) as usize as *mut u8;
                            copy_run(dst, src, b9);
                            break 'end;
                        }
                    }
                    p14 = p14.offset(i2 as isize);
                    let stride = (&raw mut DAT_0075964b).cast::<u32>().read_unaligned();
                    local_14 = local_14.wrapping_add(stride);
                    local_4 -= 1;
                    if local_4 == 0 {
                        return;
                    }
                }
            }
        }
    }
}
