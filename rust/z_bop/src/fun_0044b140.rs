// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a6fe0: i16;
    static mut DAT_009d8c00: i16;
    static mut DAT_00a0813c: i16;
    static mut DAT_00a07ce4: i16;
    static mut DAT_005233f4: i32;
    static mut DAT_005233d8: i32;
    static mut DAT_00a00530: [u16; 256];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040e1a0() -> i32;
    fn FUN_00425ae0(a: i32, b: i32, c: i32) -> i32;
}

#[inline(always)]
fn addr(base: i32, row: i32, y: i32, x: i32, dx: i32) -> *mut u16 {
    (base as u32)
        .wrapping_add((row as u32).wrapping_mul(y as u32))
        .wrapping_add((x as u32).wrapping_mul(dx as u32)) as *mut u16
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044b140(
    mut param_1: i32,
    mut param_2: i32,
    mut param_3: i32,
    mut param_4: i32,
    param_5: u8,
) {
    // SAFETY: globals and callees are the game's own; the pixel pointers
    // are computed and written exactly where the original does.
    unsafe {
        let mut i_var5 = param_4;
        let mut i_var6 = param_1;
        let mut i_var4 = DAT_005233f4;
        let u_var1: u16 = DAT_00a00530[param_5 as usize];
        let mut local_8: u32 = (param_3 as u32).wrapping_sub(param_1 as u32);
        if (local_8 as i32) < 0 {
            param_1 = param_3;
            local_8 = 0u32.wrapping_sub(local_8);
            param_4 = param_2;
            param_2 = i_var5;
            param_3 = i_var6;
        } else if local_8 == 0 {
            if param_1 < DAT_009a6fe0 as i32 {
                return;
            }
            if DAT_009d8c00 as i32 <= param_1 {
                return;
            }
            if param_4 < param_2 {
                param_4 = param_2;
                param_2 = i_var5;
            }
            if param_2 < DAT_00a0813c as i32 {
                param_2 = DAT_00a0813c as i32;
            }
            if DAT_00a07ce4 as i32 <= param_4 {
                param_4 = (DAT_00a07ce4 as i32).wrapping_sub(1);
            }
            param_4 = param_4.wrapping_sub(param_2);
            if param_4 < 1 {
                return;
            }
            i_var6 = FUN_0040e1a0();
            let mut p = addr(i_var4, i_var6, param_2, param_1, DAT_005233d8);
            if param_4 == 0 {
                return;
            }
            loop {
                param_4 = param_4.wrapping_sub(1);
                p.write_unaligned(u_var1);
                i_var4 = FUN_0040e1a0();
                p = (p as u32).wrapping_add(i_var4 as u32) as *mut u16;
                if param_4 == 0 {
                    break;
                }
            }
            return;
        }
        i_var6 = DAT_009d8c00 as i32;
        if param_1 < i_var6 {
            i_var5 = DAT_009a6fe0 as i32;
            if i_var5 <= param_3 {
                let u_var2: u32 = (param_4 as u32).wrapping_sub(param_2 as u32);
                let mut i_var8: i32 = (local_8 as i32) >> 1;
                if (u_var2 as i32) < 0 {
                    let u_var3: u32 = 0u32.wrapping_sub(u_var2);
                    if DAT_00a0813c as i32 <= param_2 {
                        i_var4 = DAT_00a07ce4 as i32;
                        if param_4 < i_var4 {
                            if i_var4 <= param_2 {
                                i_var4 = FUN_00425ae0(
                                    (param_2.wrapping_sub(i_var4) as u32).wrapping_add(1) as i32,
                                    local_8 as i32,
                                    u_var3 as i32,
                                );
                                param_1 = param_1.wrapping_add(i_var4);
                                if DAT_009d8c00 as i32 <= param_1 {
                                    return;
                                }
                                param_2 = (DAT_00a07ce4 as i32).wrapping_sub(1);
                            }
                            if param_1 < DAT_009a6fe0 as i32 {
                                i_var4 = FUN_00425ae0(
                                    (DAT_009a6fe0 as i32).wrapping_sub(param_1),
                                    u_var3 as i32,
                                    local_8 as i32,
                                );
                                param_2 = param_2.wrapping_sub(i_var4);
                                if param_2 < DAT_00a0813c as i32 {
                                    return;
                                }
                                param_1 = DAT_009a6fe0 as i32;
                            }
                            i_var4 = DAT_005233f4;
                            if DAT_009d8c00 as i32 <= param_3 {
                                param_3 = (DAT_009d8c00 as i32).wrapping_sub(1);
                            }
                            if param_4 < DAT_00a0813c as i32 {
                                param_4 = DAT_00a0813c as i32;
                            }
                            i_var6 = FUN_0040e1a0();
                            let mut p = addr(i_var4, i_var6, param_2, param_1, DAT_005233d8);
                            if (local_8 as i32) < (u_var3 as i32) {
                                i_var6 = (u_var3 as i32) >> 1;
                                i_var5 = param_3.wrapping_sub(param_1).wrapping_add(1);
                                i_var4 = param_2.wrapping_sub(param_4);
                                if i_var4 != 0 {
                                    loop {
                                        i_var4 = i_var4.wrapping_sub(1);
                                        p.write_unaligned(u_var1);
                                        i_var8 = FUN_0040e1a0();
                                        p = (p as u32).wrapping_sub(i_var8 as u32) as *mut u16;
                                        i_var6 = i_var6.wrapping_sub(local_8 as i32);
                                        if i_var6 < 0 {
                                            i_var6 = i_var6.wrapping_add(u_var3 as i32);
                                            i_var5 = i_var5.wrapping_sub(1);
                                            if i_var5 == 0 {
                                                return;
                                            }
                                            p = (p as u32).wrapping_add(DAT_005233d8 as u32)
                                                as *mut u16;
                                        }
                                        if i_var4 == 0 {
                                            break;
                                        }
                                    }
                                    return;
                                }
                            } else {
                                i_var6 = param_3.wrapping_sub(param_1);
                                i_var4 = param_2.wrapping_sub(param_4).wrapping_add(1);
                                if i_var6 != 0 {
                                    loop {
                                        i_var6 = i_var6.wrapping_sub(1);
                                        p.write_unaligned(u_var1);
                                        p = (p as u32).wrapping_add(DAT_005233d8 as u32)
                                            as *mut u16;
                                        i_var8 = i_var8.wrapping_add(u_var2 as i32);
                                        if i_var8 < 0 {
                                            i_var8 = i_var8.wrapping_add(local_8 as i32);
                                            i_var4 = i_var4.wrapping_sub(1);
                                            if i_var4 == 0 {
                                                return;
                                            }
                                            i_var5 = FUN_0040e1a0();
                                            p = (p as u32).wrapping_sub(i_var5 as u32) as *mut u16;
                                        }
                                        if i_var6 == 0 {
                                            break;
                                        }
                                    }
                                    return;
                                }
                            }
                        }
                    }
                } else if (u_var2 as i32) < 1 {
                    if DAT_00a0813c as i32 <= param_2 && param_2 < DAT_00a07ce4 as i32 {
                        if param_1 < i_var5 {
                            param_1 = i_var5;
                        }
                        if i_var6 <= param_3 {
                            param_3 = i_var6.wrapping_sub(1);
                        }
                        i_var6 = param_3.wrapping_sub(param_1);
                        if 0 < i_var6 {
                            i_var5 = FUN_0040e1a0();
                            let mut p = addr(i_var4, i_var5, param_2, param_1, DAT_005233d8);
                            while i_var6 != 0 {
                                p.write_unaligned(u_var1);
                                p = (p as u32).wrapping_add(DAT_005233d8 as u32) as *mut u16;
                                i_var6 = i_var6.wrapping_sub(1);
                            }
                        }
                    }
                } else if param_2 < DAT_00a07ce4 as i32 {
                    i_var4 = DAT_00a0813c as i32;
                    if i_var4 <= param_4 {
                        if param_2 < i_var4 {
                            i_var4 = FUN_00425ae0(
                                i_var4.wrapping_sub(param_2),
                                local_8 as i32,
                                u_var2 as i32,
                            );
                            param_1 = param_1.wrapping_add(i_var4);
                            if DAT_009d8c00 as i32 <= param_1 {
                                return;
                            }
                            param_2 = DAT_00a0813c as i32;
                        }
                        if param_1 < DAT_009a6fe0 as i32 {
                            i_var4 = FUN_00425ae0(
                                (DAT_009a6fe0 as i32).wrapping_sub(param_1),
                                u_var2 as i32,
                                local_8 as i32,
                            );
                            param_2 = param_2.wrapping_add(i_var4);
                            if DAT_00a07ce4 as i32 <= param_2 {
                                return;
                            }
                            param_1 = DAT_009a6fe0 as i32;
                        }
                        i_var4 = DAT_005233f4;
                        if DAT_009d8c00 as i32 <= param_3 {
                            param_3 = (DAT_009d8c00 as i32).wrapping_sub(1);
                        }
                        if DAT_00a07ce4 as i32 <= param_4 {
                            param_4 = (DAT_00a07ce4 as i32).wrapping_sub(1);
                        }
                        i_var6 = FUN_0040e1a0();
                        let mut p = addr(i_var4, i_var6, param_2, param_1, DAT_005233d8);
                        if (local_8 as i32) < (u_var2 as i32) {
                            i_var4 = (u_var2 as i32) >> 1;
                            i_var6 = param_3.wrapping_sub(param_1).wrapping_add(1);
                            param_4 = param_4.wrapping_sub(param_2);
                            if param_4 != 0 {
                                loop {
                                    param_4 = param_4.wrapping_sub(1);
                                    p.write_unaligned(u_var1);
                                    i_var5 = FUN_0040e1a0();
                                    p = (p as u32).wrapping_add(i_var5 as u32) as *mut u16;
                                    i_var4 = i_var4.wrapping_sub(local_8 as i32);
                                    if i_var4 < 0 {
                                        i_var4 = i_var4.wrapping_add(u_var2 as i32);
                                        i_var6 = i_var6.wrapping_sub(1);
                                        if i_var6 == 0 {
                                            return;
                                        }
                                        p = (p as u32).wrapping_add(DAT_005233d8 as u32)
                                            as *mut u16;
                                    }
                                    if param_4 == 0 {
                                        break;
                                    }
                                }
                                return;
                            }
                        } else {
                            i_var4 = param_3.wrapping_sub(param_1);
                            param_4 = param_4.wrapping_sub(param_2).wrapping_add(1);
                            if i_var4 != 0 {
                                loop {
                                    i_var4 = i_var4.wrapping_sub(1);
                                    p.write_unaligned(u_var1);
                                    p = (p as u32).wrapping_add(DAT_005233d8 as u32) as *mut u16;
                                    i_var8 = i_var8.wrapping_sub(u_var2 as i32);
                                    if i_var8 < 0 {
                                        i_var8 = i_var8.wrapping_add(local_8 as i32);
                                        param_4 = param_4.wrapping_sub(1);
                                        if param_4 == 0 {
                                            return;
                                        }
                                        i_var6 = FUN_0040e1a0();
                                        p = (p as u32).wrapping_add(i_var6 as u32) as *mut u16;
                                    }
                                    if i_var4 == 0 {
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
