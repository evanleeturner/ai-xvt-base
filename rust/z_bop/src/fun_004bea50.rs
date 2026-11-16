// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052b0e0: i32;
    static mut DAT_00664f2c: i32;
    static mut DAT_00664fa8: u32;
    static mut DAT_00664fac: u32;
    static mut DAT_00664fb0: u32;
    static mut DAT_00664fb4: u16;
    static mut DAT_00b69e28: *mut u32;
    static mut DAT_00b69cc0: i32;
    static mut DAT_00b6a2e0: i8;
    static mut DAT_00b6a5a6: u32;
    static mut DAT_00b6a5aa: u32;
    static mut DAT_00b6a5ae: [u8; 24];
    static mut DAT_00b6a606: u32;
    static mut DAT_00b6a60a: u32;
    static mut DAT_00b6d962: i32;
    static mut DAT_00b6d96a: u32;
    static mut DAT_00b6d96e: u32;
    static mut DAT_00b6d972: [u32; 6];
    static mut DAT_00b6d9aa: u32;
    static mut DAT_00b6d9ae: u32;
    static mut DAT_00bb2810: u32;
    static mut DAT_00bb73a7: i8;
    static mut DAT_0052b350: [i8; 1];
    static mut DAT_0052a68c: [i8; 1];
    static mut DAT_0052b340: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6460(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004ddc10(param_1: i32) -> *mut i8;
    fn FUN_004db640(param_1: i32, param_2: *mut i8, param_3: *mut i32, param_4: u32);
    fn FUN_004dcb30();
    fn FUN_004bf100(param_1: *mut i32);
    fn FUN_004d9d70(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: u32,
        param_7: i32,
    ) -> i32;
    fn FUN_004bef80(param_1: *mut i32, param_2: i32) -> i32;
    fn __strnicmp(param_1: *const i8, param_2: *const i8, param_3: u32) -> i32;
    fn FUN_004cb0c0(param_1: *mut i8) -> i32;
    fn FUN_004de5a0(param_1: *mut i8, param_2: i32, param_3: u32, param_4: u32, ...) -> i32;
    fn FUN_004caf60();
    fn FUN_004dd220(param_1: *mut u32) -> i32;
    fn FUN_004da380(
        param_1: *mut i32,
        param_2: *mut i8,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: *mut i8,
    ) -> i32;
    fn __strcmpi(param_1: *const i8, param_2: *const i8) -> i32;
    fn FUN_004bf3a0(param_1: *mut i8);
    fn FUN_004da650(
        param_1: *mut i32,
        param_2: *mut i8,
        param_3: i32,
        param_4: u32,
        param_5: i32,
        param_6: *mut i8,
    ) -> i32;
    fn FUN_004dca90(param_1: i32) -> u8;
    fn FUN_004dcb90(
        param_1: *mut i8,
        param_2: *mut i8,
        param_3: *mut i8,
        param_4: *mut i8,
        param_5: *mut i8,
    ) -> i32;
    fn FUN_004bf260();
}

