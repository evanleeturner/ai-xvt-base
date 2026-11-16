// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052ae18: i32;
    static mut DAT_00525518: i32;
    static mut DAT_007091ae: u16;
    static mut DAT_006d4984: u16;
    static mut DAT_005f1d88: [u8; 0];
    static mut DAT_007325e0: [u16; 0];
    static mut DAT_00732f3c: i32;
    static mut DAT_00732e8f: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004604c0(
        param_1: i32,
        param_2: i32,
        param_3: u32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    );
    fn FUN_004a6a80();
    fn FUN_004a6f90();
    fn FUN_004774e0(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: f32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00477c90(
    param_1: i32,
    param_2: i32,
    param_3: u32,
    param_4: i32,
    param_5: i32,
    mut param_6: i32,
) {
    // SAFETY: all globals and callees are the game's own; the framebuffer
    // pointer and palette table are written and read where the original does.
    unsafe {
        let i_var3 = param_4.wrapping_add(param_2);
        let u_var4 = param_3.wrapping_add(param_1 as u32);
        if !(0 < i_var3
            && 0 < (u_var4 as i32)
            && param_1 < (DAT_007091ae as i32)
            && param_2 < (DAT_006d4984 as i32)
            && 0 < param_4
            && 0 < (param_3 as i32))
        {
            return;
        }
        if DAT_0052ae18 != 0 {
            FUN_004604c0(param_1, param_2, param_3, param_4, param_5, param_6);
            return;
        }
        let mut u_var9: u32 = ((param_3 as i32) >> 3) as u32;
        let mut local_14: i32 = param_4 >> 3;
        if (u_var9 as i32) < 3 {
            u_var9 = 3;
        }
        if local_14 < 3 {
            local_14 = 3;
        }
        if (param_3 as i32) < (u_var9 as i32) {
            u_var9 = param_3;
        }
        if param_4 < local_14 {
            local_14 = param_4;
        }
        let base = (&raw mut DAT_005f1d88) as *mut u8;
        let mut pu_var8: *mut u8 = base;
        if DAT_00525518 == 2 {
            if 0 < param_1 {
                pu_var8 = base.byte_add((param_1 as u32).wrapping_mul(2) as usize);
            }
            let u_var1: u16 = ((&raw const DAT_007325e0) as *const u16)
                .offset(param_5 as isize)
                .read_unaligned();
            if 0 < (u_var9 as i32) {
                let v32: u32 = ((u_var1 as u32) << 16) | (u_var1 as u32);
                let mut p = pu_var8;
                let mut c = u_var9 >> 1;
                while c != 0 {
                    p.cast::<u32>().write_unaligned(v32);
                    p = p.byte_add(4);
                    c = c.wrapping_sub(1);
                }
                let mut c = ((u_var9 & 1) != 0) as u32;
                while c != 0 {
                    p.cast::<u16>().write_unaligned(u_var1);
                    p = p.byte_add(2);
                    c = c.wrapping_sub(1);
                }
            }
        } else {
            if 0 < param_1 {
                pu_var8 = base.byte_add(param_1 as usize);
            }
            if 0 < (u_var9 as i32) {
                let b = param_5 as u8;
                let v32: u32 =
                    ((b as u32) << 24) | ((b as u32) << 16) | ((b as u32) << 8) | (b as u32);
                let mut p = pu_var8;
                let mut c = u_var9 >> 2;
                while c != 0 {
                    p.cast::<u32>().write_unaligned(v32);
                    p = p.byte_add(4);
                    c = c.wrapping_sub(1);
                }
                let mut c = u_var9 & 3;
                while c != 0 {
                    p.write(b);
                    p = p.byte_add(1);
                    c = c.wrapping_sub(1);
                }
            }
        }
        if param_6 < 1 {
            param_6 = 1;
        }
        let f_var2: f32 = (DAT_00732f3c as f32) / (param_6 as f32);
        if DAT_00732e8f == 0 {
            FUN_004a6a80();
        }
        if -1 < param_2 {
            let mut u_var6 = u_var9.wrapping_add(param_1 as u32);
            let u_var5 = DAT_007091ae as u32;
            if 0 < (u_var6 as i32) && param_1 < (u_var5 as i32) {
                let mut i_var7 = param_1;
                if param_1 < 0 {
                    i_var7 = 0;
                }
                if (u_var5 as i32) < (u_var6 as i32) {
                    u_var6 = u_var5;
                }
                FUN_004774e0(pu_var8 as i32, i_var7, u_var6 as i32, param_2, f_var2);
            }
            let mut i_var7 = param_3.wrapping_sub(u_var9).wrapping_add(param_1 as u32) as i32;
            let u_var6 = DAT_007091ae as u32;
            if 0 < (u_var4 as i32) && i_var7 < (u_var6 as i32) {
                if i_var7 < 0 {
                    i_var7 = 0;
                }
                let mut u_var5 = u_var4;
                if (u_var6 as i32) < (u_var4 as i32) {
                    u_var5 = u_var6;
                }
                FUN_004774e0(pu_var8 as i32, i_var7, u_var5 as i32, param_2, f_var2);
            }
        }
        if i_var3 <= (DAT_006d4984 as i32) {
            let mut u_var6 = u_var9.wrapping_add(param_1 as u32);
            let u_var5 = DAT_007091ae as u32;
            if 0 < (u_var6 as i32) && param_1 < (u_var5 as i32) {
                let mut i_var7 = param_1;
                if param_1 < 0 {
                    i_var7 = 0;
                }
                if (u_var5 as i32) < (u_var6 as i32) {
                    u_var6 = u_var5;
                }
                FUN_004774e0(
                    pu_var8 as i32,
                    i_var7,
                    u_var6 as i32,
                    i_var3.wrapping_sub(1),
                    f_var2,
                );
            }
            let mut i_var7 = param_3.wrapping_sub(u_var9).wrapping_add(param_1 as u32) as i32;
            let u_var9b = DAT_007091ae as u32;
            if 0 < (u_var4 as i32) && i_var7 < (u_var9b as i32) {
                if i_var7 < 0 {
                    i_var7 = 0;
                }
                let mut u_var6 = u_var4;
                if (u_var9b as i32) < (u_var4 as i32) {
                    u_var6 = u_var9b;
                }
                FUN_004774e0(
                    pu_var8 as i32,
                    i_var7,
                    u_var6 as i32,
                    i_var3.wrapping_sub(1),
                    f_var2,
                );
            }
        }
        let mut i_var3: i32 = 1;
        if 1 < local_14 {
            loop {
                let i_var7 = param_2.wrapping_add(i_var3);
                if -1 < i_var7 && i_var7 < (DAT_006d4984 as i32) {
                    if -1 < param_1 {
                        FUN_004774e0(
                            pu_var8 as i32,
                            param_1,
                            param_1.wrapping_add(1),
                            i_var7,
                            f_var2,
                        );
                    }
                    if (u_var4 as i32) <= (DAT_007091ae as i32) {
                        FUN_004774e0(
                            pu_var8 as i32,
                            u_var4.wrapping_sub(1) as i32,
                            u_var4 as i32,
                            i_var7,
                            f_var2,
                        );
                    }
                }
                i_var3 = i_var3.wrapping_add(1);
                if !(i_var3 < local_14) {
                    break;
                }
            }
        }
        i_var3 = param_4.wrapping_sub(local_14);
        while i_var3 < param_4.wrapping_sub(1) {
            if local_14 <= i_var3 {
                let i_var7 = param_2.wrapping_add(i_var3);
                if -1 < i_var7 && i_var7 < (DAT_006d4984 as i32) {
                    if -1 < param_1 {
                        FUN_004774e0(
                            pu_var8 as i32,
                            param_1,
                            param_1.wrapping_add(1),
                            i_var7,
                            f_var2,
                        );
                    }
                    if (u_var4 as i32) <= (DAT_007091ae as i32) {
                        FUN_004774e0(
                            pu_var8 as i32,
                            u_var4.wrapping_sub(1) as i32,
                            u_var4 as i32,
                            i_var7,
                            f_var2,
                        );
                    }
                }
            }
            i_var3 = i_var3.wrapping_add(1);
        }
        if DAT_00732e8f == 0 {
            FUN_004a6f90();
        }
    }
}
