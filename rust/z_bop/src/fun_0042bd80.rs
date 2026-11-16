// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a8078: u16;
    static mut DAT_009d694a: u16;
    static mut DAT_009d694e: u16;
    static mut DAT_009d8b74: [u8; 2];
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_00a08104: i32;
    static mut DAT_00a00870: [u32; 0];
    static mut DAT_00a00874: [u32; 0];
    static mut DAT_00a00860: u16;
    static mut DAT_009ecc34: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004adc40(param_1: u32) -> i32;
    fn FUN_004ae2a0(param_1: u32, param_2: u32) -> i32;
    fn FUN_00438a30(param_1: i32);
}

unsafe fn wb(addr: i32, v: u8) {
    // SAFETY: the address is inside the game's own object, written where the original writes.
    unsafe {
        (addr as usize as *mut u8).write(v);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042bd80(param_1: u16) {
    // SAFETY: globals, callees and the pointed-to memory are the game's own.
    unsafe {
        DAT_009a8078 = 1;
        DAT_009d694a = 0;
        DAT_009d694e = 0xc;
        let b_var8 = param_1 as u8;
        if param_1 < 9 {
            DAT_009d8b74[0] = ((10u32.wrapping_sub(param_1 as u32) as i32) / 2) as u8;
            DAT_009d8b74[1] = (b_var8 & 1).wrapping_mul(0x1e);
        } else {
            DAT_009d8b74[0] = 0;
            DAT_009d8b74[1] = 0x14u8.wrapping_sub(b_var8).wrapping_mul(5);
        }
        let mut local_6: u16 = 0;
        if 0 < DAT_009a1fe0 {
            loop {
                let i_var2 = (local_6 as u32).wrapping_mul(0x23) as i32;
                let i_var4 = DAT_009a1fe8.wrapping_add(i_var2);
                if ((DAT_009a1fe8.wrapping_add(3).wrapping_add(i_var2) as usize as *const u8)
                    .read()
                    != 0)
                    && ((i_var4.wrapping_add(2) as usize as *const u8).read() == 0x0e)
                {
                    let i_tmp = (i_var4.wrapping_add(0x1f) as usize as *const i32).read_unaligned();
                    DAT_00a08104 =
                        (i_tmp.wrapping_add(0xa9) as usize as *const i32).read_unaligned();
                    let b_var1 = (DAT_009a1fe8.wrapping_add(3).wrapping_add(i_var2) as usize
                        as *const u8)
                        .read();
                    let i_count: i32 = if b_var1 < 0x49 {
                        (&raw mut DAT_00a00870)
                            .cast::<u32>()
                            .add((b_var1 as usize) * 0x65)
                            .read() as i32
                    } else {
                        FUN_004adc40(b_var1 as u32)
                    };
                    let mut u_var9: u32 = 0;
                    let mut i_var3: i32 = 0;
                    if 0 < i_count {
                        loop {
                            let b_var1 =
                                (DAT_009a1fe8.wrapping_add(3).wrapping_add(i_var2) as usize
                                    as *const u8)
                                    .read();
                            let u_var4 = b_var1 as u32;
                            let i_var5: i32;
                            if b_var1 < 0x49 {
                                let mut u_var7 = u_var9;
                                let cnt = (&raw mut DAT_00a00870)
                                    .cast::<u32>()
                                    .add((u_var4 as usize) * 0x65)
                                    .read();
                                if (cnt as i32) <= (u_var9 as i32) {
                                    u_var7 = cnt.wrapping_sub(1);
                                }
                                i_var5 = (&raw mut DAT_00a00874)
                                    .cast::<u32>()
                                    .add((u_var4 as usize) * 0x65 + u_var7 as usize)
                                    .read() as i32;
                            } else {
                                i_var5 = FUN_004ae2a0(u_var4, u_var9);
                            }
                            if local_6 == 1 {
                                if i_var5 == 1 {
                                    wb(i_var3.wrapping_add(0x33c).wrapping_add(DAT_00a08104), 0xff);
                                } else {
                                    wb(i_var3.wrapping_add(0x33c).wrapping_add(DAT_00a08104), 0);
                                }
                                wb(i_var3.wrapping_add(0x2d8).wrapping_add(DAT_00a08104), 2);
                            } else {
                                let mut common = false;
                                let mut c6: u8 = 0;
                                match i_var5 {
                                    1 => {
                                        wb(
                                            i_var3.wrapping_add(0x2d8).wrapping_add(DAT_00a08104),
                                            2,
                                        );
                                    }
                                    5 => {
                                        wb(
                                            i_var3.wrapping_add(0x33c).wrapping_add(DAT_00a08104),
                                            b_var8.wrapping_mul(2),
                                        );
                                        wb(
                                            i_var3.wrapping_add(0x2d8).wrapping_add(DAT_00a08104),
                                            0,
                                        );
                                    }
                                    0x11 => {
                                        if param_1 < 2 {
                                            wb(
                                                i_var3
                                                    .wrapping_add(0x2d8)
                                                    .wrapping_add(DAT_00a08104),
                                                2,
                                            );
                                            wb(
                                                i_var3
                                                    .wrapping_add(0x33c)
                                                    .wrapping_add(DAT_00a08104),
                                                0,
                                            );
                                        } else {
                                            c6 = 3;
                                            common = true;
                                        }
                                    }
                                    0x12 => {
                                        if param_1 < 5 {
                                            wb(
                                                i_var3
                                                    .wrapping_add(0x2d8)
                                                    .wrapping_add(DAT_00a08104),
                                                2,
                                            );
                                            wb(
                                                i_var3
                                                    .wrapping_add(0x33c)
                                                    .wrapping_add(DAT_00a08104),
                                                0,
                                            );
                                        } else {
                                            wb(
                                                i_var3
                                                    .wrapping_add(0x2d8)
                                                    .wrapping_add(DAT_00a08104),
                                                0,
                                            );
                                            wb(
                                                i_var3
                                                    .wrapping_add(0x30a)
                                                    .wrapping_add(DAT_00a08104),
                                                0,
                                            );
                                            wb(
                                                i_var3
                                                    .wrapping_add(0x33c)
                                                    .wrapping_add(DAT_00a08104),
                                                0xff,
                                            );
                                        }
                                    }
                                    0x13 => {
                                        if 2 < param_1 {
                                            c6 = 0x18;
                                            common = true;
                                        } else {
                                            wb(
                                                i_var3
                                                    .wrapping_add(0x2d8)
                                                    .wrapping_add(DAT_00a08104),
                                                2,
                                            );
                                            wb(
                                                i_var3
                                                    .wrapping_add(0x33c)
                                                    .wrapping_add(DAT_00a08104),
                                                0,
                                            );
                                        }
                                    }
                                    _ => {}
                                }
                                if common {
                                    wb(
                                        i_var3.wrapping_add(0x33c).wrapping_add(DAT_00a08104),
                                        b_var8.wrapping_mul(c6),
                                    );
                                    wb(i_var3.wrapping_add(0x2d8).wrapping_add(DAT_00a08104), 0);
                                    wb(i_var3.wrapping_add(0x30a).wrapping_add(DAT_00a08104), 0);
                                }
                            }
                            u_var9 = (u_var9 as u16).wrapping_add(1) as u32;
                            i_var3 = i_var3.wrapping_add(1);
                            if !((u_var9 as i32) < i_count) {
                                break;
                            }
                        }
                    }
                }
                local_6 = local_6.wrapping_add(1);
                if !((local_6 as i32) < DAT_009a1fe0) {
                    break;
                }
            }
        }
        if DAT_00a00860 == 0 {
            FUN_00438a30(DAT_009ecc34);
        }
    }
}
