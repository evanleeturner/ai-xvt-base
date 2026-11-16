// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732a3c: u16;
    static mut DAT_006d4980: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_006d9d78: i8;
    static mut DAT_00732e94: i32;
    static mut DAT_00732e90: i32;
    static mut DAT_00732e98: i32;
    static mut DAT_006d499c: u32;
    static mut DAT_006d4974: i32;
    static mut DAT_006d4978: i32;
    static mut DAT_006d4970: i32;
    static mut DAT_00732a2c: i32;
    static mut DAT_00732a28: i32;
    static mut DAT_00732a24: i32;
    static mut DAT_006d9d50: i16;
    static mut DAT_00732a38: i32;
    static mut DAT_00703c88: i32;
    static mut DAT_0052687a: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0049f010(param_1: u32, param_2: i16, param_3: u16) -> u8;
    fn FUN_00429a20(param_1: u32) -> u8;
    fn FUN_00412810(param_1: i32, param_2: i32, param_3: i32) -> u32;
    fn FUN_00421c70(param_1: u32, param_2: i32);
    fn FUN_00419190(param_1: i16, param_2: i16) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044efc0(
    param_1: i16,
    param_2: u16,
    param_3: i16,
    param_4: i16,
    param_5: u16,
    param_6: i16,
) -> u32 {
    // SAFETY: all globals and callees are the game's own; the pointers formed
    // from the game's object table are read where the original reads them.
    unsafe {
        let mut local_2: i16 = -1;
        let mut u_var8: u32 = DAT_00732a3c as u32;
        let mut u_var5: u32 = 0xffff_ffff;
        let mut i_var4: i32;

        if (DAT_00732a3c as i32) < DAT_006d4980 {
            loop {
                let u_var6: u32 = u_var8 & 0xffff;
                i_var4 = u_var6.wrapping_mul(0x23) as i32;
                let base = DAT_006d4988 as u32;
                'skip: {
                    if ((base.wrapping_add(3).wrapping_add(i_var4 as u32)) as usize as *const u8)
                        .read()
                        == 0
                    {
                        break 'skip;
                    }
                    let b2 = FUN_0049f010(u_var8, param_1, param_2);
                    let mut local_4: u16 = b2 as u16;
                    let b2 = FUN_0049f010(u_var8, param_4, param_5);
                    let u_var7: u16 = b2 as u16;
                    if param_3 == 1 {
                        local_4 |= u_var7;
                    } else {
                        local_4 &= u_var7;
                    }
                    if local_4 == 0 {
                        break 'skip;
                    }
                    let mut cond = false;
                    if DAT_006d9d78 == 0 {
                        cond = true;
                    } else {
                        let ptr1: u32 = (base.wrapping_add(0x1f).wrapping_add(i_var4 as u32)
                            as usize as *const u32)
                            .read_unaligned();
                        let ptr2: u32 =
                            (ptr1.wrapping_add(0xa7) as usize as *const u32).read_unaligned();
                        let sval: i16 =
                            (ptr2.wrapping_add(0x1df) as usize as *const i16).read_unaligned();
                        if sval != 0 {
                            cond = true;
                        }
                    }
                    if cond && FUN_00429a20(u_var6) != 0 {
                        let p: u32 = (DAT_006d4988 as u32).wrapping_add(i_var4 as u32);
                        let f4: i32 = (p.wrapping_add(4) as usize as *const i32).read_unaligned();
                        let f8: i32 = (p.wrapping_add(8) as usize as *const i32).read_unaligned();
                        let fc: i32 = (p.wrapping_add(0xc) as usize as *const i32).read_unaligned();
                        DAT_006d499c = FUN_00412810(
                            f4.wrapping_sub(DAT_00732e94),
                            f8.wrapping_sub(DAT_00732e90),
                            fc.wrapping_sub(DAT_00732e98),
                        );
                        if DAT_006d499c < u_var5 {
                            if param_6 != 0 {
                                FUN_00421c70(u_var6, 0);
                                DAT_006d4974 = DAT_00732a2c;
                                DAT_006d4978 = DAT_00732a28;
                                DAT_006d4970 = DAT_00732a24;
                                i_var4 = FUN_00419190(DAT_006d9d50, DAT_006d9d50);
                                if i_var4 != 0 {
                                    break 'skip;
                                }
                            }
                            u_var5 = DAT_006d499c;
                            local_2 = u_var8 as i16;
                        }
                    }
                }
                let u_var7: u16 = (u_var8 as i16).wrapping_add(1) as u16;
                u_var8 = (u_var8 & 0xffff_0000) | u_var7 as u32;
                if !((u_var7 as i32) < DAT_006d4980) {
                    break;
                }
            }
        }

        u_var8 = (u_var8 & 0xffff_0000) | (DAT_00732a38 as u16 as u32);
        i_var4 = DAT_00703c88.wrapping_add(DAT_00732a38);
        if ((DAT_00732a38 as u32) & 0xffff) as i32 > i_var4
            || ((DAT_00732a38 as u32) & 0xffff) as i32 == i_var4
        {
            // loop not entered
        } else {
            loop {
                let u_var6: u32 = u_var8 & 0xffff;
                'skip2: {
                    let b1: u8 = (u_var6
                        .wrapping_mul(0x23)
                        .wrapping_add(3)
                        .wrapping_add(DAT_006d4988 as u32)
                        as usize as *const u8)
                        .read();
                    if b1 == 0 {
                        break 'skip2;
                    }
                    let t: u8 = (&raw const DAT_0052687a)
                        .cast::<u8>()
                        .add((b1 as u32).wrapping_mul(0x16) as usize)
                        .read();
                    if (t & 2) == 0 {
                        break 'skip2;
                    }
                    let b2 = FUN_0049f010(u_var8, param_1, param_2);
                    let mut u_var7: u16 = b2 as u16;
                    let b2 = FUN_0049f010(u_var8, param_4, param_5);
                    let u_var3: u16 = b2 as u16;
                    if param_3 == 1 {
                        u_var7 |= u_var3;
                    } else {
                        u_var7 &= u_var3;
                    }
                    if u_var7 != 0 && FUN_00429a20(u_var6) != 0 {
                        FUN_00421c70(u_var6, 0);
                        DAT_006d499c = FUN_00412810(
                            DAT_00732a2c.wrapping_sub(DAT_00732e94),
                            DAT_00732a28.wrapping_sub(DAT_00732e90),
                            DAT_00732a24.wrapping_sub(DAT_00732e98),
                        );
                        if DAT_006d499c < u_var5 {
                            if param_6 != 0 {
                                DAT_006d4974 = DAT_00732a2c;
                                DAT_006d4978 = DAT_00732a28;
                                DAT_006d4970 = DAT_00732a24;
                                i_var4 = FUN_00419190(DAT_006d9d50, DAT_006d9d50);
                                if i_var4 != 0 {
                                    break 'skip2;
                                }
                            }
                            u_var5 = DAT_006d499c;
                            local_2 = u_var8 as i16;
                        }
                    }
                }
                let u_var7: u16 = (u_var8 as i16).wrapping_add(1) as u16;
                u_var8 = (u_var8 & 0xffff_0000) | u_var7 as u32;
                i_var4 = DAT_00703c88.wrapping_add(DAT_00732a38);
                if !((u_var7 as i32) < i_var4) {
                    break;
                }
            }
        }

        if 0x10000 < u_var5 {
            local_2 = -1;
        }
        (((i_var4 as u32) >> 16) << 16) | (local_2 as u16 as u32)
    }
}
