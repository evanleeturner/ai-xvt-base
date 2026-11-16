// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758830: i32;
    static mut DAT_0075964b: [u8; 4];
}

unsafe fn fill(mut p: *mut u8, n: u32, fillw: u32, b: u8) {
    // SAFETY: the destination is the game's own surface memory, written
    // where the original writes.
    unsafe {
        let mut c = n >> 2;
        while c != 0 {
            p.cast::<u32>().write_unaligned(fillw);
            p = p.add(4);
            c -= 1;
        }
        let mut c = n & 3;
        while c != 0 {
            p.write(b);
            p = p.add(1);
            c -= 1;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d23e0(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
    param_8: u8,
) {
    // SAFETY: all pointers and globals are the game's own; the RLE data and
    // the surface are read and written exactly as the original does.
    unsafe {
        let b = &raw const DAT_0075964b as *const u8;
        let stride: u32 = (b.read() as u32)
            | ((b.add(1).read() as u32) << 8)
            | ((b.add(2).read() as u32) << 16)
            | ((b.add(3).read() as u32) << 24);
        let fillw: u32 = (param_8 as u32).wrapping_mul(0x01010101);
        let mut param_5 = param_5;

        if param_1.read() == param_6 {
            let mut pb10: *const u8 = param_1.add(4).read() as usize as *const u8;
            let mut ivar9: u32 = (param_2 as u32)
                .wrapping_add((param_3 as u32).wrapping_mul(stride))
                .wrapping_add(DAT_00758830 as u32);
            if 0 < param_5 {
                loop {
                    let off = pb10.cast::<i32>().read_unaligned();
                    pb10 = pb10.byte_offset(off as isize);
                    param_5 = param_5.wrapping_sub(1);
                    if param_5 == 0 {
                        break;
                    }
                }
            }
            if 0 < param_7 {
                let mut local_8 = param_7;
                loop {
                    let mut ivar7: u32 = 0;
                    let mut pb11 = pb10.add(4);
                    loop {
                        let b3 = pb11.read();
                        pb10 = pb11.add(1);
                        if b3 == 0x80 {
                            break;
                        }
                        if (b3 & 0x80) == 0 {
                            if (b3 & 0x40) == 0 {
                                pb11 = pb11.add(2);
                                let p = ivar7.wrapping_add(ivar9) as usize as *mut u8;
                                ivar7 = ivar7.wrapping_add(b3 as u32);
                                fill(p, b3 as u32, fillw, param_8);
                            } else {
                                ivar7 = ivar7.wrapping_add((b3 & 0x3f) as u32);
                                pb11 = pb10;
                            }
                        } else {
                            let u6 = (b3 & 0x7f) as u32;
                            let p = ivar7.wrapping_add(ivar9) as usize as *mut u8;
                            ivar7 = ivar7.wrapping_add(u6);
                            fill(p, u6, fillw, param_8);
                            pb11 = pb10.add(u6 as usize);
                        }
                    }
                    ivar9 = ivar9.wrapping_add(stride);
                    local_8 = local_8.wrapping_sub(1);
                    if local_8 == 0 {
                        break;
                    }
                }
                return;
            }
        } else {
            let mut local_c: *const u8 = param_1.add(4).read() as usize as *const u8;
            let mut ivar9: u32 = (param_2 as u32)
                .wrapping_add((param_3 as u32).wrapping_mul(stride))
                .wrapping_add(DAT_00758830 as u32);
            if 0 < param_5 {
                loop {
                    let off = local_c.cast::<i32>().read_unaligned();
                    local_c = local_c.byte_offset(off as isize);
                    param_5 = param_5.wrapping_sub(1);
                    if param_5 == 0 {
                        break;
                    }
                }
            }
            if 0 < param_7 {
                let mut local_4 = param_7;
                loop {
                    let mut ivar8: u32 = 0;
                    let mut ivar12: u32 = 4;
                    let mut bvar1: i32 = 0;
                    let ivar7: i32 = local_c.cast::<i32>().read_unaligned();
                    let mut b3: u8;
                    'row: {
                        'main: loop {
                            loop {
                                let mut ivar13 = ivar12.wrapping_add(1);
                                b3 = local_c.add(ivar12 as usize).read();
                                if bvar1 == 0 {
                                    if b3 == 0x80 {
                                        break 'row;
                                    }
                                    if (b3 & 0x80) == 0 {
                                        let c2 = ivar8 as i8;
                                        if (b3 & 0x40) == 0 {
                                            ivar8 = (b3 as u32).wrapping_add(ivar8);
                                            if (ivar8 as i32) < param_4 {
                                                ivar13 = ivar12.wrapping_add(2);
                                            } else {
                                                bvar1 = 1;
                                                ivar8 = 0;
                                                b3 = b3.wrapping_sub(
                                                    (param_4 as i8).wrapping_sub(c2) as u8,
                                                );
                                                if b3 == 0 {
                                                    ivar13 = ivar12.wrapping_add(2);
                                                }
                                            }
                                        } else {
                                            b3 &= 0x3f;
                                            ivar8 = (b3 as u32).wrapping_add(ivar8);
                                            if param_4 <= ivar8 as i32 {
                                                bvar1 = 1;
                                                ivar8 = 0;
                                                let b4 = b3.wrapping_sub(
                                                    (param_4 as i8).wrapping_sub(c2) as u8,
                                                );
                                                b3 = 0;
                                                if b4 != 0 {
                                                    b3 = b4 | 0x40;
                                                }
                                            }
                                        }
                                    } else {
                                        b3 &= 0x7f;
                                        let t = ivar8.wrapping_add(b3 as u32);
                                        if (t as i32) < param_4 {
                                            ivar8 = t;
                                            ivar13 = ivar13.wrapping_add(b3 as u32);
                                        } else {
                                            bvar1 = 1;
                                            let d = (param_4 as u32).wrapping_sub(ivar8);
                                            ivar13 = ivar13.wrapping_add(d);
                                            let b4 = b3.wrapping_sub(d as u8);
                                            b3 = 0;
                                            ivar8 = 0;
                                            if b4 != 0 {
                                                b3 = b4 | 0x80;
                                            }
                                        }
                                    }
                                }
                                ivar12 = ivar13;
                                if !(bvar1 == 0 || b3 == 0) {
                                    break;
                                }
                            }
                            if b3 == 0x80 {
                                break 'row;
                            }
                            if (b3 & 0x80) == 0 {
                                if (b3 & 0x40) == 0 {
                                    ivar12 = ivar12.wrapping_add(1);
                                    let u5 = b3 as u32;
                                    if param_6 <= ivar8.wrapping_add(u5) as i32 {
                                        break 'main;
                                    }
                                    let p = ivar8.wrapping_add(ivar9) as usize as *mut u8;
                                    ivar8 = ivar8.wrapping_add(u5);
                                    fill(p, u5, fillw, param_8);
                                    continue 'main;
                                }
                                ivar8 = ivar8.wrapping_add((b3 & 0x3f) as u32);
                                if param_6 <= ivar8 as i32 {
                                    break 'row;
                                }
                                continue 'main;
                            }
                            let u5 = (b3 & 0x7f) as u32;
                            if (ivar8.wrapping_add(u5) as i32) < param_6 {
                                let p = ivar8.wrapping_add(ivar9) as usize as *mut u8;
                                ivar8 = ivar8.wrapping_add(u5);
                                fill(p, u5, fillw, param_8);
                                ivar12 = ivar12.wrapping_add(u5);
                                continue 'main;
                            }
                            break 'main;
                        }
                        // LAB_004d2695
                        let bb = (param_6 as i8).wrapping_sub(ivar8 as i8) as u8;
                        let p = ivar8.wrapping_add(ivar9) as usize as *mut u8;
                        fill(p, bb as u32, fillw, param_8);
                    }
                    local_c = local_c.byte_offset(ivar7 as isize);
                    ivar9 = ivar9.wrapping_add(stride);
                    local_4 = local_4.wrapping_sub(1);
                    if local_4 == 0 {
                        return;
                    }
                }
            }
        }
    }
}
