// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732a3c: u16;
    static mut DAT_006d4980: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_006d9d60: u8;
    static mut DAT_00732e54: i32;
    static mut DAT_006d9d78: i8;
    static mut DAT_006d9d7a: u8;
    static mut DAT_006d9d50: u16;
    static mut DAT_006d9d7b: [u8; 4];
    static mut DAT_006d9d7f: [u8; 4];
    static mut DAT_006d9d83: [u8; 4];
    static mut DAT_006d499c: u32;
    static mut DAT_00703c88: i32;
    static mut DAT_00732a38: i32;
    static mut DAT_0052687a: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0049f010(a: u32, b: u16, c: u16) -> u16;
    fn FUN_00429a20(a: u32) -> u32;
    fn FUN_00429c40(a: u32) -> u16;
    fn FUN_0044c560(a: u16, b: i16) -> u16;
    fn FUN_00412810(a: i32, b: i32, c: i32) -> u32;
    fn FUN_0042a1a0(a: u32, b: u32);
    fn FUN_004440e0(a: u16) -> u32;
}

fn rb(a: u32) -> u8 {
    // SAFETY: address inside the game's own memory, as in the original.
    unsafe { (a as usize as *const u8).read() }
}

fn rw(a: u32) -> u16 {
    // SAFETY: address inside the game's own memory, as in the original.
    unsafe { (a as usize as *const u16).read_unaligned() }
}

