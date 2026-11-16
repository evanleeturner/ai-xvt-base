// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0075964b: [u8; 4];
    static mut DAT_00758830: i32;
}

unsafe fn fill(p: u32, n: u32, v: u16) {
    let mut ptr = p as usize as *mut u16;
    let mut n = n;
    // SAFETY: the caller's destination surface, written where the original does.
    unsafe {
        while n != 0 {
            ptr.write_unaligned(v);
            ptr = ptr.wrapping_add(1);
            n -= 1;
        }
    }
}

unsafe fn pal(p: *mut i32, i: u8) -> i32 {
    // SAFETY: param_1 points at the caller's sprite header and palette.
    unsafe { p.wrapping_add(i as usize + 5).read_unaligned() }
}

unsafe fn rd(p: *const u8, o: i32) -> u8 {
    // SAFETY: p points at the caller's RLE sprite data.
    unsafe { p.wrapping_offset(o as isize).read() }
}

unsafe fn stride() -> u32 {
    // SAFETY: the game's own global bytes.
    unsafe { u32::from_le_bytes(DAT_0075964b) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d1cf0(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
) {
    // SAFETY: all pointers and globals belong to the game; memory is read and
    // written exactly where the original does.
    unsafe {
        let mut param_5 = param_5;
        if param_1.read_unaligned() == param_6 {
            let mut local_c = (param_1.add(4).read_unaligned() as u32) as usize as *const u8;
            let mut local_14: u32 = stride()
                .wrapping_mul(param_3 as u32)
                .wrapping_add((param_2 as u32).wrapping_mul(2))
                .wrapping_add(DAT_00758830 as u32);
            if 0 < param_5 {
                loop {
                    let o = (local_c as *const i32).read_unaligned();
                    local_c = local_c.wrapping_offset(o as isize);
                    param_5 = param_5.wrapping_sub(1);
                    if param_5 == 0 {
                        break;
                    }
                }
            }
            if 0 < param_7 {
                let mut local_24 = param_7;
                loop {
                    let mut i_var9: i32 = 0;
                    local_c = local_c.wrapping_add(4);
                    let mut pb7;
                    loop {
                        pb7 = local_c.wrapping_add(1);
                        let b5 = local_c.read();
                        if b5 == 0x80 {
                            break;
                        }
                        if b5 & 0x80 == 0 {
                            if b5 & 0x40 == 0 {
                                local_c = local_c.wrapping_add(2);
                                let u10 = b5 as u32;
                                let v = pal(param_1, pb7.read());
                                fill(
                                    local_14.wrapping_add((i_var9 as u32).wrapping_mul(2)),
                                    u10,
                                    v as u16,
                                );
                                i_var9 = (i_var9 as u32).wrapping_add(u10) as i32;
                            } else {
                                let u10 = (b5 & 0x3f) as u32;
                                let v = param_1.add(5).read_unaligned();
                                fill(
                                    local_14.wrapping_add((i_var9 as u32).wrapping_mul(2)),
                                    u10,
                                    v as u16,
                                );
                                i_var9 = (i_var9 as u32).wrapping_add(u10) as i32;
                                local_c = pb7;
                            }
                        } else {
                            let b4 = b5 & 0x7f;
                            let mut i_var13: i32 = 0;
                            if b4 != 0 {
                                loop {
                                    let b = pb7.wrapping_offset(i_var13 as isize).read();
                                    i_var13 = i_var13.wrapping_add(1);
                                    let addr = local_14.wrapping_add(
                                        ((i_var9.wrapping_sub(1).wrapping_add(i_var13)) as u32)
                                            .wrapping_mul(2),
                                    );
                                    (addr as usize as *mut u16)
                                        .write_unaligned(pal(param_1, b) as u16);
                                    if !(i_var13 < b4 as i32) {
                                        break;
                                    }
                                }
                            }
                            i_var9 = (i_var9 as u32).wrapping_add(b4 as u32) as i32;
                            local_c = pb7.wrapping_add(b4 as usize);
                        }
                    }
                    local_14 = local_14.wrapping_add(stride() & 0xffff_fffe);
                    local_24 = local_24.wrapping_sub(1);
                    local_c = pb7;
                    if local_24 == 0 {
                        break;
                    }
                }
                return;
            }
        } else {
            let mut local_c = (param_1.add(4).read_unaligned() as u32) as usize as *const u8;
            let mut local_14: u32 = stride()
                .wrapping_mul(param_3 as u32)
                .wrapping_add((param_2 as u32).wrapping_mul(2))
                .wrapping_add(DAT_00758830 as u32);
            if 0 < param_5 {
                loop {
                    let o = (local_c as *const i32).read_unaligned();
                    local_c = local_c.wrapping_offset(o as isize);
                    param_5 = param_5.wrapping_sub(1);
                    if param_5 == 0 {
                        break;
                    }
                }
            }
            if 0 < param_7 {
                let mut local_18 = param_7;
                loop {
                    let mut local_10: i32 = 4;
                    let i_var13: i32 = (local_c as *const i32).read_unaligned();
                    let mut b_var3 = false;
                    let mut i_var9: i32 = 0;
                    'done: loop {
                        let mut b5: u8;
                        let mut u8v: u32;
                        let mut i_var12: i32;
                        let mut i_var14: i32;
                        loop {
                            i_var14 = local_10.wrapping_add(1);
                            b5 = rd(local_c, local_10);
                            u8v = b5 as u32;
                            i_var12 = i_var9;
                            if !b_var3 {
                                if b5 == 0x80 {
                                    break 'done;
                                }
                                if b5 & 0x80 == 0 {
                                    if b5 & 0x40 == 0 {
                                        i_var12 = (b5 as i32).wrapping_add(i_var9);
                                        if i_var12 < param_4 {
                                            i_var14 = local_10.wrapping_add(2);
                                        } else {
                                            b_var3 = true;
                                            i_var12 = 0;
                                            b5 = b5.wrapping_sub(
                                                (param_4 as u8).wrapping_sub(i_var9 as u8),
                                            );
                                            u8v = b5 as u32;
                                            if b5 == 0 {
                                                i_var14 = local_10.wrapping_add(2);
                                            }
                                        }
                                    } else {
                                        u8v &= 0xffff_ff3f;
                                        i_var12 = ((b5 & 0x3f) as i32).wrapping_add(i_var9);
                                        if param_4 <= i_var12 {
                                            b_var3 = true;
                                            i_var12 = 0;
                                            b5 = (u8v as u8).wrapping_sub(
                                                (param_4 as u8).wrapping_sub(i_var9 as u8),
                                            );
                                            u8v = b5 as u32;
                                            if b5 != 0 {
                                                u8v = (b5 | 0x40) as u32;
                                            }
                                        }
                                    }
                                } else {
                                    u8v &= 0xffff_ff7f;
                                    i_var12 = ((b5 & 0x7f) as i32).wrapping_add(i_var9);
                                    if i_var12 < param_4 {
                                        i_var14 = i_var14.wrapping_add((b5 & 0x7f) as i32);
                                    } else {
                                        b_var3 = true;
                                        i_var12 = 0;
                                        b5 = (u8v as u8)
                                            .wrapping_sub(param_4.wrapping_sub(i_var9) as u8);
                                        u8v = b5 as u32;
                                        i_var14 =
                                            param_4.wrapping_sub(i_var9).wrapping_add(i_var14);
                                        if b5 != 0 {
                                            u8v = (b5 | 0x80) as u32;
                                        }
                                    }
                                }
                            }
                            local_10 = i_var14;
                            i_var9 = i_var12;
                            if b_var3 {
                                b5 = u8v as u8;
                                if b5 != 0 {
                                    break;
                                }
                            }
                        }
                        if b5 == 0x80 {
                            break 'done;
                        }
                        let c11 = i_var12 as i8;
                        let c6 = param_6 as i8;
                        let n_rem = c6.wrapping_sub(c11) as u8;
                        if u8v & 0x80 != 0 {
                            let u10 = (b5 & 0x7f) as u32;
                            if param_6 <= (u10.wrapping_add(i_var12 as u32)) as i32 {
                                i_var9 = 0;
                                if n_rem != 0 {
                                    loop {
                                        let i14 = i_var9.wrapping_add(i_var12);
                                        let i2 = i_var9.wrapping_add(local_10);
                                        i_var9 = i_var9.wrapping_add(1);
                                        let addr =
                                            local_14.wrapping_add((i14 as u32).wrapping_mul(2));
                                        (addr as usize as *mut u16).write_unaligned(pal(
                                            param_1,
                                            rd(local_c, i2),
                                        )
                                            as u16);
                                        if !(i_var9 < n_rem as i32) {
                                            break;
                                        }
                                    }
                                }
                                break 'done;
                            }
                            i_var9 = 0;
                            if u8v & 0x7f != 0 {
                                loop {
                                    let i14 = i_var9.wrapping_add(1);
                                    let addr = local_14.wrapping_add(
                                        (i_var9.wrapping_add(i_var12) as u32).wrapping_mul(2),
                                    );
                                    let src =
                                        rd(local_c, i14.wrapping_add(local_10).wrapping_sub(1));
                                    (addr as usize as *mut u16)
                                        .write_unaligned(pal(param_1, src) as u16);
                                    i_var9 = i14;
                                    if !(i14 < u10 as i32) {
                                        break;
                                    }
                                }
                            }
                            i_var9 = (i_var12 as u32).wrapping_add(u10) as i32;
                            local_10 = (local_10 as u32).wrapping_add(u10) as i32;
                            continue;
                        }
                        if u8v & 0x40 == 0 {
                            let i14 = local_10.wrapping_add(1);
                            i_var9 = u8v.wrapping_add(i_var12 as u32) as i32;
                            let v = pal(param_1, rd(local_c, local_10));
                            let dst = local_14.wrapping_add((i_var12 as u32).wrapping_mul(2));
                            if param_6 <= i_var9 {
                                fill(dst, n_rem as u32, v as u16);
                                break 'done;
                            }
                            local_10 = i14;
                            fill(dst, u8v, v as u16);
                            continue;
                        }
                        let u8c = (b5 & 0x3f) as u32;
                        i_var9 = u8c.wrapping_add(i_var12 as u32) as i32;
                        let v = param_1.add(5).read_unaligned();
                        let dst = local_14.wrapping_add((i_var12 as u32).wrapping_mul(2));
                        if param_6 <= i_var9 {
                            fill(dst, n_rem as u32, v as u16);
                            break 'done;
                        }
                        fill(dst, u8c, v as u16);
                    }
                    local_c = local_c.wrapping_offset(i_var13 as isize);
                    local_14 = local_14.wrapping_add(stride() & 0xffff_fffe);
                    local_18 = local_18.wrapping_sub(1);
                    if local_18 == 0 {
                        break;
                    }
                }
            }
        }
    }
}
