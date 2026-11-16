// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d9980: i16;
    static mut DAT_0070acc0: i16;
    static mut DAT_00732e8c: i16;
    static mut DAT_00732a34: i16;
    static mut DAT_0070acc8: i32;
    static mut DAT_00525534: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00478290(param_1: i32, param_2: u32, param_3: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00465320(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: u8,
) {
    let mut param_1 = param_1;
    let mut param_2 = param_2;
    let mut param_3 = param_3;
    // SAFETY: the globals are the game's own; the frame buffer pointer is
    // computed and written exactly where the original does.
    unsafe {
        let mut i_var3 = param_1;
        let mut local_4: u32 = (param_3 as u32).wrapping_sub(param_1 as u32);
        let mut i_var7: i32;
        if (local_4 as i32) < 0 {
            local_4 = 0u32.wrapping_sub(local_4);
            param_1 = param_3;
            i_var7 = param_2;
            param_2 = param_4;
            param_3 = i_var3;
        } else {
            i_var7 = param_4;
            if local_4 == 0 {
                if param_1 < DAT_006d9980 as i32 {
                    return;
                }
                if DAT_0070acc0 as i32 <= param_1 {
                    return;
                }
                i_var3 = param_4;
                if param_4 < param_2 {
                    i_var3 = param_2;
                    param_2 = param_4;
                }
                if param_2 < DAT_00732e8c as i32 {
                    param_2 = DAT_00732e8c as i32;
                }
                if DAT_00732a34 as i32 <= i_var3 {
                    i_var3 = (DAT_00732a34 as i32).wrapping_sub(1);
                }
                i_var3 = (i_var3 as u32).wrapping_sub(param_2 as u32) as i32;
                if i_var3 < 1 {
                    return;
                }
                let mut p = (DAT_0070acc8 as u32)
                    .wrapping_mul(param_2 as u32)
                    .wrapping_add(param_1 as u32)
                    .wrapping_add(DAT_00525534 as u32) as usize
                    as *mut u8;
                if i_var3 == 0 {
                    return;
                }
                loop {
                    i_var3 = i_var3.wrapping_sub(1);
                    p.write(param_5);
                    p = p.wrapping_add(DAT_0070acc8 as usize);
                    if i_var3 == 0 {
                        break;
                    }
                }
                return;
            }
        }
        i_var3 = DAT_0070acc0 as i32;
        if param_1 < i_var3 {
            let mut i_var1 = DAT_006d9980 as i32;
            if i_var1 <= param_3 {
                let u_var5: u32 = (i_var7 as u32).wrapping_sub(param_2 as u32);
                let mut i_var4: i32 = (local_4 as i32) >> 1;
                if (u_var5 as i32) < 0 {
                    let u_var6: u32 = 0u32.wrapping_sub(u_var5);
                    if DAT_00732e8c as i32 <= param_2 {
                        i_var3 = DAT_00732a34 as i32;
                        if i_var7 < i_var3 {
                            if i_var3 <= param_2 {
                                i_var3 = FUN_00478290(
                                    (param_2 as u32).wrapping_sub(i_var3 as u32).wrapping_add(1)
                                        as i32,
                                    local_4,
                                    u_var6,
                                );
                                param_1 = (param_1 as u32).wrapping_add(i_var3 as u32) as i32;
                                if DAT_0070acc0 as i32 <= param_1 {
                                    return;
                                }
                                param_2 = (DAT_00732a34 as i32).wrapping_sub(1);
                            }
                            if param_1 < DAT_006d9980 as i32 {
                                i_var3 = FUN_00478290(
                                    (DAT_006d9980 as i32 as u32).wrapping_sub(param_1 as u32)
                                        as i32,
                                    u_var6,
                                    local_4,
                                );
                                param_2 = (param_2 as u32).wrapping_sub(i_var3 as u32) as i32;
                                if param_2 < DAT_00732e8c as i32 {
                                    return;
                                }
                                param_1 = DAT_006d9980 as i32;
                            }
                            if DAT_0070acc0 as i32 <= param_3 {
                                param_3 = (DAT_0070acc0 as i32).wrapping_sub(1);
                            }
                            if i_var7 < DAT_00732e8c as i32 {
                                i_var7 = DAT_00732e8c as i32;
                            }
                            let mut p = (DAT_0070acc8 as u32)
                                .wrapping_mul(param_2 as u32)
                                .wrapping_add(param_1 as u32)
                                .wrapping_add(DAT_00525534 as u32)
                                as usize as *mut u8;
                            if 0u32.wrapping_sub(local_4) == u_var5
                                || (u_var6 as i32) < (local_4 as i32)
                            {
                                i_var1 = (param_3 as u32).wrapping_sub(param_1 as u32) as i32;
                                i_var3 =
                                    (param_2 as u32).wrapping_sub(i_var7 as u32).wrapping_add(1)
                                        as i32;
                                if i_var1 != 0 {
                                    loop {
                                        i_var1 = i_var1.wrapping_sub(1);
                                        p.write(param_5);
                                        p = p.wrapping_add(1);
                                        i_var4 = (i_var4 as u32).wrapping_add(u_var5) as i32;
                                        if i_var4 < 0 {
                                            i_var4 = (i_var4 as u32).wrapping_add(local_4) as i32;
                                            i_var3 = i_var3.wrapping_sub(1);
                                            if i_var3 == 0 {
                                                return;
                                            }
                                            p = p.wrapping_sub(DAT_0070acc8 as usize);
                                        }
                                        if i_var1 == 0 {
                                            break;
                                        }
                                    }
                                    return;
                                }
                            } else {
                                i_var3 = (u_var6 as i32) >> 1;
                                i_var1 = (param_3 as u32)
                                    .wrapping_sub(param_1 as u32)
                                    .wrapping_add(1)
                                    as i32;
                                i_var7 = (param_2 as u32).wrapping_sub(i_var7 as u32) as i32;
                                if i_var7 != 0 {
                                    loop {
                                        i_var7 = i_var7.wrapping_sub(1);
                                        p.write(param_5);
                                        p = p.wrapping_sub(DAT_0070acc8 as usize);
                                        i_var3 = (i_var3 as u32).wrapping_sub(local_4) as i32;
                                        if i_var3 < 0 {
                                            i_var3 = (i_var3 as u32).wrapping_add(u_var6) as i32;
                                            i_var1 = i_var1.wrapping_sub(1);
                                            if i_var1 == 0 {
                                                return;
                                            }
                                            p = p.wrapping_add(1);
                                        }
                                        if i_var7 == 0 {
                                            break;
                                        }
                                    }
                                    return;
                                }
                            }
                        }
                    }
                } else if (u_var5 as i32) < 1 {
                    if DAT_00732e8c as i32 <= param_2 && param_2 < DAT_00732a34 as i32 {
                        if param_1 < i_var1 {
                            param_1 = i_var1;
                        }
                        if i_var3 <= param_3 {
                            param_3 = i_var3.wrapping_sub(1);
                        }
                        i_var3 = (param_3 as u32).wrapping_sub(param_1 as u32) as i32;
                        if 0 < i_var3 {
                            let mut p = (DAT_0070acc8 as u32)
                                .wrapping_mul(param_2 as u32)
                                .wrapping_add(param_1 as u32)
                                .wrapping_add(DAT_00525534 as u32)
                                as usize as *mut u8;
                            while i_var3 != 0 {
                                p.write(param_5);
                                p = p.wrapping_add(1);
                                i_var3 = i_var3.wrapping_sub(1);
                            }
                        }
                    }
                } else if param_2 < DAT_00732a34 as i32 {
                    i_var3 = DAT_00732e8c as i32;
                    if i_var3 <= i_var7 {
                        if param_2 < i_var3 {
                            i_var3 = FUN_00478290(
                                (i_var3 as u32).wrapping_sub(param_2 as u32) as i32,
                                local_4,
                                u_var5,
                            );
                            param_1 = (param_1 as u32).wrapping_add(i_var3 as u32) as i32;
                            if DAT_0070acc0 as i32 <= param_1 {
                                return;
                            }
                            param_2 = DAT_00732e8c as i32;
                        }
                        if param_1 < DAT_006d9980 as i32 {
                            i_var3 = FUN_00478290(
                                (DAT_006d9980 as i32 as u32).wrapping_sub(param_1 as u32) as i32,
                                u_var5,
                                local_4,
                            );
                            param_2 = (param_2 as u32).wrapping_add(i_var3 as u32) as i32;
                            if DAT_00732a34 as i32 <= param_2 {
                                return;
                            }
                            param_1 = DAT_006d9980 as i32;
                        }
                        if DAT_0070acc0 as i32 <= param_3 {
                            param_3 = (DAT_0070acc0 as i32).wrapping_sub(1);
                        }
                        if DAT_00732a34 as i32 <= i_var7 {
                            i_var7 = (DAT_00732a34 as i32).wrapping_sub(1);
                        }
                        let mut p = (DAT_0070acc8 as u32)
                            .wrapping_mul(param_2 as u32)
                            .wrapping_add(param_1 as u32)
                            .wrapping_add(DAT_00525534 as u32)
                            as usize as *mut u8;
                        if (local_4 as i32) < (u_var5 as i32) {
                            i_var3 = (u_var5 as i32) >> 1;
                            i_var1 = (param_3 as u32)
                                .wrapping_sub(param_1 as u32)
                                .wrapping_add(1) as i32;
                            i_var7 = (i_var7 as u32).wrapping_sub(param_2 as u32) as i32;
                            if i_var7 != 0 {
                                loop {
                                    i_var7 = i_var7.wrapping_sub(1);
                                    p.write(param_5);
                                    p = p.wrapping_add(DAT_0070acc8 as usize);
                                    i_var3 = (i_var3 as u32).wrapping_sub(local_4) as i32;
                                    if i_var3 < 0 {
                                        i_var3 = (i_var3 as u32).wrapping_add(u_var5) as i32;
                                        i_var1 = i_var1.wrapping_sub(1);
                                        if i_var1 == 0 {
                                            return;
                                        }
                                        p = p.wrapping_add(1);
                                    }
                                    if i_var7 == 0 {
                                        break;
                                    }
                                }
                                return;
                            }
                        } else {
                            i_var1 = (param_3 as u32).wrapping_sub(param_1 as u32) as i32;
                            i_var3 =
                                (i_var7 as u32).wrapping_sub(param_2 as u32).wrapping_add(1) as i32;
                            if i_var1 != 0 {
                                loop {
                                    i_var1 = i_var1.wrapping_sub(1);
                                    p.write(param_5);
                                    p = p.wrapping_add(1);
                                    i_var4 = (i_var4 as u32).wrapping_sub(u_var5) as i32;
                                    if i_var4 < 0 {
                                        i_var4 = (i_var4 as u32).wrapping_add(local_4) as i32;
                                        i_var3 = i_var3.wrapping_sub(1);
                                        if i_var3 == 0 {
                                            return;
                                        }
                                        p = p.wrapping_add(DAT_0070acc8 as usize);
                                    }
                                    if i_var1 == 0 {
                                        break;
                                    }
                                }
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
}