fn rd(a: u32) -> u32 {
    // SAFETY: address inside the game's own memory, as in the original.
    unsafe { (a as usize as *const u32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044b900(
    param_1: u16,
    param_2: u16,
    param_3: i16,
    param_4: u16,
    param_5: u16,
) -> u32 {
    // SAFETY: the globals, callees and pointed-to records are the game's own;
    // they are read and written exactly where the original does.
    unsafe {
        let mut u_var9: u16 = 0;
        let mut local_8: i32 = 0;
        let mut u_var7: u32 = DAT_00732a3c as u32;

        if (DAT_00732a3c as i32) < DAT_006d4980 {
            loop {
                let mut u_var8: u32 = u_var7 & 0xffff;
                let mut i_var6: u32 = (DAT_006d4988 as u32).wrapping_add(u_var8.wrapping_mul(0x23));
                'body: {
                    if rb(i_var6.wrapping_add(3)) != 0
                        && rb(i_var6.wrapping_add(0x16)) != DAT_006d9d60
                    {
                        let mut u_var10 = FUN_0049f010(u_var7, param_1, param_2);
                        let u_var11 = FUN_0049f010(u_var7, param_4, param_5);
                        if param_3 == 1 {
                            u_var10 |= u_var11;
                        } else {
                            u_var10 &= u_var11;
                        }
                        if u_var10 != 0
                            && rw((DAT_00732e54 as u32).wrapping_add(0x342)) as i16 != u_var7 as i16
                        {
                            if FUN_00429a20(u_var8) != 0 {
                                i_var6 = rd((DAT_006d4988 as u32)
                                    .wrapping_add(0x1f)
                                    .wrapping_add(u_var8.wrapping_mul(0x23)));
                                let i_var2 = rd(i_var6.wrapping_add(0xa7));
                                let u_var5 = rw(i_var2.wrapping_add(0x1df));
                                let cond = DAT_006d9d78 == 0
                                    || (u_var5 != 0
                                        && (DAT_006d9d78 == 0
                                            || rb(i_var2.wrapping_add(0x1e5)) == 0
                                            || {
                                                u_var8 = rd((DAT_006d9d50 as u32)
                                                    .wrapping_mul(0x23)
                                                    .wrapping_add(0x1f)
                                                    .wrapping_add(DAT_006d4988 as u32));
                                                rb(u_var8.wrapping_add(0x87))
                                                    != rb(i_var6.wrapping_add(0x87))
                                            }));
                                if cond {
                                    if (DAT_006d9d7a & 4) != 0 {
                                        let r = FUN_00429c40(u_var7);
                                        if r as i16 == 0 {
                                            break 'body;
                                        }
                                    }
                                    local_8 = local_8.wrapping_add(1);
                                }
                            }
                        }
                    }
                }
                u_var9 = (u_var8 >> 16) as u16;
                let u_var10 = (u_var7 as u16).wrapping_add(1);
                u_var7 = (u_var7 & 0xffff0000) | u_var10 as u32;
                if !((u_var10 as i32) < DAT_006d4980) {
                    break;
                }
            }
        }

        u_var7 = ((u_var9 as u32) << 16) | (DAT_00732a38 as u16 as u32);
        if ((DAT_00732a38 & 0xffff) as i32) < DAT_00703c88.wrapping_add(DAT_00732a38) {
            loop {
                'body: {
                    let b_var1 = rb((u_var7 & 0xffff)
                        .wrapping_mul(0x23)
                        .wrapping_add(3)
                        .wrapping_add(DAT_006d4988 as u32));
                    if b_var1 != 0
                        && ((&raw mut DAT_0052687a)
                            .cast::<u8>()
                            .add((b_var1 as usize).wrapping_mul(0x16))
                            .read()
                            & 2)
                            != 0
                    {
                        let mut u_var10 = FUN_0049f010(u_var7, param_1, param_2);
                        let u_var11 = FUN_0049f010(u_var7, param_4, param_5);
                        if param_3 == 1 {
                            u_var10 |= u_var11;
                        } else {
                            u_var10 &= u_var11;
                        }
                        if u_var10 != 0
                            && rw((DAT_00732e54 as u32).wrapping_add(0x342)) as i16 != u_var7 as i16
                        {
                            if FUN_00429a20(u_var7 & 0xffff) != 0 {
                                if (DAT_006d9d7a & 4) != 0 {
                                    let r = FUN_00429c40(u_var7);
                                    if r as i16 == 0 {
                                        break 'body;
                                    }
                                }
                                local_8 = local_8.wrapping_add(1);
                            }
                        }
                    }
                }
                let u_var10 = (u_var7 as u16).wrapping_add(1);
                u_var7 = (u_var7 & 0xffff0000) | u_var10 as u32;
                if !((u_var10 as i32) < DAT_00703c88.wrapping_add(DAT_00732a38)) {
                    break;
                }
            }
        }

        if local_8 == 0 {
            return 0xffff;
        }

        u_var7 = (u_var7 & 0xffff0000) | DAT_00732a3c as u32;
        let mut local_4: u32 = 0xffff_ffff;
        let mut local_a: u16 = 0xffff;

        if (DAT_00732a3c as i32) < DAT_006d4980 {
            loop {
                let u_var8: u32 = u_var7 & 0xffff;
                let mut i_var6: u32 = u_var8.wrapping_mul(0x23);
                let u_var10: u16 = u_var7 as u16;
                'body: {
                    if rb((DAT_006d4988 as u32).wrapping_add(i_var6).wrapping_add(3)) != 0
                        && rb((DAT_006d4988 as u32)
                            .wrapping_add(i_var6)
                            .wrapping_add(0x16))
                            != DAT_006d9d60
                    {
                        let mut u_var11 = FUN_0049f010(u_var7, param_1, param_2);
                        let u_var5 = FUN_0049f010(u_var7, param_4, param_5);
                        if param_3 == 1 {
                            u_var11 |= u_var5;
                        } else {
                            u_var11 &= u_var5;
                        }
                        if u_var11 != 0 && rw((DAT_00732e54 as u32).wrapping_add(0x342)) != u_var10
                        {
                            if FUN_00429a20(u_var8) != 0 {
                                let i_var2 = rd((DAT_006d4988 as u32)
                                    .wrapping_add(0x1f)
                                    .wrapping_add(i_var6));
                                let i_var3 = rd(i_var2.wrapping_add(0xa7));
                                let u_var5 = rw(i_var3.wrapping_add(0x1df));
                                let cond = DAT_006d9d78 == 0
                                    || (u_var5 != 0
                                        && (DAT_006d9d78 == 0
                                            || rb(i_var3.wrapping_add(0x1e5)) == 0
                                            || {
                                                let i_var4 = rd((DAT_006d9d50 as u32)
                                                    .wrapping_mul(0x23)
                                                    .wrapping_add(0x1f)
                                                    .wrapping_add(DAT_006d4988 as u32));
                                                rb(i_var4.wrapping_add(0x87))
                                                    != rb(i_var2.wrapping_add(0x87))
                                            }));
                                if cond {
                                    if (DAT_006d9d7a & 4) != 0 {
                                        let r = FUN_00429c40(u_var7);
                                        if r as i16 == 0 {
                                            break 'body;
                                        }
                                    }
                                    if (DAT_006d9d7a & 1) != 0 {
                                        let r = FUN_0044c560(u_var10, local_8 as i16);
                                        if r as i16 == 0 {
                                            break 'body;
                                        }
                                    }
                                    if (DAT_006d9d7a & 0x20) == 0 {
                                        FUN_0042a1a0(DAT_006d9d50 as u32, u_var8);
                                    } else {
                                        i_var6 = (DAT_006d4988 as u32).wrapping_add(i_var6);
                                        let i_var2 = rd(i_var6.wrapping_add(4));
                                        let i_var3 = rd(i_var6.wrapping_add(8));
                                        let i_var4 = rd(i_var6.wrapping_add(0xc));
                                        let i_var5 =
                                            (&raw mut DAT_006d9d7b).cast::<u32>().read_unaligned();
                                        let i_var7 =
                                            (&raw mut DAT_006d9d7f).cast::<u32>().read_unaligned();
                                        let i_var8 =
                                            (&raw mut DAT_006d9d83).cast::<u32>().read_unaligned();
                                        DAT_006d499c = FUN_00412810(
                                            i_var2.wrapping_sub(i_var5) as i32,
                                            i_var3.wrapping_sub(i_var7) as i32,
                                            i_var4.wrapping_sub(i_var8) as i32,
                                        );
                                    }
                                    if 0x4000 < (DAT_006d499c as i32) {
                                        let r = FUN_004440e0(u_var10);
                                        if r as u8 == 1 {
                                            break 'body;
                                        }
                                    }
                                    if DAT_006d499c < local_4 {
                                        local_4 = DAT_006d499c;
                                        local_a = u_var10;
                                    }
                                }
                            }
                        }
                    }
                }
                let n = u_var10.wrapping_add(1);
                u_var7 = (u_var7 & 0xffff0000) | n as u32;
                if !((n as i32) < DAT_006d4980) {
                    break;
                }
            }
        }

        u_var7 = (u_var7 & 0xffff0000) | (DAT_00732a38 as u16 as u32);
        let mut i_var6: i32 = DAT_00703c88.wrapping_add(DAT_00732a38);
        if ((DAT_00732a38 & 0xffff) as i32) < i_var6 {
            loop {
                let u_var8: u32 = u_var7 & 0xffff;
                let u_var10: u16 = u_var7 as u16;
                'body: {
                    let b_var1 = rb(u_var8
                        .wrapping_mul(0x23)
                        .wrapping_add(3)
                        .wrapping_add(DAT_006d4988 as u32));
                    if b_var1 != 0
                        && ((&raw mut DAT_0052687a)
                            .cast::<u8>()
                            .add((b_var1 as usize).wrapping_mul(0x16))
                            .read()
                            & 2)
                            != 0
                    {
                        let mut u_var11 = FUN_0049f010(u_var7, param_1, param_2);
                        let u_var5 = FUN_0049f010(u_var7, param_4, param_5);
                        if param_3 == 1 {
                            u_var11 |= u_var5;
                        } else {
                            u_var11 &= u_var5;
                        }
                        if u_var11 != 0 && rw((DAT_00732e54 as u32).wrapping_add(0x342)) != u_var10
                        {
                            if FUN_00429a20(u_var8) != 0 {
                                if (DAT_006d9d7a & 4) != 0 {
                                    let r = FUN_00429c40(u_var7);
                                    if r as i16 == 0 {
                                        break 'body;
                                    }
                                }
                                if (DAT_006d9d7a & 1) != 0 {
                                    let r = FUN_0044c560(u_var10, local_8 as i16);
                                    if r as i16 == 0 {
                                        break 'body;
                                    }
                                }
                                FUN_0042a1a0(DAT_006d9d50 as u32, u_var8);
                                if DAT_006d499c < local_4 {
                                    local_4 = DAT_006d499c;
                                    local_a = u_var10;
                                }
                            }
                        }
                    }
                }
                let n = u_var10.wrapping_add(1);
                u_var7 = (u_var7 & 0xffff0000) | n as u32;
                i_var6 = DAT_00703c88.wrapping_add(DAT_00732a38);
                if !((n as i32) < i_var6) {
                    break;
                }
            }
        }

        (((i_var6 as u32) >> 16) << 16) | local_a as u32
    }
}
