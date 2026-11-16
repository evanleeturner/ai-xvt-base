// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b13: u8;
    static mut DAT_00aa6cfc: i32;
}

unsafe fn fill(addr: i32, n: u32, v: u16) {
    let mut p = addr as u32 as usize as *mut u16;
    let mut c = n;
    while c != 0 {
        // SAFETY: the destination is the game's own surface memory.
        unsafe { p.write_unaligned(v) };
        p = p.wrapping_add(1);
        c = c.wrapping_sub(1);
    }
}

unsafe fn rd8(addr: i32) -> u8 {
    // SAFETY: addr points into the game's sprite data.
    unsafe { (addr as u32 as usize as *const u8).read() }
}

unsafe fn rd32(addr: i32) -> i32 {
    // SAFETY: addr points into the game's sprite data.
    unsafe { (addr as u32 as usize as *const i32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b6ca0(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
    param_8: u16,
) {
    let mut param_5 = param_5;
    // SAFETY: globals and pointers belong to the game; reads and writes
    // happen where the original does them.
    unsafe {
        let g_7b13: u32 = (&raw mut DAT_00aa7b13).cast::<u32>().read_unaligned();
        let start: i32 = param_1.add(4).read_unaligned();
        let mut local_c: i32 = g_7b13
            .wrapping_mul(param_3 as u32)
            .wrapping_add((param_2 as u32).wrapping_mul(2))
            .wrapping_add(DAT_00aa6cfc as u32) as i32;

        if *param_1 == param_6 {
            let mut pbvar8: i32 = start;
            if 0 < param_5 {
                loop {
                    pbvar8 = pbvar8.wrapping_add(rd32(pbvar8));
                    param_5 = param_5.wrapping_sub(1);
                    if param_5 == 0 {
                        break;
                    }
                }
            }
            if 0 < param_7 {
                let mut local_50 = param_7;
                let u_var5 = g_7b13 & 0xffff_fffe;
                loop {
                    let mut i_var11: i32 = 0;
                    let mut pbvar9: i32 = pbvar8.wrapping_add(4);
                    loop {
                        let b3 = rd8(pbvar9);
                        pbvar8 = pbvar9.wrapping_add(1);
                        if b3 == 0x80 {
                            break;
                        }
                        if b3 & 0x80 == 0 {
                            if b3 & 0x40 == 0 {
                                pbvar9 = pbvar9.wrapping_add(2);
                                let u6 = b3 as u32;
                                fill(local_c.wrapping_add(i_var11.wrapping_mul(2)), u6, param_8);
                                i_var11 = (i_var11 as u32).wrapping_add(u6) as i32;
                            } else {
                                i_var11 = (i_var11 as u32).wrapping_add((b3 & 0x3f) as u32) as i32;
                                pbvar9 = pbvar8;
                            }
                        } else {
                            let u6 = (b3 & 0x7f) as u32;
                            fill(local_c.wrapping_add(i_var11.wrapping_mul(2)), u6, param_8);
                            i_var11 = (i_var11 as u32).wrapping_add(u6) as i32;
                            pbvar9 = pbvar8.wrapping_add(u6 as i32);
                        }
                    }
                    local_c = (local_c as u32).wrapping_add(u_var5) as i32;
                    local_50 = local_50.wrapping_sub(1);
                    if local_50 == 0 {
                        break;
                    }
                }
                return;
            }
        } else {
            let mut local_10: i32 = start;
            if 0 < param_5 {
                loop {
                    local_10 = local_10.wrapping_add(rd32(local_10));
                    param_5 = param_5.wrapping_sub(1);
                    if param_5 == 0 {
                        break;
                    }
                }
            }
            if 0 < param_7 {
                let mut local_1c = param_7;
                let u_var5 = g_7b13 & 0xffff_fffe;
                loop {
                    let mut i_var10: i32 = 0;
                    let mut i_var12: i32 = 4;
                    let mut b2: i32 = 0;
                    let i_var11: i32 = rd32(local_10);
                    let mut b3: u8;
                    'done: {
                        loop {
                            loop {
                                loop {
                                    loop {
                                        let mut i_var13 = i_var12.wrapping_add(1);
                                        b3 = rd8(i_var12.wrapping_add(local_10));
                                        if b2 == 0 {
                                            if b3 == 0x80 {
                                                break 'done;
                                            }
                                            if b3 & 0x80 == 0 {
                                                let c_var1 = i_var10 as i8;
                                                if b3 & 0x40 == 0 {
                                                    i_var10 = (b3 as u32)
                                                        .wrapping_add(i_var10 as u32)
                                                        as i32;
                                                    if i_var10 < param_4 {
                                                        i_var13 = i_var12.wrapping_add(2);
                                                    } else {
                                                        b2 = 1;
                                                        i_var10 = 0;
                                                        b3 = b3.wrapping_sub(
                                                            (param_4 as i8).wrapping_sub(c_var1)
                                                                as u8,
                                                        );
                                                        if b3 == 0 {
                                                            i_var13 = i_var12.wrapping_add(2);
                                                        }
                                                    }
                                                } else {
                                                    b3 &= 0x3f;
                                                    i_var10 = (b3 as u32)
                                                        .wrapping_add(i_var10 as u32)
                                                        as i32;
                                                    if param_4 <= i_var10 {
                                                        b2 = 1;
                                                        i_var10 = 0;
                                                        let b4 = b3.wrapping_sub(
                                                            (param_4 as i8).wrapping_sub(c_var1)
                                                                as u8,
                                                        );
                                                        b3 = 0;
                                                        if b4 != 0 {
                                                            b3 = b4 | 0x40;
                                                        }
                                                    }
                                                }
                                            } else {
                                                b3 &= 0x7f;
                                                i_var12 =
                                                    (b3 as u32).wrapping_add(i_var10 as u32) as i32;
                                                if i_var12 < param_4 {
                                                    i_var10 = i_var12;
                                                    i_var13 = (i_var13 as u32)
                                                        .wrapping_add(b3 as u32)
                                                        as i32;
                                                } else {
                                                    b2 = 1;
                                                    i_var13 = i_var13.wrapping_add(
                                                        param_4.wrapping_sub(i_var10),
                                                    );
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
                                        if b2 != 0 && b3 != 0 {
                                            break;
                                        }
                                    }
                                    if b3 == 0x80 {
                                        break 'done;
                                    }
                                    if b3 & 0x80 == 0 {
                                        break;
                                    }
                                    let mut u7 = (b3 & 0x7f) as u32;
                                    if param_6 <= u7.wrapping_add(i_var10 as u32) as i32 {
                                        u7 = (param_6 as u8).wrapping_sub(i_var10 as u8) as u32;
                                        fill(
                                            local_c.wrapping_add(i_var10.wrapping_mul(2)),
                                            u7,
                                            param_8,
                                        );
                                        break 'done;
                                    }
                                    fill(
                                        local_c.wrapping_add(i_var10.wrapping_mul(2)),
                                        u7,
                                        param_8,
                                    );
                                    i_var10 = (i_var10 as u32).wrapping_add(u7) as i32;
                                    i_var12 = (i_var12 as u32).wrapping_add(u7) as i32;
                                }
                                if b3 & 0x40 != 0 {
                                    break;
                                }
                                i_var12 = i_var12.wrapping_add(1);
                                let mut u7 = b3 as u32;
                                if param_6 <= u7.wrapping_add(i_var10 as u32) as i32 {
                                    u7 = (param_6 as u8).wrapping_sub(i_var10 as u8) as u32;
                                    fill(
                                        local_c.wrapping_add(i_var10.wrapping_mul(2)),
                                        u7,
                                        param_8,
                                    );
                                    break 'done;
                                }
                                fill(local_c.wrapping_add(i_var10.wrapping_mul(2)), u7, param_8);
                                i_var10 = (i_var10 as u32).wrapping_add(u7) as i32;
                            }
                            i_var10 = (i_var10 as u32).wrapping_add((b3 & 0x3f) as u32) as i32;
                            if !(i_var10 < param_6) {
                                break;
                            }
                        }
                    }
                    local_10 = local_10.wrapping_add(i_var11);
                    local_c = (local_c as u32).wrapping_add(u_var5) as i32;
                    local_1c = local_1c.wrapping_sub(1);
                    if local_1c == 0 {
                        break;
                    }
                }
            }
        }
    }
}
