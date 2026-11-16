// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0066d898: i32;
    static mut DAT_0052c208: i32;
    static mut DAT_0052c184: i32;
    static mut DAT_0052c188: i32;
    static mut DAT_0052c18c: i32;
    static mut DAT_00b6a250: i32;
    static mut DAT_00b69e30: i32;
    static mut DAT_0066d888: i32;
    static mut DAT_0066d880: i32;
    static mut DAT_0066d894: i32;
    static mut DAT_0066d88c: i32;
    static mut DAT_00aa6114: i32;
    static mut DAT_00aa6118: *mut c_void;
    static mut DAT_00aa6134: u32;
    static mut DAT_00aa6138: u32;
    static mut DAT_00aa6af0: u32;
    static mut DAT_00aa6af4: i32;
    static mut DAT_00aa5cb0: i32;
    static mut DAT_00b69e38: u32;
    static mut DAT_00b6a2a0: u32;
    static mut DAT_00b6a2c0: u32;
    static mut DAT_00b6a2e0: i8;
    static mut DAT_00b6bc9e: i8;
    static mut DAT_00b69d20: i8;
    static mut DAT_00a91b90: i8;
    static mut DAT_00bb73a8: i8;

    static mut DAT_00b6a5aa: u8;
    static mut DAT_00b6a5ae: u8;
    static mut DAT_00b6a5a6: u8;
    static mut DAT_00b6a5ba: u8;
    static mut DAT_00b6bbaa: u8;
    static mut DAT_00b6bba6: u8;
    static mut DAT_00b6bbda: u8;
    static mut DAT_00b6bc02: u8;
    static mut DAT_00b6bbae: u8;
    static mut DAT_00b6d962: u8;
    static mut DAT_00aa615e: u8;
    static mut DAT_00aa6162: u8;
    static mut DAT_00aa62ae: u8;

    static mut DAT_0052c890: i8;
    static mut DAT_0052bf74: i8;
    static mut DAT_0052a6d4: i8;
    static mut DAT_0052b69c: i8;
    static mut DAT_0052b6ac: i8;
    static mut DAT_0052bfac: i8;
    static mut DAT_0052c374: i8;
    static mut DAT_0052a6b0: i8;
    static mut DAT_0052a6a4: i8;
    static mut DAT_0052a698: i8;
    static mut DAT_0052a68c: i8;
    static mut DAT_0052a678: i8;
    static mut DAT_0052a680: i8;
    static mut DAT_0052bf94: i8;
    static mut DAT_0052bf80: i8;
    static mut DAT_0052bf88: i8;
    static mut DAT_0052c21c: i8;
    static mut DAT_0052c228: i8;
    static mut DAT_004d7370: u8;
    static mut DAT_004e1470: u8;
    static mut DAT_004f5100: u8;
    static mut DAT_004fd1f0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004de0b0(a: i32, b: i32);
    fn FUN_004b4a60(a: *mut i8, b: *mut i8);
    fn FUN_004e5000(a: i32);
    fn FUN_004d4e20();
    fn FUN_004b5de0(a: *mut i8, b: i32, c: i32);
    fn FUN_004b5570(a: *mut i8, b: i32, c: i32);
    fn FUN_004b4d40(a: *mut i8, b: i32, c: i32);
    fn FUN_004d4ec0(a: i32);
    fn FUN_004dc140(a: i32);
    fn _malloc(a: u32) -> *mut c_void;
    fn FUN_004e5590(a: *mut c_void);
    fn FUN_004fcf30();
    fn FUN_004e0490() -> i32;
    fn FUN_004cd9d0();
    fn FUN_004cfee0() -> i32;
    fn FUN_004ddc10(a: u32) -> *mut i8;
    fn FUN_004dcb90(a: *mut i8, b: *mut i8, c: *mut i8, d: *mut i8, e: *mut i8) -> i32;
    fn FUN_004dc330(a: *const c_void, b: *const c_void);
    fn FUN_004d73c0();
    fn FUN_004f5170();
    fn FUN_004e15d0();
    fn FUN_004fa420();
    fn FUN_004f70f0();
    fn FUN_004d6460(a: *mut i32, b: i32, c: i32, d: i32, e: i32);
    fn FUN_004d4cf0(a: *mut c_void);
    fn FUN_004d4d20(a: *mut c_void);
    fn FUN_004db640(a: i32, b: *mut i8, c: *mut i32, d: u32);
    fn FUN_004d64c0(a: *mut i32, b: i32, c: i32);
    fn FUN_004db7d0(a: i32, b: *mut i8, c: *mut i32, d: i32, e: i32, f: u32);
    fn FUN_004fcbf0();
    fn FUN_004d8690(a: u32);
    fn _sprintf(a: *mut i8, b: *const i8, ...) -> i32;
    fn FUN_004c9bf0(a: i32);
    fn FUN_004cef70(a: i32, b: *mut u32, c: i32);
    fn FUN_004d11e0() -> i32;
    fn FUN_004d11d0() -> i32;
    fn FUN_004d64a0(a: *mut i32, b: i32, c: i32);
    fn FUN_004fcd70(a: u32);
    fn FUN_004dad40();
    fn FUN_004dad70(a: *mut i8);
    fn FUN_004da780(
        a: *mut i32,
        b: *mut i8,
        c: *mut i8,
        d: *mut i8,
        e: i32,
        f: u32,
        g: i32,
        h: *mut i8,
    ) -> i32;
    fn FUN_004dad60();
    fn FUN_004dc380(a: *mut c_void, b: *mut i32);
    fn FUN_004bf9b0(a: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetTickCount() -> u32;
}

