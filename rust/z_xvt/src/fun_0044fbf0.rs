// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d49a0: u16;
    static mut DAT_00732a3c: u16;
    static mut DAT_006d4980: i32;
    static mut DAT_006d4988: u32;
    static mut DAT_006d9d50: u16;
    static mut DAT_006d499c: u32;
    static mut DAT_006d497c: i8;
    static mut DAT_00732a38: u32;
    static mut DAT_00703c88: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0049ec30(param_1: u16, param_2: u16, param_3: u16) -> u16;
    fn FUN_0042a1a0(param_1: u32, param_2: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044fbf0(
    param_1: u16,
    param_2: u16,
    param_3: i16,
    param_4: u16,
    param_5: u16,
) -> u16 {
    // SAFETY: the globals and callees are the game's own; the record table
    // pointer is read at the offsets the original uses.
    unsafe {
        let mut u_var4: u16 = 0;
        let mut u_var5: u16 = 0xffff;
        let mut u_var6: u32 = 0xffff_ffff;
        let mut u_var1: u16 = 0xffff;
        if 0 < DAT_006d49a0 as i32 {
            loop {
                u_var5 = u_var1;
                u_var1 = FUN_0049ec30(u_var4, param_1, param_2);
                let u_var2 = FUN_0049ec30(u_var4, param_4, param_5);
                if param_3 == 1 {
                    u_var1 |= u_var2;
                } else {
                    u_var1 &= u_var2;
                }
                if u_var1 != 0 {
                    u_var1 = DAT_00732a3c;
                    if (u_var1 as i32) < DAT_006d4980 {
                        loop {
                            let i_var3 = (u_var1 as u32)
                                .wrapping_mul(0x23)
                                .wrapping_add(DAT_006d4988);
                            if (*((i_var3.wrapping_add(3)) as *const i8) != 0)
                                && (*((i_var3.wrapping_add(0x16)) as *const u8) as u16 == u_var4)
                            {
                                FUN_0042a1a0(DAT_006d9d50 as u32, u_var1 as u32);
                                if DAT_006d499c < u_var6 {
                                    u_var6 = DAT_006d499c;
                                    DAT_006d497c = u_var4 as i8;
                                    u_var5 = u_var1;
                                }
                            }
                            u_var1 = u_var1.wrapping_add(1);
                            if !((u_var1 as i32) < DAT_006d4980) {
                                break;
                            }
                        }
                    }
                    u_var1 = DAT_00732a38 as u16;
                    if ((DAT_00732a38 & 0xffff) as i32)
                        < (DAT_00703c88.wrapping_add(DAT_00732a38)) as i32
                    {
                        loop {
                            let i_var3 = (u_var1 as u32)
                                .wrapping_mul(0x23)
                                .wrapping_add(DAT_006d4988);
                            if (*((i_var3.wrapping_add(3)) as *const i8) != 0)
                                && (*((i_var3.wrapping_add(0x16)) as *const u8) as u16 == u_var4)
                            {
                                FUN_0042a1a0(DAT_006d9d50 as u32, u_var1 as u32);
                                if DAT_006d499c < u_var6 {
                                    u_var6 = DAT_006d499c;
                                    DAT_006d497c = u_var4 as i8;
                                    u_var5 = u_var1;
                                }
                            }
                            u_var1 = u_var1.wrapping_add(1);
                            if !((u_var1 as i32) < (DAT_00703c88.wrapping_add(DAT_00732a38)) as i32)
                            {
                                break;
                            }
                        }
                    }
                }
                u_var4 = u_var4.wrapping_add(1);
                u_var1 = u_var5;
                if !((u_var4 as i32) < DAT_006d49a0 as i32) {
                    break;
                }
            }
        }
        u_var5
    }
}
