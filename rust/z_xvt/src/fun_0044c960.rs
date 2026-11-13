// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732a3c: u16;
    static mut DAT_006d4980: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_006d9d60: u16;
    static mut DAT_006d9d78: i8;
    static mut DAT_006d9d50: u16;
    static mut DAT_00732a38: i32;
    static mut DAT_00703c88: i32;
    static mut DAT_0052687a: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0049ec30(param_1: u16, param_2: u16, param_3: u16) -> u16;
    fn FUN_00429a20(param_1: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044c960(
    param_1: u16,
    param_2: u16,
    param_3: i16,
    param_4: u16,
    param_5: u16,
) -> i32 {
    // SAFETY: all globals and callees are the game's own; the pointers
    // computed from the entity table base are read exactly where the
    // original reads them.
    unsafe {
        let mut local_6: u16 = DAT_00732a3c;
        let mut i_var1: i32 = 0;
        let mut i_var8: i32 = 0;
        if (DAT_00732a3c as i32) < DAT_006d4980 {
            loop {
                i_var1 = i_var8;
                i_var8 = (local_6 as u32).wrapping_mul(0x23) as i32;
                let i_var7 = DAT_006d4988.wrapping_add(i_var8);
                if *((i_var7.wrapping_add(3)) as *const i8) != 0 {
                    let u_var5: u16 = *((i_var7.wrapping_add(0x16)) as *const u8) as u16;
                    if u_var5 != DAT_006d9d60 {
                        let mut u_var4 = FUN_0049ec30(u_var5, param_1, param_2);
                        let u_var5b = FUN_0049ec30(u_var5, param_4, param_5);
                        if param_3 == 1 {
                            u_var4 |= u_var5b;
                        } else {
                            u_var4 &= u_var5b;
                        }
                        if u_var4 != 0 {
                            let b_var3 = FUN_00429a20(local_6 as u32);
                            if b_var3 != 0 {
                                i_var8 = ((DAT_006d4988.wrapping_add(0x1f).wrapping_add(i_var8))
                                    as *const i32)
                                    .read_unaligned();
                                let i_var7 =
                                    (i_var8.wrapping_add(0xa7) as *const i32).read_unaligned();
                                let mut count = false;
                                if DAT_006d9d78 == 0 {
                                    count = true;
                                } else if (i_var7.wrapping_add(0x1df) as *const i16)
                                    .read_unaligned()
                                    != 0
                                {
                                    if DAT_006d9d78 == 0
                                        || *(i_var7.wrapping_add(0x1e5) as *const i8) == 0
                                    {
                                        count = true;
                                    } else {
                                        let p = ((DAT_006d9d50 as u32)
                                            .wrapping_mul(0x23)
                                            .wrapping_add(0x1f)
                                            .wrapping_add(DAT_006d4988 as u32))
                                            as *const i32;
                                        let q = p.read_unaligned();
                                        if *(q.wrapping_add(0x87) as *const i8)
                                            != *(i_var8.wrapping_add(0x87) as *const i8)
                                        {
                                            count = true;
                                        }
                                    }
                                }
                                if count {
                                    i_var1 = i_var1.wrapping_add(1);
                                }
                            }
                        }
                    }
                }
                local_6 = local_6.wrapping_add(1);
                i_var8 = i_var1;
                if !((local_6 as i32) < DAT_006d4980) {
                    break;
                }
            }
        }
        let mut u_var5: u16 = DAT_00732a38 as u16;
        if (DAT_00732a38 & 0xffff) < DAT_00703c88.wrapping_add(DAT_00732a38) {
            loop {
                let i_var8 = ((u_var5 as u32).wrapping_mul(0x23) as i32).wrapping_add(DAT_006d4988);
                let b = *(i_var8.wrapping_add(3) as *const u8);
                if b != 0 {
                    let flags = *(&raw mut DAT_0052687a as *mut u8)
                        .add((b as u32).wrapping_mul(0x16) as usize);
                    if (flags & 2) != 0 {
                        let b2 = *(i_var8.wrapping_add(0x16) as *const u8);
                        let mut u_var4 = FUN_0049ec30(b2 as u16, param_1, param_2);
                        let u_var6 = FUN_0049ec30(b2 as u16, param_4, param_5);
                        if param_3 == 1 {
                            u_var4 |= u_var6;
                        } else {
                            u_var4 &= u_var6;
                        }
                        if u_var4 != 0 {
                            let r = FUN_00429a20(u_var5 as u32);
                            if r != 0 {
                                i_var1 = i_var1.wrapping_add(1);
                            }
                        }
                    }
                }
                u_var5 = u_var5.wrapping_add(1);
                if !((u_var5 as i32) < DAT_00703c88.wrapping_add(DAT_00732a38)) {
                    break;
                }
            }
        }
        if i_var1 == 0 {
            return 0xffff;
        }
        i_var1
    }
}
