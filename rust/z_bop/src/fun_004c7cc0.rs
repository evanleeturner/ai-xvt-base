// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b6a2e0: u8;
    static mut DAT_00b6bc9e: u8;
    static mut DAT_00b69d20: i8;
    static mut DAT_00528118: i8;
    static mut DAT_0052b48c: i8;
    static mut DAT_0052ad74: i8;
    static mut DAT_00525034: u8;
    static mut DAT_00b6d962: u8;
    static mut DAT_00b7c342: u8;
    static mut DAT_00b7c6c6: u8;
    static mut DAT_00b7ca4a: u8;
    static mut DAT_00b7d6ca: u8;
    static mut DAT_00b7ca5e: u8;
    static mut DAT_00b7d6de: u8;
    static mut DAT_00b6a248: i32;
    static mut DAT_00b6a258: *mut core::ffi::c_void;
    static mut DAT_00b6a25c: u32;
    static mut DAT_00aa6114: *mut core::ffi::c_void;
    static mut DAT_00aa6138: u32;
    static mut DAT_00664f2c: i32;
    static mut DAT_00664eac: i32;
    static mut DAT_00664fb8: i32;
    static mut DAT_00b69e34: u32;
    static mut DAT_00b6a2c0: i32;
    static mut DAT_00b6a2a0: i32;
    static mut DAT_00bb2810: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004de090(a: *mut i32, b: *mut i32);
    fn FUN_004d6460(r: *mut i32, a: i32, b: i32, c: i32, d: i32);
    fn FUN_004ddc10(a: i32) -> *mut i8;
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004db640(a: i32, s: *mut i8, r: *mut i32, c: i32);
    fn free(p: *mut core::ffi::c_void);
    fn FUN_004e5000(a: i32);
    fn FUN_004b4cc0(s: *mut i8, r: *mut i32);
    fn FUN_004d9d70(r: *mut i32, a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32;
    fn FUN_004d4cf0(r: *mut i32);
    fn FUN_004d4d20(r: *mut i32);
    fn FUN_004d64a0(r: *mut i32, a: i32, b: i32);
    fn FUN_004b5de0(s: *mut i8, a: i32, b: i32);
    fn FUN_004d6b90(r: *mut i32, a: i32, b: i32) -> i32;
    fn FUN_004d6950(r: *mut i32, a: i32, b: i32, c: i16);
    fn FUN_004da780(
        r: *mut i32,
        a: *mut u8,
        b: *mut u8,
        c: *mut i8,
        d: i32,
        e: i32,
        f: i32,
        g: *mut i8,
    ) -> i32;
    fn FUN_004d3350();
    fn FUN_004d4f70();
    fn FUN_004d4970();
    fn FUN_004d4c10();
    fn FUN_004d49a0();
    fn FUN_004efce0(s: *mut i8, a: i32);
    fn FUN_004d4f60();
    fn FUN_004caf60();
    fn FUN_004d3400();
}

unsafe fn rd(addr: i32) -> i32 {
    // SAFETY: the address is a game-memory location the original reads.
    unsafe { (addr as usize as *const i32).read_unaligned() }
}

unsafe fn rdb(base: *mut u8, off: i32) -> i32 {
    // SAFETY: base is one of the game's tables; offset as in the original.
    unsafe { base.offset(off as isize).cast::<i32>().read_unaligned() }
}

