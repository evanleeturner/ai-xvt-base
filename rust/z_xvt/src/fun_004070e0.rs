// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052575c: i32;
    static mut DAT_00730884: i32;
    static mut DAT_00525518: i32;
    static mut DAT_006db598: i32;
    static mut DAT_006db6d0: i32;
    static mut DAT_0071b718: i16;
    static mut DAT_0071b716: i16;
    static mut DAT_0071b6b2: i16;
    static mut DAT_0071e518: i16;
    static mut DAT_0071e522: i16;
    static mut DAT_00703ad4: i16;
    static mut DAT_0071f2e0: i16;
    static mut DAT_00732902: i16;
    static mut DAT_00732900: i16;
    static mut DAT_0071b708: i16;
    static mut DAT_0071b714: i16;
    static mut DAT_00703c60: i16;
    static mut DAT_006ffbe6: i16;
    static mut DAT_0071e6d6: u8;
    static mut DAT_0071e6d7: u8;
    static mut DAT_0071e8d8: [u16; 0];
    static mut DAT_0071e6d8: [i16; 0];
    static mut DAT_00525750: [u8; 0];
    static mut DAT_00525740: [u8; 0];
    static mut DAT_006d8180: [i32; 0];
    static mut DAT_0074110c: i32;
    static mut DAT_006d9d96: i16;
    static mut DAT_006da53c: i16;
    static mut DAT_006da53e: i16;
    static mut DAT_00732a20: i16;
    static mut DAT_00703c94: i16;
    static mut DAT_0070a170: i16;
    static mut DAT_0070acd4: i16;
    static mut DAT_0070acea: i16;
    static mut DAT_0070acec: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00407ed0() -> i32;
    fn FUN_00408260() -> i32;
    fn FUN_004085e0() -> i32;
    fn FUN_00408950() -> i32;
    fn FUN_00408d10() -> i32;
    fn FUN_00409050() -> i32;
    fn FUN_00409420() -> i32;
    fn FUN_004097b0() -> i32;
    fn FUN_00408140() -> i32;
    fn FUN_004084e0() -> i32;
    fn FUN_00408bd0() -> i32;
    fn FUN_004092e0() -> i32;
    fn FUN_00409680() -> i32;
    fn FUN_004079f0();
}

