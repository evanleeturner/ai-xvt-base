// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b69ce0: i32;
    static mut DAT_00bb72d0: i8;
    static mut DAT_00bb72d1: i8;
    static mut DAT_00bb7311: i8;
    static mut DAT_0052c184: i32;
    static mut DAT_00b69e30: i32;
    static mut DAT_00aa6150: [u32; 0x54];
    static mut DAT_00665440: [u32; 8];
    static mut DAT_00b69cd0: *mut u32;
    static mut DAT_00b6a2c4: i32;
    static mut DAT_00aa62b0: [u32; 0x1f0];
    static mut DAT_00aa6cf4: i32;
    static mut DAT_0052c208: i32;
    static mut DAT_00aa613c: i32;
    static mut DAT_00aa6cf8: i32;
    static mut DAT_00aa6ae8: u32;
    static mut DAT_0052bf50: i32;
    static mut DAT_00b6a2a8: i32;
    static mut DAT_00aa6118: *mut u8;
    static mut DAT_00b6a250: i32;
    static mut DAT_00aa6114: i32;
    static mut DAT_00aa6134: i32;
    static mut DAT_00aa6138: i32;
    static mut DAT_00aa6144: i32;
    static mut DAT_00aa6a70: u32;
    static mut DAT_00aa6a74: u8;
    static mut DAT_00aa62e0: u8;
    static mut DAT_00aa62e4: u8;
    static mut DAT_00aa62e8: u8;
    static mut DAT_00aa62ec: u8;
    static mut DAT_00b6a5aa: [u8; 4];
    static mut DAT_00b6a5ae: [u8; 0];
    static mut DAT_00aa6cf0: u32;
    static mut DAT_00b69d20: [i8; 0];
    static mut DAT_00b6a2e0: i8;
    static mut DAT_00b6bc9e: i8;
    static mut DAT_00b6a2c0: u32;
    static mut DAT_00bb73a8: i8;
    static mut DAT_00b6a5c6: [i8; 0];
    static mut DAT_00bb735f: [u8; 4];
    static mut DAT_00bb7363: [u8; 4];
    static mut DAT_00bb7367: [u8; 4];
    static mut DAT_00bb736b: [u8; 4];
    static mut DAT_00aa6af0: u32;
    static mut DAT_00aa6af4: u32;
    static mut DAT_00aa6af8: u32;
    static mut DAT_00aa6afc: u32;
    static mut DAT_00aa6b00: u32;
    static mut DAT_00aa6b04: u32;

    static mut LAB_004be050: i8;

    static mut DAT_0052bfb4: [i8; 0];
    static mut DAT_0052bf74: [i8; 0];
    static mut DAT_0052a6d4: [i8; 0];
    static mut DAT_0052b69c: [i8; 0];
    static mut DAT_0052b6ac: [i8; 0];
    static mut DAT_0052bfac: [i8; 0];
    static mut DAT_0052bfa0: [i8; 0];
    static mut DAT_0052b1b4: [i8; 0];
    static mut DAT_0052a6b0: [i8; 0];
    static mut DAT_0052a6a4: [i8; 0];
    static mut DAT_0052a698: [i8; 0];
    static mut DAT_0052a68c: [i8; 0];
    static mut DAT_0052a678: [i8; 0];
    static mut DAT_0052a680: [i8; 0];
    static mut DAT_0052bf94: [i8; 0];
    static mut DAT_0052bf80: [i8; 0];
    static mut DAT_0052bf88: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004be1b0();
    fn FUN_004b7f30();
    fn FUN_004d7e70();
    fn FUN_004dc330(a: *mut c_void, b: *mut c_void);
    fn FUN_004d8c20(a: u32) -> u32;
    fn FUN_004ddc10(a: u32) -> *mut i8;
    fn FUN_004dcb90(a: *mut i8, b: *mut i8, c: *mut i8, d: *mut i8, e: *mut i8) -> i32;
    fn FUN_004d6460(a: *mut i32, b: i32, c: i32, d: i32, e: i32);
    fn FUN_004dc380(a: *mut c_void, b: *mut c_void);
    fn FUN_004de0b0(a: i32, b: i32);
    fn FUN_004d1420();
    fn _malloc(a: u32) -> *mut c_void;
    fn FUN_004b4a60(a: *mut i8, b: *mut i8);
    fn FUN_004d4e20();
    fn FUN_004b5de0(a: *mut i8, b: i32, c: i32);
    fn FUN_004b5570(a: *mut i8, b: i32, c: i32);
    fn FUN_004b4d40(a: *mut i8, b: i32, c: i32);
    fn FUN_004d4ec0(a: i32);
    fn FUN_004dc140(a: i32);
    fn FUN_004db640(a: i32, b: *mut i8, c: *mut i32, d: u32);
    fn FUN_004d92b0();
    fn FUN_004e0490() -> i32;
    fn FUN_004f1680();
    fn FUN_004e5000(a: i32);
    fn FUN_004e5590(a: *mut u8);
    fn FUN_004cd9d0();
    fn FUN_004d7020(a: u32) -> i32;
    fn FUN_004d97f0(a: i32) -> i32;
    fn FUN_004d9680();
    fn FUN_004dc310();
    fn FUN_004d8540();
    fn FUN_004d8350();
    fn FUN_004d8690(a: u32);
    fn _sprintf(a: *mut i8, b: *const i8, ...) -> i32;
    fn FUN_004d9170();
    fn FUN_004bf9b0(a: i32) -> i32;
    fn FUN_004dad40();
    fn FUN_004dad70(a: *mut i8);
    fn FUN_004da780(
        a: *mut i32,
        b: *mut i8,
        c: *mut i8,
        d: *mut i8,
        e: i32,
        f: i32,
        g: i32,
        h: *mut i8,
    ) -> i32;
    fn FUN_004d11d0() -> i32;
    fn FUN_004cef70(a: i32, b: *mut u32, c: i32);
    fn FUN_004dad60();
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetTickCount() -> u32;
}

