// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758d49: i32;
    static mut DAT_00758d4d: i32;
    static mut DAT_00758d51: i32;
    static mut DAT_00758d55: i32;
    static mut DAT_00758e86: i32;
    static mut DAT_00758e8a: *mut u32;
    static mut DAT_00758e8e: i32;
    static mut DAT_00758e92: i32;
    static mut DAT_00758e96: u32;
    static mut DAT_00758e9a: i32;
    static mut DAT_00758e9e: i8;
    static mut DAT_0075964b: i32;
    static mut DAT_00759657: i32;
    static mut DAT_00758830: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dd030(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_004dd000(param_1: *mut i32, param_2: *mut i32);
    fn FUN_004dd0b0(param_1: *mut i32);
    fn FUN_004d4d00() -> i32;
    fn FUN_004d0e60(param_1: *mut i8, param_2: i32, param_3: i32);
    fn FUN_004d4d90();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004dd860() {
    // SAFETY: all globals and callees are the game's own memory and code; the
    // pixel pointers are the game's surface and buffer, accessed where the
    // original accesses them.
    unsafe {
        let mut i_var2: i32 = (&raw const DAT_00758e92).read_unaligned();
        let i_var1: i32 = (&raw const DAT_00758e8e).read_unaligned();
        let mut x: i32 = (&raw const DAT_00758d49).read_unaligned();
        if !(-1 < x && x < 0x280) {
            return;
        }
        let mut y: i32 = (&raw const DAT_00758d4d).read_unaligned();
        if !(-1 < y && y < 0x1e0) {
            return;
        }
        let mut local_20: [i32; 4] = [0; 4];
        let mut local_10: [i32; 4] = [0; 4];
        local_20[0] = 0;
        local_20[2] = (i_var1 as u32).wrapping_sub(1) as i32;
        local_20[1] = 0;
        local_20[3] = (i_var2 as u32).wrapping_sub(1) as i32;
        FUN_004dd030(local_20.as_mut_ptr(), x, y);
        FUN_004dd000(local_10.as_mut_ptr(), local_20.as_mut_ptr());
        FUN_004dd0b0(local_20.as_mut_ptr());
        let u_var7: u32 =
            (i_var1 as u32).wrapping_add((local_20[2] as u32).wrapping_sub(local_10[2] as u32));
        i_var2 = (local_20[3] as u32)
            .wrapping_sub(local_10[3] as u32)
            .wrapping_add(i_var2 as u32) as i32;
        DAT_00758830 = FUN_004d4d00();
        let mut local_28: i32 = i_var2;
        x = (&raw const DAT_00758d49).read_unaligned();
        y = (&raw const DAT_00758d4d).read_unaligned();
        let stride: i32 = (&raw const DAT_0075964b).read_unaligned();
        let bpp: i32 = (&raw const DAT_00759657).read_unaligned();
        let base: u32 = DAT_00758830 as u32;

        if DAT_00758e9e == 0 {
            if bpp == 8 {
                let mut puvar3: u32 = base
                    .wrapping_add((y as u32).wrapping_mul(stride as u32))
                    .wrapping_add(x as u32);
                let mut i_var6: i32 = (&raw const DAT_00758e86).read_unaligned();
                let mut puvar8: u32 = (&raw const DAT_00758e8a).read_unaligned() as u32;
                let mut local_2c: i32 = i_var2;
                if 0 < i_var2 {
                    loop {
                        let mut p10: u32 = puvar3;
                        let mut p12: u32 = puvar8;
                        let mut u_var4: u32 = u_var7 >> 2;
                        while u_var4 != 0 {
                            (p12 as *mut u32).write_unaligned((p10 as *const u32).read_unaligned());
                            p10 = p10.wrapping_add(4);
                            p12 = p12.wrapping_add(4);
                            u_var4 = u_var4.wrapping_sub(1);
                        }
                        u_var4 = u_var7 & 3;
                        while u_var4 != 0 {
                            *(p12 as *mut u8) = *(p10 as *const u8);
                            p10 = p10.wrapping_add(1);
                            p12 = p12.wrapping_add(1);
                            u_var4 = u_var4.wrapping_sub(1);
                        }
                        let mut i_var11: i32 = 0;
                        if 0 < u_var7 as i32 {
                            loop {
                                let b =
                                    *((i_var11 as u32).wrapping_add(i_var6 as u32) as *const u8);
                                if b != 0 {
                                    *((i_var11 as u32).wrapping_add(puvar3) as *mut u8) = b;
                                }
                                i_var11 = i_var11.wrapping_add(1);
                                if !(i_var11 < u_var7 as i32) {
                                    break;
                                }
                            }
                        }
                        puvar3 = puvar3.wrapping_add(stride as u32);
                        local_2c = local_2c.wrapping_sub(1);
                        i_var6 = i_var6.wrapping_add(i_var1);
                        puvar8 = puvar8.wrapping_add(i_var1 as u32);
                        if local_2c == 0 {
                            break;
                        }
                    }
                }
            } else if bpp == 0x10 {
                let mut local_24: u32 = base
                    .wrapping_add((y as u32).wrapping_mul(stride as u32))
                    .wrapping_add((x as u32).wrapping_mul(2));
                if 0 < i_var2 {
                    let mut i_var6: i32 = (&raw const DAT_00758e86).read_unaligned();
                    let mut puvar8: u32 = (&raw const DAT_00758e8a).read_unaligned() as u32;
                    loop {
                        let mut i_var11: i32 = 0;
                        let mut puvar3: u32 = puvar8;
                        let mut puvar5: u32 = local_24;
                        if 0 < u_var7 as i32 {
                            loop {
                                (puvar3 as *mut u16)
                                    .write_unaligned((puvar5 as *const u16).read_unaligned());
                                let b =
                                    *((i_var11 as u32).wrapping_add(i_var6 as u32) as *const i8);
                                if b == 1 {
                                    (puvar5 as *mut u16).write_unaligned(0x1f);
                                } else if b == -1 {
                                    (puvar5 as *mut u16).write_unaligned(0xffff);
                                }
                                i_var11 = i_var11.wrapping_add(1);
                                puvar3 = puvar3.wrapping_add(2);
                                puvar5 = puvar5.wrapping_add(2);
                                if !(i_var11 < u_var7 as i32) {
                                    break;
                                }
                            }
                        }
                        i_var6 = i_var6.wrapping_add(i_var1);
                        puvar8 = puvar8.wrapping_add((i_var1 as u32).wrapping_mul(2));
                        local_24 = local_24.wrapping_add((stride as u32) & 0xffff_fffe);
                        local_28 = local_28.wrapping_sub(1);
                        if local_28 == 0 {
                            break;
                        }
                    }
                }
            }
        } else {
            if bpp == 8 {
                let mut puvar3: u32 = base
                    .wrapping_add((y as u32).wrapping_mul(stride as u32))
                    .wrapping_add(x as u32);
                let mut i_var6: i32 = i_var2;
                let mut puvar8: u32 = (&raw const DAT_00758e8a).read_unaligned() as u32;
                if 0 < i_var2 {
                    loop {
                        let mut p10: u32 = puvar3;
                        let mut p12: u32 = puvar8;
                        let mut u_var4: u32 = u_var7 >> 2;
                        while u_var4 != 0 {
                            (p12 as *mut u32).write_unaligned((p10 as *const u32).read_unaligned());
                            p10 = p10.wrapping_add(4);
                            p12 = p12.wrapping_add(4);
                            u_var4 = u_var4.wrapping_sub(1);
                        }
                        u_var4 = u_var7 & 3;
                        while u_var4 != 0 {
                            *(p12 as *mut u8) = *(p10 as *const u8);
                            p10 = p10.wrapping_add(1);
                            p12 = p12.wrapping_add(1);
                            u_var4 = u_var4.wrapping_sub(1);
                        }
                        puvar3 = puvar3.wrapping_add(stride as u32);
                        i_var6 = i_var6.wrapping_sub(1);
                        puvar8 = puvar8.wrapping_add(i_var1 as u32);
                        if i_var6 == 0 {
                            break;
                        }
                    }
                }
            } else if bpp == 0x10 {
                let mut puvar5: u32 = (y as u32)
                    .wrapping_mul(stride as u32)
                    .wrapping_add((x as u32).wrapping_mul(2))
                    .wrapping_add(base);
                if 0 < i_var2 {
                    let mut puvar8: u32 = (&raw const DAT_00758e8a).read_unaligned() as u32;
                    loop {
                        let mut u_var4: u32 = u_var7;
                        let mut puvar3: u32 = puvar8;
                        let mut puvar9: u32 = puvar5;
                        if 0 < u_var7 as i32 {
                            loop {
                                (puvar3 as *mut u16)
                                    .write_unaligned((puvar9 as *const u16).read_unaligned());
                                u_var4 = u_var4.wrapping_sub(1);
                                puvar3 = puvar3.wrapping_add(2);
                                puvar9 = puvar9.wrapping_add(2);
                                if u_var4 == 0 {
                                    break;
                                }
                            }
                        }
                        puvar8 = puvar8.wrapping_add((i_var1 as u32).wrapping_mul(2));
                        puvar5 = puvar5.wrapping_add((stride as u32) & 0xffff_fffe);
                        local_28 = local_28.wrapping_sub(1);
                        if local_28 == 0 {
                            break;
                        }
                    }
                }
            }
            FUN_004d0e60(&raw mut DAT_00758e9e, x, y);
        }
        FUN_004d4d90();
        x = (&raw const DAT_00758d49).read_unaligned();
        y = (&raw const DAT_00758d4d).read_unaligned();
        (&raw mut DAT_00758d51).write_unaligned(x);
        (&raw mut DAT_00758d55).write_unaligned(y);
        (&raw mut DAT_00758e96).write_unaligned(u_var7);
        (&raw mut DAT_00758e9a).write_unaligned(i_var2);
    }
}
