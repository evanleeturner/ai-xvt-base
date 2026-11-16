// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa6cfc: i32;
    static mut DAT_00aa7b13: [u8; 4];
}

unsafe fn fill_run(mut dst: *mut u8, fill: u32, b: u8, n: u32) {
    // SAFETY: the caller guarantees dst points at n writable bytes.
    unsafe {
        let mut c = n >> 2;
        while c != 0 {
            dst.cast::<u32>().write_unaligned(fill);
            dst = dst.byte_add(4);
            c -= 1;
        }
        let mut c = n & 3;
        while c != 0 {
            *dst = b;
            dst = dst.byte_add(1);
            c -= 1;
        }
    }
}

unsafe fn copy_run(mut dst: *mut u8, mut src: *const u8, n: u32) {
    // SAFETY: the caller guarantees src has n readable and dst n writable bytes.
    unsafe {
        let mut c = n >> 2;
        while c != 0 {
            dst.cast::<u32>()
                .write_unaligned(src.cast::<u32>().read_unaligned());
            dst = dst.byte_add(4);
            src = src.byte_add(4);
            c -= 1;
        }
        let mut c = n & 3;
        while c != 0 {
            *dst = *src;
            dst = dst.byte_add(1);
            src = src.byte_add(1);
            c -= 1;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b5770(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
) {
    // SAFETY: all pointers are the game's own RLE data and frame buffer,
    // read and written exactly where the original does.
    unsafe {
        let mut param_5 = param_5;
        let mut p11 = param_1.add(4).read() as u32 as usize as *const u8;
        let stride = (&raw mut DAT_00aa7b13).cast::<i32>().read_unaligned();
        let mut local_14: u32 = (stride as u32)
            .wrapping_mul(param_3 as u32)
            .wrapping_add(param_2 as u32)
            .wrapping_add(DAT_00aa6cfc as u32);
        if *param_1 == param_6 {
            while 0 < param_5 {
                let off = p11.cast::<i32>().read_unaligned();
                p11 = p11.offset(off as isize);
                param_5 -= 1;
            }
            if 0 < param_7 {
                let mut local_4 = param_7;
                loop {
                    let mut ivar10: i32 = 0;
                    let mut p12 = p11.add(4);
                    loop {
                        let b9 = *p12;
                        p11 = p12.add(1);
                        if b9 == 0x80 {
                            break;
                        }
                        if b9 & 0x80 == 0 {
                            if b9 & 0x40 == 0 {
                                let b3 = *p11;
                                p12 = p12.add(2);
                                let dst = local_14.wrapping_add(ivar10 as u32) as usize as *mut u8;
                                ivar10 = ivar10.wrapping_add(b9 as i32);
                                let fill = (b3 as u32).wrapping_mul(0x01010101);
                                fill_run(
                                    dst,
                                    fill,
                                    b3,
                                    (b9 & 3) as u32 | (((b9 >> 2) as u32) << 2),
                                );
                            } else {
                                ivar10 = ivar10.wrapping_add((b9 & 0x3f) as i32);
                                p12 = p11;
                            }
                        } else {
                            let n = (b9 & 0x7f) as u32;
                            let dst = local_14.wrapping_add(ivar10 as u32) as usize as *mut u8;
                            ivar10 = ivar10.wrapping_add(n as i32);
                            p12 = p11.add(n as usize);
                            copy_run(dst, p11, n);
                        }
                    }
                    let stride = (&raw mut DAT_00aa7b13).cast::<i32>().read_unaligned();
                    local_14 = local_14.wrapping_add(stride as u32);
                    local_4 -= 1;
                    if local_4 == 0 {
                        break;
                    }
                }
            }
            return;
        }
        while 0 < param_5 {
            let off = p11.cast::<i32>().read_unaligned();
            p11 = p11.offset(off as isize);
            param_5 -= 1;
        }
        if 0 < param_7 {
            let mut local_c = param_7;
            loop {
                let mut ivar7: i32 = 4;
                let ivar1 = p11.cast::<i32>().read_unaligned();
                let mut bvar2: i32 = 0;
                let mut ivar10: i32 = 0;
                'done: {
                    'main: loop {
                        let mut b9 = *p11.offset(ivar7 as isize);
                        let mut u5 = b9 as u32;
                        let mut ivar8 = ivar7.wrapping_add(1);
                        let mut ivar6 = ivar10;
                        if bvar2 == 0 {
                            if b9 == 0x80 {
                                break 'done;
                            }
                            if b9 & 0x80 == 0 {
                                if b9 & 0x40 == 0 {
                                    ivar6 = (b9 as i32).wrapping_add(ivar10);
                                    if ivar6 < param_4 {
                                        ivar8 = ivar7.wrapping_add(2);
                                    } else {
                                        bvar2 = 1;
                                        ivar6 = 0;
                                        b9 = b9.wrapping_sub(param_4.wrapping_sub(ivar10) as u8);
                                        u5 = b9 as u32;
                                        if b9 == 0 {
                                            ivar8 = ivar7.wrapping_add(2);
                                        }
                                    }
                                } else {
                                    u5 &= 0xffffff3f;
                                    ivar6 = ((b9 & 0x3f) as i32).wrapping_add(ivar10);
                                    if param_4 <= ivar6 {
                                        bvar2 = 1;
                                        ivar6 = 0;
                                        b9 = (u5 as u8)
                                            .wrapping_sub(param_4.wrapping_sub(ivar10) as u8);
                                        u5 = b9 as u32;
                                        if b9 != 0 {
                                            u5 = (b9 | 0x40) as u32;
                                        }
                                    }
                                }
                            } else {
                                u5 &= 0xffffff7f;
                                ivar6 = ((b9 & 0x7f) as i32).wrapping_add(ivar10);
                                if ivar6 < param_4 {
                                    ivar8 = ivar8.wrapping_add((b9 & 0x7f) as i32);
                                } else {
                                    bvar2 = 1;
                                    ivar6 = 0;
                                    ivar8 = ivar8.wrapping_add(param_4.wrapping_sub(ivar10));
                                    b9 =
                                        (u5 as u8).wrapping_sub(param_4.wrapping_sub(ivar10) as u8);
                                    u5 = b9 as u32;
                                    if b9 != 0 {
                                        u5 = (b9 | 0x80) as u32;
                                    }
                                }
                            }
                        }
                        ivar7 = ivar8;
                        ivar10 = ivar6;
                        if bvar2 == 0 {
                            continue 'main;
                        }
                        b9 = u5 as u8;
                        if b9 == 0 {
                            continue 'main;
                        }
                        if b9 == 0x80 {
                            break 'done;
                        }
                        if u5 & 0x80 == 0 {
                            if u5 & 0x40 == 0 {
                                let v = *p11.offset(ivar7 as isize);
                                ivar7 = ivar7.wrapping_add(1);
                                ivar10 = (ivar6 as u32).wrapping_add(u5) as i32;
                                let fill = (v as u32).wrapping_mul(0x01010101);
                                let dst = local_14.wrapping_add(ivar6 as u32) as usize as *mut u8;
                                if param_6 <= ivar10 {
                                    let b3 = (param_6 as u8).wrapping_sub(ivar6 as u8);
                                    fill_run(dst, fill, v, b3 as u32);
                                    break 'done;
                                }
                                fill_run(dst, fill, v, u5);
                                continue 'main;
                            }
                            ivar10 = ivar6.wrapping_add((u5 & 0x3f) as i32);
                            if param_6 <= ivar10 {
                                break 'done;
                            }
                            continue 'main;
                        }
                        let n = (b9 & 0x7f) as u32;
                        ivar10 = n.wrapping_add(ivar6 as u32) as i32;
                        if ivar10 < param_6 {
                            let src = p11.offset(ivar7 as isize);
                            ivar7 = ivar7.wrapping_add(n as i32);
                            let dst = local_14.wrapping_add(ivar6 as u32) as usize as *mut u8;
                            copy_run(dst, src, n);
                            continue 'main;
                        }
                        let b = (param_6 as u8).wrapping_sub(ivar6 as u8);
                        let src = p11.offset(ivar7 as isize);
                        let dst = local_14.wrapping_add(ivar6 as u32) as usize as *mut u8;
                        copy_run(dst, src, b as u32);
                        break 'done;
                    }
                }
                p11 = p11.offset(ivar1 as isize);
                let stride = (&raw mut DAT_00aa7b13).cast::<i32>().read_unaligned();
                local_14 = local_14.wrapping_add(stride as u32);
                local_c -= 1;
                if local_c == 0 {
                    return;
                }
            }
        }
    }
}