unsafe fn clear_665440() {
    // SAFETY: the game's own array, cleared word by word as the original does.
    unsafe {
        let mut p = (&raw mut DAT_00665440).cast::<u32>();
        let mut n = 8;
        while n != 0 {
            p.write_unaligned(0);
            p = p.add(1);
            n -= 1;
        }
    }
}

unsafe fn clear_118() {
    // SAFETY: DAT_00aa6118 points at the game's 0x1000-byte buffer.
    unsafe {
        let mut p = DAT_00aa6118;
        let mut n = 0x400;
        while n != 0 {
            *p = 0;
            *p.add(1) = 0;
            *p.add(2) = 0;
            *p.add(3) = 0;
            p = p.add(4);
            n -= 1;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d73c0(param_1: u32) -> u32 {
    // SAFETY: all globals and callees are the game's own; pointers are used
    // exactly where the original uses them.
    unsafe {
        let mut local_20: [i32; 4] = [0; 4];
        let mut local_10: [i32; 4] = [0; 4];

        DAT_00b69ce0 = 0;
        if DAT_00bb72d0 != 0 {
            if DAT_0052c184 != 0 {
                DAT_00b69e30 = 0;
                FUN_004dc330(
                    FUN_004be1b0 as *const c_void as *mut c_void,
                    (&raw mut LAB_004be050).cast::<c_void>(),
                );
                return 0;
            }
            if DAT_00b69e30 == 3 {
                if DAT_00bb72d0 == 1 {
                    if DAT_00bb7311 == 0 {
                        let pc4 = FUN_004ddc10(0x13);
                        let pc5 = FUN_004ddc10(0x20b);
                        let pc6 = FUN_004ddc10(0x2b7);
                        let pc7 = FUN_004ddc10(0x2b6);
                        let pc8 = FUN_004ddc10(0x2b5);
                        let r = FUN_004dcb90(pc8, pc7, pc6, pc5, pc4);
                        if r == 0 {
                            DAT_00b69e30 = 0;
                            FUN_004dc330(
                                FUN_004be1b0 as *const c_void as *mut c_void,
                                (&raw mut LAB_004be050).cast::<c_void>(),
                            );
                            return 0;
                        }
                        FUN_004d6460(local_10.as_mut_ptr(), 0, 0, 0x280, 0x1e0);
                        FUN_004dc380(
                            FUN_004b7f30 as *const c_void as *mut c_void,
                            local_10.as_mut_ptr().cast::<c_void>(),
                        );
                        return 0;
                    }
                } else if DAT_00bb72d0 == 2 && DAT_00bb72d1 == 0 {
                    let pc4 = FUN_004ddc10(0x13);
                    let pc5 = FUN_004ddc10(0x20b);
                    let pc6 = FUN_004ddc10(0x2ba);
                    let pc7 = FUN_004ddc10(0x2b9);
                    let pc8 = FUN_004ddc10(0x2b8);
                    let r = FUN_004dcb90(pc8, pc7, pc6, pc5, pc4);
                    if r == 0 {
                        DAT_00b69e30 = 0;
                        FUN_004dc330(
                            FUN_004be1b0 as *const c_void as *mut c_void,
                            (&raw mut LAB_004be050).cast::<c_void>(),
                        );
                        return 0;
                    }
                    FUN_004d6460(local_10.as_mut_ptr(), 0, 0, 0x280, 0x1e0);
                    FUN_004dc380(
                        FUN_004b7f30 as *const c_void as *mut c_void,
                        local_10.as_mut_ptr().cast::<c_void>(),
                    );
                    return 0;
                }
            }
            return FUN_004d8c20(param_1);
        }
        if param_1 == 0 {
            FUN_004de0b0(0x19f, 0x79);
            let mut p = (&raw mut DAT_00aa6150).cast::<u32>();
            let mut n = 0x54;
            while n != 0 {
                p.write_unaligned(0);
                p = p.add(1);
                n -= 1;
            }
            FUN_004d1420();
            clear_665440();
            if !DAT_00b69cd0.is_null() {
                let mut p = DAT_00b69cd0;
                let mut n = 0x100;
                while n != 0 {
                    p.write_unaligned(0);
                    p = p.add(1);
                    n -= 1;
                }
                DAT_00b6a2c4 = 0;
            }
            if DAT_0052c184 == 0 {
                let mut p = (&raw mut DAT_00aa62b0).cast::<u32>();
                let mut n = 0x1f0;
                while n != 0 {
                    p.write_unaligned(0);
                    p = p.add(1);
                    n -= 1;
                }
                DAT_00aa6cf4 = 0;
            }
            DAT_0052c208 = 0;
            DAT_00b69ce0 = 0;
            DAT_00aa613c = 0;
            DAT_00aa6cf8 = 0;
            DAT_00aa6ae8 = 0xffffffff;
            DAT_0052bf50 = -1;
            DAT_00b6a2a8 = 0;
            DAT_0052c184 = 0;
            if DAT_00aa6118.is_null() {
                DAT_00aa6118 = _malloc(0x1000).cast::<u8>();
            }
            FUN_004b4a60(
                (&raw mut DAT_0052bfb4).cast::<i8>(),
                (&raw mut DAT_0052bf74).cast::<i8>(),
            );
            FUN_004d4e20();
            FUN_004b5de0((&raw mut DAT_0052bf74).cast::<i8>(), 0, 0);
            FUN_004b5570((&raw mut DAT_0052a6d4).cast::<i8>(), 0, 0);
            let pc4 = if DAT_00b6a250 == 0 {
                (&raw mut DAT_0052b69c).cast::<i8>()
            } else {
                (&raw mut DAT_0052b6ac).cast::<i8>()
            };
            FUN_004b5570(pc4, 0, 0);
            FUN_004b4d40((&raw mut DAT_0052bfac).cast::<i8>(), 0, 0);
            FUN_004b4d40((&raw mut DAT_0052bfa0).cast::<i8>(), 0, 0);
            FUN_004d4ec0(1);
            FUN_004dc140(0x14);
        }
        FUN_004d6460(local_20.as_mut_ptr(), 0x9e, 0x34, 0x1eb, 0x44);
        FUN_004db640(
            0xc,
            (&raw mut DAT_00665440).cast::<i8>(),
            local_20.as_mut_ptr(),
            0xffff,
        );
        let u_var12: u32 = ((param_1 as i32) >> 0x1f) as u32;

        let mut skip = false;
        if (param_1 as i32) % 0xa0 == 0 {
            if DAT_0052bf50 == -1 {
                FUN_004d92b0();
                if DAT_0052bf50 == -1 {
                    skip = true;
                }
            }
        } else if DAT_0052bf50 == -1 {
            skip = true;
        }

        'l7a1e: {
            'l7a03: {
                if skip {
                    break 'l7a1e;
                }
                let iv = FUN_004e0490();
                if iv == 0x41 {
                    let tick = GetTickCount();
                    let idx = (DAT_0052bf50 as u32).wrapping_mul(0x3e) as usize;
                    (&raw mut DAT_00aa62e4)
                        .byte_add(idx)
                        .cast::<u32>()
                        .write_unaligned(tick);
                    let u2 = DAT_00aa6a74;
                    (&raw mut DAT_00aa62e0)
                        .byte_add(idx)
                        .cast::<u32>()
                        .write_unaligned(DAT_00aa6a70);
                    (&raw mut DAT_00aa62ec).byte_add(idx).write(u2);
                    FUN_004f1680();
                    (&raw mut DAT_00b6a5aa)
                        .cast::<i32>()
                        .write_unaligned(DAT_00aa6cf8);
                    (&raw mut DAT_00b6a5ae)
                        .cast::<u8>()
                        .byte_add((DAT_00aa6cf8 as u32).wrapping_mul(4) as usize)
                        .cast::<u32>()
                        .write_unaligned(DAT_00aa6ae8);
                    let t: i32 = (&raw const DAT_00b6a5aa).cast::<i32>().read_unaligned();
                    FUN_004e5000(t);
                    FUN_004e5590(DAT_00aa6118);
                    if DAT_00aa6114 != 0 {
                        DAT_00aa6134 = 0;
                        if DAT_00aa6138 != 0 {
                            let mut p13 = (DAT_00aa6114.wrapping_add(0x140)) as *const i32;
                            loop {
                                let t: i32 =
                                    (&raw const DAT_00b6a5aa).cast::<i32>().read_unaligned();
                                let t2: i32 = (&raw const DAT_00b6a5ae)
                                    .cast::<u8>()
                                    .byte_add((t as u32).wrapping_mul(4) as usize)
                                    .cast::<i32>()
                                    .read_unaligned();
                                if p13.read_unaligned() == t2 {
                                    break;
                                }
                                p13 = p13.wrapping_add(0x52);
                                DAT_00aa6134 = DAT_00aa6134.wrapping_add(1);
                                if !(DAT_00aa6134 < DAT_00aa6138) {
                                    break;
                                }
                            }
                        }
                    }
                    DAT_0052c208 = 0;
                    break 'l7a1e;
                }
                if iv == 0x46
                    || iv == 0x49
                    || iv == 0x4b
                    || iv == 0x4c
                    || iv == 0x5a
                    || iv == 0x66
                    || iv == 0x76
                {
                    FUN_004cd9d0();
                    FUN_004d92b0();
                    DAT_00aa6cf0 = 0;
                    DAT_0052bf50 = -1;
                    DAT_00aa6ae8 = 0xffffffff;
                    break 'l7a03;
                }
                if iv == 0x69 {
                    FUN_004cd9d0();
                    DAT_00aa6cf0 = 0;
                    DAT_0052bf50 = -1;
                    DAT_00aa6ae8 = 0xffffffff;
                    FUN_004d92b0();
                    break 'l7a03;
                }
                if iv != 0x44 && iv != 0x5e && iv != 0x5f && iv != 0x60 {
                    break 'l7a1e;
                }
                FUN_004cd9d0();
                'sw: {
                    let pc4: *mut i8;
                    let pc5: *mut i8;
                    let pc6: *mut i8;
                    let pc7: *mut i8;
                    let uv: u32;
                    match iv {
                        0x44 => {
                            pc7 = core::ptr::null_mut();
                            pc6 = core::ptr::null_mut();
                            pc4 = FUN_004ddc10(0x221);
                            pc5 = FUN_004ddc10(0x220);
                            uv = 0x21f;
                        }
                        0x5e => {
                            pc7 = core::ptr::null_mut();
                            pc6 = core::ptr::null_mut();
                            pc4 = FUN_004ddc10(0x224);
                            pc5 = FUN_004ddc10(0x223);
                            uv = 0x222;
                        }
                        0x5f => {
                            let pc8 = core::ptr::null_mut();
                            let pc7 = core::ptr::null_mut();
                            let pc4 = FUN_004ddc10(0x227);
                            let pc5 = FUN_004ddc10(0x226);
                            let pc6 = FUN_004ddc10(0x225);
                            FUN_004dcb90(pc6, pc5, pc4, pc7, pc8);
                            FUN_004d6460(local_10.as_mut_ptr(), 0, 0, 0x280, 0x1e0);
                            FUN_004dc380(
                                FUN_004b7f30 as *const c_void as *mut c_void,
                                local_10.as_mut_ptr().cast::<c_void>(),
                            );
                            break 'sw;
                        }
                        0x60 => {
                            pc7 = core::ptr::null_mut();
                            pc6 = core::ptr::null_mut();
                            pc4 = FUN_004ddc10(0x22a);
                            pc5 = FUN_004ddc10(0x229);
                            uv = 0x228;
                        }
                        _ => break 'sw,
                    }
                    let pc8 = FUN_004ddc10(uv);
                    FUN_004dcb90(pc8, pc5, pc4, pc6, pc7);
                }
                DAT_00aa6cf0 = 0;
                DAT_0052bf50 = -1;
                DAT_00aa6ae8 = 0xffffffff;
                FUN_004d92b0();
            }
            clear_665440();
            clear_118();
        }

        let r = FUN_004d7020(param_1);
        if r != -1 {
            if DAT_0052bf50 == r {
                DAT_0052bf50 = -1;
                DAT_00aa6ae8 = 0xffffffff;
                DAT_00aa6cf0 = 0;
                clear_665440();
                clear_118();
                FUN_004cd9d0();
            } else {
                DAT_0052bf50 = r;
                let r2 = FUN_004d97f0(r);
                if r2 == 0 {
                    FUN_004cd9d0();
                    DAT_00aa6cf0 = 0;
                    DAT_0052bf50 = -1;
                    DAT_00aa6ae8 = 0xffffffff;
                    FUN_004d92b0();
                    clear_665440();
                    clear_118();
                } else {
                    FUN_004d9680();
                    let idx = (DAT_0052bf50 as u32).wrapping_mul(0x3e) as usize;
                    let t: i32 = (&raw const DAT_00aa62e0)
                        .byte_add(idx)
                        .cast::<i32>()
                        .read_unaligned();
                    if t != 0 {
                        FUN_004dc310();
                        FUN_004de0b0(0x25, 0x1bd);
                    }
                }
            }
        }
        FUN_004d8540();
        FUN_004d8350();
        if DAT_0052bf50 != -1 {
            FUN_004d8690(param_1);
        }
        FUN_004d6460(local_20.as_mut_ptr(), 0x1fb, 0x1c4, 0x232, 0x1d0);
        _sprintf(
            (&raw mut DAT_00b69d20).cast::<i8>(),
            (&raw const DAT_0052b1b4).cast::<i8>(),
            2i32,
            0i32,
        );
        FUN_004db640(
            0xc,
            (&raw mut DAT_00b69d20).cast::<i8>(),
            local_20.as_mut_ptr(),
            0xffff,
        );
        FUN_004d6460(local_20.as_mut_ptr(), 200, 0x1c4, 0x1b4, 0x1d0);
        if DAT_00b6a2e0 != 0 {
            _sprintf(
                (&raw mut DAT_00b69d20).cast::<i8>(),
                (&raw const DAT_0052a6b0).cast::<i8>(),
                6i32,
                &raw mut DAT_00b6bc9e,
                1i32,
                &raw mut DAT_00b6a2e0,
            );
            FUN_004db640(
                0xc,
                (&raw mut DAT_00b69d20).cast::<i8>(),
                local_20.as_mut_ptr(),
                DAT_00b6a2c0,
            );
            let iv9: i32 = ((((param_1 ^ u_var12).wrapping_sub(u_var12) & 0x1f) ^ u_var12)
                .wrapping_sub(u_var12) as i32)
                >> 1;
            _sprintf(
                (&raw mut DAT_00b69d20).cast::<i8>(),
                (&raw const DAT_0052a6a4).cast::<i8>(),
                iv9,
            );
            FUN_004b5570((&raw mut DAT_00b69d20).cast::<i8>(), 0xcc, 0x1c5);
            _sprintf(
                (&raw mut DAT_00b69d20).cast::<i8>(),
                (&raw const DAT_0052a698).cast::<i8>(),
                iv9,
            );
            FUN_004b5570((&raw mut DAT_00b69d20).cast::<i8>(), 0x1a4, 0x1c5);
        }
        FUN_004d9170();
        if FUN_004bf9b0(0) == 1 {
            return 1;
        }
        FUN_004d6460(local_20.as_mut_ptr(), 0x55, 0x1bf, 0xb0, 0x1d7);
        if DAT_00bb73a8 != 0 {
            FUN_004dad40();
        }
        let pc4 = FUN_004ddc10(0x239);
        FUN_004dad70(pc4);
        let pc5 = (&raw mut DAT_0052a68c).cast::<i8>();
        let pc4 = FUN_004ddc10(0x102);
        let r = FUN_004da780(
            local_20.as_mut_ptr(),
            (&raw mut DAT_0052a678).cast::<i8>(),
            (&raw mut DAT_0052a680).cast::<i8>(),
            pc4,
            0xc,
            0,
            8,
            pc5,
        );
        if r != 0 {
            DAT_00b69e30 = 0;
            FUN_004cd9d0();
            FUN_004dc330(
                FUN_004be1b0 as *const c_void as *mut c_void,
                (&raw mut LAB_004be050).cast::<c_void>(),
            );
        }
        FUN_004d6460(local_20.as_mut_ptr(), 8, 0x195, 0x47, 0x1d6);
        if DAT_0052bf50 != -1 {
            let idx = (DAT_0052bf50 as u32).wrapping_mul(0x3e) as usize;
            let t: i32 = (&raw const DAT_00aa62e0)
                .byte_add(idx)
                .cast::<i32>()
                .read_unaligned();
            if t != 0 {
                let idx = (DAT_0052bf50 as u32).wrapping_mul(0x3e) as usize;
                let t2: i32 = (&raw const DAT_00aa62e8)
                    .byte_add(idx)
                    .cast::<i32>()
                    .read_unaligned();
                if t2 == 0x65 {
                    let pc4 = FUN_004ddc10(0x12);
                    FUN_004dad70(pc4);
                    let pc5 = (&raw mut DAT_0052bf94).cast::<i8>();
                    let pc4 = FUN_004ddc10(0x12);
                    let r = FUN_004da780(
                        local_20.as_mut_ptr(),
                        (&raw mut DAT_0052bf80).cast::<i8>(),
                        (&raw mut DAT_0052bf88).cast::<i8>(),
                        pc4,
                        0xc,
                        0,
                        7,
                        pc5,
                    );
                    if r != 0 {
                        let mut u12: u32 = 0xffffffff;
                        DAT_00aa6144 = 0;
                        let start = (&raw mut DAT_00aa62b0)
                            .cast::<i8>()
                            .byte_add((DAT_0052bf50 as u32).wrapping_mul(0x3e) as usize);
                        let mut p4 = start;
                        loop {
                            if u12 == 0 {
                                break;
                            }
                            u12 = u12.wrapping_sub(1);
                            let c = *p4;
                            p4 = p4.add(1);
                            if c == 0 {
                                break;
                            }
                        }
                        u12 = !u12;
                        let mut p4 = p4.wrapping_sub(u12 as usize);
                        let mut p5 = (&raw mut DAT_00b6a5c6).cast::<i8>();
                        let mut u11 = u12 >> 2;
                        while u11 != 0 {
                            let ut = p4.cast::<u32>().read_unaligned();
                            p5.cast::<u32>().write_unaligned(ut);
                            p4 = p4.add(4);
                            p5 = p5.add(4);
                            u11 = u11.wrapping_sub(1);
                        }
                        let mut rem = u12 & 3;
                        while rem != 0 {
                            *p5 = *p4;
                            p4 = p4.add(1);
                            p5 = p5.add(1);
                            rem = rem.wrapping_sub(1);
                        }
                        let iv9 = FUN_004d11d0();
                        DAT_00aa6af0 = 0x42;
                        DAT_00aa6af4 = 0x65;
                        DAT_00aa6af8 = (&raw const DAT_00bb735f).cast::<u32>().read_unaligned();
                        DAT_00aa6afc = (&raw const DAT_00bb7363).cast::<u32>().read_unaligned();
                        DAT_00aa6b00 = (&raw const DAT_00bb7367).cast::<u32>().read_unaligned();
                        DAT_00aa6b04 = (&raw const DAT_00bb736b).cast::<u32>().read_unaligned();
                        FUN_004cef70(iv9, &raw mut DAT_00aa6af0, 0x18);
                        FUN_004dc330(
                            FUN_004d7e70 as *const c_void as *mut c_void,
                            core::ptr::null_mut(),
                        );
                    }
                }
            }
        }
        FUN_004dad60();
        0
    }
}