unsafe fn ad(a: i32) -> *mut i8 {
    a as usize as *mut i8
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004c7cc0() -> u32 {
    let mut local_14: i32 = 0;
    let mut local_18: i32 = 0;
    let mut local_28: [i32; 4] = [0; 4];
    let mut local_38: [i32; 4] = [0; 4];
    let mut local_10: [i32; 4] = [0; 4];
    let mut local_3c: i32;
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are used the way the original uses them.
    unsafe {
        FUN_004de090(&raw mut local_14, &raw mut local_18);
        FUN_004d6460(local_28.as_mut_ptr(), 0x54, 0x5a, 0x194, 0x6a);
        if DAT_00b6a2e0 == 0 {
            let pc = FUN_004ddc10(0x31c);
            sprintf(&raw mut DAT_00b69d20, &raw const DAT_00528118, pc);
        } else {
            let p13 = &raw mut DAT_00b6a2e0;
            let u12: u32 = 4;
            let p11 = &raw mut DAT_00b6bc9e;
            let u10: u32 = 6;
            let pc = FUN_004ddc10(0x31c);
            sprintf(
                &raw mut DAT_00b69d20,
                &raw const DAT_0052b48c,
                pc,
                u10,
                p11,
                u12,
                p13,
            );
        }
        FUN_004db640(0xf, &raw mut DAT_00b69d20, local_28.as_mut_ptr(), 0xffff);
        if DAT_00b6a2e0 == 0 {
            return 0;
        }
        if DAT_00b6a248 == 0 {
            return 0;
        }
        if DAT_00664f2c != 0 {
            if !DAT_00b6a258.is_null() {
                free(DAT_00b6a258);
                DAT_00b6a258 = core::ptr::null_mut();
            }
            FUN_004e5000(5);
            let mut u8_: u32 = 0;
            DAT_00b6a258 = DAT_00aa6114;
            DAT_00b6a25c = DAT_00aa6138;
            DAT_00aa6114 = core::ptr::null_mut();
            if DAT_00aa6138 != 0 {
                let mut iv9: i32 = 0;
                loop {
                    let mut u6: u32 = 0xffffffff;
                    let mut p = (DAT_00b6a258 as i32).wrapping_add(iv9).wrapping_add(0x40);
                    loop {
                        if u6 == 0 {
                            break;
                        }
                        u6 = u6.wrapping_sub(1);
                        let c = *ad(p);
                        p = p.wrapping_add(1);
                        if c == 0 {
                            break;
                        }
                    }
                    let mut iv7: i32 = (!u6).wrapping_sub(2) as i32;
                    while iv7 != 0 {
                        let q = ad((DAT_00b6a258 as i32)
                            .wrapping_add(iv7)
                            .wrapping_add(iv9)
                            .wrapping_add(0x40));
                        if *q == 0x28 {
                            *q = 0;
                            break;
                        }
                        iv7 = iv7.wrapping_sub(1);
                    }
                    iv9 = iv9.wrapping_add(0x148);
                    u8_ = u8_.wrapping_add(1);
                    if !(u8_ < DAT_00b6a25c) {
                        break;
                    }
                }
            }
            let mut iv9: i32 = 0;
            DAT_00664eac = 0;
            DAT_00664fb8 = 0;
            let mut local_48: u32 = 0;
            if DAT_00b6a25c != 0 {
                loop {
                    let ibase = rdb(&raw mut DAT_00b6d962, 0);
                    let mut iv7: i32 =
                        (rd((DAT_00b6a258 as i32).wrapping_add(iv9).wrapping_add(0x140)) as u32)
                            .wrapping_mul(0x24)
                            .wrapping_add((ibase as u32).wrapping_mul(0x109e0))
                            as i32;
                    let mut skip = rdb(&raw mut DAT_00b7c342, iv7) != 0;
                    if !skip {
                        skip = rdb(&raw mut DAT_00b7c6c6, iv7) != 0;
                    }
                    if skip {
                        let mut u8_: u32 = 0;
                        let mut b2 = false;
                        if DAT_00b69e34 != 0 {
                            iv7 = 0;
                            loop {
                                'skip: {
                                    if rd((DAT_00b6a258 as i32)
                                        .wrapping_add(iv9)
                                        .wrapping_add(0x140))
                                        == rd(DAT_00b6a248.wrapping_add(0x120).wrapping_add(iv7))
                                    {
                                        let iv5 = iv7.wrapping_add(DAT_00b6a248);
                                        if rd(iv7.wrapping_add(0x124).wrapping_add(DAT_00b6a248))
                                            == 0
                                        {
                                            let ib = rdb(&raw mut DAT_00b6d962, 0);
                                            let iv4 = ((ib as u32)
                                                .wrapping_mul(0x84f)
                                                .wrapping_add(rd(iv5.wrapping_add(0x128)) as u32)
                                                .wrapping_mul(0x20))
                                                as i32;
                                            if rdb(&raw mut DAT_00b7ca4a, iv4) == 0 {
                                                if rdb(&raw mut DAT_00b7d6ca, iv4) == 0 {
                                                    break 'skip;
                                                }
                                            }
                                        } else {
                                            let ib = rdb(&raw mut DAT_00b6d962, 0);
                                            let iv4 = ((ib as u32)
                                                .wrapping_mul(0x84f)
                                                .wrapping_add(rd(iv5.wrapping_add(0x128)) as u32)
                                                .wrapping_mul(0x20))
                                                as i32;
                                            if rdb(&raw mut DAT_00b7ca5e, iv4) == 0 {
                                                if rdb(&raw mut DAT_00b7d6de, iv4) == 0 {
                                                    break 'skip;
                                                }
                                            }
                                        }
                                        if !b2 {
                                            b2 = true;
                                            DAT_00664fb8 = DAT_00664fb8.wrapping_add(2);
                                        }
                                        DAT_00664fb8 = DAT_00664fb8.wrapping_add(1);
                                        FUN_004b4cc0(
                                            ad(iv5.wrapping_add(0x80)),
                                            local_38.as_mut_ptr(),
                                        );
                                        DAT_00664fb8 = DAT_00664fb8.wrapping_add(
                                            ((local_38[3]
                                                .wrapping_sub(local_38[1])
                                                .wrapping_add(0x12))
                                                as u32
                                                / 0xf)
                                                as i32,
                                        );
                                    }
                                }
                                iv7 = iv7.wrapping_add(300);
                                u8_ = u8_.wrapping_add(1);
                                if !(u8_ < DAT_00b69e34) {
                                    break;
                                }
                            }
                        }
                    }
                    iv9 = iv9.wrapping_add(0x148);
                    local_48 = local_48.wrapping_add(1);
                    if !(local_48 < DAT_00b6a25c) {
                        break;
                    }
                }
            }
            DAT_00664f2c = 0;
        }
        FUN_004d6460(local_28.as_mut_ptr(), 0x1a9, 0x6b, 0x1b2, 0x1b1);
        if 0x15 < DAT_00664fb8 {
            DAT_00664eac = FUN_004d9d70(
                local_28.as_mut_ptr(),
                DAT_00664eac,
                DAT_00664fb8,
                0,
                5,
                DAT_00bb2810,
                0xb,
            );
        }
        FUN_004d4cf0(local_10.as_mut_ptr());
        FUN_004d6460(local_28.as_mut_ptr(), 0x58, 0x6f, 0x1a8, 0x1b1);
        FUN_004d4d20(local_28.as_mut_ptr());
        local_3c = 0;
        let mut iv9: i32 = (DAT_00664eac as u32)
            .wrapping_mul(0xfffffff1)
            .wrapping_add(0x6f) as i32;
        FUN_004d6460(
            local_28.as_mut_ptr(),
            0x58,
            iv9,
            0x1a8,
            (DAT_00664eac as u32)
                .wrapping_mul(0xfffffff1)
                .wrapping_add(0x7d) as i32,
        );
        let mut local_48: u32 = 0;
        if DAT_00b6a25c != 0 {
            let mut local_44: i32 = 0;
            'done: {
                loop {
                    let ibase = rdb(&raw mut DAT_00b6d962, 0);
                    let mut iv7: i32 = (rd((DAT_00b6a258 as i32)
                        .wrapping_add(local_44)
                        .wrapping_add(0x140)) as u32)
                        .wrapping_mul(0x24)
                        .wrapping_add((ibase as u32).wrapping_mul(0x109e0))
                        as i32;
                    let mut skip = rdb(&raw mut DAT_00b7c342, iv7) != 0;
                    if !skip {
                        skip = rdb(&raw mut DAT_00b7c6c6, iv7) != 0;
                    }
                    if skip {
                        let mut b2 = false;
                        let mut local_40: u32 = 0;
                        if DAT_00b69e34 != 0 {
                            iv7 = 0;
                            loop {
                                'skip: {
                                    if rd((DAT_00b6a258 as i32)
                                        .wrapping_add(local_44)
                                        .wrapping_add(0x140))
                                        == rd(DAT_00b6a248.wrapping_add(0x120).wrapping_add(iv7))
                                    {
                                        let iv5a = iv7.wrapping_add(DAT_00b6a248);
                                        if rd(iv5a.wrapping_add(0x124)) == 0 {
                                            let ib = rdb(&raw mut DAT_00b6d962, 0);
                                            let iv5 = ((ib as u32)
                                                .wrapping_mul(0x84f)
                                                .wrapping_add(rd(iv5a.wrapping_add(0x128)) as u32)
                                                .wrapping_mul(0x20))
                                                as i32;
                                            if rdb(&raw mut DAT_00b7ca4a, iv5) == 0 {
                                                if rdb(&raw mut DAT_00b7d6ca, iv5) == 0 {
                                                    break 'skip;
                                                }
                                            }
                                        } else {
                                            let ib = rdb(&raw mut DAT_00b6d962, 0);
                                            let iv5 = ((ib as u32)
                                                .wrapping_mul(0x84f)
                                                .wrapping_add(rd(iv5a.wrapping_add(0x128)) as u32)
                                                .wrapping_mul(0x20))
                                                as i32;
                                            if rdb(&raw mut DAT_00b7ca5e, iv5) == 0 {
                                                if rdb(&raw mut DAT_00b7d6de, iv5) == 0 {
                                                    break 'skip;
                                                }
                                            }
                                        }
                                        if !b2 {
                                            iv9 = iv9.wrapping_add(0x1e);
                                            b2 = true;
                                            FUN_004db640(
                                                0xc,
                                                ad((DAT_00b6a258 as i32)
                                                    .wrapping_add(local_44)
                                                    .wrapping_add(0x40)),
                                                local_28.as_mut_ptr(),
                                                DAT_00b6a2c0,
                                            );
                                            FUN_004d64a0(local_28.as_mut_ptr(), 0, 0x1e);
                                        }
                                        FUN_004db640(
                                            0xc,
                                            ad(DAT_00b6a248.wrapping_add(iv7).wrapping_add(0xa0)),
                                            local_28.as_mut_ptr(),
                                            0xffff,
                                        );
                                        FUN_004d64a0(local_28.as_mut_ptr(), 0, 0xf);
                                        FUN_004b4cc0(
                                            ad(DAT_00b6a248.wrapping_add(iv7).wrapping_add(0x80)),
                                            local_38.as_mut_ptr(),
                                        );
                                        let iv4 = 0x100u32.wrapping_sub(
                                            (local_38[2].wrapping_sub(local_38[0]).wrapping_add(1)
                                                as u32)
                                                >> 1,
                                        ) as i32;
                                        let iv5 = (((local_38[3]
                                            .wrapping_sub(local_38[1])
                                            .wrapping_add(0x12))
                                            as u32
                                            / 0xf)
                                            .wrapping_mul(0xf))
                                            as i32;
                                        FUN_004b5de0(
                                            ad(DAT_00b6a248.wrapping_add(iv7).wrapping_add(0x80)),
                                            iv4,
                                            iv9.wrapping_add(0x12),
                                        );
                                        FUN_004d64a0(
                                            local_38.as_mut_ptr(),
                                            iv4,
                                            iv9.wrapping_add(0x12),
                                        );
                                        let r =
                                            FUN_004d6b90(local_38.as_mut_ptr(), local_14, local_18);
                                        if r != 0 {
                                            FUN_004d6950(
                                                local_38.as_mut_ptr(),
                                                0,
                                                0,
                                                DAT_00b6a2a0 as i16,
                                            );
                                            let r2 = FUN_004da780(
                                                local_38.as_mut_ptr(),
                                                &raw mut DAT_00525034,
                                                &raw mut DAT_00525034,
                                                core::ptr::null_mut(),
                                                0xf,
                                                0xffff,
                                                0x24,
                                                &raw mut DAT_0052ad74,
                                            );
                                            if r2 != 0 {
                                                local_3c = local_40.wrapping_add(1) as i32;
                                            }
                                        }
                                        iv9 =
                                            (iv9 as u32).wrapping_add(0xf).wrapping_add(iv5 as u32)
                                                as i32;
                                        FUN_004d64a0(local_28.as_mut_ptr(), 0, iv5);
                                        if 0x1b1 < iv9 {
                                            break 'done;
                                        }
                                    }
                                }
                                iv7 = iv7.wrapping_add(300);
                                local_40 = local_40.wrapping_add(1);
                                if !(local_40 < DAT_00b69e34) {
                                    break;
                                }
                            }
                        }
                        if 0x1b1 < iv9 {
                            break;
                        }
                    }
                    local_44 = local_44.wrapping_add(0x148);
                    local_48 = local_48.wrapping_add(1);
                    if !(local_48 < DAT_00b6a25c) {
                        break;
                    }
                }
            }
        }
        FUN_004d4d20(local_10.as_mut_ptr());
        if local_3c != 0 {
            FUN_004d3350();
            FUN_004d4f70();
            FUN_004d4970();
            FUN_004d4c10();
            FUN_004d49a0();
            FUN_004d4c10();
            FUN_004efce0(
                ad((local_3c as u32)
                    .wrapping_mul(300)
                    .wrapping_add(DAT_00b6a248 as u32)
                    .wrapping_sub(300) as i32),
                0,
            );
            FUN_004d4c10();
            FUN_004d49a0();
            FUN_004d4c10();
            FUN_004d4f60();
            FUN_004caf60();
            FUN_004d3400();
        }
    }
    1
}
