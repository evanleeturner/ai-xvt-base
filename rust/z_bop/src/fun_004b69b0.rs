// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b13: u8;
    static mut DAT_00aa6cfc: i32;
}

unsafe fn fill(p: u32, n: u32, v: u8) {
    let mut q = p as *mut u8;
    let mut c = n >> 2;
    // SAFETY: the destination is the game's own pixel buffer, written as the original does.
    unsafe {
        while c != 0 {
            q.cast::<u32>()
                .write_unaligned((v as u32).wrapping_mul(0x01010101));
            q = q.byte_add(4);
            c -= 1;
        }
        let mut r = n & 3;
        while r != 0 {
            q.write(v);
            q = q.byte_add(1);
            r -= 1;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b69b0(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
    param_8: u8,
) {
    // SAFETY: all pointers and globals are the game's own memory; reads and
    // writes happen in the original's order.
    unsafe {
        let stride: u32 = (&raw const DAT_00aa7b13).cast::<u32>().read_unaligned();
        let mut param_5 = param_5;

        if param_1.read_unaligned() == param_6 {
            let mut pbv7: u32 = param_1.byte_add(0).add(4).read_unaligned() as u32;
            let mut iv10: u32 = (param_2 as u32)
                .wrapping_add(stride.wrapping_mul(param_3 as u32))
                .wrapping_add(DAT_00aa6cfc as u32);
            if 0 < param_5 {
                loop {
                    let d = (pbv7 as *const i32).read_unaligned();
                    pbv7 = pbv7.wrapping_add(d as u32);
                    param_5 -= 1;
                    if param_5 == 0 {
                        break;
                    }
                }
            }
            if 0 < param_7 {
                let mut local_8 = param_7;
                loop {
                    let mut iv11: u32 = 0;
                    let mut pbv8 = pbv7.wrapping_add(4);
                    loop {
                        let b3 = (pbv8 as *const u8).read();
                        pbv7 = pbv8.wrapping_add(1);
                        if b3 == 0x80 {
                            break;
                        }
                        if b3 & 0x80 == 0 {
                            if b3 & 0x40 == 0 {
                                pbv8 = pbv8.wrapping_add(2);
                                fill(iv11.wrapping_add(iv10), b3 as u32, param_8);
                                iv11 = iv11.wrapping_add(b3 as u32);
                            } else {
                                iv11 = iv11.wrapping_add((b3 & 0x3f) as u32);
                                pbv8 = pbv7;
                            }
                        } else {
                            let u6 = (b3 & 0x7f) as u32;
                            fill(iv11.wrapping_add(iv10), u6, param_8);
                            iv11 = iv11.wrapping_add(u6);
                            pbv8 = pbv7.wrapping_add(u6);
                        }
                    }
                    iv10 = iv10.wrapping_add(stride);
                    local_8 -= 1;
                    if local_8 == 0 {
                        return;
                    }
                }
            }
        } else {
            let mut local_c: u32 = param_1.add(4).read_unaligned() as u32;
            let mut iv10: u32 = (param_2 as u32)
                .wrapping_add(stride.wrapping_mul(param_3 as u32))
                .wrapping_add(DAT_00aa6cfc as u32);
            if 0 < param_5 {
                loop {
                    let d = (local_c as *const i32).read_unaligned();
                    local_c = local_c.wrapping_add(d as u32);
                    param_5 -= 1;
                    if param_5 == 0 {
                        break;
                    }
                }
            }
            if 0 < param_7 {
                let mut local_4 = param_7;
                loop {
                    let mut iv9: i32 = 0;
                    let mut iv12: i32 = 4;
                    let mut b1: bool = false;
                    let iv11: i32 = (local_c as *const i32).read_unaligned();
                    'row: {
                        'fill: {
                            loop {
                                let mut b3: u8;
                                loop {
                                    let mut iv13 = iv12.wrapping_add(1);
                                    b3 = (iv12.wrapping_add(local_c as i32) as u32 as *const u8)
                                        .read();
                                    if !b1 {
                                        if b3 == 0x80 {
                                            break 'row;
                                        }
                                        if b3 & 0x80 == 0 {
                                            let c2 = iv9 as u8;
                                            if b3 & 0x40 == 0 {
                                                iv9 = iv9.wrapping_add(b3 as i32);
                                                if iv9 < param_4 {
                                                    iv13 = iv12.wrapping_add(2);
                                                } else {
                                                    b1 = true;
                                                    iv9 = 0;
                                                    b3 = b3.wrapping_sub(
                                                        (param_4 as u8).wrapping_sub(c2),
                                                    );
                                                    if b3 == 0 {
                                                        iv13 = iv12.wrapping_add(2);
                                                    }
                                                }
                                            } else {
                                                b3 &= 0x3f;
                                                iv9 = iv9.wrapping_add(b3 as i32);
                                                if param_4 <= iv9 {
                                                    b1 = true;
                                                    iv9 = 0;
                                                    let b4 = b3.wrapping_sub(
                                                        (param_4 as u8).wrapping_sub(c2),
                                                    );
                                                    b3 = 0;
                                                    if b4 != 0 {
                                                        b3 = b4 | 0x40;
                                                    }
                                                }
                                            }
                                        } else {
                                            b3 &= 0x7f;
                                            let t = iv9.wrapping_add(b3 as i32);
                                            if t < param_4 {
                                                iv9 = t;
                                                iv13 = iv13.wrapping_add(b3 as i32);
                                            } else {
                                                b1 = true;
                                                iv13 = iv13.wrapping_add(param_4.wrapping_sub(iv9));
                                                let b4 = b3
                                                    .wrapping_sub(param_4.wrapping_sub(iv9) as u8);
                                                b3 = 0;
                                                iv9 = 0;
                                                if b4 != 0 {
                                                    b3 = b4 | 0x80;
                                                }
                                            }
                                        }
                                    }
                                    iv12 = iv13;
                                    if b1 && b3 != 0 {
                                        break;
                                    }
                                }

                                if b3 == 0x80 {
                                    break 'row;
                                }
                                if b3 & 0x80 == 0 {
                                    if b3 & 0x40 == 0 {
                                        iv12 = iv12.wrapping_add(1);
                                        let u5 = b3 as u32;
                                        if param_6 <= (u5.wrapping_add(iv9 as u32)) as i32 {
                                            break 'fill;
                                        }
                                        fill((iv9 as u32).wrapping_add(iv10), u5, param_8);
                                        iv9 = iv9.wrapping_add(u5 as i32);
                                        continue;
                                    }
                                    iv9 = iv9.wrapping_add((b3 & 0x3f) as i32);
                                    if param_6 <= iv9 {
                                        break 'row;
                                    }
                                    continue;
                                }
                                let u5 = (b3 & 0x7f) as u32;
                                if ((u5.wrapping_add(iv9 as u32)) as i32) < param_6 {
                                    fill((iv9 as u32).wrapping_add(iv10), u5, param_8);
                                    iv9 = iv9.wrapping_add(u5 as i32);
                                    iv12 = iv12.wrapping_add(u5 as i32);
                                    continue;
                                }
                                break 'fill;
                            }
                        }
                        let b3 = (param_6 as u8).wrapping_sub(iv9 as u8);
                        fill((iv9 as u32).wrapping_add(iv10), b3 as u32, param_8);
                    }
                    iv10 = iv10.wrapping_add(stride);
                    local_c = local_c.wrapping_add(iv11 as u32);
                    local_4 -= 1;
                    if local_4 == 0 {
                        return;
                    }
                }
            }
        }
    }
}
