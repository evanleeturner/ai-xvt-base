// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005270b0: i32;
    static mut DAT_00527e88: *mut i32;
    static mut DAT_00527e84: i32;
    static mut DAT_00622c50: [f32; 3];
    static mut DAT_00622c60: [f32; 3];
    static mut DAT_00622c6c: u32;
    static mut DAT_00622c8c: u32;
    static mut DAT_00622c90: u32;
    static mut DAT_00622c98: f32;
    static mut DAT_00518208: f32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00479de0(param_1: i32, param_2: *mut i8) -> *mut u32;
    fn FUN_004a6560(
        param_1: *mut f32,
        param_2: *mut f32,
        param_3: *mut f32,
        param_4: *mut f32,
        param_5: *mut f32,
    ) -> i32;
    fn FUN_004a66d0(param_1: *mut f32, param_2: i32, param_3: *mut i32, param_4: *mut f32) -> i32;
    fn FUN_00421300(param_1: *mut f32, param_2: *mut f32);
    fn FUN_00420fe0(param_1: *mut f32, param_2: *mut f32);
}

#[inline(always)]
unsafe fn rd(p: *mut u32, i: usize) -> u32 {
    // SAFETY: caller guarantees p points at a valid node of the game.
    unsafe { p.add(i).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a6080(param_1: i32, param_2: *mut u32) -> u32 {
    let mut param_2 = param_2;
    // SAFETY: all globals and callees are the game's own; node pointers
    // are the game's tree nodes, read where the original reads them.
    unsafe {
        loop {
            if param_2.is_null() {
                return 0;
            }
            let mut local_64: i32 = 0;
            while rd(param_2, 1) == 7 {
                if DAT_005270b0 == 0 {
                    param_2 = FUN_00479de0(param_1, rd(param_2, 5) as *mut i8);
                } else if *(rd(param_2, 5) as *const i8) == 0 {
                    param_2 = rd(param_2, 0) as *mut u32;
                } else {
                    let p1 = FUN_00479de0(param_1, rd(param_2, 5) as *mut i8);
                    param_2.write_unaligned(p1 as u32);
                    *(rd(param_2, 5) as *mut i8) = 0;
                    param_2 = rd(param_2, 0) as *mut u32;
                }
                if param_2.is_null() {
                    return 0;
                }
            }
            match rd(param_2, 1) {
                1 | 0xf | 0x10 | 0x11 => {
                    let mut pi5 = rd(param_2, 5).wrapping_add(4) as *mut i32;
                    let mut local_60: i32 = 0;
                    let mut pf3 =
                        pi5.wrapping_add(rd(param_2, 4).wrapping_mul(0x10) as usize) as *mut f32;
                    let i_var6 = DAT_00527e88.add(5).read_unaligned();
                    if 0 < rd(param_2, 4) as i32 {
                        loop {
                            let mut local_68: f32 = 0.0;
                            let i_var4 = FUN_004a6560(
                                pf3,
                                (i_var6 as u32)
                                    .wrapping_add((pi5.read_unaligned() as u32).wrapping_mul(0xc))
                                    as *mut f32,
                                &raw mut DAT_00622c50 as *mut f32,
                                &raw mut DAT_00622c60 as *mut f32,
                                &mut local_68,
                            );
                            if i_var4 != 0 {
                                let bits = local_68.to_bits() as i32;
                                if (DAT_00527e84 == 0 || 0x3dcccccc < bits)
                                    && local_68 < DAT_00622c98
                                {
                                    let mut local_5c = [0.0f32; 3];
                                    local_5c[0] = (DAT_00622c60[0] - DAT_00622c50[0]) * local_68
                                        + DAT_00622c50[0];
                                    local_5c[1] = (DAT_00622c60[1] - DAT_00622c50[1]) * local_68
                                        + DAT_00622c50[1];
                                    local_5c[2] = (DAT_00622c60[2] - DAT_00622c50[2]) * local_68
                                        + DAT_00622c50[2];
                                    let r = FUN_004a66d0(pf3, i_var6, pi5, local_5c.as_mut_ptr());
                                    if r != 0 {
                                        DAT_00622c98 = local_68;
                                        DAT_00622c6c = DAT_00622c8c;
                                    }
                                }
                            }
                            pf3 = pf3.wrapping_add(3);
                            pi5 = pi5.wrapping_add(0x10);
                            local_60 += 1;
                            if !(local_60 < rd(param_2, 4) as i32) {
                                break;
                            }
                        }
                    }
                }
                3 => {
                    DAT_00527e88 = param_2 as *mut i32;
                    let pf3 = rd(param_2, 5)
                        .wrapping_sub(0x18)
                        .wrapping_add(rd(param_2, 4).wrapping_mul(0xc))
                        as *const f32;
                    let f = |i: usize| pf3.add(i).read_unaligned();
                    if DAT_00622c50[0] < f(0) && DAT_00622c60[0] < f(0) {
                        return 1;
                    }
                    if DAT_00622c50[1] < f(1) && DAT_00622c60[1] < f(1) {
                        return 1;
                    }
                    if DAT_00622c50[2] < f(2) && DAT_00622c60[2] < f(2) {
                        return 1;
                    }
                    if f(3) < DAT_00622c50[0] && f(3) < DAT_00622c60[0] {
                        return 1;
                    }
                    if f(4) < DAT_00622c50[1] && f(4) < DAT_00622c60[1] {
                        return 1;
                    }
                    if f(5) < DAT_00622c50[2] && f(5) < DAT_00622c60[2] {
                        return 1;
                    }
                }
                0x15 => {
                    local_64 = 1;
                }
                0x17 => {
                    if (DAT_00622c90 & 0x7fffffff) != 0 {
                        let pf3 = rd(param_2, 5) as *const f32;
                        let f = |i: usize| pf3.add(i).read_unaligned();
                        DAT_00622c50[0] = DAT_00622c50[0] - f(0);
                        DAT_00622c50[1] = DAT_00622c50[1] - f(1);
                        DAT_00622c50[2] = DAT_00622c50[2] - f(2);
                        DAT_00622c60[0] = DAT_00622c60[0] - f(0);
                        DAT_00622c60[1] = DAT_00622c60[1] - f(1);
                        DAT_00622c60[2] = DAT_00622c60[2] - f(2);
                        let mut local_50 = [0.0f32; 4];
                        local_50[0] = f(3) * DAT_00518208;
                        local_50[1] = f(4) * DAT_00518208;
                        local_50[2] = f(5) * DAT_00518208;
                        local_50[3] = f32::from_bits(DAT_00622c90);
                        let mut local_40 = [0.0f32; 16];
                        FUN_00421300(local_40.as_mut_ptr(), local_50.as_mut_ptr());
                        FUN_00420fe0(&raw mut DAT_00622c50 as *mut f32, local_40.as_mut_ptr());
                        FUN_00420fe0(&raw mut DAT_00622c60 as *mut f32, local_40.as_mut_ptr());
                        DAT_00622c50[0] = f(0) + DAT_00622c50[0];
                        DAT_00622c50[1] = f(1) + DAT_00622c50[1];
                        DAT_00622c50[2] = f(2) + DAT_00622c50[2];
                        DAT_00622c60[0] = f(0) + DAT_00622c60[0];
                        DAT_00622c60[1] = f(1) + DAT_00622c60[1];
                        DAT_00622c60[2] = f(2) + DAT_00622c60[2];
                    }
                }
                _ => {}
            }
            if rd(param_2, 2) == 0 {
                return 0;
            }
            if local_64 == 0 {
                let mut i_var6: i32 = 0;
                if 0 < rd(param_2, 2) as i32 {
                    let mut i_var4: u32 = 0;
                    loop {
                        let child = rd(param_2, 3).wrapping_add(i_var4) as *const u32;
                        let i_var2 =
                            FUN_004a6080(param_1, child.read_unaligned() as *mut u32) as i32;
                        if i_var2 != 0 {
                            return 1;
                        }
                        i_var4 = i_var4.wrapping_add(4);
                        i_var6 += 1;
                        if !(i_var6 < rd(param_2, 2) as i32) {
                            break;
                        }
                    }
                }
                return 0;
            }
            if local_64 == -1 {
                return 0;
            }
            let addr = rd(param_2, 3)
                .wrapping_sub(4)
                .wrapping_add((local_64 as u32).wrapping_mul(4))
                as *const u32;
            param_2 = addr.read_unaligned() as *mut u32;
        }
    }
}
