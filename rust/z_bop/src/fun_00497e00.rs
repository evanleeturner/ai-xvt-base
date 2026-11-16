// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00527e90: i32;
    static mut DAT_005233d8: i32;
    static mut DAT_009d682c: u16;
    static mut DAT_009a1fe4: u16;
    static mut DAT_006122d8: [u8; 0];
    static mut DAT_00a00530: [u16; 0];
    static mut DAT_00a081ec: i32;
    static mut DAT_00a0813f: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040c430(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32);
    fn FUN_004aba50();
    fn FUN_00497940(a: i32, b: i32, c: i32, d: i32, e: f32);
    fn FUN_004abe50();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00497e00(
    param_1: i32,
    param_2: i32,
    param_3: u32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
) {
    let mut param_6 = param_6;
    // SAFETY: the globals and callees are the game's own; the pixel buffer
    // pointer is the game's surface, written where the original writes.
    unsafe {
        let i_var3 = param_4.wrapping_add(param_2);
        let u_var4 = param_3.wrapping_add(param_1 as u32);
        if !(0 < i_var3
            && 0 < (u_var4 as i32)
            && param_1 < (DAT_009d682c as i32)
            && param_2 < (DAT_009a1fe4 as i32)
            && 0 < param_4
            && 0 < (param_3 as i32))
        {
            return;
        }
        if DAT_00527e90 != 0 {
            FUN_0040c430(param_1, param_2, param_3 as i32, param_4, param_5, param_6);
            return;
        }
        let mut u_var8: u32 = ((param_3 as i32) >> 3) as u32;
        let mut local_14: i32 = param_4 >> 3;
        if (u_var8 as i32) < 3 {
            u_var8 = 3;
        }
        if local_14 < 3 {
            local_14 = 3;
        }
        if (param_3 as i32) < (u_var8 as i32) {
            u_var8 = param_3;
        }
        if param_4 < local_14 {
            local_14 = param_4;
        }
        let base = (&raw mut DAT_006122d8) as *mut u8;
        let mut p_var7: *mut u8 = base;
        if DAT_005233d8 == 2 {
            if 0 < param_1 {
                p_var7 = base.wrapping_add((param_1 as u32).wrapping_mul(2) as usize);
            }
            let u_var1: u16 = ((&raw const DAT_00a00530) as *const u16)
                .offset(param_5 as isize)
                .read_unaligned();
            if 0 < (u_var8 as i32) {
                let pat: u32 = ((u_var1 as u32) << 16) | (u_var1 as u32);
                let mut p9 = p_var7;
                let mut n = u_var8 >> 1;
                while n != 0 {
                    p9.cast::<u32>().write_unaligned(pat);
                    p9 = p9.wrapping_add(4);
                    n = n.wrapping_sub(1);
                }
                let mut n = ((u_var8 & 1) != 0) as u32;
                while n != 0 {
                    p9.cast::<u16>().write_unaligned(u_var1);
                    p9 = p9.wrapping_add(2);
                    n = n.wrapping_sub(1);
                }
            }
        } else {
            if 0 < param_1 {
                p_var7 = base.offset(param_1 as isize);
            }
            if 0 < (u_var8 as i32) {
                let b = param_5 as u8;
                let pat: u32 =
                    ((b as u32) << 24) | ((b as u32) << 16) | ((b as u32) << 8) | (b as u32);
                let mut p9 = p_var7;
                let mut n = u_var8 >> 2;
                while n != 0 {
                    p9.cast::<u32>().write_unaligned(pat);
                    p9 = p9.wrapping_add(4);
                    n = n.wrapping_sub(1);
                }
                let mut n = u_var8 & 3;
                while n != 0 {
                    *p9 = b;
                    p9 = p9.wrapping_add(1);
                    n = n.wrapping_sub(1);
                }
            }
        }
        if param_6 < 1 {
            param_6 = 1;
        }
        let f_var2: f32 = (DAT_00a081ec as f32) / (param_6 as f32);
        if DAT_00a0813f == 0 {
            FUN_004aba50();
        }
        let p7 = p_var7 as usize as i32;
        if -1 < param_2 {
            let mut u_var6 = (param_1 as u32).wrapping_add(u_var8);
            let u_var5 = DAT_009d682c as u32;
            if 0 < (u_var6 as i32) && param_1 < (u_var5 as i32) {
                let mut i_var10 = param_1;
                if param_1 < 0 {
                    i_var10 = 0;
                }
                if (u_var5 as i32) < (u_var6 as i32) {
                    u_var6 = u_var5;
                }
                FUN_00497940(p7, i_var10, u_var6 as i32, param_2, f_var2);
            }
            let mut i_var10 = param_3.wrapping_sub(u_var8).wrapping_add(param_1 as u32) as i32;
            let u_var6 = DAT_009d682c as u32;
            if 0 < (u_var4 as i32) && i_var10 < (u_var6 as i32) {
                if i_var10 < 0 {
                    i_var10 = 0;
                }
                let mut u_var5 = u_var4;
                if (u_var6 as i32) < (u_var4 as i32) {
                    u_var5 = u_var6;
                }
                FUN_00497940(p7, i_var10, u_var5 as i32, param_2, f_var2);
            }
        }
        if i_var3 <= (DAT_009a1fe4 as i32) {
            let mut u_var6 = (param_1 as u32).wrapping_add(u_var8);
            let u_var5 = DAT_009d682c as u32;
            if 0 < (u_var6 as i32) && param_1 < (u_var5 as i32) {
                let mut i_var10 = param_1;
                if param_1 < 0 {
                    i_var10 = 0;
                }
                if (u_var5 as i32) < (u_var6 as i32) {
                    u_var6 = u_var5;
                }
                FUN_00497940(p7, i_var10, u_var6 as i32, i_var3.wrapping_sub(1), f_var2);
            }
            let mut i_var10 = param_3.wrapping_sub(u_var8).wrapping_add(param_1 as u32) as i32;
            let u_var8b = DAT_009d682c as u32;
            if 0 < (u_var4 as i32) && i_var10 < (u_var8b as i32) {
                if i_var10 < 0 {
                    i_var10 = 0;
                }
                let mut u_var6 = u_var4;
                if (u_var8b as i32) < (u_var4 as i32) {
                    u_var6 = u_var8b;
                }
                FUN_00497940(p7, i_var10, u_var6 as i32, i_var3.wrapping_sub(1), f_var2);
            }
        }
        let mut i_var3b: i32 = 1;
        if 1 < local_14 {
            loop {
                let i_var10 = param_2.wrapping_add(i_var3b);
                if -1 < i_var10 && i_var10 < (DAT_009a1fe4 as i32) {
                    if -1 < param_1 {
                        FUN_00497940(p7, param_1, param_1.wrapping_add(1), i_var10, f_var2);
                    }
                    if (u_var4 as i32) <= (DAT_009d682c as i32) {
                        FUN_00497940(
                            p7,
                            u_var4.wrapping_sub(1) as i32,
                            u_var4 as i32,
                            i_var10,
                            f_var2,
                        );
                    }
                }
                i_var3b = i_var3b.wrapping_add(1);
                if !(i_var3b < local_14) {
                    break;
                }
            }
        }
        let mut i_var3c = param_4.wrapping_sub(local_14);
        while i_var3c < param_4.wrapping_sub(1) {
            let i_var10 = param_2.wrapping_add(i_var3c);
            if local_14 <= i_var3c && -1 < i_var10 && i_var10 < (DAT_009a1fe4 as i32) {
                if -1 < param_1 {
                    FUN_00497940(p7, param_1, param_1.wrapping_add(1), i_var10, f_var2);
                }
                if (u_var4 as i32) <= (DAT_009d682c as i32) {
                    FUN_00497940(
                        p7,
                        u_var4.wrapping_sub(1) as i32,
                        u_var4 as i32,
                        i_var10,
                        f_var2,
                    );
                }
            }
            i_var3c = i_var3c.wrapping_add(1);
        }
        if DAT_00a0813f == 0 {
            FUN_004abe50();
        }
    }
}
