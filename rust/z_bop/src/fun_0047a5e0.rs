// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009d6940: u16;
    static mut DAT_00523410: i32;
    static mut DAT_009a1fe8: *mut u8;
    static mut DAT_009e9a67: u8;
    static mut DAT_009e9a69: u8;
    static mut DAT_009e9a8a: u8;
    static mut DAT_009e9a84: u8;
    static mut DAT_009e9a80: u8;
    static mut DAT_009e9a73: u8;
    static mut DAT_009e9a75: u8;
    static mut DAT_009e9670: u8;
    static mut DAT_009e9683: u8;
    static mut DAT_009e9681: u8;
    static mut DAT_009e9684: u8;
    static mut DAT_009e9680: u8;
    static mut DAT_009e967e: u8;
    static mut DAT_009e96c1: u8;
    static mut DAT_009e96c3: u8;
    static mut DAT_009e9a5a: u8;
    static mut DAT_009ecc34: i32;
    static mut DAT_009d6930: i32;
    static mut DAT_009ecc3c: i32;
    static mut DAT_005233d8: i32;
    static mut DAT_00523400: i32;
    static mut DAT_009a8c20: *const core::ffi::c_void;
    static mut DAT_00a07be0: i16;
    static mut DAT_005233f8: u8;
    static mut DAT_00523404: u32;
    static mut DAT_00a07cce: i8;
    static mut DAT_00a080f8: i32;
    static mut DAT_009a8c2c: i32;
    static mut DAT_00527ed0: i32;
    static mut DAT_009a8c12: i16;
    static mut DAT_00a00730: i32;
    static mut DAT_009a7b54: i32;
    static mut DAT_00523440: u8;
    static mut DAT_00a07c70: i8;
    static mut DAT_009ec474: u16;
    static mut DAT_009ec5fe: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00427720(a: i32, b: i32);
    fn FUN_00411810(a: i32) -> u32;
    fn FUN_00411540();
    fn FUN_00450550();
    fn FUN_00450650(a: i32, b: i32);
    fn FUN_0042e4d0(a: i32, b: i32, c: i32);
    fn FUN_00484160(a: u32);
    fn FUN_004ac760() -> i32;
    fn FUN_004aba50();
    fn FUN_004abe50();
    fn FUN_004abee0();
    fn FUN_0042d6d0();
    fn FUN_00410fe0();
    fn FUN_00411010();
    fn FUN_00455c80(a: u16);
    fn FUN_0045a1e0(a: i32, b: i32, c: i32, d: i32) -> i32;
    fn FUN_00485080(a: i32);
    fn FUN_004851d0(a: i32);
    fn FUN_00452880(a: i32);
    fn FUN_00405890(a: i32);
    fn FUN_00438010(a: i32) -> u32;
    fn FUN_004820b0(a: i32) -> u32;
    fn FUN_004833d0(a: u32, b: i32);
    fn FUN_0047ac10(a: i32);
    fn FUN_00481420(a: i32);
    fn FUN_00480570(a: i32);
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_00452960();
}

