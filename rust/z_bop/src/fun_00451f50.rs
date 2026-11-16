// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a07be6: i16;
    static mut DAT_00a07be2: i16;
    static mut DAT_00a07be4: i16;
    static mut DAT_009e9640: i32;
    static mut DAT_00a00854: i32;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1fe8: *mut u8;
    static mut DAT_009d8c88: u8;
    static mut DAT_009d8c80: u8;
    static mut DAT_009d7188: u8;
    static mut DAT_009e967c: u8;
    static mut DAT_009e967e: u8;
    static mut DAT_009e9670: u8;
    static mut DAT_009fd458: u8;
    static mut DAT_009d8e25: i8;
    static mut DAT_009d8e29: i8;
    static mut DAT_009a2196: i8;
    static mut DAT_009a210a: i8;
    static mut DAT_009a8080: u8;
    static mut DAT_009a8081: u8;
    static mut DAT_009a8082: u8;
    static mut DAT_009a1ed0: i8;
    static mut DAT_009d8b8a: u8;
    static mut DAT_009ecc34: i32;
    static mut DAT_009d7694: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004525a0(a: u32, b: i32, c: *mut i8);
    fn FUN_00451bf0(a: i32, b: *mut i8);
    fn FUN_00432fa0(a: u16, b: u16, c: u16) -> i16;
    fn FUN_00404670(a: i32, b: i32, c: i16) -> i32;
    fn FUN_00450650(a: i32, b: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00451f50(
    param_1: u32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: u32,
    param_6: i32,
) -> u32 {
    let _ = (param_5, param_6);
    // SAFETY: all globals and pointers are the game's own memory; reads and
    // writes occur at the same offsets and in the same order as the original.
    unsafe {
        DAT_00a07be6 = 0x14b;
        let u_var13: u32 = param_1 & 0xffff;
        let mut local_8: u32 = 0;
        if DAT_009e9640 <= u_var13 as i32 && (u_var13 as i32) < DAT_00a00854 {
            return 0;
        }
        let local_24: *mut u8;
        if (u_var13 as i32) < DAT_009a1fe0 {
            let p_tmp: *mut u8 = DAT_009a1fe8
                .wrapping_add(0x1f)
                .wrapping_add(u_var13.wrapping_mul(0x23) as usize)
                .cast::<*mut u8>()
                .read_unaligned();
            local_24 = p_tmp.wrapping_add(0xa9).cast::<*mut u8>().read_unaligned();
        } else {
            local_24 = core::ptr::null_mut();
        }
        let i_var15: u32 = u_var13.wrapping_mul(0x23);
        let i_var16: *mut u8 = DAT_009a1fe8.wrapping_add(i_var15 as usize);
        let b_var1: u8 = i_var16.add(0x16).read();
        let mut u_var19: u32 = b_var1 as u32;
        let i_var6: usize = (param_2 as u32).wrapping_mul(0x5bd) as usize;
        let i_var5: usize = u_var19.wrapping_mul(0x566) as usize;
        let p_8c88 = (&raw mut DAT_009d8c88).cast::<u8>();
        let p_967c = (&raw mut DAT_009e967c).cast::<u8>();
        let b_var2: u8 = p_8c88.wrapping_add(i_var5).read();
        let mut us: u16 = p_967c.wrapping_add(i_var6).cast::<u16>().read_unaligned();
        let mut u_var18: u32 = (&raw mut DAT_009d7188)
            .cast::<u8>()
            .wrapping_add(u_var19.wrapping_add((us as u32).wrapping_mul(0x30)) as usize)
            .read() as u32;
        if u_var18 == 0 {
            if local_24.is_null() {
                if i_var16.add(2).read() == 8 {
                    u_var18 = 0x11;
                } else {
                    let b_var3 = i_var16.add(3).read();
                    if b_var3 < 0x46 || 0x4a < b_var3 {
                        if b_var3 < 0x50 || 0x52 < b_var3 {
                            if 0x52 < b_var3 && b_var3 < 0x55 {
                                u_var18 = 0x14;
                            }
                        } else {
                            u_var18 = 0x13;
                        }
                    } else {
                        u_var18 = 0x12;
                    }
                }
            } else {
                us = (&raw mut DAT_009e967e)
                    .cast::<u8>()
                    .wrapping_add(i_var6)
                    .cast::<u16>()
                    .read_unaligned();
                if us as u32 == u_var19 {
                    u_var18 = 0xe;
                } else {
                    us = p_967c.wrapping_add(i_var6).cast::<u16>().read_unaligned();
                    if b_var2 as u32 == us as u32 {
                        u_var18 = 0xd;
                    } else if i_var16.add(2).read() != 3 {
                        u_var18 = 0xf;
                    } else {
                        u_var18 = 0x10;
                        if local_24.wrapping_add(0x92).cast::<i16>().read_unaligned() != 0 {
                            u_var18 = 0xf;
                        }
                    }
                }
            }
        }
        FUN_004525a0(param_1, 2, &raw mut DAT_009a1ed0);
        FUN_00451bf0(0, &raw mut DAT_009a1ed0);
        DAT_00a07be2 = 0x14b;
        if u_var18 == 0 {
            DAT_00a07be4 = 0x14b;
        } else {
            if ((u_var18.wrapping_add(0x5240b8)) as *const i8).read() != 0 {
                us = p_967c.wrapping_add(i_var6).cast::<u16>().read_unaligned();
                let u_var17 = us as u32;
                if b_var2 as u32 == u_var17 {
                    DAT_00a07be2 = 0x14e;
                } else {
                    let g = DAT_009a1fe8
                        .wrapping_add(0x16)
                        .wrapping_add(i_var15 as usize)
                        .read() as u32;
                    let idx = p_8c88.wrapping_add(g.wrapping_mul(0x566) as usize).read() as u32;
                    let b_var21: i32 = if u_var17 == idx {
                        0
                    } else {
                        ((&raw mut DAT_009fd458)
                            .cast::<u8>()
                            .wrapping_add(idx.wrapping_add(u_var17.wrapping_mul(0x1e5)) as usize)
                            .read()
                            == 0) as i32
                    };
                    DAT_00a07be2 = 0x14d;
                    if b_var21 != 0 {
                        DAT_00a07be2 = 0x14c;
                    }
                }
            }
            DAT_00a07be4 = (u_var18 as i16).wrapping_add(0x135);
        }
        let mut b_var7: i32 = 0;
        let mut b_var11: i32 = 0;
        let mut b_var10: i32 = 0;
        u_var18 = 0;
        let mut pc14: *const i8 = (&raw mut DAT_009d8e25).cast::<i8>().wrapping_add(i_var5);
        let mut b_var8: i32 = 0;
        let mut b_var9: i32 = 0;
        let mut b_var21: i32 = 0;
        us = p_967c.wrapping_add(i_var6).cast::<u16>().read_unaligned();
        let mut pc20: *const i8 = (&raw mut DAT_009d8e29)
            .cast::<i8>()
            .wrapping_add(us as usize)
            .wrapping_add(i_var5);
        loop {
            us = p_967c.wrapping_add(i_var6).cast::<u16>().read_unaligned();
            if pc20.read() != 0
                && pc14.read() == 0
                && (&raw mut DAT_009a2196)
                    .cast::<i8>()
                    .wrapping_add(
                        u_var19
                            .wrapping_mul(0x126)
                            .wrapping_add(u_var18)
                            .wrapping_add((us as u32).wrapping_mul(8))
                            as usize,
                    )
                    .read()
                    == 4
            {
                if pc14.add(2).read() == 6 {
                    if (&raw mut DAT_009a210a)
                        .cast::<i8>()
                        .wrapping_add(u_var19.wrapping_mul(0x126) as usize)
                        .read()
                        == 0
                    {
                        b_var7 = 1;
                    }
                    b_var21 = 1;
                }
                let c = pc14.add(1).read();
                if c == 2 {
                    b_var11 = 1;
                } else if c != 3 {
                    if c == 8 {
                        b_var10 = 1;
                    } else if c == 5 {
                        b_var7 = 1;
                    } else if c == 4 || c == 0x2c {
                        b_var8 = 1;
                    } else if c == 6 {
                        b_var9 = 1;
                    }
                }
            }
            pc14 = pc14.wrapping_add(0x4e);
            pc20 = pc20.wrapping_add(0x4e);
            u_var18 = u_var18.wrapping_add(1);
            if u_var18 >= 8 {
                break;
            }
        }
        u_var19 = 0;
        loop {
            u_var18 = 0;
            loop {
                us = p_967c.wrapping_add(i_var6).cast::<u16>().read_unaligned();
                let idx = u_var18
                    .wrapping_add((us as u32).wrapping_mul(0x126))
                    .wrapping_add(u_var19) as usize;
                let c = (&raw mut DAT_009a8080)
                    .cast::<u8>()
                    .wrapping_add(idx)
                    .read() as i8;
                if c != 10 {
                    let s = FUN_00432fa0(
                        b_var1 as u16,
                        (&raw mut DAT_009a8081)
                            .cast::<u8>()
                            .wrapping_add(idx)
                            .read() as u16,
                        (&raw mut DAT_009a8082)
                            .cast::<u8>()
                            .wrapping_add(idx)
                            .read() as u16,
                    );
                    if s != 0 {
                        if c == 2 {
                            b_var11 = 1;
                        } else if c != 3 {
                            if c == 8 {
                                b_var10 = 1;
                            } else if c == 5 {
                                b_var7 = 1;
                            } else if c == 4 || c == 0x2c {
                                b_var8 = 1;
                            } else if c == 6 {
                                b_var9 = 1;
                            }
                        }
                    }
                }
                u_var18 = u_var18.wrapping_add(4);
                if u_var18 >= 8 {
                    break;
                }
            }
            u_var19 = u_var19.wrapping_add(0xb);
            if u_var19 >= 0x16 {
                break;
            }
        }
        if (u_var13 as i32) < DAT_009a1fe0 {
            us = p_967c.wrapping_add(i_var6).cast::<u16>().read_unaligned();
            if b_var7 != 0
                && local_24
                    .wrapping_add(us as usize)
                    .wrapping_add(0x1f6)
                    .cast::<i8>()
                    .read()
                    != 0
            {
                b_var7 = 0;
            }
            let d80 = (&raw mut DAT_009d8c80)
                .cast::<u8>()
                .wrapping_add(i_var5)
                .read() as i8;
            let l0 = local_24.wrapping_add(0xb0).cast::<i8>().read();
            if b_var8 != 0 || b_var9 != 0 {
                let p_tmp: *mut u8 = DAT_009a1fe8
                    .wrapping_add(0x1f)
                    .wrapping_add(i_var15 as usize)
                    .cast::<*mut u8>()
                    .read_unaligned();
                let ss = p_tmp.wrapping_add(0x79).cast::<i16>().read_unaligned();
                if ss != 0 {
                    b_var10 = 1;
                }
                if b_var21 != 0 && d80 != l0 {
                    b_var8 = 0;
                    b_var9 = 0;
                }
            }
            if b_var10 != 0 && b_var21 != 0 && d80 != l0 {
                b_var10 = 0;
            }
            if b_var11 != 0 && b_var21 != 0 && d80 != l0 {
                b_var11 = 0;
            }
        }
        'end: {
            if b_var7 != 0 {
                DAT_00a07be6 = 0x158;
            } else {
                let r: i32;
                if b_var10 != 0 {
                    if b_var8 != 0 {
                        DAT_00a07be6 = 0x155;
                    } else {
                        DAT_00a07be6 = 0x159;
                        if b_var9 == 0 {
                            DAT_00a07be6 = 0x15b;
                        }
                    }
                    let tmp = (&raw mut DAT_009e9670)
                        .cast::<u8>()
                        .wrapping_add(i_var6)
                        .cast::<i32>()
                        .read_unaligned();
                    r = FUN_00404670(tmp, 0x13, param_1 as i16);
                } else {
                    if b_var8 != 0 {
                        DAT_00a07be6 = 0x155;
                        break 'end;
                    }
                    if b_var9 != 0 {
                        DAT_00a07be6 = 0x159;
                        break 'end;
                    }
                    if b_var11 == 0 {
                        break 'end;
                    }
                    DAT_00a07be6 = 0x151;
                    let tmp = (&raw mut DAT_009e9670)
                        .cast::<u8>()
                        .wrapping_add(i_var6)
                        .cast::<i32>()
                        .read_unaligned();
                    r = FUN_00404670(tmp, 0x45, param_1 as i16);
                }
                if r != 1 {
                    break 'end;
                }
                DAT_00a07be6 = DAT_00a07be6.wrapping_add(1);
            }
            local_8 = 1;
        }
        if param_3 != 0 && param_2 == DAT_009ecc34 {
            FUN_00450650(0x135, param_2);
            (&raw mut DAT_009d8b8a)
                .cast::<u8>()
                .wrapping_add((DAT_009ecc34 as u32).wrapping_mul(0x10) as usize)
                .cast::<i16>()
                .write_unaligned(0x49c);
            DAT_009d7694 = DAT_00a07be6 as u16 as u32;
        }
        if param_4 != 0 {
            return local_8;
        }
        DAT_00a07be6 as u16 as u32
    }
}
