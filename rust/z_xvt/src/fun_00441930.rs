// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732a3c: u16;
    static mut DAT_00732a38: i32;
    static mut DAT_006d4988: u32;
    static mut DAT_006d4980: i32;
    static mut DAT_00703c88: i32;
    static mut DAT_0071b730: [u8; 1];
    static mut DAT_0071b743: [i8; 1];
    static mut DAT_0052687a: [u8; 1];
    static mut DAT_006d9d40: u16;
    static mut DAT_006d499c: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00441fc0(a: u32, b: u32, c: u32, d: i32, e: u32) -> u64;
    fn FUN_004a1d70(a: u32, b: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00441930(param_1: u32) -> u32 {
    // SAFETY: all globals and callees are the game's own memory; the
    // computed addresses point into the game's tables as the original does.
    unsafe {
        let unaff_esi: u32 = 0;
        let mut u_var7: u32 = ((unaff_esi >> 16) << 16) | (DAT_00732a3c as u32);
        let mut u_var4: u16 = 0xffff;
        let mut local_4: u16 = 0xffff;
        let mut u_var5: u32 = 0xffff_ffff;
        let mut local_2: u16 = 0xffff;

        if (DAT_00732a3c as i32) < DAT_00732a38 {
            loop {
                let b_var2 = ((u_var7 & 0xffff)
                    .wrapping_mul(0x23)
                    .wrapping_add(3)
                    .wrapping_add(DAT_006d4988) as usize
                    as *const u8)
                    .read();
                let u_var6 = u_var7 as u16;
                if b_var2 != 0 {
                    let tmp = (&raw mut DAT_0071b730 as *mut u8)
                        .byte_add(param_1.wrapping_mul(0x5bd) as usize)
                        .cast::<u32>()
                        .read_unaligned();
                    if tmp != (u_var7 & 0xffff) {
                        let u_var3 = b_var2 as u32;
                        let f = (&raw mut DAT_0052687a as *mut u8)
                            .byte_add(u_var3.wrapping_mul(0x16) as usize)
                            .read();
                        if (f & 1) != 0 {
                            let u_var8 = FUN_00441fc0(
                                u_var3.wrapping_mul(0xb),
                                u_var3.wrapping_mul(5),
                                u_var7,
                                1,
                                param_1,
                            );
                            if (u_var8 as u16) == 0 {
                                if DAT_006d9d40 < u_var4 {
                                    local_4 = u_var6;
                                    u_var4 = DAT_006d9d40;
                                }
                            } else if DAT_006d499c < u_var5 {
                                u_var5 = DAT_006d499c;
                                local_2 = u_var6;
                            }
                        }
                    }
                }
                u_var7 = (u_var7 & 0xffff_0000) | (u_var6.wrapping_add(1) as u32);
                if !((u_var6.wrapping_add(1) as i32) < DAT_00732a38) {
                    break;
                }
            }
        }

        u_var7 = (u_var7 & 0xffff_0000) | ((DAT_00732a38 as u32) & 0xffff);
        if ((DAT_00732a38 as u32 & 0xffff) as i32)
            < ((DAT_00703c88 as u32).wrapping_add(DAT_00732a38 as u32) as i32)
        {
            loop {
                let b_var2 = ((u_var7 & 0xffff)
                    .wrapping_mul(0x23)
                    .wrapping_add(3)
                    .wrapping_add(DAT_006d4988) as usize
                    as *const u8)
                    .read();
                let u_var6 = u_var7 as u16;
                if b_var2 != 0 {
                    let u_var3 = b_var2 as u32;
                    let f = (&raw mut DAT_0052687a as *mut u8)
                        .byte_add(u_var3.wrapping_mul(0x16) as usize)
                        .read();
                    if (f & 1) != 0 {
                        let u_var8 = FUN_00441fc0(
                            u_var3.wrapping_mul(0xb),
                            u_var3.wrapping_mul(5),
                            u_var7,
                            1,
                            param_1,
                        );
                        if (u_var8 as u16) == 0 {
                            if DAT_006d9d40 < u_var4 {
                                local_4 = u_var6;
                                u_var4 = DAT_006d9d40;
                            }
                        } else if DAT_006d499c < u_var5 {
                            u_var5 = DAT_006d499c;
                            local_2 = u_var6;
                        }
                    }
                }
                u_var7 = (u_var7 & 0xffff_0000) | (u_var6.wrapping_add(1) as u32);
                if !((u_var6.wrapping_add(1) as i32)
                    < ((DAT_00703c88 as u32).wrapping_add(DAT_00732a38 as u32) as i32))
                {
                    break;
                }
            }
        }

        let flag_byte = (&raw mut DAT_0071b743 as *mut u8)
            .byte_add(param_1.wrapping_mul(0x5bd) as usize)
            .read();

        'aed: {
            if local_2 == 0xffff {
                if flag_byte == 0 {
                    if 0x31 < u_var4 {
                        break 'aed;
                    }
                } else if 0xf9 < u_var4 {
                    break 'aed;
                }
                local_2 = local_4;
            }
        }

        u_var5 = param_1.wrapping_mul(0x16f);
        if flag_byte == 0 {
            'check: {
                'clear: {
                    if local_2 == 0xffff {
                        break 'clear;
                    }
                    u_var5 = DAT_006d4980 as u32;
                    if DAT_006d4980 <= (local_2 as i32) {
                        break 'clear;
                    }
                    u_var5 = (local_2 as u32)
                        .wrapping_mul(0x23)
                        .wrapping_add(DAT_006d4988);
                    let i_var = (u_var5.wrapping_add(0x1b) as usize as *const i32).read_unaligned();
                    if i_var == -1 {
                        break 'clear;
                    }
                    let p = (u_var5.wrapping_add(0x1f) as usize as *const u32).read_unaligned();
                    let q = (p.wrapping_add(0xa7) as usize as *const u32).read_unaligned();
                    u_var5 = q;
                    if u_var5 == 0 {
                        break 'clear;
                    }
                    if ((u_var5.wrapping_add(0x1e0) as usize as *const u8).read() & 1) == 0 {
                        break 'clear;
                    }
                    if (u_var5.wrapping_add(0x237) as usize as *const u8).read() == 0 {
                        break 'clear;
                    }
                    if (u_var5.wrapping_add(0x233) as usize as *const u8).read() != 3 {
                        break 'clear;
                    }
                    let sv = (u_var5.wrapping_add(0x238) as usize as *const u16).read_unaligned();
                    u_var5 = (u_var5 & 0xffff_ff00) | 1;
                    if sv == 0 {
                        break 'clear;
                    }
                    break 'check;
                }
                u_var5 &= 0xffff_ff00;
            }
            if (u_var5 as u8) == 1 {
                local_2 = 0xffff;
                u_var5 = FUN_004a1d70(0x100, param_1);
            }
        }
        ((u_var5 >> 16) << 16) | (local_2 as u32)
    }
}