#[inline(always)]
unsafe fn at(base: *mut u8, off: i32) -> *mut u8 {
    // SAFETY: the offset addresses the game's own record array.
    unsafe { base.byte_offset(off as isize) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0047a5e0(param_1: i32) {
    // SAFETY: all globals and callees belong to the game; record offsets
    // are read and written exactly as the original does.
    unsafe {
        if (DAT_009d6940 as u8) == 1 {
            return;
        }
        if DAT_00523410 == 0 {
            let i = param_1.wrapping_mul(0x5bd);
            let u2 = at(&raw mut DAT_009e9a67, i).cast::<u16>().read_unaligned();
            if u2 != 0xffff && *DAT_009a1fe8.add((u2 as usize) * 0x23 + 3) == 0 {
                if *at(&raw mut DAT_009e9683, i) == 0 {
                    at(&raw mut DAT_009e9a8a, i)
                        .cast::<u16>()
                        .write_unaligned(0);
                    at(&raw mut DAT_009e9a84, i)
                        .cast::<u16>()
                        .write_unaligned(0);
                    at(&raw mut DAT_009e9a80, i)
                        .cast::<u16>()
                        .write_unaligned(0);
                    let d32 = at(&raw mut DAT_009e9670, i).cast::<u32>().read_unaligned();
                    at(&raw mut DAT_009e9a67, i)
                        .cast::<u16>()
                        .write_unaligned(d32 as u16);
                    FUN_00427720(0, param_1);
                    at(&raw mut DAT_009e9a73, i)
                        .cast::<u16>()
                        .write_unaligned(0);
                    at(&raw mut DAT_009e9a75, i)
                        .cast::<u16>()
                        .write_unaligned(0);
                } else {
                    at(&raw mut DAT_009e9a67, i)
                        .cast::<u16>()
                        .write_unaligned(0xffff);
                }
            }
            if *at(&raw mut DAT_009e9683, i) != 0 {
                let w2 = at(&raw mut DAT_009e9a69, i).cast::<u16>().read_unaligned();
                if w2 != 0xffff && *DAT_009a1fe8.add((w2 as usize) * 0x23 + 3) == 0 {
                    at(&raw mut DAT_009e9a69, i)
                        .cast::<u16>()
                        .write_unaligned(0xffff);
                }
            }
        }
        FUN_00411810(param_1);
        FUN_00411540();
        if param_1 == DAT_009ecc34 && DAT_00523410 == 0 {
            match DAT_009d6930 {
                0x4c => {
                    if DAT_009ecc3c == 0 {
                        DAT_009ecc3c = 1;
                        FUN_00450650(0x193, param_1);
                    } else {
                        DAT_009ecc3c = 0;
                        FUN_00450550();
                        FUN_00450650(0x192, param_1);
                    }
                }
                0x81 => {
                    if DAT_005233d8 == 1 {
                        DAT_00523400 = DAT_00523400.wrapping_add(0x40);
                        if DAT_00523400 == 0x300 {
                            DAT_00523400 = 0x100;
                        }
                        let f: unsafe extern "C" fn() = core::mem::transmute(DAT_009a8c20);
                        f();
                        DAT_00a07be0 = (((DAT_00523400 as u32).wrapping_sub(0x100) >> 6) as i16)
                            .wrapping_add(1);
                        FUN_00450650(0x11f, param_1);
                        FUN_0042e4d0(0x2b, -1, param_1);
                    }
                }
                0x83 => {
                    DAT_005233f8 = DAT_005233f8.wrapping_add(1);
                    if 3 < DAT_005233f8 {
                        DAT_005233f8 = 0;
                    }
                    FUN_00484160(
                        (((((DAT_009d6930.wrapping_sub(0x4c)) as u32) >> 8) << 8)
                            | (DAT_005233f8 as u32))
                            & 0xffff00ff,
                    );
                    FUN_00450650((DAT_005233f8 as i32).wrapping_add(0x66), param_1);
                    FUN_0042e4d0(0x2b, -1, param_1);
                }
                0x88 => {
                    DAT_00523404 = (DAT_00523404 == 0) as u32;
                }
                0x8c => {
                    if DAT_00a07cce == 0 {
                        DAT_00a07cce = 1;
                    } else {
                        DAT_00a07cce = 0;
                    }
                }
                0x8f => {
                    if DAT_00a080f8 == 1 {
                        let r = FUN_004ac760();
                        DAT_009a8c2c = DAT_009a8c2c.wrapping_add(r);
                        FUN_0042e4d0(0x2b, -1, param_1);
                        FUN_00450650(1, param_1);
                        DAT_00527ed0 = 0;
                        FUN_004aba50();
                        FUN_00452960();
                        FUN_004abe50();
                        FUN_004abee0();
                        DAT_00527ed0 = 1;
                        FUN_0042d6d0();
                        loop {
                            let u = FUN_00411810(-2);
                            if (u as i16) != 0 {
                                break;
                            }
                        }
                        FUN_004ac760();
                        FUN_00450650(2, param_1);
                        DAT_009a8c12 = 0;
                        DAT_00a00730 = 0;
                        FUN_00410fe0();
                    }
                }
                0x92 => {
                    if DAT_009a7b54 == 0 {
                        DAT_009a7b54 = 1;
                        FUN_00450650(0x191, param_1);
                    } else {
                        DAT_009a7b54 = 0;
                        FUN_00450650(400, param_1);
                    }
                }
                0x95 => {
                    FUN_0042e4d0(0x2b, -1, param_1);
                    FUN_00450650(0, param_1);
                }
                0xe8 => {
                    FUN_0042e4d0(0x2b, -1, param_1);
                    FUN_00411010();
                }
                _ => {}
            }
        }
        if 1 < DAT_00523440 && DAT_00a07c70 != 0 {
            return;
        }
        let i = param_1.wrapping_mul(0x5bd);
        if *at(&raw mut DAT_009e9681, i) != 0 {
            if DAT_00523410 != 0 {
                return;
            }
            let d32 = at(&raw mut DAT_009e9670, i).cast::<u32>().read_unaligned();
            if (d32 as i32) == -1 {
                return;
            }
            if *DAT_009a1fe8.add((d32.wrapping_mul(0x23).wrapping_add(3)) as usize) != 0 {
                return;
            }
            let w = at(&raw mut DAT_009e967e, i).cast::<u16>().read_unaligned();
            FUN_00455c80(w);
            let r = FUN_0045a1e0(param_1, -1, 0, 0);
            if r == 0 {
                if param_1 != DAT_009ecc34 {
                    return;
                }
                FUN_00452880(0x122);
                return;
            }
            FUN_00485080(param_1);
            FUN_004851d0(param_1);
            return;
        }
        'ab0a: {
            if *at(&raw mut DAT_009e9684, i) != 0 {
                break 'ab0a;
            }
            let w = at(&raw mut DAT_009e9a80, i).cast::<u16>().read_unaligned();
            if ((DAT_009ec474 as u8) & 0xd) == 1 && w == 0 && *at(&raw mut DAT_009e9683, i) == 0 {
                FUN_00405890(param_1);
            }
            let w = at(&raw mut DAT_009e96c1, i).cast::<u16>().read_unaligned();
            let u2 = w & 0xe;
            if ((DAT_009ec474 as u8) & 0xe) == 2 {
                let mut tmp = at(&raw mut DAT_009e96c3, i).cast::<u16>().read_unaligned();
                if u2 == 2 {
                    tmp = tmp.wrapping_add(DAT_009ec5fe);
                } else {
                    tmp = DAT_009ec5fe;
                }
                at(&raw mut DAT_009e96c3, i)
                    .cast::<u16>()
                    .write_unaligned(tmp);
                at(&raw mut DAT_009e96c1, i)
                    .cast::<u16>()
                    .write_unaligned(DAT_009ec474);
                if tmp < 0x3b {
                    DAT_009ec474 &= 0xfffd;
                }
                break 'ab0a;
            }
            let tmp = at(&raw mut DAT_009e96c3, i).cast::<u16>().read_unaligned();
            'aaf4: {
                if u2 == 2 && tmp < 0x3b {
                    let u5: u32;
                    let s3: i16;
                    if *at(&raw mut DAT_009e9683, i) == 0 {
                        let w = at(&raw mut DAT_009e9a80, i).cast::<u16>().read_unaligned();
                        if ((DAT_009d6940 >> 8) & 0xff) != 0 || w != 0 {
                            break 'aaf4;
                        }
                        u5 = FUN_004820b0(param_1);
                        s3 = u5 as i16;
                    } else {
                        u5 = FUN_00438010(param_1);
                        s3 = u5 as i16;
                    }
                    if s3 != -1 {
                        FUN_004833d0(u5, param_1);
                    }
                }
            }
            at(&raw mut DAT_009e96c1, i)
                .cast::<u16>()
                .write_unaligned(DAT_009ec474);
            at(&raw mut DAT_009e96c3, i)
                .cast::<u16>()
                .write_unaligned(0);
        }
        if *at(&raw mut DAT_009e9a5a, i) == 0 {
            FUN_0047ac10(param_1);
        } else {
            FUN_00481420(param_1);
        }
        if *at(&raw mut DAT_009e9680, i) != 0 {
            FUN_00480570(param_1);
        }
    }
}
