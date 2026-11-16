// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a6fe0: i16;
    static mut DAT_009d8c00: i16;
    static mut DAT_00a0813c: i16;
    static mut DAT_00a07ce4: i16;
    static mut DAT_009d8c08: i32;
    static mut DAT_005233f4: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00425ae0(a: i32, b: u32, c: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00411120(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: u8,
) {
    let mut param_1 = param_1;
    let mut param_2 = param_2;
    let mut param_3 = param_3;
    // SAFETY: the globals are the game's own; the pixel pointer addresses the
    // game's frame buffer, written where the original writes.
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
                if param_1 < DAT_009a6fe0 as i32 {
                    return;
                }
                if DAT_009d8c00 as i32 <= param_1 {
                    return;
                }
                i_var3 = param_4;
                if param_4 < param_2 {
                    i_var3 = param_2;
                    param_2 = param_4;
                }
                if param_2 < DAT_00a0813c as i32 {
                    param_2 = DAT_00a0813c as i32;
                }
                if DAT_00a07ce4 as i32 <= i_var3 {
                    i_var3 = (DAT_00a07ce4 as i32 as u32).wrapping_sub(1) as i32;
                }
                i_var3 = (i_var3 as u32).wrapping_sub(param_2 as u32) as i32;
                if i_var3 < 1 {
                    return;
                }
                let mut p = (param_1 as u32)
                    .wrapping_add((param_2 as u32).wrapping_mul(DAT_009d8c08 as u32))
                    .wrapping_add(DAT_005233f4 as u32) as *mut u8;
                if i_var3 == 0 {
                    return;
                }
                loop {
                    i_var3 = (i_var3 as u32).wrapping_sub(1) as i32;
                    p.write(param_5);
                    p = p.wrapping_offset(DAT_009d8c08 as isize);
                    if i_var3 == 0 {
                        break;
                    }
                }
                return;
            }
        }
        i_var3 = DAT_009d8c00 as i32;
        if param_1 < i_var3 {
            let mut i_var1 = DAT_009a6fe0 as i32;
            if i_var1 <= param_3 {
                let u_var5: u32 = (i_var7 as u32).wrapping_sub(param_2 as u32);
                let mut i_var4: i32 = (local_4 as i32) >> 1;
                if (u_var5 as i32) < 0 {
                    let u_var6: u32 = 0u32.wrapping_sub(u_var5);
                    if DAT_00a0813c as i32 <= param_2 && {
                        i_var3 = DAT_00a07ce4 as i32;
                        i_var7 < i_var3
                    } {
                        if i_var3 <= param_2 {
                            i_var3 = FUN_00425ae0(
                                (param_2 as u32).wrapping_sub(i_var3 as u32).wrapping_add(1) as i32,
                                local_4,
                                u_var6,
                            );
                            param_1 = (param_1 as u32).wrapping_add(i_var3 as u32) as i32;
                            if DAT_009d8c00 as i32 <= param_1 {
                                return;
                            }
                            param_2 = (DAT_00a07ce4 as i32 as u32).wrapping_sub(1) as i32;
                        }
                        if param_1 < DAT_009a6fe0 as i32 {
                            i_var3 = FUN_00425ae0(
                                (DAT_009a6fe0 as i32 as u32).wrapping_sub(param_1 as u32) as i32,
                                u_var6,
                                local_4,
                            );
                            param_2 = (param_2 as u32).wrapping_sub(i_var3 as u32) as i32;
                            if param_2 < DAT_00a0813c as i32 {
                                return;
                            }
                            param_1 = DAT_009a6fe0 as i32;
                        }
                        if DAT_009d8c00 as i32 <= param_3 {
                            param_3 = (DAT_009d8c00 as i32 as u32).wrapping_sub(1) as i32;
                        }
                        if i_var7 < DAT_00a0813c as i32 {
                            i_var7 = DAT_00a0813c as i32;
                        }
                        let mut p = (param_1 as u32)
                            .wrapping_add((param_2 as u32).wrapping_mul(DAT_009d8c08 as u32))
                            .wrapping_add(DAT_005233f4 as u32)
                            as *mut u8;
                        if 0u32.wrapping_sub(local_4) == u_var5
                            || (u_var6 as i32) < (local_4 as i32)
                        {
                            i_var1 = (param_3 as u32).wrapping_sub(param_1 as u32) as i32;
                            i_var3 =
                                (param_2 as u32).wrapping_sub(i_var7 as u32).wrapping_add(1) as i32;
                            if i_var1 != 0 {
                                loop {
                                    i_var1 = (i_var1 as u32).wrapping_sub(1) as i32;
                                    p.write(param_5);
                                    p = p.wrapping_offset(1);
                                    i_var4 = (i_var4 as u32).wrapping_add(u_var5) as i32;
                                    if i_var4 < 0 {
                                        i_var4 = (i_var4 as u32).wrapping_add(local_4) as i32;
                                        i_var3 = (i_var3 as u32).wrapping_sub(1) as i32;
                                        if i_var3 == 0 {
                                            return;
                                        }
                                        p = p.wrapping_offset(
                                            (DAT_009d8c08 as isize).wrapping_neg(),
                                        );
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
                                .wrapping_add(1) as i32;
                            i_var7 = (param_2 as u32).wrapping_sub(i_var7 as u32) as i32;
                            if i_var7 != 0 {
                                loop {
                                    i_var7 = (i_var7 as u32).wrapping_sub(1) as i32;
                                    p.write(param_5);
                                    p = p.wrapping_offset((DAT_009d8c08 as isize).wrapping_neg());
                                    i_var3 = (i_var3 as u32).wrapping_sub(local_4) as i32;
                                    if i_var3 < 0 {
                                        i_var3 = (i_var3 as u32).wrapping_add(u_var6) as i32;
                                        i_var1 = (i_var1 as u32).wrapping_sub(1) as i32;
                                        if i_var1 == 0 {
                                            return;
                                        }
                                        p = p.wrapping_offset(1);
                                    }
                                    if i_var7 == 0 {
                                        break;
                                    }
                                }
                                return;
                            }
                        }
                    }
                } else if (u_var5 as i32) < 1 {
                    if DAT_00a0813c as i32 <= param_2 && param_2 < DAT_00a07ce4 as i32 {
                        if param_1 < i_var1 {
                            param_1 = i_var1;
                        }
                        if i_var3 <= param_3 {
                            param_3 = (i_var3 as u32).wrapping_sub(1) as i32;
                        }
                        i_var3 = (param_3 as u32).wrapping_sub(param_1 as u32) as i32;
                        if 0 < i_var3 {
                            let mut p = (param_1 as u32)
                                .wrapping_add((param_2 as u32).wrapping_mul(DAT_009d8c08 as u32))
                                .wrapping_add(DAT_005233f4 as u32)
                                as *mut u8;
                            while i_var3 != 0 {
                                p.write(param_5);
                                p = p.wrapping_offset(1);
                                i_var3 = (i_var3 as u32).wrapping_sub(1) as i32;
                            }
                        }
                    }
                } else if param_2 < DAT_00a07ce4 as i32 && {
                    i_var3 = DAT_00a0813c as i32;
                    i_var3 <= i_var7
                } {
                    if param_2 < i_var3 {
                        i_var3 = FUN_00425ae0(
                            (i_var3 as u32).wrapping_sub(param_2 as u32) as i32,
                            local_4,
                            u_var5,
                        );
                        param_1 = (param_1 as u32).wrapping_add(i_var3 as u32) as i32;
                        if DAT_009d8c00 as i32 <= param_1 {
                            return;
                        }
                        param_2 = DAT_00a0813c as i32;
                    }
                    if param_1 < DAT_009a6fe0 as i32 {
                        i_var3 = FUN_00425ae0(
                            (DAT_009a6fe0 as i32 as u32).wrapping_sub(param_1 as u32) as i32,
                            u_var5,
                            local_4,
                        );
                        param_2 = (param_2 as u32).wrapping_add(i_var3 as u32) as i32;
                        if DAT_00a07ce4 as i32 <= param_2 {
                            return;
                        }
                        param_1 = DAT_009a6fe0 as i32;
                    }
                    if DAT_009d8c00 as i32 <= param_3 {
                        param_3 = (DAT_009d8c00 as i32 as u32).wrapping_sub(1) as i32;
                    }
                    if DAT_00a07ce4 as i32 <= i_var7 {
                        i_var7 = (DAT_00a07ce4 as i32 as u32).wrapping_sub(1) as i32;
                    }
                    let mut p = (param_1 as u32)
                        .wrapping_add((param_2 as u32).wrapping_mul(DAT_009d8c08 as u32))
                        .wrapping_add(DAT_005233f4 as u32)
                        as *mut u8;
                    if (local_4 as i32) < (u_var5 as i32) {
                        i_var3 = (u_var5 as i32) >> 1;
                        i_var1 = (param_3 as u32)
                            .wrapping_sub(param_1 as u32)
                            .wrapping_add(1) as i32;
                        i_var7 = (i_var7 as u32).wrapping_sub(param_2 as u32) as i32;
                        if i_var7 != 0 {
                            loop {
                                i_var7 = (i_var7 as u32).wrapping_sub(1) as i32;
                                p.write(param_5);
                                p = p.wrapping_offset(DAT_009d8c08 as isize);
                                i_var3 = (i_var3 as u32).wrapping_sub(local_4) as i32;
                                if i_var3 < 0 {
                                    i_var3 = (i_var3 as u32).wrapping_add(u_var5) as i32;
                                    i_var1 = (i_var1 as u32).wrapping_sub(1) as i32;
                                    if i_var1 == 0 {
                                        return;
                                    }
                                    p = p.wrapping_offset(1);
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
                                i_var1 = (i_var1 as u32).wrapping_sub(1) as i32;
                                p.write(param_5);
                                p = p.wrapping_offset(1);
                                i_var4 = (i_var4 as u32).wrapping_sub(u_var5) as i32;
                                if i_var4 < 0 {
                                    i_var4 = (i_var4 as u32).wrapping_add(local_4) as i32;
                                    i_var3 = (i_var3 as u32).wrapping_sub(1) as i32;
                                    if i_var3 == 0 {
                                        return;
                                    }
                                    p = p.wrapping_offset(DAT_009d8c08 as isize);
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
