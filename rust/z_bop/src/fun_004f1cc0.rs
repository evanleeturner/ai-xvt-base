// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052bf54: i32;
    static mut DAT_006691d0: i32;
    static mut DAT_0052c208: i32;
    static mut DAT_00bb2818: i32;
    static mut DAT_00b69e30: i32;
    static mut DAT_00aa6af0: u32;
    static mut DAT_00aa6af4: u32;
    static mut DAT_00b6a250: i32;
    static mut DAT_0052c184: i32;
    static mut DAT_00a91b8c: i32;
    static mut DAT_00b6a606: u8;
    static mut DAT_00b6a5aa: u8;
    static mut DAT_00b6a5a6: u8;
    static mut DAT_00b6a5ae: u8;
    static mut DAT_00aa615e: u8;
    static mut DAT_00aa6150: i8;
    static mut DAT_00aa6162: u8;
    static mut DAT_00b6bba6: u8;
    static mut DAT_00b6bbae: u8;
    static mut DAT_00b6d962: u8;
    static mut DAT_00aa6120: i32;
    static mut DAT_00aa6124: i32;
    static mut DAT_00aa6128: i32;
    static mut DAT_00aa612c: i32;
    static mut DAT_00aa6130: i32;
    static mut DAT_00aa5e64: i32;
    static mut DAT_00aa5e38: i32;
    static mut DAT_00aa5e60: i32;
    static mut DAT_00aa5e40: [i32; 8];
    static mut DAT_00aa6114: i32;
    static mut DAT_00aa6134: i32;
    static mut DAT_00aa6138: i32;
    static mut DAT_00aa5cb0: [i32; 80];
    static mut DAT_00aa5df0: [i32; 8];
    static mut DAT_00aa5e10: [i32; 8];
    static mut DAT_00aa5e68: i32;
    static mut DAT_0052c204: i32;
    static mut DAT_00aa6118: *mut c_void;
    static mut DAT_00b69d20: i8;
    static mut DAT_0052ada8: i8;
    static mut DAT_00b69e38: u32;
    static mut DAT_00b6a2a0: u32;
    static mut DAT_00b6a2c0: u32;
    static mut DAT_0052c18c: i32;
    static mut DAT_00b6a2e0: i8;
    static mut DAT_00b6bc9e: i8;
    static mut DAT_00aa5aac: i16;
    static mut DAT_00a91d24: i8;
    static mut DAT_00a91d17: i8;
    static mut DAT_00bb73a8: i8;
    static mut DAT_00bb7397: i8;
    static mut DAT_00b6a270: u8;

    static mut DAT_0052c890: i8;
    static mut DAT_0052bf74: i8;
    static mut DAT_0052a6d4: i8;
    static mut DAT_0052b69c: i8;
    static mut DAT_0052b6ac: i8;
    static mut DAT_0052bfac: i8;
    static mut DAT_0052c884: i8;
    static mut DAT_0052c878: i8;
    static mut DAT_0052c86c: i8;
    static mut DAT_0052a6b0: i8;
    static mut DAT_0052a6a4: i8;
    static mut DAT_0052a698: i8;
    static mut DAT_0052a68c: i8;
    static mut DAT_0052a678: i8;
    static mut DAT_0052a680: i8;
    static mut DAT_0052bf94: i8;
    static mut DAT_0052bf80: i8;
    static mut DAT_0052bf88: i8;

    static mut DAT_004be050: u8;
    static mut DAT_004d7370: u8;
    static mut DAT_004f5100: u8;
    static mut DAT_004e1470: u8;
    static mut DAT_004fbe10: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn malloc(size: u32) -> *mut c_void;

    fn FUN_004cc9f0(a: i32) -> i32;
    fn FUN_004d2e50();
    fn FUN_004ddc10(a: i32) -> *mut i8;
    fn FUN_004dcb90(a: *mut i8, b: *mut i8, c: *mut i8, d: *mut i8, e: *mut i8) -> i32;
    fn FUN_004cfee0() -> i32;
    fn FUN_004d11d0() -> i32;
    fn FUN_004cef70(a: i32, b: *mut u32, c: u32);
    fn FUN_004cd9c0();
    fn FUN_004cd9d0();
    fn FUN_004c9ee0();
    fn FUN_004dc330(a: *mut c_void, b: *mut c_void);
    fn FUN_004f4b80() -> i32;
    fn FUN_004df5f0(a: i32) -> i32;
    fn FUN_004f4750() -> i32;
    fn FUN_004de0b0(a: i32, b: i32);
    fn FUN_004e5000(a: i32);
    fn FUN_004f70f0();
    fn FUN_004f36d0();
    fn FUN_004f3ea0();
    fn FUN_004d1400() -> i32;
    fn FUN_004d11e0() -> i32;
    fn FUN_004b4a60(a: *mut i8, b: *mut i8);
    fn FUN_004d4e20();
    fn FUN_004b5de0(a: *mut i8, b: i32, c: i32);
    fn FUN_004b5570(a: *mut i8, b: i32, c: i32);
    fn FUN_004b4d40(a: *mut i8, b: i32, c: i32);
    fn FUN_004d6460(a: *mut i32, b: i32, c: i32, d: i32, e: i32);
    fn FUN_004d64a0(a: *mut i32, b: i32, c: i32);
    fn FUN_004d4ec0(a: i32);
    fn FUN_004dc140(a: i32);
    fn FUN_004e5590(a: *mut c_void);
    fn FUN_004d4cf0(a: *mut i32);
    fn FUN_004d4d20(a: *mut i32);
    fn FUN_004db640(a: i32, b: *mut i8, c: *mut i32, d: u32);
    fn FUN_004e0490() -> i32;
    fn FUN_004f1680();
    fn FUN_004dc2e0(a: i32) -> u8;
    fn FUN_004dc1d0();
    fn FUN_004f32b0(a: u32);
    fn FUN_004f3740(a: u32);
    fn FUN_004f4f80();
    fn FUN_004d8690(a: u32);
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
    fn FUN_004f3f70() -> i32;
    fn FUN_004dad60();
    fn FUN_004f4710();
    fn FUN_004f4010();
    fn FUN_004bf9b0(a: i32) -> i32;
    fn FUN_004dc2a0(a: i32) -> u8;
    fn FUN_004dc2c0(a: i32) -> u8;
    fn FUN_004de090(a: *mut i32, b: *mut i32);
    fn FUN_004db4e0(a: i32, b: *mut i8, c: i32, d: i32, e: u32);

    fn FUN_004be1b0();
    fn FUN_004d73c0();
    fn FUN_004f5170();
    fn FUN_004e15d0();
    fn FUN_004fbe90();
}

