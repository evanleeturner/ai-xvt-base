// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006da9f8: i16;
    static mut DAT_007092ca: i16;
    static mut DAT_007092ce: i16;
    static mut DAT_0070ac34: u8;
    static mut DAT_0070ac35: u8;
    static mut DAT_006d4980: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_00732e54: i32;
    static mut DAT_00732910: i16;
    static mut DAT_0071ecf4: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004490f0(param_1: u32) -> i32;
    fn FUN_00449790(param_1: u32, param_2: u32) -> i32;
    fn FUN_004a7d20(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00401960(param_1: u16) {
    // SAFETY: all globals and callees are the game's own memory; pointers
    // are formed from the game's integer addresses exactly as the original.
    unsafe {
        macro_rules! slot {
            ($off:expr, $i:expr) => {
                ((DAT_00732e54 as u32).wrapping_add(($off as i32).wrapping_add($i) as u32))
                    as *mut u8
            };
        }

        DAT_006da9f8 = 1;
        DAT_007092ca = 0;
        DAT_007092ce = 0xc;
        let b_var4 = param_1 as u8;
        if param_1 < 9 {
            DAT_0070ac34 = ((10u32.wrapping_sub(param_1 as u32)) as i32 / 2) as u8;
            DAT_0070ac35 = ((b_var4 & 1) as i32).wrapping_mul(0x1e) as u8;
        } else {
            DAT_0070ac34 = 0;
            DAT_0070ac35 = (0x14i32.wrapping_sub(b_var4 as i32)).wrapping_mul(5) as u8;
        }
        let mut local_6: u16 = 0;
        if 0 < DAT_006d4980 {
            loop {
                let i_var3 =
                    (DAT_006d4988 as u32).wrapping_add((local_6 as u32).wrapping_mul(0x23));
                if (i_var3.wrapping_add(3) as *const u8).read() != 0
                    && (i_var3.wrapping_add(2) as *const u8).read() == 0x0e
                {
                    let i_tmp: i32 = (i_var3.wrapping_add(0x1f) as *const i32).read_unaligned();
                    DAT_00732e54 =
                        ((i_tmp as u32).wrapping_add(0xa7) as *const i32).read_unaligned();
                    let mut i_var5: i32 = 0;
                    let mut u_var6: u32 = 0;
                    let n = FUN_004490f0((i_var3.wrapping_add(3) as *const u8).read() as u32);
                    if 0 < n {
                        loop {
                            i_var5 = i_var5.wrapping_add(1);
                            let sel = ((DAT_006d4988 as u32)
                                .wrapping_add(3)
                                .wrapping_add((local_6 as u32).wrapping_mul(0x23))
                                as *const u8)
                                .read() as u32;
                            let i_var1 = FUN_00449790(sel, u_var6);
                            macro_rules! lab {
                                ($c:expr) => {
                                    *slot!(0x319, i_var5) = (b_var4 as i32).wrapping_mul($c) as u8;
                                    *slot!(0x2c9, i_var5) = 0;
                                    *slot!(0x2f1, i_var5) = 0;
                                };
                            }
                            if local_6 == 1 {
                                if i_var1 == 1 {
                                    *slot!(0x319, i_var5) = 0xff;
                                } else {
                                    *slot!(0x319, i_var5) = 0;
                                }
                                *slot!(0x2c9, i_var5) = 2;
                            } else {
                                match i_var1 {
                                    1 => {
                                        *slot!(0x2c9, i_var5) = 2;
                                    }
                                    5 => {
                                        *slot!(0x319, i_var5) = b_var4.wrapping_mul(2);
                                        *slot!(0x2c9, i_var5) = 0;
                                    }
                                    0x11 => {
                                        if param_1 < 2 {
                                            *slot!(0x2c9, i_var5) = 2;
                                            *slot!(0x319, i_var5) = 0;
                                        } else {
                                            lab!(3);
                                        }
                                    }
                                    0x12 => {
                                        if param_1 < 5 {
                                            *slot!(0x2c9, i_var5) = 2;
                                            *slot!(0x319, i_var5) = 0;
                                        } else {
                                            *slot!(0x2c9, i_var5) = 0;
                                            *slot!(0x2f1, i_var5) = 0;
                                            *slot!(0x319, i_var5) = 0xff;
                                        }
                                    }
                                    0x13 => {
                                        if 2 < param_1 {
                                            lab!(0x18);
                                        } else {
                                            *slot!(0x2c9, i_var5) = 2;
                                            *slot!(0x319, i_var5) = 0;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            u_var6 = (u_var6 as u16).wrapping_add(1) as u32;
                            if !((u_var6 as i32) < n) {
                                break;
                            }
                        }
                    }
                }
                local_6 = local_6.wrapping_add(1);
                if !((local_6 as i32) < DAT_006d4980) {
                    break;
                }
            }
        }
        if DAT_00732910 == 0 {
            FUN_004a7d20(DAT_0071ecf4);
        }
    }
}
