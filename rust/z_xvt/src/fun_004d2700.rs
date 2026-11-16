// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0075964b: [u8; 4];
    static mut DAT_00758830: i32;
}

#[inline(always)]
unsafe fn fill(addr: u32, n: u32, v: u16) {
    let mut p = addr as *mut u16;
    let mut c = n;
    while c != 0 {
        // SAFETY: destination is the game's own surface memory, written as the original does.
        unsafe {
            p.write_unaligned(v);
        }
        p = p.wrapping_add(1);
        c = c.wrapping_sub(1);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d2700(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
    param_8: u16,
) {
    // SAFETY: all pointers and globals belong to the game; reads and writes
    // follow the original's sequence exactly.
    unsafe {
        let u_var8: u32 = (&raw const DAT_0075964b).cast::<u32>().read_unaligned();
        let mut param_5 = param_5;

        if param_1.read_unaligned() == param_6 {
            let mut pbvar8: u32 = param_1.add(4).read_unaligned() as u32;
            let mut local_c: u32 = (param_3 as u32)
                .wrapping_mul(u_var8)
                .wrapping_add((param_2 as u32).wrapping_mul(2))
                .wrapping_add(DAT_00758830 as u32);
            if 0 < param_5 {
                loop {
                    let d = (pbvar8 as *const i32).read_unaligned() as u32;
                    pbvar8 = pbvar8.wrapping_add(d);
                    param_5 = param_5.wrapping_sub(1);
                    if param_5 == 0 {
                        break;
                    }
                }
            }
            if 0 < param_7 {
                let mut local_20 = param_7;
                let u_var5 = u_var8 & 0xffff_fffe;
                loop {
                    let mut i_var11: i32 = 0;
                    let mut pbvar9: u32 = pbvar8.wrapping_add(4);
                    loop {
                        let b3 = (pbvar9 as *const u8).read();
                        pbvar8 = pbvar9.wrapping_add(1);
                        if b3 == 0x80 {
                            break;
                        }
                        if b3 & 0x80 == 0 {
                            if b3 & 0x40 == 0 {
                                pbvar9 = pbvar9.wrapping_add(2);
                                let u6 = b3 as u32;
                                let addr = local_c.wrapping_add((i_var11 as u32).wrapping_mul(2));
                                fill(addr, u6, param_8);
                                i_var11 = i_var11.wrapping_add(u6 as i32);
                            } else {
                                i_var11 = i_var11.wrapping_add((b3 & 0x3f) as i32);
                                pbvar9 = pbvar8;
                            }
                        } else {
                            let u6 = (b3 & 0x7f) as u32;
                            let addr = local_c.wrapping_add((i_var11 as u32).wrapping_mul(2));
                            fill(addr, u6, param_8);
                            i_var11 = i_var11.wrapping_add(u6 as i32);
                            pbvar9 = pbvar8.wrapping_add(u6);
                        }
                    }
                    local_c = local_c.wrapping_add(u_var5);
                    local_20 = local_20.wrapping_sub(1);
                    if local_20 == 0 {
                        break;
                    }
                }
                return;
            }
        } else {
            let mut local_10: u32 = param_1.add(4).read_unaligned() as u32;
            let mut local_c: u32 = (param_3 as u32)
                .wrapping_mul(u_var8)
                .wrapping_add((param_2 as u32).wrapping_mul(2))
                .wrapping_add(DAT_00758830 as u32);
            if 0 < param_5 {
                loop {
                    let d = (local_10 as *const i32).read_unaligned() as u32;
                    local_10 = local_10.wrapping_add(d);
                    param_5 = param_5.wrapping_sub(1);
                    if param_5 == 0 {
                        break;
                    }
                }
            }
            if 0 < param_7 {
                let mut local_14 = param_7;
                let u_var5 = u_var8 & 0xffff_fffe;
                loop {
                    let mut i_var10: i32 = 0;
                    let mut i_var12: i32 = 4;
                    let mut b_var2: u8 = 0;
                    let i_var11: i32 = (local_10 as *const i32).read_unaligned();
                    let mut b3: u8;
                    'outer: loop {
                        'l1: loop {
                            'l2: loop {
                                loop {
                                    let mut i_var13 = i_var12.wrapping_add(1);
                                    b3 =
                                        (local_10.wrapping_add(i_var12 as u32) as *const u8).read();
                                    if b_var2 == 0 {
                                        if b3 == 0x80 {
                                            break 'outer;
                                        }
                                        if b3 & 0x80 == 0 {
                                            let c_var1 = i_var10 as i8;
                                            if b3 & 0x40 == 0 {
                                                i_var10 = (b3 as i32).wrapping_add(i_var10);
                                                if i_var10 < param_4 {
                                                    i_var13 = i_var12.wrapping_add(2);
                                                } else {
                                                    b_var2 = 1;
                                                    i_var10 = 0;
                                                    b3 = b3.wrapping_sub(
                                                        (param_4 as i8).wrapping_sub(c_var1) as u8,
                                                    );
                                                    if b3 == 0 {
                                                        i_var13 = i_var12.wrapping_add(2);
                                                    }
                                                }
                                            } else {
                                                b3 &= 0x3f;
                                                i_var10 = (b3 as i32).wrapping_add(i_var10);
                                                if param_4 <= i_var10 {
                                                    b_var2 = 1;
                                                    i_var10 = 0;
                                                    let b4 = b3.wrapping_sub(
                                                        (param_4 as i8).wrapping_sub(c_var1) as u8,
                                                    );
                                                    b3 = 0;
                                                    if b4 != 0 {
                                                        b3 = b4 | 0x40;
                                                    }
                                                }
                                            }
                                        } else {
                                            b3 &= 0x7f;
                                            i_var12 = i_var10.wrapping_add(b3 as i32);
                                            if i_var12 < param_4 {
                                                i_var10 = i_var12;
                                                i_var13 = i_var13.wrapping_add(b3 as i32);
                                            } else {
                                                b_var2 = 1;
                                                i_var13 = i_var13
                                                    .wrapping_add(param_4.wrapping_sub(i_var10));
                                                let b4 = b3.wrapping_sub(
                                                    param_4.wrapping_sub(i_var10) as u8,
                                                );
                                                b3 = 0;
                                                i_var10 = 0;
                                                if b4 != 0 {
                                                    b3 = b4 | 0x80;
                                                }
                                            }
                                        }
                                    }
                                    i_var12 = i_var13;
                                    if !(b_var2 == 0 || b3 == 0) {
                                        break;
                                    }
                                }
                                if b3 == 0x80 {
                                    break 'outer;
                                }
                                if b3 & 0x80 == 0 {
                                    break 'l2;
                                }
                                let mut u7 = (b3 & 0x7f) as u32;
                                if param_6 <= i_var10.wrapping_add(u7 as i32) {
                                    u7 = (param_6 as i8).wrapping_sub(i_var10 as i8) as u8 as u32;
                                    let addr =
                                        local_c.wrapping_add((i_var10 as u32).wrapping_mul(2));
                                    fill(addr, u7, param_8);
                                    break 'outer;
                                }
                                let addr = local_c.wrapping_add((i_var10 as u32).wrapping_mul(2));
                                fill(addr, u7, param_8);
                                i_var10 = i_var10.wrapping_add(u7 as i32);
                                i_var12 = i_var12.wrapping_add(u7 as i32);
                            }
                            if b3 & 0x40 != 0 {
                                break 'l1;
                            }
                            i_var12 = i_var12.wrapping_add(1);
                            let mut u7 = b3 as u32;
                            if param_6 <= i_var10.wrapping_add(u7 as i32) {
                                u7 = (param_6 as i8).wrapping_sub(i_var10 as i8) as u8 as u32;
                                let addr = local_c.wrapping_add((i_var10 as u32).wrapping_mul(2));
                                fill(addr, u7, param_8);
                                break 'outer;
                            }
                            let addr = local_c.wrapping_add((i_var10 as u32).wrapping_mul(2));
                            fill(addr, u7, param_8);
                            i_var10 = i_var10.wrapping_add(u7 as i32);
                        }
                        i_var10 = i_var10.wrapping_add((b3 & 0x3f) as i32);
                        if !(i_var10 < param_6) {
                            break 'outer;
                        }
                    }
                    local_10 = local_10.wrapping_add(i_var11 as u32);
                    local_c = local_c.wrapping_add(u_var5);
                    local_14 = local_14.wrapping_sub(1);
                    if local_14 == 0 {
                        break;
                    }
                }
            }
        }
    }
}
