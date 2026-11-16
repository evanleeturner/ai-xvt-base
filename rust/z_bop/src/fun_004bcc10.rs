// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00664e88: i32;
    static mut DAT_00664e8c: i32;
    static mut DAT_00664e90: i32;
    static mut DAT_00664e94: i32;
    static mut DAT_00b6a2a0: u32;
    static mut DAT_00b6a2c0: u32;
    static mut DAT_00bb2810: i32;
    static mut DAT_00bb72a0: i32;
    static mut DAT_00bb73a7: i8;
    static mut DAT_00b69d20: [i8; 0];
    static mut DAT_00bb2820: [i8; 0];
    static mut DAT_00bb2821: i8;
    static mut DAT_00bb2835: i8;
    static mut DAT_00bb7380: [i8; 0];
    static mut DAT_00bb7390: [i8; 0];
    static mut DAT_0052ada8: [i8; 0];
    static mut s_configsound_0052a708: [i8; 0];
    static mut s_settingsound_0052ad80: [i8; 0];
    static mut s__s__s___c_s_0052ad90: [i8; 0];
    static mut s__s__d___c_s_0052ad9c: [i8; 0];
    static mut s__c_s___c_s_0052adb0: [i8; 0];
    static mut s__c_s__d__c_s_0052adbc: [i8; 0];
    static mut s__c_s__s__c_s_0052adcc: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004de090(a: *mut i32, b: *mut i32);
    fn FUN_004d6460(a: *mut i32, b: i32, c: i32, d: i32, e: i32);
    fn FUN_004ddc10(a: u32) -> *mut i8;
    fn FUN_004db640(a: i32, b: *mut i8, c: *mut i32, d: u32);
    fn _sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004db7d0(a: i32, b: *mut i8, c: *mut i32, d: i32, e: i32, f: u32);
    fn FUN_004d64a0(a: *mut i32, b: i32, c: i32);
    fn FUN_004d62c0(a: i32) -> u8;
    fn FUN_004d6240(a: i32) -> i32;
    fn FUN_004de5a0(a: *mut i8, b: i32, c: i32, d: i32);
    fn FUN_004d6300(a: i32) -> u8;
    fn FUN_004d62e0(a: i32) -> u8;
    fn FUN_004d9d70(a: *mut i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32) -> i32;
    fn FUN_004d67e0(a: *mut i32, b: i32, c: i32, d: i32, e: i32);
    fn FUN_004d64c0(a: *mut i32, b: i32, c: i32);
    fn FUN_004d6b90(a: *mut i32, b: i32, c: i32) -> i32;
    fn FUN_004d6950(a: *mut i32, b: i32, c: i32, d: i16);
    fn FUN_004dc220() -> u8;
    fn FUN_004dc240() -> u8;
    fn FUN_004bd770() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004bcc10() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers derived from them are used as the original does.
    unsafe {
        let buf = (&raw mut DAT_00b69d20).cast::<i8>();
        let arr_2820 = (&raw mut DAT_00bb2820).cast::<i8>();
        let arr_7380 = (&raw mut DAT_00bb7380).cast::<i8>();
        let arr_7390 = (&raw mut DAT_00bb7390).cast::<i8>();
        let snd_cfg = (&raw mut s_configsound_0052a708).cast::<i8>();
        let snd_set = (&raw mut s_settingsound_0052ad80).cast::<i8>();
        let fmt_adb0 = (&raw mut s__c_s___c_s_0052adb0).cast::<i8>() as *const i8;
        let base_2821 = (&raw mut DAT_00bb2821) as usize as u32;
        let base_2835 = (&raw mut DAT_00bb2835) as usize as u32;

        let mut local_1c: [i32; 4] = [0; 4];
        let r = local_1c.as_mut_ptr();
        let mut local_8: i32 = 0;
        let mut local_4: i32 = 0;

        FUN_004de090(&raw mut local_4, &raw mut local_8);
        FUN_004d6460(r, 0x54, 0x5a, 0x25c, 0x6a);
        let pc = FUN_004ddc10(0x19f);
        FUN_004db640(0xf, pc, r, 0xffff);
        FUN_004d6460(r, 0x58, 0x6f, 0x142, 0x7d);
        let (pc3, pc4, pc5, fmt, uv13): (*mut i8, *mut i8, *mut i8, *const i8, i32);
        if DAT_00664e90 < 0x10 {
            pc4 = (DAT_00664e90 as u32).wrapping_add(1) as usize as *mut i8;
            pc3 = FUN_004ddc10(0x1a1);
            uv13 = 1;
            pc5 = FUN_004ddc10(0x1a0);
            fmt = (&raw mut s__c_s__d__c_s_0052adbc).cast::<i8>() as *const i8;
        } else {
            let u = (DAT_00664e90 as u32).wrapping_add(0x277);
            pc3 = FUN_004ddc10(0x1a1);
            uv13 = 1;
            pc4 = FUN_004ddc10(u);
            pc5 = FUN_004ddc10(0x286);
            fmt = (&raw mut s__c_s__s__c_s_0052adcc).cast::<i8>() as *const i8;
        }
        _sprintf(buf, fmt, 4i32, pc5, pc4, uv13, pc3);
        FUN_004db7d0(0xc, buf, r, 0, 1, 0xffff);
        FUN_004d64a0(r, 0, 0xf);
        if DAT_00bb72a0 != 0 {
            if DAT_00664e94 != 0 {
                let m = (DAT_00664e94 as u32).wrapping_mul(0x95);
                _sprintf(
                    buf,
                    fmt_adb0,
                    2i32,
                    base_2821.wrapping_add(m) as i32,
                    1i32,
                    base_2835.wrapping_add(m) as i32,
                );
            } else {
                _sprintf(
                    buf,
                    (&raw mut DAT_0052ada8).cast::<i8>() as *const i8,
                    2i32,
                    &raw mut DAT_00bb2821,
                );
            }
            FUN_004db7d0(0xc, buf, r, 0, 1, 0xffff);
        }
        FUN_004d64a0(r, 0, 0x1e);
        let pc = FUN_004ddc10(0x28d);
        FUN_004db7d0(0xc, pc, r, 0, 1, DAT_00b6a2a0);
        FUN_004d64a0(r, 0, 0xf);
        let pc = FUN_004ddc10(0x28e);
        FUN_004db7d0(0xc, pc, r, 0, 1, DAT_00b6a2a0);
        FUN_004d64a0(r, 0, 0xf);
        let pc = FUN_004ddc10(0x28f);
        FUN_004db7d0(0xc, pc, r, 0, 1, DAT_00b6a2a0);
        FUN_004d64a0(r, 0, 0xf);
        let pc = FUN_004ddc10(0x290);
        FUN_004db7d0(0xc, pc, r, 0, 1, DAT_00b6a2a0);
        FUN_004d64a0(r, 0, 0xf);
        let pc = FUN_004ddc10(0x291);
        FUN_004db7d0(0xc, pc, r, 0, 1, DAT_00b6a2a0);

        let mut iv12 = FUN_004d62c0(0) as i32;
        let mut iv14 = FUN_004d6240(0);
        if (iv14 != -1 && iv14 < 0x10) || iv12 != 0 {
            if iv14 == -1 {
                if iv12 != 0 {
                    if iv12.wrapping_sub(DAT_00664e90) != -0xf && DAT_00bb73a7 != 0 {
                        FUN_004de5a0(snd_cfg, 1, 0, 0xff);
                    }
                    DAT_00664e90 = iv12.wrapping_add(0xf);
                    let u2 = FUN_004d6300(0) as i32;
                    iv12 = iv12.wrapping_add(-1).wrapping_add(u2);
                    if iv12 < DAT_00664e8c || 9 < iv12.wrapping_sub(DAT_00664e8c) {
                        DAT_00664e8c = iv12;
                    }
                }
            } else {
                if DAT_00664e90 != iv14 && DAT_00bb73a7 != 0 {
                    FUN_004de5a0(snd_cfg, 1, 0, 0xff);
                }
                DAT_00664e90 = iv14;
                if iv14 < DAT_00664e8c || 9 < iv14.wrapping_sub(DAT_00664e8c) {
                    DAT_00664e8c = iv14;
                }
            }
            iv12 = 0;
            if 0 < DAT_00bb72a0 {
                let mut p = arr_2820;
                loop {
                    if *p == *arr_7380.offset(DAT_00664e90 as isize) {
                        DAT_00664e94 = iv12;
                    }
                    p = p.wrapping_add(0x95);
                    iv12 = iv12.wrapping_add(1);
                    if !(iv12 < DAT_00bb72a0) {
                        break;
                    }
                }
            }
        }
        iv12 = FUN_004d6300(0) as i32;
        let mut local_c = iv12;
        if FUN_004d62e0(0) as i32 != 0 {
            local_c = iv12.wrapping_add(4);
        }
        FUN_004d6460(r, 0x58, 0xf6, 0x142, 0x104);
        let pc = FUN_004ddc10(0x28c);
        FUN_004db640(0xc, pc, r, DAT_00b6a2c0);
        let mut uv13b: i32 = 0x142;
        if 10 < local_c {
            FUN_004d6460(r, 0x139, 0x105, 0x142, 0x19f);
            DAT_00664e8c = FUN_004d9d70(r, DAT_00664e8c, local_c, 0, 5, DAT_00bb2810, 1);
            uv13b = 0x138;
        }
        FUN_004d6460(r, 0x58, 0x105, uv13b, 0x19f);
        FUN_004d67e0(r, 0, 0, 0xffff, 0);
        FUN_004d64c0(r, 2, 2);
        FUN_004d67e0(r, 0, 0, 0, 0);
        local_1c[3] = (local_1c[1] as u32).wrapping_add(0xe) as i32;
        FUN_004d64c0(r, 2, 0);
        iv14 = DAT_00664e8c;
        if DAT_00664e8c < DAT_00664e8c.wrapping_add(10) {
            loop {
                if local_c <= iv14 {
                    break;
                }
                let mut iv8: i32 = 0;
                if 0 < DAT_00bb72a0 {
                    let mut p = arr_2820;
                    loop {
                        if iv14 < iv12 {
                            if *arr_7380.offset(iv14 as isize) == *p {
                                break;
                            }
                        } else if *arr_7390.offset(iv14.wrapping_sub(iv12) as isize) == *p {
                            break;
                        }
                        p = p.wrapping_add(0x95);
                        iv8 = iv8.wrapping_add(1);
                        if !(iv8 < DAT_00bb72a0) {
                            break;
                        }
                    }
                }
                if iv8 < DAT_00bb72a0 {
                    let iv6 = FUN_004d6b90(r, local_4, local_8);
                    let mut iv7 = DAT_00664e90;
                    let mut iv1 = DAT_00664e94;
                    if iv6 != 0 {
                        FUN_004d6950(r, 0, 0, DAT_00b6a2a0 as i16);
                        let mut hit = FUN_004dc220() != 0;
                        if !hit {
                            let u2 = FUN_004dc240();
                            iv7 = DAT_00664e90;
                            iv1 = DAT_00664e94;
                            hit = u2 != 0;
                        }
                        if hit {
                            iv1 = iv8;
                            if iv14 < iv12 {
                                iv7 = iv14;
                                if DAT_00664e90 != iv14 && DAT_00bb73a7 != 0 {
                                    FUN_004de5a0(snd_cfg, 1, 0, 0xff);
                                }
                            } else {
                                if iv12.wrapping_sub(DAT_00664e90).wrapping_add(iv14) != 0x10
                                    && DAT_00bb73a7 != 0
                                {
                                    FUN_004de5a0(snd_cfg, 1, 0, 0xff);
                                }
                                iv7 = iv14.wrapping_sub(iv12).wrapping_add(0x10);
                            }
                        }
                    }
                    DAT_00664e94 = iv1;
                    DAT_00664e90 = iv7;
                    let uv9: u16;
                    let a3: *mut i8;
                    let a4: *mut i8;
                    let f: *const i8;
                    let a8: i32;
                    if iv14 < iv12 {
                        let mut u = 0xffffu16;
                        if iv7 == iv14 {
                            u = DAT_00b6a2c0 as u16;
                        }
                        uv9 = u;
                        a3 = (iv14 as u32).wrapping_add(1) as usize as *mut i8;
                        a8 = base_2821.wrapping_add((iv8 as u32).wrapping_mul(0x95)) as i32;
                        a4 = FUN_004ddc10(0x1a0);
                        f = (&raw mut s__s__d___c_s_0052ad9c).cast::<i8>() as *const i8;
                    } else {
                        let mut u = 0xffffu16;
                        if iv14.wrapping_sub(iv12).wrapping_sub(iv7) == -0x10 {
                            u = DAT_00b6a2c0 as u16;
                        }
                        uv9 = u;
                        a8 = base_2821.wrapping_add((iv8 as u32).wrapping_mul(0x95)) as i32;
                        a3 = FUN_004ddc10(
                            (iv14 as u32).wrapping_sub(iv12 as u32).wrapping_add(0x287),
                        );
                        a4 = FUN_004ddc10(0x286);
                        f = (&raw mut s__s__s___c_s_0052ad90).cast::<i8>() as *const i8;
                    }
                    _sprintf(buf, f, a4, a3, 2i32, a8);
                    FUN_004db7d0(0xc, buf, r, 0, 1, uv9 as u32);
                }
                iv14 = iv14.wrapping_add(1);
                FUN_004d64a0(r, 0, 0xf);
                if !(iv14 < DAT_00664e8c.wrapping_add(10)) {
                    break;
                }
            }
        }

        FUN_004d6460(r, 0x14a, 0x6f, 0x24e, 0x7d);
        let pc = FUN_004ddc10(0x28b);
        FUN_004db640(0xc, pc, r, DAT_00b6a2c0);
        FUN_004d6460(r, 0x24f, 0x7e, 600, 0x19f);
        DAT_00664e88 = FUN_004d9d70(r, DAT_00664e88, DAT_00bb72a0, 0, 5, DAT_00bb2810, 2);
        FUN_004d6460(r, 0x14a, 0x7e, 0x24e, 0x19f);
        FUN_004d67e0(r, 0, 0, 0xffff, 0);
        FUN_004d64c0(r, 2, 2);
        FUN_004d67e0(r, 0, 0, 0, 0);
        local_1c[3] = (local_1c[1] as u32).wrapping_add(0xe) as i32;
        FUN_004d64c0(r, 2, 0);
        if DAT_00664e88 < DAT_00664e88.wrapping_add(0x13) {
            let mut p10 = arr_2820
                .cast::<u8>()
                .wrapping_add((DAT_00664e88 as u32).wrapping_mul(0x95) as usize);
            let mut i12 = DAT_00664e88;
            loop {
                if DAT_00bb72a0 <= i12 {
                    break;
                }
                let i14 = FUN_004d6b90(r, local_4, local_8);
                if i14 != 0 {
                    FUN_004d6950(r, 0, 0, DAT_00b6a2a0 as i16);
                    if FUN_004dc220() != 0 || FUN_004dc240() != 0 {
                        if i12 != DAT_00664e94 && DAT_00bb73a7 != 0 {
                            FUN_004de5a0(snd_set, 1, 0, 0xff);
                        }
                        DAT_00664e94 = i12;
                        arr_7380
                            .offset(DAT_00664e90 as isize)
                            .write(p10.read() as i8);
                    }
                }
                _sprintf(
                    buf,
                    fmt_adb0,
                    2i32,
                    p10.wrapping_add(1),
                    1i32,
                    p10.wrapping_add(0x15),
                );
                let mut uv15 = DAT_00b6a2c0;
                if i12 != DAT_00664e94 {
                    uv15 = 0xffff;
                }
                p10 = p10.wrapping_add(0x95);
                i12 = i12.wrapping_add(1);
                FUN_004db7d0(0xc, buf, r, 0, 1, uv15);
                FUN_004d64a0(r, 0, 0xf);
                if !(i12 < DAT_00664e88.wrapping_add(0x13)) {
                    break;
                }
            }
        }

        let mut i12: i32 = 0;
        let uv15 = FUN_004bd770();
        if 0 < DAT_00bb72a0 {
            let mut pc3 = arr_2820;
            let mut i14 = DAT_00bb72a0;
            let mut i8 = DAT_00664e90;
            loop {
                if (uv15 as u8 as i8) != 0 && *pc3 == (uv15 as u8 as i8) {
                    if i12 != DAT_00664e94 && DAT_00bb73a7 != 0 {
                        FUN_004de5a0(snd_set, 1, 0, 0xff);
                        i14 = DAT_00bb72a0;
                        i8 = DAT_00664e90;
                    }
                    DAT_00664e88 = i12;
                    DAT_00664e94 = i12;
                    arr_7380.offset(i8 as isize).write(*pc3);
                }
                pc3 = pc3.wrapping_add(0x95);
                i12 = i12.wrapping_add(1);
                if !(i12 < i14) {
                    break;
                }
            }
        }
    }
}