macro_rules! fp {
    ($f:ident) => {
        $f as *const c_void as *mut c_void
    };
}

macro_rules! lb {
    ($d:ident) => {
        (&raw mut $d).cast::<c_void>()
    };
}

macro_rules! rv {
    ($d:ident) => {
        rd(&raw mut $d)
    };
}

macro_rules! rx {
    ($d:ident, $off:expr) => {
        rd((&raw mut $d).byte_offset(($off) as isize))
    };
}

#[inline(always)]
unsafe fn rd(p: *const u8) -> i32 {
    // SAFETY: caller passes a pointer into the game's packed globals.
    unsafe { p.cast::<i32>().read_unaligned() }
}

#[inline(always)]
unsafe fn wr(p: *mut u8, v: i32) {
    // SAFETY: caller passes a pointer into the game's packed globals.
    unsafe { p.cast::<i32>().write_unaligned(v) }
}

#[inline(always)]
fn tiny_idx(p: u32, u: u32) -> i32 {
    ((((((p ^ u).wrapping_sub(u)) & 0x1f) ^ u).wrapping_sub(u)) as i32) >> 1
}

#[unsafe(no_mangle)]
#[allow(
    non_snake_case,
    unused_assignments,
    unused_mut,
    unused_variables,
    unused_labels
)]
pub unsafe extern "C" fn FUN_004f1cc0(param_1: u32) -> u32 {
    // SAFETY: every global, callee and buffer below belongs to the game; the
    // accesses and calls replay the original's order and offsets.
    unsafe {
        let mut i3: i32 = 0;
        let mut i9: i32;
        let mut i12: i32;
        let mut i13: i32 = 0;
        let mut u22: u32 = 0;
        let mut u18: u32;
        let mut local_2c: i32 = 0;
        let mut local_28: [i32; 4] = [0; 4];
        let mut local_10: [i32; 4] = [0; 4];
        let mut local_14: i32 = 0;
        let mut local_18: i32 = 0;
        let l28: *mut i32 = local_28.as_mut_ptr();
        let l10: *mut i32 = local_10.as_mut_ptr();
        let cb0: *mut i32 = (&raw mut DAT_00aa5cb0).cast::<i32>();
        let df0: *mut i32 = (&raw mut DAT_00aa5df0).cast::<i32>();
        let e10: *mut i32 = (&raw mut DAT_00aa5e10).cast::<i32>();
        let e40: *mut i32 = (&raw mut DAT_00aa5e40).cast::<i32>();

        macro_rules! fill {
            ($lim:expr) => {{
                local_2c = 0;
                if DAT_00aa5e68 != 0 {
                    let mut pf = df0;
                    let mut pa = (&raw mut DAT_00aa615e) as *const u8;
                    loop {
                        let mut i12: i32 = 0;
                        if 0 < e10.offset(i13 as isize).read() {
                            let mut p11 = cb0.offset(i13.wrapping_mul(8) as isize);
                            loop {
                                if p11.read() == 0 {
                                    let u = rd(pa);
                                    pf.write(u);
                                    pf = pf.add(1);
                                    pa = pa.add(0x2a);
                                    cb0.offset(i12.wrapping_add(i13.wrapping_mul(8)) as isize)
                                        .write(u);
                                    local_2c += 1;
                                    break;
                                }
                                p11 = p11.add(1);
                                i12 += 1;
                                if !(i12 < e10.offset(i13 as isize).read()) {
                                    break;
                                }
                            }
                        }
                        i13 += 1;
                        if DAT_00aa5e68 <= i13 {
                            i13 = 0;
                        }
                        if !(local_2c < $lim) {
                            break;
                        }
                    }
                }
            }};
        }

        if param_1 == 0 {
            DAT_0052bf54 = 0;
            DAT_006691d0 = 0;
            DAT_0052c208 = 0;
            loop {
                i3 = FUN_004cc9f0(DAT_00bb2818);
                if i3 != 0 {
                    break;
                }
                FUN_004d2e50();
                let p4 = FUN_004ddc10(0x13);
                let p5 = FUN_004ddc10(0x20b);
                let p6 = FUN_004ddc10(0x2c4);
                let p7 = FUN_004ddc10(0x2c3);
                let p8 = FUN_004ddc10(0x2c2);
                i3 = FUN_004dcb90(p8, p7, p6, p5, p4);
                if i3 == 0 {
                    if DAT_00b69e30 != 2 {
                        i3 = FUN_004cfee0();
                        if i3 == 0 {
                            DAT_00aa6af0 = 0x47;
                            i3 = FUN_004d11d0();
                        } else {
                            DAT_00aa6af0 = 0x46;
                            i3 = 0;
                        }
                        FUN_004cef70(i3, &raw mut DAT_00aa6af0, 4);
                        FUN_004cd9c0();
                    }
                    return 1;
                }
            }
            FUN_004c9ee0();
            if DAT_00b6a250 == 0 && DAT_00b69e30 != 3 {
                let p8: *mut i8 = core::ptr::null_mut();
                let p7: *mut i8 = core::ptr::null_mut();
                let p4 = FUN_004ddc10(0x2ef);
                let p5 = FUN_004ddc10(0x2ee);
                let p6 = FUN_004ddc10(0x2ed);
                FUN_004dcb90(p6, p5, p4, p7, p8);
                if DAT_00b69e30 != 2 {
                    DAT_00aa6af0 = 0x46;
                    FUN_004cef70(0, &raw mut DAT_00aa6af0, 4);
                    FUN_004cd9d0();
                }
                DAT_00b69e30 = 0;
                FUN_004dc330(fp!(FUN_004be1b0), lb!(DAT_004be050));
                return 0;
            }
            if DAT_0052c184 == 0 && DAT_00a91b8c != 1 && rv!(DAT_00b6a606) == 1 {
                if rv!(DAT_00b6a5aa) == 0 {
                    i3 = FUN_004f4b80();
                    if i3 == 0 {
                        DAT_00aa6120 = 0;
                        DAT_00aa6124 = 0;
                        DAT_00aa6128 = 0;
                        DAT_00aa612c = 0;
                        DAT_00aa6130 = 0;
                    }
                    i3 = FUN_004df5f0(0);
                    if DAT_00b69e30 != 2 && i3 == 0 {
                        DAT_00aa6af0 = 0x47;
                        i3 = FUN_004d11d0();
                        FUN_004cef70(i3, &raw mut DAT_00aa6af0, 4);
                        FUN_004cd9d0();
                        FUN_004dc330(fp!(FUN_004d73c0), lb!(DAT_004d7370));
                        return 0;
                    }
                } else if rv!(DAT_00b6a5aa) == 3 {
                    i3 = FUN_004f4750();
                    if i3 == 0 {
                        DAT_00aa6120 = 0;
                        DAT_00aa6124 = 0;
                        DAT_00aa6128 = 0;
                        DAT_00aa612c = 0;
                        DAT_00aa6130 = 0;
                    }
                }
            }
            if DAT_00a91b8c != 0 {
                DAT_00a91b8c = 0;
                DAT_00aa5e64 = 1;
                FUN_004dc330(fp!(FUN_004f5170), lb!(DAT_004f5100));
                return 0;
            }
            FUN_004de0b0(0x25, 0x1bd);
            DAT_00aa5e38 = 0;
            DAT_00a91b8c = 0;
            DAT_00aa5e60 = 0;
            i3 = 8;
            let mut p14 = e40;
            while i3 != 0 {
                p14.write(0);
                p14 = p14.add(1);
                i3 -= 1;
            }
            FUN_004e5000(rv!(DAT_00b6a5aa));
            if DAT_00aa6114 != 0 {
                DAT_00aa6134 = 0;
                if DAT_00aa6138 != 0 {
                    let mut pa = DAT_00aa6114.wrapping_add(0x140);
                    loop {
                        if (pa as *const i32).read_unaligned()
                            == rx!(DAT_00b6a5ae, rv!(DAT_00b6a5aa).wrapping_mul(4))
                        {
                            break;
                        }
                        pa = pa.wrapping_add(0x148);
                        DAT_00aa6134 = DAT_00aa6134.wrapping_add(1);
                        if !(DAT_00aa6134 < DAT_00aa6138) {
                            break;
                        }
                    }
                }
            }
            FUN_004f70f0();
            if DAT_0052c184 == 0 {
                i3 = 0x50;
                let mut p = cb0;
                while i3 != 0 {
                    p.write(0);
                    p = p.add(1);
                    i3 -= 1;
                }
                i3 = 8;
                let mut p = df0;
                while i3 != 0 {
                    p.write(0);
                    p = p.add(1);
                    i3 -= 1;
                }
                FUN_004f36d0();
            } else {
                FUN_004f3ea0();
            }
            if DAT_00aa5e68 == 1 {
                i3 = 1;
                if DAT_00b69e30 != 2 {
                    i3 = FUN_004d1400();
                }
                if i3 == 1 {
                    if DAT_0052c184 != 1 {
                        cb0.write(rv!(DAT_00aa615e));
                        df0.write(rv!(DAT_00aa615e));
                        wr(&raw mut DAT_00b6a5a6, 0);
                        FUN_004dc330(fp!(FUN_004f5170), lb!(DAT_004f5100));
                        DAT_00aa5e64 = 1;
                        DAT_0052c184 = 0;
                        return 0;
                    }
                    FUN_004dc330(fp!(FUN_004e15d0), lb!(DAT_004e1470));
                    DAT_0052c184 = 0;
                    return 0;
                }
                i13 = 0;
                if DAT_0052c184 == 0 {
                    fill!(i3);
                }
            } else {
                if DAT_00b69e30 == 2 {
                    if DAT_0052c184 == 0 {
                        if rv!(DAT_00b6a5aa) == 3 || rv!(DAT_00b6a5aa) == 4 {
                            u22 = (rv!(DAT_00b6d962) as u32) ^ 1;
                            df0.write(rv!(DAT_00aa615e));
                            wr(&raw mut DAT_00b6a5a6, u22 as i32);
                            cb0.offset(u22.wrapping_mul(8) as isize)
                                .write(rv!(DAT_00aa615e));
                            let mut b17 = false;
                            if rv!(DAT_00b6a5aa) == 3
                                && rv!(DAT_00b6a606) == 1
                                && DAT_00bb7397 == 2
                                && rv!(DAT_00b6bba6) != 0
                                && (rx!(DAT_00b6bbae, rv!(DAT_00b6bba6).wrapping_mul(4)) as u32)
                                    != u22
                            {
                                b17 = true;
                            }
                            if b17 {
                                FUN_004dc330(fp!(FUN_004fbe90), lb!(DAT_004fbe10));
                            } else {
                                FUN_004dc330(fp!(FUN_004f5170), lb!(DAT_004f5100));
                            }
                            DAT_00aa5e64 = 1;
                            DAT_0052c184 = 0;
                            return 0;
                        }
                    } else if DAT_00aa5e64 != 0 {
                        FUN_004dc330(fp!(FUN_004e15d0), lb!(DAT_004e1470));
                        DAT_0052c184 = 0;
                        return 0;
                    }
                }
                if DAT_0052c184 == 0 {
                    i3 = 1;
                    if DAT_00b69e30 != 2 {
                        i3 = FUN_004d1400();
                    }
                    i13 = 0;
                    fill!(i3);
                }
            }
            i13 = 0;
            DAT_0052c184 = 0;
            DAT_00aa5e64 = 0;
            i3 = rv!(DAT_00b6a5a6);
            if DAT_00b69e30 == 2 && 0 < DAT_00aa5e68 {
                let mut p11 = cb0;
                loop {
                    i3 = i13;
                    if p11.read() == 1 {
                        break;
                    }
                    p11 = p11.add(8);
                    i13 += 1;
                    i3 = rv!(DAT_00b6a5a6);
                    if !(i13 < DAT_00aa5e68) {
                        break;
                    }
                }
            }
            wr(&raw mut DAT_00b6a5a6, i3);
            if rv!(DAT_00b6a5aa) == 1 {
                i3 = 0;
                let mut b17 = DAT_00aa5e68 == 0;
                if 0 < DAT_00aa5e68 {
                    let mut p11 = e10;
                    loop {
                        if 1 < p11.read() {
                            break;
                        }
                        p11 = p11.add(1);
                        i3 += 1;
                        if !(i3 < DAT_00aa5e68) {
                            break;
                        }
                    }
                    b17 = i3 == DAT_00aa5e68;
                }
                if b17 {
                    i3 = 0;
                    wr(&raw mut DAT_00b6a5a6, 0);
                    if DAT_00b69e30 != 2 && 0 < DAT_00aa5e68 {
                        let mut p10 = cb0;
                        let mut p11 = e10;
                        loop {
                            i13 = 0;
                            let mut p15 = p10;
                            if 0 < p11.read() {
                                loop {
                                    i12 = FUN_004d11e0();
                                    if i12 == p15.read() {
                                        wr(&raw mut DAT_00b6a5a6, i3);
                                    }
                                    i13 += 1;
                                    p15 = p15.add(1);
                                    if !(i13 < p11.read()) {
                                        break;
                                    }
                                }
                            }
                            p11 = p11.add(1);
                            i3 += 1;
                            p10 = p10.add(8);
                            if !(i3 < DAT_00aa5e68) {
                                break;
                            }
                        }
                    }
                    DAT_0052c184 = 0;
                    FUN_004dc330(fp!(FUN_004f5170), lb!(DAT_004f5100));
                    DAT_00aa5e64 = 1;
                    return 0;
                }
            }
            if DAT_00b69e30 == 2 && DAT_0052c204 == 1 {
                FUN_004dc330(fp!(FUN_004f5170), lb!(DAT_004f5100));
                DAT_00aa5e64 = 1;
                DAT_0052c184 = 0;
                return 0;
            }
            if rv!(DAT_00b6a5aa) == 0 && DAT_00aa5e68 == 1 && e10.read() == 8 {
                i3 = 0;
                let mut p11 = cb0;
                loop {
                    i13 = 0;
                    let mut p15 = p11;
                    loop {
                        i9 = FUN_004d11e0();
                        i12 = i3;
                        if i9 == p15.read() {
                            break;
                        }
                        p15 = p15.add(1);
                        i13 += 1;
                        i12 = rv!(DAT_00b6a5a6);
                        if !(i13 < 8) {
                            break;
                        }
                    }
                    wr(&raw mut DAT_00b6a5a6, i12);
                    p11 = p11.add(8);
                    i3 += 1;
                    if DAT_00aa5e68 <= i3 {
                        FUN_004dc330(fp!(FUN_004f5170), lb!(DAT_004f5100));
                        DAT_00aa5e64 = 1;
                        DAT_0052c184 = 0;
                        return 0;
                    }
                }
            }
            FUN_004b4a60(&raw mut DAT_0052c890, &raw mut DAT_0052bf74);
            FUN_004d4e20();
            FUN_004b5de0(&raw mut DAT_0052bf74, 0, 0);
            FUN_004b5570(&raw mut DAT_0052a6d4, 0, 0);
            let p4: *mut i8 = if DAT_00b6a250 == 0 {
                &raw mut DAT_0052b69c
            } else {
                &raw mut DAT_0052b6ac
            };
            FUN_004b5570(p4, 0, 0);
            if DAT_00b69e30 != 2 {
                FUN_004b4d40(&raw mut DAT_0052bfac, 0, 0);
            }
            FUN_004b4d40(&raw mut DAT_0052c884, 0, 0);
            i3 = 0;
            FUN_004d6460(l28, 0x58, 0xcf, 0x100, 0xdd);
            if 0 < DAT_00aa5e68 {
                loop {
                    if (DAT_00aa5e68 == 8 || DAT_00aa5e68 == 7) && i3 == 4 {
                        FUN_004d6460(l28, 0x104, 0xcf, 0x1ac, 0xdd);
                    }
                    i13 = 0;
                    FUN_004d64a0(l28, 0, 0xf);
                    if 0 < e10.offset(i3 as isize).read() {
                        loop {
                            let p4: *mut i8 = if i13 == 0 {
                                &raw mut DAT_0052c878
                            } else {
                                &raw mut DAT_0052c86c
                            };
                            i13 += 1;
                            FUN_004b4d40(p4, l28.read(), l28.add(1).read());
                            FUN_004d64a0(l28, 0, 0xf);
                            if !(i13 < e10.offset(i3 as isize).read()) {
                                break;
                            }
                        }
                    }
                    i3 += 1;
                    if !(i3 < DAT_00aa5e68) {
                        break;
                    }
                }
            }
            FUN_004d4ec0(1);
            FUN_004dc140(0x14);
            DAT_00aa6118 = malloc(0x1000);
            FUN_004e5590(DAT_00aa6118);
        }

        FUN_004d6460(l28, 0x9e, 0x34, 0x1eb, 0x44);
        FUN_004d4cf0(l10);
        FUN_004d4d20(l28);
        sprintf(
            &raw mut DAT_00b69d20,
            &raw mut DAT_0052ada8,
            4i32,
            ((DAT_00aa6134 as u32)
                .wrapping_mul(0x148)
                .wrapping_add(DAT_00aa6114 as u32)
                .wrapping_add(0x40)) as i32,
        );
        u22 = 0xffffffff;
        let mut p4: *mut i8 = &raw mut DAT_00b69d20;
        loop {
            if u22 == 0 {
                break;
            }
            u22 = u22.wrapping_sub(1);
            let c = p4.read();
            p4 = p4.add(1);
            if c == 0 {
                break;
            }
        }
        i3 = (!u22).wrapping_sub(2) as i32;
        while 0 < i3 {
            let pc = (&raw mut DAT_00b69d20).offset(i3 as isize);
            if pc.read() == b'(' as i8 {
                pc.write(0);
                break;
            }
            i3 -= 1;
        }
        FUN_004db640(0xc, &raw mut DAT_00b69d20, l28, 0xffff);
        FUN_004d4d20(l10);
        FUN_004d6460(l28, 0x54, 0x5a, 0x1b2, 0x6c);
        u22 = 0xffff;
        u18 = if rv!(DAT_00b6a5aa) == 3 { 0x1d2 } else { 0x1d1 };
        let p4 = FUN_004ddc10(u18 as i32);
        FUN_004db640(0xf, p4, l28, u22);
        if DAT_006691d0 == 0 {
            FUN_004d6460(l28, 0x54, 400, 0x1b2, 0x19e);
            i3 = FUN_004cfee0();
            let mut goto27c6 = false;
            if i3 == 0 {
                if DAT_00b69e30 == 2 {
                    goto27c6 = true;
                } else {
                    i3 = 0;
                    if 0 < DAT_00aa5e68 {
                        let mut p11 = cb0;
                        loop {
                            i13 = FUN_004d11e0();
                            if i13 == p11.read() {
                                u22 = DAT_00b69e38;
                                let p4 = FUN_004ddc10(0x2e5);
                                FUN_004db640(0xc, p4, l28, u22);
                            }
                            p11 = p11.add(8);
                            i3 += 1;
                            if !(i3 < DAT_00aa5e68) {
                                break;
                            }
                        }
                    }
                    FUN_004d64a0(l28, 0, 0xf);
                    u18 = 0x1d5;
                    u22 = DAT_00b69e38;
                }
            } else {
                if DAT_00b69e30 != 2 {
                    u22 = DAT_00b6a2a0;
                    let p4 = FUN_004ddc10(0x1d4);
                    FUN_004db640(0xc, p4, l28, u22);
                }
                goto27c6 = true;
            }
            if goto27c6 {
                FUN_004d64a0(l28, 0, 0xf);
                u18 = 0x1d3;
                u22 = DAT_00b6a2a0;
            }
            let p4 = FUN_004ddc10(u18 as i32);
            FUN_004db640(0xc, p4, l28, u22);
        }
        if DAT_00b69e30 != 2 {
            i13 = FUN_004e0490();
            i3 = DAT_00aa5e60;
            if i13 == 0x46 {
                i3 = FUN_004cfee0();
                if i3 == 0 {
                    let p8: *mut i8 = core::ptr::null_mut();
                    let p7: *mut i8 = core::ptr::null_mut();
                    let p4 = FUN_004ddc10(0x279);
                    let p5 = FUN_004ddc10(0x278);
                    let p6 = FUN_004ddc10(0x277);
                    FUN_004dcb90(p6, p5, p4, p7, p8);
                }
                DAT_00b69e30 = 3;
                DAT_0052c184 = 1;
                FUN_004dc330(fp!(FUN_004d73c0), lb!(DAT_004d7370));
                FUN_004cd9d0();
            } else if i13 == 0x41 {
                FUN_004f1680();
                FUN_004f3ea0();
            } else {
                if i13 == 0x48 {
                    i3 = 0;
                    if 0 < DAT_00aa5e68 {
                        let mut p11 = cb0;
                        loop {
                            i13 = 0;
                            let mut p15 = p11;
                            loop {
                                i9 = FUN_004d11e0();
                                i12 = i3;
                                if i9 == p15.read() {
                                    break;
                                }
                                p15 = p15.add(1);
                                i13 += 1;
                                i12 = rv!(DAT_00b6a5a6);
                                if !(i13 < 8) {
                                    break;
                                }
                            }
                            wr(&raw mut DAT_00b6a5a6, i12);
                            p11 = p11.add(8);
                            i3 += 1;
                            if !(i3 < DAT_00aa5e68) {
                                break;
                            }
                        }
                    }
                    if rv!(DAT_00b6a5aa) == 3
                        && rv!(DAT_00b6a606) == 1
                        && DAT_00bb7397 == 2
                        && rv!(DAT_00b6bba6) != 0
                    {
                        if DAT_00b69e30 != 2 {
                            i3 = FUN_004d1400();
                            if i3 != 1 {
                                FUN_004dc330(fp!(FUN_004fbe90), lb!(DAT_004fbe10));
                                return 0;
                            }
                        }
                        if rx!(DAT_00b6bbae, rv!(DAT_00b6bba6).wrapping_mul(4)) != rv!(DAT_00b6a5a6)
                        {
                            FUN_004dc330(fp!(FUN_004fbe90), lb!(DAT_004fbe10));
                            return 0;
                        }
                    }
                    FUN_004dc330(fp!(FUN_004f5170), lb!(DAT_004f5100));
                    return 0;
                }
                if i13 == 0x53 {
                    DAT_0052c184 = 1;
                    FUN_004dc330(fp!(FUN_004e15d0), lb!(DAT_004e1470));
                    return 0;
                }
                if i13 == 0x67 {
                    i3 = 0;
                    if 0 < DAT_00aa5e60 {
                        let mut p11 = e40;
                        loop {
                            if p11.read() == DAT_0052c18c {
                                DAT_00aa5e60 = DAT_00aa5e60.wrapping_sub(1);
                                break;
                            }
                            p11 = p11.add(1);
                            i3 += 1;
                            if !(i3 < DAT_00aa5e60) {
                                break;
                            }
                        }
                    }
                    if i3 < 7 {
                        let mut p14 = e40.offset(i3 as isize);
                        loop {
                            let p16 = p14.add(1);
                            p14.write(p16.read());
                            p14 = p16;
                            if !(p16 < e40.add(7)) {
                                break;
                            }
                        }
                    }
                    e40.add(7).write(0);
                } else if i13 == 0x68 {
                    i13 = 0;
                    let mut b17 = DAT_00aa5e60 == 0;
                    if 0 < DAT_00aa5e60 {
                        let mut p11 = e40;
                        loop {
                            if p11.read() == DAT_0052c18c {
                                break;
                            }
                            p11 = p11.add(1);
                            i13 += 1;
                            if !(i13 < DAT_00aa5e60) {
                                break;
                            }
                        }
                        b17 = i13 == DAT_00aa5e60;
                    }
                    if b17 {
                        DAT_00aa5e60 = DAT_00aa5e60.wrapping_add(1);
                        e40.offset(i3 as isize).write(DAT_0052c18c);
                    }
                } else if i13 == 0x55 || i13 == 0x54 {
                    DAT_00aa5e60 = 0;
                    i3 = 8;
                    let mut p14 = e40;
                    while i3 != 0 {
                        p14.write(0);
                        p14 = p14.add(1);
                        i3 -= 1;
                    }
                    DAT_00aa5e38 = 0;
                    let b = FUN_004dc2e0(2);
                    if b != 0 {
                        FUN_004dc1d0();
                    }
                }
            }
        }
        FUN_004f32b0(param_1);
        if DAT_006691d0 == 0 {
            FUN_004f3740(param_1);
        } else {
            FUN_004f4f80();
        }
        if DAT_00b69e30 != 2 {
            FUN_004d8690(param_1);
        }
        FUN_004d6460(l28, 200, 0x1c4, 0x1b4, 0x1d0);
        if DAT_00b6a2e0 != 0 {
            sprintf(
                &raw mut DAT_00b69d20,
                &raw mut DAT_0052a6b0,
                6i32,
                &raw mut DAT_00b6bc9e,
                1i32,
                &raw mut DAT_00b6a2e0,
            );
            FUN_004db640(0xc, &raw mut DAT_00b69d20, l28, DAT_00b6a2c0);
            u22 = ((param_1 as i32) >> 0x1f) as u32;
            'l2d4a: {
                'l2d30: {
                    'l2ce6: {
                        if rv!(DAT_00b6a5aa) == 1 || rv!(DAT_00b6a5aa) == 2 {
                            break 'l2ce6;
                        }
                        if rv!(DAT_00b6a5aa) == 0 {
                            i3 = 0;
                            if 0 < DAT_00aa5aac {
                                let mut p4: *mut i8 = &raw mut DAT_00a91d24;
                                loop {
                                    if p4.read() != 0 {
                                        let t = (&raw mut DAT_00a91d17)
                                            .offset(i3.wrapping_mul(0x562) as isize)
                                            .read();
                                        if t == 0 {
                                            sprintf(
                                                &raw mut DAT_00b69d20,
                                                &raw mut DAT_0052a6a4,
                                                tiny_idx(param_1, u22),
                                            );
                                            FUN_004b5570(&raw mut DAT_00b69d20, 0xcc, 0x1c5);
                                        } else {
                                            if t != 1 {
                                                break 'l2ce6;
                                            }
                                            sprintf(
                                                &raw mut DAT_00b69d20,
                                                &raw mut DAT_0052a698,
                                                tiny_idx(param_1, u22),
                                            );
                                            FUN_004b5570(&raw mut DAT_00b69d20, 0xcc, 0x1c5);
                                        }
                                        break 'l2d30;
                                    }
                                    p4 = p4.offset(0x562);
                                    i3 += 1;
                                    if !(i3 < DAT_00aa5aac as i32) {
                                        break;
                                    }
                                }
                            }
                            break 'l2d4a;
                        }
                        i13 = 0;
                        i3 = FUN_004d11e0();
                        let mut b17 = DAT_00aa5e68 == 0;
                        if 0 < DAT_00aa5e68 {
                            let mut p11 = cb0;
                            let mut p15 = e10;
                            loop {
                                i9 = 0;
                                i12 = p15.read();
                                let mut p10 = p11;
                                if 0 < i12 {
                                    loop {
                                        if p10.read() == i3 {
                                            break;
                                        }
                                        i9 += 1;
                                        p10 = p10.add(1);
                                        if !(i9 < i12) {
                                            break;
                                        }
                                    }
                                    if i9 < i12 {
                                        break;
                                    }
                                }
                                p11 = p11.add(8);
                                p15 = p15.add(1);
                                i13 += 1;
                                if !(i13 < DAT_00aa5e68) {
                                    break;
                                }
                            }
                            b17 = i13 == DAT_00aa5e68;
                        }
                        if b17 {
                            break 'l2ce6;
                        }
                        if i13 == 0 {
                            sprintf(
                                &raw mut DAT_00b69d20,
                                &raw mut DAT_0052a698,
                                tiny_idx(param_1, u22),
                            );
                            FUN_004b5570(&raw mut DAT_00b69d20, 0xcc, 0x1c5);
                        } else {
                            sprintf(
                                &raw mut DAT_00b69d20,
                                &raw mut DAT_0052a6a4,
                                tiny_idx(param_1, u22),
                            );
                            FUN_004b5570(&raw mut DAT_00b69d20, 0xcc, 0x1c5);
                        }
                        break 'l2d30;
                    }
                    i3 = tiny_idx(param_1, u22);
                    sprintf(&raw mut DAT_00b69d20, &raw mut DAT_0052a6a4, i3);
                    FUN_004b5570(&raw mut DAT_00b69d20, 0xcc, 0x1c5);
                    sprintf(&raw mut DAT_00b69d20, &raw mut DAT_0052a698, i3);
                }
                FUN_004b5570(&raw mut DAT_00b69d20, 0x1a4, 0x1c5);
            }
        }
        FUN_004d6460(l28, 0x55, 0x1bf, 0xb0, 0x1d7);
        if DAT_00bb73a8 != 0 {
            FUN_004dad40();
        }
        i3 = FUN_004cfee0();
        'l2f58: {
            let pc: *mut c_void;
            let pu: *mut c_void;
            if i3 == 0 && DAT_00b69e30 != 2 {
                let p4 = FUN_004ddc10(0x239);
                FUN_004dad70(p4);
                let p5 = &raw mut DAT_0052a68c;
                let p4 = FUN_004ddc10(0xcc);
                i3 = FUN_004da780(
                    l28,
                    &raw mut DAT_0052a678,
                    &raw mut DAT_0052a680,
                    p4,
                    0xc,
                    0,
                    8,
                    p5,
                );
                if i3 == 0 {
                    break 'l2f58;
                }
                let p4 = FUN_004ddc10(0x13);
                let p5 = FUN_004ddc10(0x20b);
                let p6 = FUN_004ddc10(0x22d);
                let p7 = FUN_004ddc10(0x22c);
                let p8 = FUN_004ddc10(0x22b);
                i3 = FUN_004dcb90(p8, p7, p6, p5, p4);
                if i3 == 0 {
                    break 'l2f58;
                }
                DAT_00aa6af0 = 0x47;
                i3 = FUN_004d11d0();
                FUN_004cef70(i3, &raw mut DAT_00aa6af0, 4);
                FUN_004cd9d0();
                DAT_0052c184 = 1;
                pu = lb!(DAT_004d7370);
                pc = fp!(FUN_004d73c0);
            } else {
                let p4 = FUN_004ddc10(0x239);
                FUN_004dad70(p4);
                let p5 = &raw mut DAT_0052a68c;
                let p4 = FUN_004ddc10(0x104);
                i3 = FUN_004da780(
                    l28,
                    &raw mut DAT_0052a678,
                    &raw mut DAT_0052a680,
                    p4,
                    0xc,
                    0,
                    8,
                    p5,
                );
                if i3 == 0 {
                    break 'l2f58;
                }
                if rv!(DAT_00b6a606) == 1 && rv!(DAT_00b6a5aa) == 0 {
                    let p4 = FUN_004ddc10(0x13);
                    let p5 = FUN_004ddc10(0x20b);
                    let p6 = FUN_004ddc10(0x30d);
                    let p7 = FUN_004ddc10(0x30c);
                    let p8 = FUN_004ddc10(0x30b);
                    FUN_004dcb90(p8, p7, p6, p5, p4);
                }
                if DAT_00b69e30 != 2 {
                    DAT_00aa6af0 = 0x53;
                    FUN_004cef70(0, &raw mut DAT_00aa6af0, 4);
                    if DAT_00b69e30 != 2 {
                        break 'l2f58;
                    }
                }
                pu = lb!(DAT_004e1470);
                pc = fp!(FUN_004e15d0);
            }
            FUN_004dc330(pc, pu);
        }
        FUN_004d6460(l28, 8, 0x195, 0x47, 0x1d6);
        if DAT_00b69e30 == 2 {
            i3 = FUN_004f3f70();
            if i3 != 0 {
                let p4 = FUN_004ddc10(0xd4);
                FUN_004dad70(p4);
                let p5 = &raw mut DAT_0052bf94;
                let p4 = FUN_004ddc10(0x1db);
                i3 = FUN_004da780(
                    l28,
                    &raw mut DAT_0052bf80,
                    &raw mut DAT_0052bf88,
                    p4,
                    0xc,
                    0,
                    7,
                    p5,
                );
                if i3 != 0 {
                    i3 = 0;
                    if 0 < DAT_00aa5e68 {
                        let mut p11 = cb0;
                        loop {
                            i13 = 0;
                            let mut p15 = p11;
                            loop {
                                i12 = i3;
                                if p15.read() == 1 {
                                    break;
                                }
                                p15 = p15.add(1);
                                i13 += 1;
                                i12 = rv!(DAT_00b6a5a6);
                                if !(i13 < 8) {
                                    break;
                                }
                            }
                            wr(&raw mut DAT_00b6a5a6, i12);
                            p11 = p11.add(8);
                            i3 += 1;
                            if !(i3 < DAT_00aa5e68) {
                                break;
                            }
                        }
                    }
                    if rv!(DAT_00b6a5aa) == 3
                        && rv!(DAT_00b6a606) == 1
                        && DAT_00bb7397 == 2
                        && rv!(DAT_00b6bba6) != 0
                        && rx!(DAT_00b6bbae, rv!(DAT_00b6bba6).wrapping_mul(4)) != rv!(DAT_00b6a5a6)
                    {
                        FUN_004dc330(fp!(FUN_004fbe90), lb!(DAT_004fbe10));
                        FUN_004dc330(fp!(FUN_004f5170), lb!(DAT_004f5100));
                        return 0;
                    }
                    FUN_004dc330(fp!(FUN_004f5170), lb!(DAT_004f5100));
                    FUN_004dad60();
                    return 0;
                }
            }
        } else {
            i3 = FUN_004cfee0();
            if i3 != 0 {
                i3 = FUN_004f3f70();
                if i3 != 0 {
                    let p4 = FUN_004ddc10(0xd4);
                    FUN_004dad70(p4);
                    let p5 = &raw mut DAT_0052bf94;
                    let p4 = FUN_004ddc10(0x1db);
                    i3 = FUN_004da780(
                        l28,
                        &raw mut DAT_0052bf80,
                        &raw mut DAT_0052bf88,
                        p4,
                        0xc,
                        0,
                        7,
                        p5,
                    );
                    if i3 != 0 {
                        FUN_004f4710();
                    }
                }
            }
        }
        FUN_004dad60();
        FUN_004f4010();
        i3 = FUN_004bf9b0(1);
        if i3 == 1 {
            return 1;
        }
        let b = FUN_004dc2e0(2);
        if b != 0 {
            if FUN_004dc2a0(2) == 0 && FUN_004dc2c0(2) == 0 {
                i13 = 0;
                i3 = ((0u32.wrapping_sub((DAT_00b69e30 == 2) as u32) & 0xfffffff9u32)
                    .wrapping_add(8)) as i32;
                if 0 < i3 {
                    let mut pa = (&raw mut DAT_00aa615e) as *const u8;
                    loop {
                        let t = rd(pa);
                        if t == DAT_00aa5e38 {
                            break;
                        }
                        pa = pa.add(0x2a);
                        i13 += 1;
                        if i3 <= i13 {
                            return 0;
                        }
                    }
                    FUN_004de090(&raw mut local_14, &raw mut local_18);
                    i3 = ((&raw mut DAT_00aa6150) as usize as u32)
                        .wrapping_add((i13 as u32).wrapping_mul(0x2a))
                        as i32;
                    let u19: u32 = 1;
                    let p4 = FUN_004ddc10(
                        rx!(DAT_00aa6162, (i13 as u32).wrapping_mul(0x2a)).wrapping_add(0x9a),
                    );
                    sprintf(
                        &raw mut DAT_00b69d20,
                        &raw mut DAT_0052a6b0,
                        6i32,
                        p4,
                        u19,
                        i3,
                    );
                    i3 = FUN_004d11e0();
                    if i3 != rx!(DAT_00aa615e, (i13 as u32).wrapping_mul(0x2a)) && DAT_00b69e30 != 2
                    {
                        FUN_004db4e0(
                            0xc,
                            &raw mut DAT_00b69d20,
                            local_14.wrapping_sub(7),
                            local_18.wrapping_sub(7),
                            DAT_00b6a2c0,
                        );
                        return 0;
                    }
                    let off = (((((param_1 as i32) % 0x18) as u32) & 0xfffffffeu32).wrapping_mul(2))
                        as usize;
                    FUN_004db4e0(
                        0xc,
                        &raw mut DAT_00b69d20,
                        local_14.wrapping_sub(7),
                        local_18.wrapping_sub(7),
                        (&raw mut DAT_00b6a270)
                            .byte_add(off)
                            .cast::<u32>()
                            .read_unaligned(),
                    );
                    return 0;
                }
            } else {
                if DAT_00b69e30 != 2 {
                    DAT_00aa6af4 = DAT_00aa5e38 as u32;
                    DAT_00aa6af0 = 0x67;
                    FUN_004cef70(0, &raw mut DAT_00aa6af0, 8);
                }
                FUN_004dc1d0();
                DAT_00aa5e38 = 0;
            }
        }
        0
    }
}
