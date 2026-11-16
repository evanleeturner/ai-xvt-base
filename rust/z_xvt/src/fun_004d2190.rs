// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758830: i32;
    static mut DAT_0075964b: i32;
    static mut DAT_00759657: i32;
    static mut DAT_0075dc46: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dd000(param_1: *mut i32, param_2: *mut i32);
    fn FUN_004dd0b0(param_1: *mut i32) -> u32;
    fn FUN_004df890(param_1: u32) -> u32;
    fn FUN_004d23e0(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
        param_8: i8,
    );
    fn FUN_004d2700(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
        param_8: i16,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d2190(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: u32,
    param_5: i32,
) -> u32 {
    let mut param_4 = param_4;
    if param_1.is_null() {
        return 0;
    }
    // SAFETY: param_1 points at the caller's surface descriptor; the globals
    // and callees are the game's own; pixel pointers are computed and
    // accessed exactly where the original does.
    unsafe {
        let mut local_20: [i32; 4] = [0; 4];
        let mut local_10: [i32; 4] = [0; 4];
        let p0 = param_1.read_unaligned();
        let p1 = param_1.add(1).read_unaligned();
        local_20[0] = param_2;
        local_20[1] = param_3;
        local_20[2] = p0.wrapping_add(param_2).wrapping_sub(1);
        local_20[3] = p1.wrapping_add(param_3).wrapping_sub(1);
        FUN_004dd000(local_10.as_mut_ptr(), local_20.as_mut_ptr());
        let u_var2 = FUN_004dd0b0(local_20.as_mut_ptr());
        if local_20[0] <= local_20[2] {
            if local_20[3] < local_20[1] {
                return u_var2;
            }
            local_10[0] = local_20[0].wrapping_sub(local_10[0]);
            local_10[1] = local_20[1].wrapping_sub(local_10[1]);
            let i_var1 = local_20[2]
                .wrapping_sub(local_10[0])
                .wrapping_sub(local_10[2])
                .wrapping_add(p0);
            let mut i_var5 = p1
                .wrapping_sub(local_10[1])
                .wrapping_sub(local_10[3])
                .wrapping_add(local_20[3]);
            let g_59657: i32 = (&raw mut DAT_00759657).read_unaligned();
            let g_5964b: i32 = (&raw mut DAT_0075964b).read_unaligned();
            if param_1.add(2).read_unaligned() == 0 {
                if g_59657 == 8 {
                    let mut i_var3 = local_10[1]
                        .wrapping_mul(p0)
                        .wrapping_add(param_1.add(4).read_unaligned())
                        .wrapping_add(local_10[0]);
                    let mut i_var4 = param_3
                        .wrapping_add(local_10[1])
                        .wrapping_mul(g_5964b)
                        .wrapping_add(DAT_00758830)
                        .wrapping_add(local_10[0])
                        .wrapping_add(param_2);
                    if 0 < i_var5 {
                        loop {
                            let mut i_var6: i32 = 0;
                            if 0 < i_var1 {
                                loop {
                                    if *((i_var3.wrapping_add(i_var6)) as *const u8) != 0 {
                                        *((i_var4.wrapping_add(i_var6)) as *mut u8) = param_4 as u8;
                                    }
                                    i_var6 = i_var6.wrapping_add(1);
                                    if !(i_var6 < i_var1) {
                                        break;
                                    }
                                }
                            }
                            i_var3 = i_var3.wrapping_add(param_1.read_unaligned());
                            i_var4 = i_var4.wrapping_add(g_5964b);
                            i_var5 = i_var5.wrapping_sub(1);
                            if i_var5 == 0 {
                                break;
                            }
                        }
                    }
                } else if g_59657 == 0x10 {
                    let g_5dc46: i32 = (&raw mut DAT_0075dc46).read_unaligned();
                    if param_5 != 0 && g_5dc46 != 0 {
                        param_4 = FUN_004df890(param_4);
                    }
                    let mut i_var3 = p0
                        .wrapping_mul(local_10[1])
                        .wrapping_add(param_1.add(4).read_unaligned())
                        .wrapping_add(local_10[0]);
                    let mut i_var4 = param_3
                        .wrapping_add(local_10[1])
                        .wrapping_mul(g_5964b)
                        .wrapping_add(local_10[0].wrapping_add(param_2).wrapping_mul(2))
                        .wrapping_add(DAT_00758830);
                    if 0 < i_var5 {
                        loop {
                            let mut i_var6: i32 = 0;
                            if 0 < i_var1 {
                                loop {
                                    if *((i_var3.wrapping_add(i_var6)) as *const u8) != 0 {
                                        ((i_var4.wrapping_add(i_var6.wrapping_mul(2))) as *mut i16)
                                            .write_unaligned(param_4 as i16);
                                    }
                                    i_var6 = i_var6.wrapping_add(1);
                                    if !(i_var6 < i_var1) {
                                        break;
                                    }
                                }
                            }
                            i_var3 = i_var3.wrapping_add(param_1.read_unaligned());
                            i_var4 = i_var4.wrapping_add(((g_5964b as u32) & 0xffff_fffe) as i32);
                            i_var5 = i_var5.wrapping_sub(1);
                            if i_var5 == 0 {
                                break;
                            }
                        }
                    }
                }
            } else if g_59657 == 8 {
                FUN_004d23e0(
                    param_1,
                    local_10[0].wrapping_add(param_2),
                    param_3.wrapping_add(local_10[1]),
                    local_10[0],
                    local_10[1],
                    i_var1,
                    i_var5,
                    param_4 as i8,
                );
            } else if g_59657 == 0x10 {
                let g_5dc46: i32 = (&raw mut DAT_0075dc46).read_unaligned();
                if param_5 != 0 && g_5dc46 != 0 {
                    param_4 = FUN_004df890(param_4);
                }
                FUN_004d2700(
                    param_1,
                    local_10[0].wrapping_add(param_2),
                    param_3.wrapping_add(local_10[1]),
                    local_10[0],
                    local_10[1],
                    i_var1,
                    i_var5,
                    param_4 as i16,
                );
            }
        }
        u_var2
    }
}
