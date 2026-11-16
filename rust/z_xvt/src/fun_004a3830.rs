// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732956: u16;
    static mut DAT_0071b704: i32;
    static mut DAT_00732904: i32;
    static mut DAT_006d4980: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_0070ad48: [u8; 0];
    static mut DAT_0070ad40: [u8; 0];
    static mut DAT_00709b08: [u8; 0];
    static mut DAT_0071b73c: [u8; 0];
    static mut DAT_0071b73e: [u8; 0];
    static mut DAT_0071b730: [u8; 0];
    static mut DAT_006d4870: i8;
    static mut _DAT_00732e70: *mut i8;
    static mut _DAT_00732950: u16;
    static mut _DAT_00732952: u16;
    static mut _DAT_00732954: u16;
    static mut DAT_0070aee5: [i8; 0];
    static mut DAT_0070aee9: [i8; 0];
    static mut DAT_006d4b36: [u8; 0];
    static mut DAT_006d4aaa: [u8; 0];
    static mut DAT_006daa00: [u8; 0];
    static mut DAT_006daa01: [u8; 0];
    static mut DAT_006daa02: [u8; 0];
    static mut DAT_0071ecf4: i32;
    static mut DAT_0070ac4a: [u8; 0];
    static mut DAT_0070a014: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a3e40(param_1: u32, param_2: i32, param_3: *mut i8);
    fn FUN_00444040(param_1: u32, param_2: u32) -> u8;
    fn FUN_0049ec30(param_1: u16, param_2: u16, param_3: u16) -> i16;
    fn FUN_0042b2a0(param_1: i32, param_2: i32, param_3: i16) -> i32;
    fn FUN_004a1d70(param_1: i32, param_2: i32);
}

unsafe fn r8(a: u32) -> u8 {
    // SAFETY: address inside the game's own memory, as in the original.
    unsafe { (a as usize as *const u8).read() }
}

unsafe fn r16(a: u32) -> u16 {
    // SAFETY: address inside the game's own memory, as in the original.
    unsafe { (a as usize as *const u16).read_unaligned() }
}

