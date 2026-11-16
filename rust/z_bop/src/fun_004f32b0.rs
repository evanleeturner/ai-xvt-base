// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa5df0: [i32; 8];
    static mut DAT_00aa5e38: i32;
    static mut DAT_00aa5e40: [i32; 0];
    static mut DAT_00aa5e60: i32;
    static mut DAT_00aa5e68: i32;
    static mut DAT_00aa5e10: [i32; 0];
    static mut DAT_00aa5cb0: [i32; 0];
    static mut DAT_00aa615e: [u8; 0];
    static mut DAT_00aa6af0: i32;
    static mut DAT_00aa6af4: i32;
    static mut DAT_00aa6af8: i32;
    static mut DAT_00b69d20: [i8; 0];
    static mut DAT_00b69e30: i32;
    static mut DAT_00b6a2c0: u32;
    static mut _DAT_00b69ccc: u32;
    static mut DAT_00b6a270: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004d6460(p: *mut i32, a: i32, b: i32, c: i32, d: i32);
    fn FUN_004ddc10(a: i32) -> *mut i8;
    fn FUN_004db7d0(a: i32, b: *mut i8, c: *mut i32, d: i32, e: i32, f: u32);
    fn FUN_004d1400() -> i32;
    fn FUN_004d4cf0(p: *mut core::ffi::c_void);
    fn FUN_004d4d20(p: *mut core::ffi::c_void);
    fn FUN_004d11e0() -> i32;
    fn FUN_004cfee0() -> i32;
    fn FUN_004dc2e0(a: i32) -> i8;
    fn FUN_004de090(a: *mut i32, b: *mut i32);
    fn FUN_004dc1e0() -> i8;
    fn FUN_004dc200() -> i8;
    fn FUN_004d6b90(p: *mut i32, a: i32, b: i32) -> i32;
    fn FUN_004dc1c0(a: i32);
    fn FUN_004d11d0() -> i32;
    fn FUN_004cef70(a: i32, p: *mut u32, b: i32);
    fn FUN_004d64a0(p: *mut i32, a: i32, b: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004f32b0(param_1: i32) -> i32 {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are used at the offsets and widths the original uses.
    unsafe {
        let a5df0: *mut i32 = (&raw mut DAT_00aa5df0).cast::<i32>();
        let a5e40: *mut i32 = (&raw mut DAT_00aa5e40).cast::<i32>();
        let a5e10: *mut i32 = (&raw mut DAT_00aa5e10).cast::<i32>();
        let a5cb0: *mut i32 = (&raw mut DAT_00aa5cb0).cast::<i32>();
        let buf: *mut i8 = (&raw mut DAT_00b69d20).cast::<i8>();
        let a6af0: *mut u32 = (&raw mut DAT_00aa6af0).cast::<u32>();

        let mut local_30: [i32; 4] = [0; 4];
        let mut local_10: [u32; 4] = [0; 4];
        let mut local_20: i32 = 0;
        let mut local_1c: i32 = 0;
        let p30: *mut i32 = local_30.as_mut_ptr();

        FUN_004d6460(p30, 0x58, 0x72, 0x1ae, 0x80);
        let mut uvar14: u32 = 0xffff;
        let pc = FUN_004ddc10(0x1da);
        let mut ivar9: i32 = 1;
        FUN_004db7d0(0xf, pc, p30, 0, 1, uvar14);
        if DAT_00b69e30 != 2 {
            ivar9 = FUN_004d1400();
        }
        let mut local_34: i32 = 0;
        FUN_004d6460(p30, 0x58, 0x86, 0x102, 0x94);
        if 0 < ivar9 {
            let mut local_40: *mut u8 = (&raw mut DAT_00aa615e).cast::<u8>();
            let mut ivar11: i32 = DAT_00aa5e60;
            let mut local_14: i32 = ivar9;
            loop {
                let mut ivar12: i32 = 0;
                let mut pvar5: *mut i32 = a5df0;
                let mut tmp: i32 = local_40.cast::<i32>().read_unaligned();
                ivar9 = tmp;
                loop {
                    if *pvar5 == ivar9 {
                        break;
                    }
                    pvar5 = pvar5.add(1);
                    ivar12 = ivar12.wrapping_add(1);
                    if !(pvar5 < a5df0.add(8)) {
                        break;
                    }
                }
                if ivar12 == 8 && DAT_00aa5e38 != ivar9 {
                    ivar12 = 0;
                    if 0 < ivar11 {
                        pvar5 = a5e40;
                        loop {
                            if *pvar5 == ivar9 {
                                break;
                            }
                            pvar5 = pvar5.add(1);
                            ivar12 = ivar12.wrapping_add(1);
                            if !(ivar12 < ivar11) {
                                break;
                            }
                        }
                    }
                    FUN_004d4cf0(local_10.as_mut_ptr().cast());
                    FUN_004d4d20(p30.cast());
                    if ivar12 == DAT_00aa5e60 {
                        ivar9 = (local_40 as usize as i32).wrapping_sub(0xe);
                        let uvar13: u32 = 1;
                        tmp = local_40.add(4).cast::<i32>().read_unaligned();
                        let pc = FUN_004ddc10(tmp.wrapping_add(0x9a));
                        sprintf(buf, c"%c%s %c%s".as_ptr(), 6i32, pc, uvar13, ivar9);
                        ivar9 = FUN_004d11e0();
                        tmp = local_40.cast::<i32>().read_unaligned();
                        let off = ((param_1 % 0x18) as u32 & 0xfffffffe).wrapping_mul(2);
                        let sel: u32 = (&raw mut DAT_00b6a270)
                            .cast::<u8>()
                            .byte_add(off as usize)
                            .cast::<u32>()
                            .read_unaligned();
                        if ivar9 == tmp {
                            uvar14 = sel;
                        } else {
                            uvar14 = DAT_00b6a2c0;
                            if DAT_00b69e30 == 2 {
                                uvar14 = sel;
                            }
                        }
                    } else {
                        ivar9 = (local_40 as usize as i32).wrapping_sub(0xe);
                        tmp = local_40.add(4).cast::<i32>().read_unaligned();
                        let pc = FUN_004ddc10(tmp.wrapping_add(0x9a));
                        sprintf(buf, c"%s %s".as_ptr(), pc, ivar9);
                        uvar14 = _DAT_00b69ccc;
                    }
                    FUN_004db7d0(0xc, buf, p30, 0, 1, uvar14);
                    local_34 = local_34.wrapping_add(1);
                    FUN_004d4d20(local_10.as_mut_ptr().cast());
                    ivar9 = FUN_004cfee0();
                    if (ivar9 != 0 || DAT_00b69e30 == 2) && FUN_004dc2e0(2) == 0 {
                        FUN_004de090(&raw mut local_1c, &raw mut local_20);
                        if (FUN_004dc1e0() != 0 || FUN_004dc200() != 0) && {
                            ivar9 = FUN_004d6b90(p30, local_1c, local_20);
                            ivar9 != 0
                        } {
                            if DAT_00b69e30 == 2 {
                                DAT_00aa5e38 = local_40.cast::<i32>().read_unaligned();
                                FUN_004dc1c0(2);
                            } else {
                                ivar9 = 0;
                                if 0 < DAT_00aa5e60 {
                                    pvar5 = a5e40;
                                    loop {
                                        tmp = local_40.cast::<i32>().read_unaligned();
                                        if *pvar5 == tmp {
                                            break;
                                        }
                                        pvar5 = pvar5.add(1);
                                        ivar9 = ivar9.wrapping_add(1);
                                        if !(ivar9 < DAT_00aa5e60) {
                                            break;
                                        }
                                    }
                                }
                                if ivar9 == DAT_00aa5e60 {
                                    DAT_00aa5e60 = DAT_00aa5e60.wrapping_add(1);
                                    DAT_00aa6af0 = 0x68;
                                    DAT_00aa6af4 = local_40.cast::<i32>().read_unaligned();
                                    *a5e40.offset(ivar9 as isize) = DAT_00aa6af4;
                                    DAT_00aa6af8 = FUN_004d11d0();
                                    FUN_004cef70(0, a6af0, 0xc);
                                    DAT_00aa5e38 = local_40.cast::<i32>().read_unaligned();
                                    FUN_004dc1c0(2);
                                    DAT_00aa6af0 = 0x4f;
                                    DAT_00aa6af4 = DAT_00aa5e38;
                                    DAT_00aa6af8 = 10;
                                    FUN_004cef70(0, a6af0, 0xc);
                                    ivar11 = DAT_00aa5e68;
                                    ivar9 = DAT_00aa5e38;
                                    pvar5 = a5df0;
                                    loop {
                                        if *pvar5 == ivar9 {
                                            *pvar5 = 0;
                                        }
                                        pvar5 = pvar5.add(1);
                                        if !(pvar5 < a5df0.add(8)) {
                                            break;
                                        }
                                    }
                                    if 0 < ivar11 {
                                        let mut puvar10: *mut i32 = a5cb0.add(7);
                                        ivar12 = 0;
                                        let mut local_38: *mut i32 = a5e10;
                                        let mut local_18: i32 = ivar11;
                                        loop {
                                            let mut ivar6: i32 = 0;
                                            ivar11 = *local_38;
                                            if 0 < ivar11 {
                                                loop {
                                                    if *a5cb0
                                                        .offset(ivar12.wrapping_add(ivar6) as isize)
                                                        == ivar9
                                                    {
                                                        let ivar1 = ivar6.wrapping_add(1);
                                                        if ivar1 < 8 {
                                                            let mut ivar7 =
                                                                8i32.wrapping_sub(ivar1);
                                                            let mut puvar8: *mut u32 = a5cb0
                                                                .offset(
                                                                    ivar12
                                                                        .wrapping_add(ivar1)
                                                                        .wrapping_sub(1)
                                                                        as isize,
                                                                )
                                                                .cast::<u32>();
                                                            loop {
                                                                ivar7 = ivar7.wrapping_sub(1);
                                                                *puvar8 = *puvar8.add(1);
                                                                puvar8 = puvar8.add(1);
                                                                if ivar7 == 0 {
                                                                    break;
                                                                }
                                                            }
                                                        }
                                                        *puvar10 = 0;
                                                    }
                                                    ivar6 = ivar6.wrapping_add(1);
                                                    if !(ivar6 < ivar11) {
                                                        break;
                                                    }
                                                }
                                            }
                                            puvar10 = puvar10.add(8);
                                            ivar12 = ivar12.wrapping_add(8);
                                            local_38 = local_38.add(1);
                                            local_18 = local_18.wrapping_sub(1);
                                            if local_18 == 0 {
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if local_34 == 4 {
                        FUN_004d6460(p30, 0x103, 0x86, 0x1ae, 0x94);
                        ivar11 = DAT_00aa5e60;
                    } else {
                        FUN_004d64a0(p30, 0, 0xf);
                        ivar11 = DAT_00aa5e60;
                    }
                }
                local_40 = local_40.add(0x2a);
                local_14 = local_14.wrapping_sub(1);
                if local_14 == 0 {
                    break;
                }
            }
        }
        1
    }
}