#[allow(non_snake_case)]
unsafe fn copy_profile() {
    // SAFETY: all globals are the game's own memory; packed ones are accessed unaligned.
    unsafe {
        let local_idx: i32 = (&raw const DAT_00b6d962).cast::<i32>().read_unaligned();
        let off = local_idx.wrapping_mul(0x4278) as isize;
        let mut local_val: u32 = (&raw const DAT_00b6d96a)
            .wrapping_offset(off)
            .read_unaligned();
        (&raw mut DAT_00b6a5a6).write_unaligned(local_val);
        local_val = (&raw const DAT_00b6d96e)
            .wrapping_offset(off)
            .read_unaligned();
        (&raw mut DAT_00b6a5aa).write_unaligned(local_val);
        let mut src = (&raw const DAT_00b6d972).cast::<u32>().wrapping_offset(off);
        let mut dst = (&raw mut DAT_00b6a5ae).cast::<u8>();
        let mut n = 6;
        while n != 0 {
            dst.cast::<u32>().write_unaligned(src.read_unaligned());
            src = src.wrapping_add(1);
            dst = dst.wrapping_add(4);
            n -= 1;
        }
        local_val = (&raw const DAT_00b6d9aa)
            .wrapping_offset(off)
            .read_unaligned();
        (&raw mut DAT_00b6a606).write_unaligned(local_val);
        local_val = (&raw const DAT_00b6d9ae)
            .wrapping_offset(off)
            .read_unaligned();
        (&raw mut DAT_00b6a60a).write_unaligned(local_val);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004bea50(param_1: i32) -> u32 {
    let mut local_14: i32 = 0;
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: the callees and globals are the game's own; pointers passed are
    // to live locals or the game's globals.
    unsafe {
        FUN_004d6460(local_10.as_mut_ptr(), 0x1c3, 0x5a, 0x25d, 0x6a);
        let pc = FUN_004ddc10(0x1cf);
        FUN_004db640(0xc, pc, local_10.as_mut_ptr(), 0xffff);
        if param_1 == 0 {
            FUN_004dcb30();
            DAT_00664fa8 = 0;
            DAT_00664fac = 0;
            DAT_00664fb0 = 0;
            DAT_00664fb4 = 0;
            if DAT_00b69e28.is_null() || (DAT_00b69e28.add(1).read() as i32) < 0xe {
                FUN_004bf100(&raw mut local_14);
            } else {
                FUN_004bf100(&raw mut DAT_0052b0e0);
            }
        }
        if !DAT_00b69e28.is_null() && 0xd < (DAT_00b69e28.add(1).read() as i32) {
            FUN_004d6460(local_10.as_mut_ptr(), 0x254, 0x75, 0x25d, 0x140);
            DAT_0052b0e0 = FUN_004d9d70(
                local_10.as_mut_ptr(),
                DAT_0052b0e0,
                DAT_00b69e28.add(1).read() as i32,
                0,
                5,
                DAT_00bb2810,
                4,
            );
        }
        FUN_004d6460(local_10.as_mut_ptr(), 0x1cd, 0x75, 0x253, 0x140);
        local_14 = FUN_004bef80(local_10.as_mut_ptr(), DAT_0052b0e0);
        if local_14 != 0 {
            let p2 = (local_14 as u32)
                .wrapping_mul(0xe)
                .wrapping_add(DAT_00b69cc0 as u32)
                .wrapping_add(0xffff_fff2) as usize as *const i8;
            let r = __strnicmp(&raw const DAT_00b6a2e0, p2, 0xc);
            if r != 0 {
                let mut puvar7 = (*DAT_00b69e28) as usize as *mut u32;
                let mut i3 = local_14.wrapping_sub(1);
                if 0 < i3 {
                    loop {
                        puvar7 = puvar7.add(1).read() as usize as *mut u32;
                        i3 = i3.wrapping_sub(1);
                        if i3 == 0 {
                            break;
                        }
                    }
                }
                let i3 = FUN_004cb0c0(puvar7.read() as usize as *mut i8);
                if i3 != 0 {
                    if DAT_00bb73a7 != 0 {
                        FUN_004de5a0((&raw mut DAT_0052b350).cast::<i8>(), 1, 0, 0xff, 0i32, 0i32);
                    }
                    copy_profile();
                }
            }
            DAT_00664fa8 = 0;
            DAT_00664fac = 0;
            DAT_00664fb0 = 0;
            DAT_00664fb4 = 0;
            FUN_004bf100(&raw mut local_14);
            FUN_004caf60();
            DAT_00664f2c = 1;
        }
        if DAT_00b69e28.add(1).read() == 0 {
            local_14 = FUN_004dd220(&raw mut DAT_00664fa8);
        } else {
            FUN_004d6460(local_10.as_mut_ptr(), 0x1cd, 0x141, 0x253, 0x154);
            local_14 = FUN_004da380(
                local_10.as_mut_ptr(),
                (&raw mut DAT_00664fa8).cast::<i8>(),
                0xd,
                0,
                0xc,
                (&raw mut DAT_0052b340).cast::<i8>(),
            );
        }
        let mut ivar3: i32 = 0;
        if local_14 != 0 && (DAT_00664fa8 as u8) != 0 {
            local_14 = 0;
            DAT_00664f2c = 1;
            if !DAT_00b69e28.is_null() {
                let mut ivar8: i32 = 0;
                let mut puvar7 = (*DAT_00b69e28) as usize as *mut u32;
                if DAT_00b69cc0 != 0 && 0 < (DAT_00b69e28.add(1).read() as i32) {
                    loop {
                        let p1 =
                            (DAT_00b69cc0 as u32).wrapping_add(ivar3 as u32) as usize as *const i8;
                        let iv4 = __strcmpi(p1, (&raw const DAT_00664fa8).cast::<i8>());
                        if iv4 == 0 {
                            let r = FUN_004cb0c0(puvar7.read() as usize as *mut i8);
                            if r != 0 {
                                if DAT_00bb73a7 != 0 {
                                    FUN_004de5a0(
                                        (&raw mut DAT_0052b350).cast::<i8>(),
                                        1,
                                        0,
                                        0xff,
                                        0i32,
                                        0i32,
                                    );
                                }
                                copy_profile();
                            }
                            break;
                        }
                        ivar8 = ivar8.wrapping_add(1);
                        puvar7 = puvar7.add(1).read() as usize as *mut u32;
                        ivar3 = ivar3.wrapping_add(0xe);
                        if !(ivar8 < (DAT_00b69e28.add(1).read() as i32)) {
                            break;
                        }
                    }
                }
            }
            if local_14 == 0 {
                if DAT_00bb73a7 != 0 {
                    FUN_004de5a0((&raw mut DAT_0052b350).cast::<i8>(), 1, 0, 0xff);
                }
                FUN_004bf3a0((&raw mut DAT_00664fa8).cast::<i8>());
                if DAT_00b69e28.is_null() || (DAT_00b69e28.add(1).read() as i32) < 0xe {
                    FUN_004bf100(&raw mut local_14);
                } else {
                    FUN_004bf100(&raw mut DAT_0052b0e0);
                }
            }
            DAT_00664fa8 = 0;
            DAT_00664fac = 0;
            DAT_00664fb0 = 0;
            DAT_00664fb4 = 0;
            FUN_004caf60();
        }
        FUN_004d6460(local_10.as_mut_ptr(), 0x1c3, 0x166, 0x25d, 0x17a);
        let pc = FUN_004ddc10(0x1d0);
        local_14 = FUN_004da650(
            local_10.as_mut_ptr(),
            pc,
            0xf,
            0xffff,
            0x14,
            (&raw mut DAT_0052a68c).cast::<i8>(),
        );
        if (local_14 != 0 || FUN_004dca90(0x2e) != 0) && DAT_00b6a2e0 != 0 {
            let pc2 = FUN_004ddc10(0x212);
            let pc14 = FUN_004ddc10(0x211);
            let pc11: *mut i8 = core::ptr::null_mut();
            let pc5 = FUN_004ddc10(0x210);
            let pc6 = FUN_004ddc10(0x20f);
            let r = FUN_004dcb90(pc6, pc5, pc11, pc14, pc2);
            if r != 0 {
                FUN_004bf260();
            }
            DAT_0052b0e0 = 0;
        }
    }
    1
}