unsafe fn tbl(idx: i16) -> i16 {
    // SAFETY: reads the game's own table at the offset the original uses.
    unsafe {
        ((DAT_00730884 as u32)
            .wrapping_add(0x1934)
            .wrapping_add((idx as i32 as u32).wrapping_mul(2)) as *const i16)
            .read_unaligned()
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004070e0(param_1: *mut u8, param_2: i32) {
    let mut param_1 = param_1;
    // SAFETY: all globals, tables and the buffers reached through them belong
    // to the game; accesses mirror the original's reads, writes and calls.
    unsafe {
        DAT_0052575c = 1;
        let i_stack_54: i32 = match DAT_00730884 as u32 as usize {
            _ => {
                let w = ((DAT_00730884 as u32).wrapping_add(0x16) as *const u16).read_unaligned();
                match w {
                    0 => FUN_00407ed0(),
                    1 => FUN_00408260(),
                    2 => FUN_004085e0(),
                    3 => FUN_00408950(),
                    4 => FUN_00408d10(),
                    5 => FUN_00409050(),
                    6 => FUN_00409420(),
                    7 => FUN_004097b0(),
                    _ => 0,
                }
            }
        };
        if i_stack_54 == 0 {
            return;
        }
        DAT_00732902 = DAT_0071b718;
        DAT_00732900 = DAT_0071b716;
        DAT_0071b708 = DAT_0071e518;
        DAT_0071b714 = DAT_00703ad4;
        if DAT_0071b6b2 < 1 {
            let v = (DAT_0071f2e0 as i32 as u32)
                .wrapping_sub(DAT_0071b716 as i32 as u32)
                .wrapping_mul(DAT_006db6d0 as u32)
                .wrapping_add((DAT_0071b718 as i32 as u32).wrapping_mul(DAT_00525518 as u32))
                .wrapping_add(DAT_006db598 as u32);
            ((DAT_00730884 as u32).wrapping_add(0x25b4) as *mut i32).write_unaligned(v as i32);
            ((DAT_00730884 as u32).wrapping_add(0x3eb8) as *mut i32).write_unaligned(DAT_006db6d0);
        } else {
            let v = (DAT_0071b718 as i32 as u32)
                .wrapping_mul(DAT_00525518 as u32)
                .wrapping_add((DAT_0071b716 as i32 as u32).wrapping_mul(DAT_006db6d0 as u32))
                .wrapping_add(DAT_006db598 as u32);
            ((DAT_00730884 as u32).wrapping_add(0x25b4) as *mut i32).write_unaligned(v as i32);
            ((DAT_00730884 as u32).wrapping_add(0x3eb8) as *mut i32)
                .write_unaligned(0u32.wrapping_sub(DAT_006db6d0 as u32) as i32);
        }
        DAT_006db598 = ((DAT_00730884 as u32).wrapping_add(0x25b4) as *const i32).read_unaligned();
        let mut b_stack_6: u8 = 0;
        DAT_00703c60 = 0;
        let mut b_stack_5: u8 = 0;
        let mut b_stack_7: u8 = 0;
        DAT_006ffbe6 = 1;
        let mut b_var2: u8 = 0;
        let tab_e8d8 = (&raw mut DAT_0071e8d8).cast::<u16>();
        let tab_e6d8 = (&raw mut DAT_0071e6d8).cast::<i16>();
        let tab_5750 = (&raw mut DAT_00525750).cast::<u8>();
        let tab_5740 = (&raw mut DAT_00525740).cast::<u8>();
        let tab_8180 = (&raw mut DAT_006d8180).cast::<i32>();

        while *param_1 != 0xff {
            let mut u_var10: u16 = b_stack_7 as u16;
            let mut b_var7: u8 = b_stack_5;
            if DAT_0071e6d6.wrapping_add(b_stack_6) < b_stack_6 {
                b_var7 = b_stack_5.wrapping_add(1);
                if b_var7 == 0 {
                    u_var10 = b_stack_7.wrapping_add(1) as u16;
                }
            }
            if b_var7.wrapping_add(DAT_0071e6d7) < b_var7 {
                u_var10 = (u_var10 as u8).wrapping_add(1) as u16;
            }
            let mut i_stack_1c: i32 = 0;
            let s_var11: i16 = ((u_var10 as u32).wrapping_mul(0x100))
                .wrapping_add(b_var7.wrapping_add(DAT_0071e6d7) as u32)
                .wrapping_sub(
                    (b_stack_7 as u32)
                        .wrapping_mul(0x100)
                        .wrapping_add(b_stack_5 as u32),
                ) as i16;
            let mut i_stack_18: i32 = 0;
            let mut s_stack_c: i16 = s_var11;
            if DAT_0052575c == 1 {
                b_var7 = *param_1;
                let mut pi16 = tab_8180;
                let mut u_var10: u16 = 0;
                loop {
                    let i_var3 = i_stack_18;
                    if b_var7 == 0xfe {
                        break;
                    }
                    b_var7 = *param_1;
                    let mut pi14 = pi16;
                    let u_var4: u16;
                    if b_var7 == 0xfb {
                        b_var2 = *param_1.add(1);
                        u_var4 = u_var10;
                        param_1 = param_1.add(3);
                    } else if b_var7 == 0xfc {
                        let idx = *param_1.add(1) as usize;
                        let nb = param_1.add(2);
                        i_stack_18 =
                            (i_stack_18 as u32).wrapping_add(*tab_e8d8.add(idx) as u32) as i32;
                        u_var4 = u_var10.wrapping_add(*tab_e6d8.add(idx) as u16);
                        param_1 = nb;
                        if u_var4 < u_var10 {
                            i_stack_18 = i_stack_18.wrapping_add(1);
                        }
                    } else {
                        let b_var12: u8;
                        if b_var7 == 0xfd {
                            b_var7 = *param_1.add(1);
                            b_var12 = *param_1.add(2);
                            param_1 = param_1.add(3);
                        } else {
                            let sh = (*tab_5750.offset(param_2 as isize) & 0x1f) as u32;
                            b_var12 = (b_var7 >> sh).wrapping_add(b_var2);
                            b_var7 &= *tab_5740.offset(param_2 as isize);
                            param_1 = param_1.add(1);
                        }
                        u_var4 = u_var10.wrapping_add(*tab_e6d8.add(b_var7 as usize) as u16);
                        i_stack_18 = (*tab_e8d8.add(b_var7 as usize) as u32)
                            .wrapping_add(i_stack_18 as u32)
                            as i32;
                        if u_var4 < u_var10 {
                            i_stack_18 = i_stack_18.wrapping_add(1);
                        }
                        *pi16 = i_var3;
                        pi14 = pi16.wrapping_add(3);
                        i_stack_1c = i_stack_1c.wrapping_add(1);
                        *pi16.add(1) = b_var12 as u32 as i32;
                        *pi16.add(2) = i_stack_18.wrapping_sub(i_var3).wrapping_add(1);
                    }
                    pi16 = pi14;
                    u_var10 = u_var4;
                    b_var7 = *param_1;
                }
                param_1 = param_1.add(1);
            }
            loop {
                let i_var3: i32 = DAT_006db598;
                if i_stack_1c != 0 && -1 < DAT_00703ad4 {
                    let s_var5: i16 = (i_stack_18 as i16).wrapping_add(DAT_0071e522);
                    let two = DAT_00525518 == 2;
                    let i_var6: i32 = DAT_00730884.wrapping_add(0x25b8);
                    if s_var5 < 0
                        || DAT_00703ad4 <= s_var5
                        || DAT_0071e522 < 0
                        || DAT_0071e522 < DAT_0071e518
                    {
                        DAT_0074110c = i_stack_1c;
                        let mut pi16 = tab_8180;
                        loop {
                            let mut i_var15 = (DAT_0071e522 as i32).wrapping_add(*pi16);
                            let mut i_var9 = (*pi16.add(2)).wrapping_add(i_var15);
                            let i_var8 = DAT_0071e518 as i32;
                            let i_var13 = DAT_00703ad4 as i32;
                            if i_var15 < i_var8 {
                                i_var15 = i_var8;
                            }
                            if i_var15 <= i_var13 && i_var8 <= i_var9 {
                                if i_var13 < i_var9 {
                                    i_var9 = i_var13;
                                }
                                i_var9 = i_var9.wrapping_sub(i_var15);
                                if i_var9 != 0 {
                                    let val = *pi16.add(1);
                                    let mut pi14 = (i_var15 as u32)
                                        .wrapping_mul(4)
                                        .wrapping_add(i_var6 as u32)
                                        as *mut i32;
                                    loop {
                                        let off = pi14.read_unaligned();
                                        pi14 = pi14.wrapping_add(1);
                                        ((i_var3 as u32).wrapping_add(off as u32) as *mut u8)
                                            .write(val as u8);
                                        if two {
                                            ((i_var3 as u32)
                                                .wrapping_add(1)
                                                .wrapping_add(off as u32)
                                                as *mut u8)
                                                .write(0x80);
                                        }
                                        i_var9 = i_var9.wrapping_sub(1);
                                        if i_var9 == 0 {
                                            break;
                                        }
                                    }
                                }
                            }
                            DAT_0074110c = DAT_0074110c.wrapping_sub(1);
                            pi16 = pi16.wrapping_add(3);
                            if DAT_0074110c == 0 {
                                break;
                            }
                        }
                    } else {
                        DAT_0074110c = i_stack_1c;
                        let mut pi16 = tab_8180;
                        loop {
                            let i_var9 = *pi16.add(1);
                            let i_var15 = *pi16;
                            let mut i_var8 = *pi16.add(2);
                            pi16 = pi16.wrapping_add(3);
                            let mut pi14 = ((DAT_0071e522 as i32 as u32)
                                .wrapping_add(i_var15 as u32))
                            .wrapping_mul(4)
                            .wrapping_add(i_var6 as u32)
                                as *mut i32;
                            loop {
                                let off = pi14.read_unaligned();
                                pi14 = pi14.wrapping_add(1);
                                ((i_var3 as u32).wrapping_add(off as u32) as *mut u8)
                                    .write(i_var9 as u8);
                                if two {
                                    ((i_var3 as u32).wrapping_add(1).wrapping_add(off as u32)
                                        as *mut u8)
                                        .write(0x80);
                                }
                                i_var8 = i_var8.wrapping_sub(1);
                                if i_var8 == 0 {
                                    break;
                                }
                            }
                            DAT_0074110c = DAT_0074110c.wrapping_sub(1);
                            if DAT_0074110c == 0 {
                                break;
                            }
                        }
                    }
                }
                'skip: {
                    if s_var11 == 0 {
                        break 'skip;
                    }
                    let w =
                        ((DAT_00730884 as u32).wrapping_add(0x16) as *const u16).read_unaligned();
                    let i_stack_14: i32 = 'sw: {
                        match w {
                            0 => FUN_00408140(),
                            1 => FUN_004084e0(),
                            2 => {
                                DAT_006db598 = (DAT_006db598 as u32).wrapping_add(
                                    ((DAT_00730884 as u32).wrapping_add(0x3eb8) as *const i32)
                                        .read_unaligned()
                                        as u32,
                                ) as i32;
                                DAT_0071b716 = DAT_0071b716.wrapping_sub(1);
                                if DAT_0071b716 == DAT_0071f2e0 {
                                    DAT_0071e518 = 0;
                                    DAT_00703ad4 = -1;
                                    DAT_00703c94 = -1;
                                } else if DAT_0071b716 < 0 {
                                    if DAT_0071b716 == -1 {
                                        DAT_006d9d96 = -1;
                                    }
                                    DAT_006d9d96 = DAT_006d9d96.wrapping_add(1);
                                    DAT_0071e518 = DAT_0071e518.wrapping_add(tbl(DAT_006d9d96));
                                }
                                DAT_006da53c = DAT_006da53c.wrapping_sub(1);
                                if DAT_006da53c == DAT_0071f2e0 {
                                    DAT_00703ad4 = (((DAT_00730884 as u32).wrapping_add(0xe)
                                        as *const i16)
                                        .read_unaligned())
                                    .wrapping_sub(1);
                                } else {
                                    if DAT_006da53c < 0 {
                                        break 'sw 0;
                                    }
                                    if DAT_00732a20 <= DAT_006da53c && DAT_0071b716 < DAT_00732a20 {
                                        DAT_00703c94 = DAT_00703c94.wrapping_add(1);
                                        let s = tbl(DAT_00703c94);
                                        DAT_00703ad4 = DAT_00703ad4.wrapping_add(s);
                                    }
                                }
                                1
                            }
                            3 => FUN_00408bd0(),
                            4 => {
                                DAT_0071b718 = DAT_0071b718.wrapping_sub(1);
                                DAT_006db598 =
                                    (DAT_006db598 as u32).wrapping_sub(DAT_00525518 as u32) as i32;
                                if DAT_0071b718 == DAT_0070a170 {
                                    DAT_0071e518 = 0;
                                    DAT_00703ad4 = -1;
                                    DAT_0070acd4 = -1;
                                } else if DAT_0071b718 < 0 {
                                    DAT_0070acea = DAT_0070acea.wrapping_add(1);
                                    DAT_0071e518 = DAT_0071e518.wrapping_add(tbl(DAT_0070acea));
                                }
                                DAT_006da53e = DAT_006da53e.wrapping_sub(1);
                                if -1 < DAT_006da53e {
                                    if DAT_006da53e == DAT_0070a170 {
                                        DAT_00703ad4 = (((DAT_00730884 as u32).wrapping_add(0xe)
                                            as *const i16)
                                            .read_unaligned())
                                        .wrapping_sub(1);
                                    } else if DAT_0070acec <= DAT_006da53e
                                        && DAT_0071b718 < DAT_0070acec
                                    {
                                        DAT_0070acd4 = DAT_0070acd4.wrapping_add(1);
                                        let s = tbl(DAT_0070acd4);
                                        DAT_00703ad4 = DAT_00703ad4.wrapping_add(s);
                                    }
                                    1
                                } else {
                                    0
                                }
                            }
                            5 => FUN_004092e0(),
                            6 => FUN_00409680(),
                            7 => {
                                DAT_006db598 =
                                    (DAT_006db598 as u32).wrapping_add(DAT_00525518 as u32) as i32;
                                DAT_0071b718 = DAT_0071b718.wrapping_add(1);
                                if DAT_0071b718 == 0 {
                                    DAT_0071e518 = 0;
                                    DAT_00703ad4 = -1;
                                    DAT_0070acd4 = -1;
                                } else if DAT_0070acec <= DAT_0071b718 {
                                    if DAT_0070acec == DAT_0071b718 {
                                        DAT_0070acea = -1;
                                    }
                                    DAT_0070acea = DAT_0070acea.wrapping_add(1);
                                    DAT_0071e518 = DAT_0071e518.wrapping_add(tbl(DAT_0070acea));
                                }
                                DAT_006da53e = DAT_006da53e.wrapping_add(1);
                                if DAT_006da53e == 0 {
                                    DAT_00703ad4 = (((DAT_00730884 as u32).wrapping_add(0xe)
                                        as *const i16)
                                        .read_unaligned())
                                    .wrapping_sub(1);
                                } else {
                                    if DAT_0070acec <= DAT_006da53e {
                                        break 'sw 0;
                                    }
                                    if DAT_006da53e < 0 && -1 < DAT_0071b718 {
                                        DAT_0070acd4 = DAT_0070acd4.wrapping_add(1);
                                        let s = tbl(DAT_0070acd4);
                                        DAT_00703ad4 = DAT_00703ad4.wrapping_add(s);
                                    }
                                }
                                1
                            }
                            _ => 0,
                        }
                    };
                    if i_stack_14 == 0 {
                        return;
                    }
                    FUN_004079f0();
                }
                s_stack_c = s_stack_c.wrapping_sub(1);
                if !(s_stack_c != 0 && s_var11 != 0) {
                    break;
                }
            }
            if b_stack_6.wrapping_add(DAT_0071e6d6) < b_stack_6 {
                b_stack_5 = b_stack_5.wrapping_add(1);
            }
            if b_stack_5.wrapping_add(DAT_0071e6d7) < b_stack_5 {
                b_stack_7 = b_stack_7.wrapping_add(1);
            }
            b_stack_6 = b_stack_6.wrapping_add(DAT_0071e6d6);
            b_stack_5 = b_stack_5.wrapping_add(DAT_0071e6d7);
        }
    }
}