#[allow(non_snake_case)]
unsafe fn rd(p: *mut u8) -> i32 {
    // SAFETY: caller passes a valid game address.
    unsafe { p.cast::<i32>().read_unaligned() }
}

#[allow(non_snake_case)]
unsafe fn rdi(p: *mut u8, idx: i32) -> i32 {
    // SAFETY: caller passes a valid game address; offset is idx*4 bytes.
    unsafe { rd(p.byte_add((idx as u32).wrapping_mul(4) as usize)) }
}

#[allow(non_snake_case)]
unsafe fn wr(p: *mut u8, v: i32) {
    // SAFETY: caller passes a valid game address.
    unsafe { p.cast::<i32>().write_unaligned(v) }
}

#[allow(non_snake_case)]
unsafe fn tbl(idx: i32) -> i32 {
    // SAFETY: game table DAT_00aa5cb0, index scaled by 8 ints.
    unsafe {
        (&raw mut DAT_00aa5cb0)
            .add((idx as u32).wrapping_mul(8) as usize)
            .read()
    }
}

#[allow(non_snake_case)]
fn absm(p: u32) -> i32 {
    let u = ((p as i32) >> 31) as u32;
    (((p ^ u).wrapping_sub(u) & 0x1f) ^ u).wrapping_sub(u) as i32
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004fbe90(param_1: u32) -> u8 {
    let mut local_20: [i32; 4] = [0; 4];
    let mut local_10: [u32; 4] = [0; 4];
    let l20 = local_20.as_mut_ptr();
    // SAFETY: all globals and callees are the game's own; pointers refer to
    // game memory or locals, accessed where the original does.
    unsafe {
        if param_1 == 0 {
            DAT_0066d898 = 0;
            FUN_004de0b0(0x25, 0x1bd);
            DAT_0052c208 = 0;
            FUN_004b4a60(&raw mut DAT_0052c890, &raw mut DAT_0052bf74);
            let ta = rd(&raw mut DAT_00b6a5aa);
            FUN_004e5000(ta);
            if DAT_00aa6114 != 0 {
                DAT_00aa6134 = 0;
                if DAT_00aa6138 != 0 {
                    let mut p4 = (DAT_00aa6114 as u32).wrapping_add(0x140) as *mut i32;
                    loop {
                        let ta = rd(&raw mut DAT_00b6a5aa);
                        let tb = rdi(&raw mut DAT_00b6a5ae, ta);
                        if p4.read() == tb {
                            break;
                        }
                        p4 = p4.wrapping_add(0x52);
                        DAT_00aa6134 = DAT_00aa6134.wrapping_add(1);
                        if !(DAT_00aa6134 < DAT_00aa6138) {
                            break;
                        }
                    }
                }
            }
            FUN_004d4e20();
            FUN_004b5de0(&raw mut DAT_0052bf74, 0, 0);
            FUN_004b5570(&raw mut DAT_0052a6d4, 0, 0);
            let s = if DAT_00b6a250 == 0 {
                &raw mut DAT_0052b69c
            } else {
                &raw mut DAT_0052b6ac
            };
            FUN_004b5570(s, 0, 0);
            if DAT_00b69e30 != 2 {
                FUN_004b4d40(&raw mut DAT_0052bfac, 0, 0);
            }
            FUN_004b4d40(&raw mut DAT_0052c374, 0, 0);
            FUN_004d4ec0(1);
            FUN_004dc140(0x14);
            DAT_0066d888 = 120000;
            DAT_0066d880 = GetTickCount() as i32;
            DAT_0066d894 = 0x78;
            DAT_0066d88c = DAT_0066d880;
            DAT_00aa6118 = _malloc(0x1000);
            FUN_004e5590(DAT_00aa6118);
            FUN_004fcf30();
        }
        if DAT_00b69e30 != 2 {
            let mut iv = FUN_004e0490();
            if iv == 0x46 {
                FUN_004cd9d0();
                iv = FUN_004cfee0();
                if iv == 0 {
                    let p6 = FUN_004ddc10(0x279);
                    let p2 = FUN_004ddc10(0x278);
                    let p3 = FUN_004ddc10(0x277);
                    FUN_004dcb90(p3, p2, p6, core::ptr::null_mut(), core::ptr::null_mut());
                }
                DAT_0052c184 = 1;
                DAT_00b69e30 = 3;
                FUN_004dc330(
                    FUN_004d73c0 as *const c_void,
                    &raw const DAT_004d7370 as *const c_void,
                );
            } else if iv == 0x41 {
                FUN_004fa420();
            } else {
                if iv == 0x58 {
                    let b5 = DAT_00aa6114 != 0;
                    wr(&raw mut DAT_00b6bbaa, DAT_0052c18c);
                    let ta = rd(&raw mut DAT_00b6bba6);
                    wr(
                        (&raw mut DAT_00b6bbda).byte_add((ta as u32).wrapping_mul(4) as usize),
                        DAT_0052c18c,
                    );
                    if b5 {
                        DAT_00aa6134 = 0;
                        if DAT_00aa6138 != 0 {
                            let mut p4 = (DAT_00aa6114 as u32).wrapping_add(0x140) as *mut i32;
                            loop {
                                if p4.read() == DAT_0052c18c {
                                    let ta = rd(&raw mut DAT_00b6bba6);
                                    wr(
                                        (&raw mut DAT_00b6bc02)
                                            .byte_add((ta as u32).wrapping_mul(4) as usize),
                                        DAT_00aa6134 as i32,
                                    );
                                    break;
                                }
                                p4 = p4.wrapping_add(0x52);
                                DAT_00aa6134 = DAT_00aa6134.wrapping_add(1);
                                if !(DAT_00aa6134 < DAT_00aa6138) {
                                    break;
                                }
                            }
                        }
                    }
                    FUN_004f70f0();
                    FUN_004dc330(
                        FUN_004f5170 as *const c_void,
                        &raw const DAT_004f5100 as *const c_void,
                    );
                    return 0;
                }
                if iv == 0x53 {
                    DAT_0052c184 = 1;
                    FUN_004dc330(
                        FUN_004e15d0 as *const c_void,
                        &raw const DAT_004e1470 as *const c_void,
                    );
                    return 0;
                }
                if iv == 100 {
                    if DAT_0052c18c < DAT_0066d888 {
                        DAT_0066d888 = DAT_0052c18c;
                    }
                } else if iv == 0x75 {
                    let mut i: i32 = 0;
                    let mut pb = &raw mut DAT_00aa615e;
                    loop {
                        let ta = rd(pb);
                        if ta == DAT_0052c188 {
                            wr(
                                (&raw mut DAT_00aa6162)
                                    .byte_add((i as u32).wrapping_mul(0x2a) as usize),
                                DAT_0052c18c,
                            );
                            break;
                        }
                        pb = pb.wrapping_add(0x2a);
                        i = i.wrapping_add(1);
                        if !(pb < &raw mut DAT_00aa62ae) {
                            break;
                        }
                    }
                }
            }
        }
        FUN_004d6460(l20, 0x9e, 0x34, 0x1eb, 0x44);
        FUN_004d4cf0(local_10.as_mut_ptr().cast());
        FUN_004d4d20(l20.cast());
        FUN_004db640(0xc, &raw mut DAT_00a91b90, l20, 0xffff);
        FUN_004d4d20(local_10.as_mut_ptr().cast());
        FUN_004d6460(l20, 0x54, 0x5a, 0x1b2, 0x6c);
        let p6 = FUN_004ddc10(0x32c);
        FUN_004db640(0xf, p6, l20, 0xffff);
        let iv = FUN_004cfee0();
        let u9: u32 = if iv == 0 && DAT_00b69e30 != 2 {
            0x1b2
        } else {
            0x1a0
        };
        FUN_004d6460(l20, 0x54, 0x70, u9 as i32, 0x82);
        if DAT_00aa6134 < DAT_00aa6138 {
            FUN_004d64c0(l20, 4, 0);
            let a = DAT_00aa6134
                .wrapping_mul(0x148)
                .wrapping_add(DAT_00aa6114 as u32)
                .wrapping_add(0x40);
            FUN_004db7d0(0xf, a as *mut i8, l20, 0, 1, DAT_00b6a2c0);
        }
        FUN_004fcbf0();
        if DAT_00b69e30 != 2 {
            FUN_004d8690(param_1);
        }
        FUN_004d6460(l20, 200, 0x1c4, 0x1b4, 0x1d0);
        if DAT_00b6a2e0 != 0 {
            _sprintf(
                &raw mut DAT_00b69d20,
                &raw const DAT_0052a6b0,
                6i32,
                &raw mut DAT_00b6bc9e,
                1i32,
                &raw mut DAT_00b6a2e0,
            );
            FUN_004db640(0xc, &raw mut DAT_00b69d20, l20, DAT_00b6a2c0);
            let iv = absm(param_1);
            let mut do_both = true;
            if DAT_00b69e30 == 2 {
                let ta = rd(&raw mut DAT_00b6a5aa);
                if ta == 1 || ta == 2 {
                    do_both = true;
                } else {
                    do_both = false;
                    let tb = rd(&raw mut DAT_00b6d962);
                    if tb == 0 {
                        _sprintf(&raw mut DAT_00b69d20, &raw const DAT_0052a6a4, iv >> 1);
                        FUN_004b5570(&raw mut DAT_00b69d20, 0xcc, 0x1c5);
                    } else {
                        _sprintf(&raw mut DAT_00b69d20, &raw const DAT_0052a698, iv >> 1);
                        FUN_004b5570(&raw mut DAT_00b69d20, 0xcc, 0x1c5);
                    }
                }
            }
            if do_both {
                _sprintf(&raw mut DAT_00b69d20, &raw const DAT_0052a6a4, iv >> 1);
                FUN_004b5570(&raw mut DAT_00b69d20, 0xcc, 0x1c5);
                _sprintf(&raw mut DAT_00b69d20, &raw const DAT_0052a698, iv >> 1);
            }
            FUN_004b5570(&raw mut DAT_00b69d20, 0x1a4, 0x1c5);
        }
        if DAT_00b69e30 != 2 {
            FUN_004d6460(l20, 0x1fb, 0x1c4, 0x232, 0x1d0);
            FUN_004c9bf0(DAT_0066d888 / 1000);
            FUN_004db640(0xc, &raw mut DAT_00b69d20, l20, 0xffff);
            if DAT_00b69e30 != 2 {
                DAT_0066d880 = GetTickCount() as i32;
                DAT_0066d888 = (DAT_0066d888 as u32)
                    .wrapping_add((DAT_0066d88c as u32).wrapping_sub(DAT_0066d880 as u32))
                    as i32;
                let iv = FUN_004cfee0();
                if iv != 0 && DAT_0066d894 != DAT_0066d888 / 1000 {
                    DAT_00aa6af4 = DAT_0066d888;
                    DAT_00aa6af0 = 100;
                    DAT_0066d894 = DAT_0066d888 / 1000;
                    FUN_004cef70(0, &raw mut DAT_00aa6af0, 8);
                }
                if DAT_0066d888 < 0 {
                    DAT_0066d888 = 0;
                    if DAT_0066d898 == 0 {
                        DAT_0066d898 = 1;
                        let ta = rd(&raw mut DAT_00b6bba6);
                        let tb = rdi(&raw mut DAT_00b6bbae, ta);
                        let tc = rd(&raw mut DAT_00b6a5a6);
                        if tb != tc {
                            let iv = FUN_004d11e0();
                            if iv == tbl(tc) {
                                DAT_00aa6af4 = rd(&raw mut DAT_00b6a5ba);
                                DAT_00aa6af0 = 0x7d;
                                let iv = FUN_004d11d0();
                                FUN_004cef70(iv, &raw mut DAT_00aa6af0, 4);
                            }
                        }
                    }
                    return 0;
                }
                DAT_0066d88c = DAT_0066d880;
            }
        }
        FUN_004d6460(l20, 0x54, 400, 0x1b2, 0x19e);
        let u7: u32;
        let u14: u32;
        if DAT_00b69e30 == 2 {
            let p = FUN_004ddc10(0x32d);
            FUN_004db640(0xc, p, l20, DAT_00b6a2a0);
            FUN_004d64a0(l20, 0, 0xf);
            u7 = 0x32e;
            u14 = DAT_00b6a2a0;
        } else {
            let ta = rd(&raw mut DAT_00b6bba6);
            let tb = rdi(&raw mut DAT_00b6bbae, ta);
            let tc = rd(&raw mut DAT_00b6a5a6);
            if tb == tc {
                let p = FUN_004ddc10(0x32f);
                FUN_004db640(0xc, p, l20, DAT_00b69e38);
                FUN_004d64a0(l20, 0, 0xf);
                u7 = 0x330;
                u14 = DAT_00b69e38;
            } else {
                let iv = FUN_004d11e0();
                if iv == tbl(tc) {
                    let p = FUN_004ddc10(0x331);
                    FUN_004db640(0xc, p, l20, DAT_00b6a2a0);
                    FUN_004d64a0(l20, 0, 0xf);
                    u7 = 0x32e;
                    u14 = DAT_00b6a2a0;
                } else {
                    let p = FUN_004ddc10(0x331);
                    FUN_004db640(0xc, p, l20, DAT_00b69e38);
                    FUN_004d64a0(l20, 0, 0xf);
                    u7 = 0x332;
                    u14 = DAT_00b69e38;
                }
            }
        }
        let p = FUN_004ddc10(u7);
        FUN_004db640(0xc, p, l20, u14);
        FUN_004fcd70(param_1);
        FUN_004d6460(l20, 0x55, 0x1bf, 0xb0, 0x1d7);
        if DAT_00bb73a8 != 0 {
            FUN_004dad40();
        }
        'l9d8: {
            if DAT_00b69e30 == 2 {
                let p = FUN_004ddc10(0x239);
                FUN_004dad70(p);
                let p6 = FUN_004ddc10(0x104);
                let iv = FUN_004da780(
                    l20,
                    &raw mut DAT_0052a678,
                    &raw mut DAT_0052a680,
                    p6,
                    0xc,
                    0,
                    8,
                    &raw mut DAT_0052a68c,
                );
                if iv == 0 {
                    break 'l9d8;
                }
                let p6 = FUN_004ddc10(0x13);
                let p2 = FUN_004ddc10(0x20b);
                let p3 = FUN_004ddc10(0x2ab);
                let p8 = FUN_004ddc10(0x2aa);
                let p13 = FUN_004ddc10(0x2a9);
                let iv = FUN_004dcb90(p13, p8, p3, p2, p6);
                if iv == 0 {
                    break 'l9d8;
                }
                DAT_0052c184 = 1;
                FUN_004dc330(
                    FUN_004e15d0 as *const c_void,
                    &raw const DAT_004e1470 as *const c_void,
                );
            } else {
                let iv = FUN_004cfee0();
                if iv != 0 {
                    let p = FUN_004ddc10(0x29c);
                    FUN_004dad70(p);
                    let p6 = FUN_004ddc10(0x104);
                    let iv = FUN_004da780(
                        l20,
                        &raw mut DAT_0052a678,
                        &raw mut DAT_0052a680,
                        p6,
                        0xc,
                        0,
                        8,
                        &raw mut DAT_0052a68c,
                    );
                    if iv != 0 {
                        let p6 = FUN_004ddc10(0x13);
                        let p2 = FUN_004ddc10(0x20b);
                        let p3 = FUN_004ddc10(0x2f2);
                        let p8 = FUN_004ddc10(0x2f1);
                        let p13 = FUN_004ddc10(0x2f0);
                        let iv = FUN_004dcb90(p13, p8, p3, p2, p6);
                        if iv != 0 {
                            DAT_00aa6af0 = 0x53;
                            FUN_004cef70(0, &raw mut DAT_00aa6af0, 4);
                        }
                    }
                    break 'l9d8;
                }
                let p = FUN_004ddc10(0xcc);
                FUN_004dad70(p);
                let p6 = FUN_004ddc10(0xcc);
                let iv = FUN_004da780(
                    l20,
                    &raw mut DAT_0052a678,
                    &raw mut DAT_0052a680,
                    p6,
                    0xc,
                    0,
                    8,
                    &raw mut DAT_0052a68c,
                );
                if iv == 0 {
                    break 'l9d8;
                }
                let p6 = FUN_004ddc10(0x13);
                let p2 = FUN_004ddc10(0x20b);
                let p3 = FUN_004ddc10(0x22d);
                let p8 = FUN_004ddc10(0x22c);
                let p13 = FUN_004ddc10(0x22b);
                let iv = FUN_004dcb90(p13, p8, p3, p2, p6);
                if iv == 0 {
                    break 'l9d8;
                }
                DAT_0052c184 = 1;
                DAT_00aa6af0 = 0x47;
                let iv = FUN_004d11d0();
                FUN_004cef70(iv, &raw mut DAT_00aa6af0, 4);
                FUN_004cd9d0();
                FUN_004dc330(
                    FUN_004d73c0 as *const c_void,
                    &raw const DAT_004d7370 as *const c_void,
                );
            }
        }
        FUN_004d6460(l20, 8, 0x195, 0x47, 0x1d6);
        if DAT_00b69e30 == 2 {
            let p = FUN_004ddc10(0xd4);
            FUN_004dad70(p);
            let p6 = FUN_004ddc10(0x1db);
            let iv = FUN_004da780(
                l20,
                &raw mut DAT_0052bf80,
                &raw mut DAT_0052bf88,
                p6,
                0xc,
                0,
                7,
                &raw mut DAT_0052bf94,
            );
            if iv != 0 {
                FUN_004dc330(
                    FUN_004f5170 as *const c_void,
                    &raw const DAT_004f5100 as *const c_void,
                );
                FUN_004dad60();
                return 0;
            }
        } else {
            let ta = rd(&raw mut DAT_00b6bba6);
            let tb = rdi(&raw mut DAT_00b6bbae, ta);
            let tc = rd(&raw mut DAT_00b6a5a6);
            if tb != tc && FUN_004d11e0() == tbl(tc) {
                let p = FUN_004ddc10(0xd4);
                FUN_004dad70(p);
                let p6 = FUN_004ddc10(0x1db);
                let iv = FUN_004da780(
                    l20,
                    &raw mut DAT_0052bf80,
                    &raw mut DAT_0052bf88,
                    p6,
                    0xc,
                    0,
                    7,
                    &raw mut DAT_0052bf94,
                );
                if iv != 0 {
                    DAT_00aa6af4 = rd(&raw mut DAT_00b6a5ba);
                    DAT_00aa6af0 = 0x7d;
                    let iv = FUN_004d11d0();
                    FUN_004cef70(iv, &raw mut DAT_00aa6af0, 4);
                }
            }
        }
        let mut b5: u8 = 0;
        FUN_004dad60();
        if DAT_00b69e30 == 2 {
            b5 = 1;
        } else {
            let ta = rd(&raw mut DAT_00b6bba6);
            let tb = rdi(&raw mut DAT_00b6bbae, ta);
            let tc = rd(&raw mut DAT_00b6a5a6);
            if tb != tc && FUN_004d11e0() == tbl(tc) {
                b5 = 1;
            }
        }
        if b5 != 0 {
            FUN_004d6460(l20, 0x1a1, 0x70, 0x1b2, 0x82);
            let p6 = FUN_004ddc10(0x2ac);
            let iv = FUN_004da780(
                l20,
                &raw mut DAT_0052c21c,
                &raw mut DAT_0052c228,
                p6,
                0xf,
                0xffff,
                9,
                &raw mut DAT_0052a68c,
            );
            if iv != 0 {
                FUN_004d6460(l20, 0, 0, 0x27f, 0x1df);
                FUN_004dc380(&raw mut DAT_004fd1f0 as *mut c_void, l20);
            }
        }
        let iv = FUN_004bf9b0(1);
        (iv == 1) as u8
    }
}