unsafe fn r32(a: u32) -> u32 {
    // SAFETY: address inside the game's own memory, as in the original.
    unsafe { (a as usize as *const u32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a3830(
    param_1: u32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: u32,
    param_6: i32,
) -> u32 {
    let _ = (param_5, param_6);
    // SAFETY: all globals and callees are the game's own; addresses computed
    // here are read and written exactly where the original does.
    unsafe {
        DAT_00732956 = 0x141;
        let u_var13: u32 = param_1 & 0xffff;
        let mut local_8: u32 = 0;
        if DAT_0071b704 <= u_var13 as i32 && (u_var13 as i32) < DAT_00732904 {
            return 0;
        }
        let local_24: u32;
        if (u_var13 as i32) < DAT_006d4980 {
            let i_inner = r32((DAT_006d4988 as u32)
                .wrapping_add(0x1f)
                .wrapping_add(u_var13.wrapping_mul(0x23)));
            local_24 = r32(i_inner.wrapping_add(0xa7));
        } else {
            local_24 = 0;
        }
        let i_var15: u32 = (DAT_006d4988 as u32).wrapping_add(u_var13.wrapping_mul(0x23));
        let b_var1: u8 = r8(i_var15.wrapping_add(0x16));
        let u_var18_0: u32 = b_var1 as u32;
        let i_var5: u32 = (param_2 as u32).wrapping_mul(0x5bd);
        let i_var14: u32 = u_var18_0.wrapping_mul(0x566);
        let b_var2: u8 = r8((&raw mut DAT_0070ad48 as usize as u32).wrapping_add(i_var14));
        let u_var_w: u16 = r16((&raw mut DAT_0071b73c as usize as u32).wrapping_add(i_var5));
        let mut u_var17: u32 = r8((&raw mut DAT_00709b08 as usize as u32)
            .wrapping_add(u_var18_0)
            .wrapping_add((u_var_w as u32).wrapping_mul(0x30)))
            as u32;
        if u_var17 == 0 {
            if local_24 == 0 {
                if r8(i_var15.wrapping_add(2)) == 8 {
                    u_var17 = 0x11;
                } else {
                    let b_var3 = r8(i_var15.wrapping_add(3));
                    if b_var3 < 0x46 || 0x4a < b_var3 {
                        if b_var3 < 0x50 || 0x52 < b_var3 {
                            if 0x52 < b_var3 && b_var3 < 0x55 {
                                u_var17 = 0x14;
                            }
                        } else {
                            u_var17 = 0x13;
                        }
                    } else {
                        u_var17 = 0x12;
                    }
                }
            } else {
                let u_var_w2 = r16((&raw mut DAT_0071b73e as usize as u32).wrapping_add(i_var5));
                if u_var_w2 as u32 == u_var18_0 {
                    u_var17 = 0xe;
                } else if u_var_w as u32 == b_var2 as u32 {
                    u_var17 = 0xd;
                } else if r8(i_var15.wrapping_add(2)) != 3 {
                    u_var17 = 0xf;
                } else {
                    u_var17 = 0x10;
                    if r16(local_24.wrapping_add(0x92)) != 0 {
                        u_var17 = 0xf;
                    }
                }
            }
        }
        FUN_004a3e40(param_1, 2, &raw mut DAT_006d4870);
        _DAT_00732e70 = &raw mut DAT_006d4870;
        _DAT_00732950 = 0x8000;
        _DAT_00732952 = 0x141;
        if u_var17 == 0 {
            _DAT_00732954 = 0x141;
        } else {
            if r8(u_var17.wrapping_add(0x52adf8)) != 0 {
                if u_var_w as u32 == b_var2 as u32 {
                    _DAT_00732952 = 0x144;
                } else {
                    let b = FUN_00444040(param_1, u_var_w as u32);
                    if b == 0 {
                        _DAT_00732952 = _DAT_00732952.wrapping_add(2);
                    } else {
                        _DAT_00732952 = _DAT_00732952.wrapping_add(1);
                    }
                }
            }
            _DAT_00732954 = (u_var17 as u16).wrapping_add(299);
        }
        let mut b6: u8 = 0;
        let mut b10: u8 = 0;
        let mut b9: u8 = 0;
        u_var17 = 0;
        let mut pc16: u32 = (&raw mut DAT_0070aee5 as usize as u32).wrapping_add(i_var14);
        let mut b7: u8 = 0;
        let mut b8: u8 = 0;
        let mut b11: u8 = 0;
        let mut pc19: u32 = (&raw mut DAT_0070aee9 as usize as u32)
            .wrapping_add(u_var_w as u32)
            .wrapping_add(i_var14);
        loop {
            if r8(pc19) != 0
                && r8(pc16) == 0
                && r8((&raw mut DAT_006d4b36 as usize as u32)
                    .wrapping_add(u_var18_0.wrapping_mul(0x126))
                    .wrapping_add(u_var17)
                    .wrapping_add((u_var_w as u32).wrapping_mul(8)))
                    == 4
            {
                if r8(pc16.wrapping_add(2)) == 6 {
                    if r8((&raw mut DAT_006d4aaa as usize as u32)
                        .wrapping_add(u_var18_0.wrapping_mul(0x126)))
                        == 0
                    {
                        b6 = 1;
                    }
                    b11 = 1;
                }
                let c = r8(pc16.wrapping_add(1));
                if c == 2 {
                    b10 = 1;
                } else if c != 3 {
                    if c == 8 {
                        b9 = 1;
                    } else if c == 5 {
                        b6 = 1;
                    } else if c == 4 || c == 0x2c {
                        b7 = 1;
                    } else if c == 6 {
                        b8 = 1;
                    }
                }
            }
            pc16 = pc16.wrapping_add(0x4e);
            pc19 = pc19.wrapping_add(0x4e);
            u_var17 = u_var17.wrapping_add(1);
            if u_var17 >= 8 {
                break;
            }
        }
        let mut u_var18: u32 = 0;
        loop {
            u_var17 = 0;
            loop {
                let base = u_var18.wrapping_add((u_var_w as u32).wrapping_mul(0x126));
                let c = r8((&raw mut DAT_006daa00 as usize as u32)
                    .wrapping_add(u_var17)
                    .wrapping_add(base));
                let idx = base.wrapping_add(u_var17);
                if c != 10 {
                    let s = FUN_0049ec30(
                        b_var1 as u16,
                        r8((&raw mut DAT_006daa01 as usize as u32).wrapping_add(idx)) as u16,
                        r8((&raw mut DAT_006daa02 as usize as u32).wrapping_add(idx)) as u16,
                    );
                    if s != 0 {
                        if c == 2 {
                            b10 = 1;
                        } else if c != 3 {
                            if c == 8 {
                                b9 = 1;
                            } else if c == 5 {
                                b6 = 1;
                            } else if c == 4 || c == 0x2c {
                                b7 = 1;
                            } else if c == 6 {
                                b8 = 1;
                            }
                        }
                    }
                }
                u_var17 = u_var17.wrapping_add(4);
                if u_var17 >= 8 {
                    break;
                }
            }
            u_var18 = u_var18.wrapping_add(0xb);
            if u_var18 >= 0x16 {
                break;
            }
        }
        let ad40 = r8((&raw mut DAT_0070ad40 as usize as u32).wrapping_add(i_var14));
        if (u_var13 as i32) < DAT_006d4980 {
            if b6 != 0 && r8((u_var_w as u32).wrapping_add(0x1f0).wrapping_add(local_24)) != 0 {
                b6 = 0;
            }
            if b7 != 0 || b8 != 0 {
                let i_inner = r32((DAT_006d4988 as u32)
                    .wrapping_add(0x1f)
                    .wrapping_add(u_var13.wrapping_mul(0x23)));
                let s_tmp = r16(i_inner.wrapping_add(0x79));
                if s_tmp != 0 {
                    b9 = 1;
                }
                if b11 != 0 && ad40 != r8(local_24.wrapping_add(0xb0)) {
                    b7 = 0;
                    b8 = 0;
                }
            }
            if b9 != 0 && b11 != 0 && ad40 != r8(local_24.wrapping_add(0xb0)) {
                b9 = 0;
            }
            if b10 != 0 && b11 != 0 && ad40 != r8(local_24.wrapping_add(0xb0)) {
                b10 = 0;
            }
        }
        'l: {
            if b6 != 0 {
                DAT_00732956 = 0x14e;
            } else {
                let i_var14b: i32;
                if b9 != 0 {
                    if b7 != 0 {
                        DAT_00732956 = 0x14b;
                    } else {
                        DAT_00732956 = 0x14f;
                        if b8 == 0 {
                            DAT_00732956 = 0x151;
                        }
                    }
                    let i_tmp =
                        r32((&raw mut DAT_0071b730 as usize as u32).wrapping_add(i_var5)) as i32;
                    i_var14b = FUN_0042b2a0(i_tmp, 0x13, param_1 as i16);
                } else {
                    if b7 != 0 {
                        DAT_00732956 = 0x14b;
                        break 'l;
                    }
                    if b8 != 0 {
                        DAT_00732956 = 0x14f;
                        break 'l;
                    }
                    if b10 == 0 {
                        break 'l;
                    }
                    DAT_00732956 = 0x147;
                    let i_tmp =
                        r32((&raw mut DAT_0071b730 as usize as u32).wrapping_add(i_var5)) as i32;
                    i_var14b = FUN_0042b2a0(i_tmp, 0x45, param_1 as i16);
                }
                if i_var14b != 1 {
                    break 'l;
                }
                DAT_00732956 = DAT_00732956.wrapping_add(1);
            }
            local_8 = 1;
        }
        if param_3 != 0 && param_2 == DAT_0071ecf4 {
            FUN_004a1d70(299, param_2);
            let p = (&raw mut DAT_0070ac4a as usize as u32)
                .wrapping_add((DAT_0071ecf4 as u32).wrapping_mul(0x10));
            (p as usize as *mut u16).write_unaligned(0x49c);
            DAT_0070a014 = DAT_00732956 as u32;
        }
        if param_4 != 0 {
            return local_8;
        }
        DAT_00732956 as u32
    }
}
