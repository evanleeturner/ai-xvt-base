// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00528118: [i8; 1];
    static mut s__s___c_s__c_s_0052b48c: [i8; 1];
    static mut DAT_00b69d20: [i8; 1];
    static mut DAT_00b6a2e0: [i8; 1];
    static mut DAT_00b6bc9e: [i8; 1];
    static mut DAT_00b69cd8: *mut i32;
    static mut DAT_00b69cc8: u32;
    static mut DAT_00664f2c: i32;
    static mut DAT_00b6a258: *mut core::ffi::c_void;
    static mut DAT_00b6a2d4: *mut core::ffi::c_void;
    static mut DAT_00b6a2cc: *mut core::ffi::c_void;
    static mut DAT_00aa6114: *mut core::ffi::c_void;
    static mut DAT_00aa6138: u32;
    static mut DAT_00b6a25c: u32;
    static mut DAT_00b69d18: u32;
    static mut DAT_00b69e30: i32;
    static mut DAT_00bb2814: u32;
    static mut DAT_00bb2810: u32;
    static mut DAT_00664ee8: [u32; 16];
    static mut DAT_00664f60: [u32; 16];
    static mut DAT_00664ed0: i32;
    static mut DAT_00664fbc: i32;
    static mut DAT_00664ec8: i32;
    static mut DAT_00664f28: i32;
    static mut DAT_00b6a2c0: i32;
    static mut DAT_00b6d962: [u8; 4];
    static mut DAT_00b7c342: i8;
    static mut DAT_00b7c6c6: i8;
    static mut DAT_00b7ca46: i8;
    static mut DAT_00b7ca4a: i8;
    static mut DAT_00b7ca4e: i8;
    static mut DAT_00b7d6c6: i8;
    static mut DAT_00b7d6ca: i8;
    static mut DAT_00b7d6ce: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6460(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004ddc10(param_1: i32) -> *mut i8;
    fn _sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004db640(param_1: i32, param_2: *mut i8, param_3: *mut i32, param_4: i32);
    fn _free(p: *mut core::ffi::c_void);
    fn FUN_004e5000(param_1: i32);
    fn FUN_004d9d70(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: u32,
        param_7: i32,
    ) -> i32;
    fn FUN_004b4cc0(param_1: *mut i8, param_2: *mut i32);
    fn FUN_004b5570(param_1: *mut i8, param_2: i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004c83a0() -> u32 {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointer arithmetic mirrors the original's offsets.
    unsafe {
        let mut local_20: [i32; 4] = [0; 4];
        let mut local_10: [i32; 4] = [0; 4];
        let l20 = local_20.as_mut_ptr();
        let l10 = local_10.as_mut_ptr();

        let ver = || -> i32 { (&raw mut DAT_00b6d962).cast::<i32>().read_unaligned() };

        FUN_004d6460(l20, 0x54, 0x5a, 0x194, 0x6a);
        if (&raw mut DAT_00b6a2e0).cast::<i8>().read() == 0 {
            let pc = FUN_004ddc10(0x33a);
            _sprintf(
                (&raw mut DAT_00b69d20).cast::<i8>(),
                (&raw mut DAT_00528118).cast::<i8>() as *const i8,
                pc,
            );
        } else {
            let p15 = (&raw mut DAT_00b6a2e0).cast::<i8>();
            let u14: u32 = 4;
            let p8 = (&raw mut DAT_00b6bc9e).cast::<i8>();
            let u13: u32 = 6;
            let pc = FUN_004ddc10(0x33a);
            _sprintf(
                (&raw mut DAT_00b69d20).cast::<i8>(),
                (&raw mut s__s___c_s__c_s_0052b48c).cast::<i8>() as *const i8,
                pc,
                u13,
                p8,
                u14,
                p15,
            );
        }
        FUN_004db640(0xf, (&raw mut DAT_00b69d20).cast::<i8>(), l20, 0xffff);
        if (&raw mut DAT_00b6a2e0).cast::<i8>().read() == 0 {
            return 0;
        }
        if DAT_00b69cd8.is_null() {
            return 0;
        }
        if DAT_00664f2c != 0 {
            if !DAT_00b6a258.is_null() {
                _free(DAT_00b6a258);
                DAT_00b6a258 = core::ptr::null_mut();
            }
            if !DAT_00b6a2d4.is_null() {
                _free(DAT_00b6a2d4);
                DAT_00b6a2d4 = core::ptr::null_mut();
            }
            if !DAT_00b6a2cc.is_null() {
                _free(DAT_00b6a2cc);
                DAT_00b6a2cc = core::ptr::null_mut();
            }
            FUN_004e5000(5);
            let mut u10: u32 = 0;
            DAT_00b6a258 = DAT_00aa6114;
            DAT_00b6a25c = DAT_00aa6138;
            DAT_00aa6114 = core::ptr::null_mut();
            FUN_004e5000(0);
            DAT_00b6a2d4 = DAT_00aa6114;
            DAT_00b69d18 = DAT_00aa6138;
            DAT_00b69e30 = 4;
            DAT_00aa6114 = core::ptr::null_mut();
            FUN_004e5000(0);
            DAT_00b69e30 = 0;
            DAT_00b6a2cc = DAT_00aa6114;
            DAT_00bb2814 = DAT_00aa6138;
            DAT_00aa6114 = core::ptr::null_mut();

            let mut p8 = (&raw mut DAT_00664ee8).cast::<u32>();
            let mut i4: i32 = 0x10;
            while i4 != 0 {
                p8.write(0);
                p8 = p8.wrapping_add(1);
                i4 -= 1;
            }
            p8 = (&raw mut DAT_00664f60).cast::<u32>();
            i4 = 0x10;
            while i4 != 0 {
                p8.write(0);
                p8 = p8.wrapping_add(1);
                i4 -= 1;
            }
            if DAT_00b6a25c != 0 {
                let mut iv4: i32 = 0;
                loop {
                    let mut u5: u32 = 0xffffffff;
                    let mut pc = (DAT_00b6a258 as *mut u8)
                        .wrapping_offset(iv4 as isize)
                        .wrapping_add(0x40) as *const i8;
                    loop {
                        if u5 == 0 {
                            break;
                        }
                        u5 = u5.wrapping_sub(1);
                        let c = pc.read();
                        pc = pc.wrapping_add(1);
                        if c == 0 {
                            break;
                        }
                    }
                    let mut iv6: i32 = (!u5).wrapping_sub(2) as i32;
                    while iv6 != 0 {
                        let q = (DAT_00b6a258 as *mut u8)
                            .wrapping_offset(iv6 as isize)
                            .wrapping_offset(iv4 as isize)
                            .wrapping_add(0x40) as *mut i8;
                        if q.read() == 0x28 {
                            q.write(0);
                            break;
                        }
                        iv6 = iv6.wrapping_sub(1);
                    }
                    iv4 = (iv4 as u32).wrapping_add(0x148) as i32;
                    u10 = u10.wrapping_add(1);
                    if !(u10 < DAT_00b6a25c) {
                        break;
                    }
                }
            }
            DAT_00664ed0 = 0;
            DAT_00664fbc = 0;
            if DAT_00b6a25c != 0 {
                let mut p7 = (DAT_00b6a258 as *mut u8).wrapping_add(0x140) as *mut i32;
                let mut local_24 = DAT_00b6a25c;
                let v = ver();
                let iv4: i32 = (v as u32).wrapping_mul(0x109e0) as i32;
                loop {
                    let iv6: i32 =
                        (iv4 as u32).wrapping_add((p7.read() as u32).wrapping_mul(0x24)) as i32;
                    let t0 = (&raw mut DAT_00b7c342)
                        .byte_offset(iv6 as isize)
                        .cast::<i32>()
                        .read_unaligned();
                    let t1 = (&raw mut DAT_00b7c6c6)
                        .byte_offset(iv6 as isize)
                        .cast::<i32>()
                        .read_unaligned();
                    if t0 != 0 || t1 != 0 {
                        let mut u10: u32 = 0;
                        let mut p11 = DAT_00b69cd8;
                        if DAT_00b69cc8 != 0 {
                            loop {
                                if p11.read() == p7.read() {
                                    break;
                                }
                                u10 = u10.wrapping_add(1);
                                p11 = p11.wrapping_add(0x109);
                                if !(u10 < DAT_00b69cc8) {
                                    break;
                                }
                            }
                        }
                        if u10 != DAT_00b69cc8 {
                            u10 = 0;
                            DAT_00664ec8 = 0;
                            if DAT_00b69d18 != 0 {
                                let mut p8 = (&raw mut DAT_00664ee8).cast::<u32>();
                                let mut p11 =
                                    (DAT_00b6a2d4 as *mut u8).wrapping_add(0x140) as *mut i32;
                                let v = ver();
                                let iv6: i32 = (v as u32).wrapping_mul(0x84f) as i32;
                                loop {
                                    let iv3: i32 = ((p11.read() as u32)
                                        .wrapping_add(iv6 as u32)
                                        .wrapping_mul(0x20))
                                        as i32;
                                    let t0 = (&raw mut DAT_00b7ca46)
                                        .byte_offset(iv3 as isize)
                                        .cast::<i32>()
                                        .read_unaligned();
                                    let t1 = (&raw mut DAT_00b7ca4a)
                                        .byte_offset(iv3 as isize)
                                        .cast::<i32>()
                                        .read_unaligned();
                                    if t0 == p7.read() && t1 != 0 {
                                        let t2 = (&raw mut DAT_00b7ca4e)
                                            .byte_offset(iv3 as isize)
                                            .cast::<i32>()
                                            .read_unaligned();
                                        if t2 != 0 {
                                            p8.write(1);
                                            DAT_00664ec8 = DAT_00664ec8.wrapping_add(1);
                                        }
                                        p8 = p8.wrapping_add(1);
                                        if (p8 as usize as u32) > 0x664f27 {
                                            break;
                                        }
                                    }
                                    p11 = p11.wrapping_add(0x52);
                                    u10 = u10.wrapping_add(1);
                                    if !(u10 < DAT_00b69d18) {
                                        break;
                                    }
                                }
                            }
                            u10 = 0;
                            DAT_00664f28 = 0;
                            if DAT_00bb2814 != 0 {
                                let mut p8 = (&raw mut DAT_00664f60).cast::<u32>();
                                let mut p11 =
                                    (DAT_00b6a2cc as *mut u8).wrapping_add(0x140) as *mut i32;
                                let v = ver();
                                let iv6: i32 = (v as u32).wrapping_mul(0x84f) as i32;
                                loop {
                                    let iv3: i32 = ((p11.read() as u32)
                                        .wrapping_add(iv6 as u32)
                                        .wrapping_mul(0x20))
                                        as i32;
                                    let t0 = (&raw mut DAT_00b7d6c6)
                                        .byte_offset(iv3 as isize)
                                        .cast::<i32>()
                                        .read_unaligned();
                                    let t1 = (&raw mut DAT_00b7d6ca)
                                        .byte_offset(iv3 as isize)
                                        .cast::<i32>()
                                        .read_unaligned();
                                    if t0 == p7.read() && t1 != 0 {
                                        let t2 = (&raw mut DAT_00b7d6ce)
                                            .byte_offset(iv3 as isize)
                                            .cast::<i32>()
                                            .read_unaligned();
                                        if t2 != 0 {
                                            p8.write(1);
                                            DAT_00664f28 = DAT_00664f28.wrapping_add(1);
                                        }
                                        p8 = p8.wrapping_add(1);
                                        if (p8 as usize as u32) > 0x664f9f {
                                            break;
                                        }
                                    }
                                    p11 = p11.wrapping_add(0x52);
                                    u10 = u10.wrapping_add(1);
                                    if !(u10 < DAT_00bb2814) {
                                        break;
                                    }
                                }
                            }
                            DAT_00664fbc = DAT_00664fbc.wrapping_add(1);
                        }
                    }
                    p7 = p7.wrapping_add(0x52);
                    local_24 = local_24.wrapping_sub(1);
                    if local_24 == 0 {
                        break;
                    }
                }
            }
            DAT_00664f2c = 0;
        }
        FUN_004d6460(l20, 0x1a9, 0x6b, 0x1b2, 0x1b1);
        if 1 < DAT_00664fbc {
            DAT_00664ed0 = FUN_004d9d70(l20, DAT_00664ed0, DAT_00664fbc, 0, 5, DAT_00bb2810, 0xc);
        }
        let mut i4: i32 = 0;
        let mut u10: u32 = 0;
        if DAT_00b6a25c != 0 {
            let mut p7 = (DAT_00b6a258 as *mut u8).wrapping_add(0x140) as *mut i32;
            loop {
                let v = ver();
                let iv6: i32 = (p7.read() as u32)
                    .wrapping_mul(0x24)
                    .wrapping_add((v as u32).wrapping_mul(0x109e0))
                    as i32;
                let t0 = (&raw mut DAT_00b7c342)
                    .byte_offset(iv6 as isize)
                    .cast::<i32>()
                    .read_unaligned();
                let t1 = (&raw mut DAT_00b7c6c6)
                    .byte_offset(iv6 as isize)
                    .cast::<i32>()
                    .read_unaligned();
                if t0 != 0 || t1 != 0 {
                    let mut u5: u32 = 0;
                    let mut b12 = DAT_00b69cc8 == 0;
                    let mut p11 = DAT_00b69cd8;
                    if DAT_00b69cc8 != 0 {
                        loop {
                            if p11.read() == p7.read() {
                                break;
                            }
                            u5 = u5.wrapping_add(1);
                            p11 = p11.wrapping_add(0x109);
                            if !(u5 < DAT_00b69cc8) {
                                break;
                            }
                        }
                        b12 = u5 == DAT_00b69cc8;
                    }
                    if !b12 {
                        if DAT_00664ed0 == i4 {
                            FUN_004d6460(l20, 0x58, 0x6f, 0x1a8, 0x7d);
                            FUN_004db640(
                                0xc,
                                (DAT_00b6a258 as *mut u8)
                                    .wrapping_add((u10 as usize).wrapping_mul(0x148))
                                    .wrapping_add(0x40) as *mut i8,
                                l20,
                                DAT_00b6a2c0,
                            );
                            i4 = u5.wrapping_mul(0x424) as i32;
                            let img = DAT_00b69cd8
                                .wrapping_add((u5 as usize).wrapping_mul(0x109))
                                .wrapping_add(1) as *mut i8;
                            FUN_004b4cc0(img, l10);
                            let w = (local_10[2].wrapping_sub(local_10[0]) as u32).wrapping_add(1)
                                as i32
                                >> 1;
                            let h = (local_10[3].wrapping_sub(local_10[1]) as u32).wrapping_add(1)
                                as i32
                                >> 1;
                            let iv3 = 0x100u32.wrapping_sub(w as u32) as i32;
                            let iv6 = 0x117u32.wrapping_sub(h as u32) as i32;
                            FUN_004b5570(img, iv3, iv6);
                            if DAT_00664ec8 != 0 {
                                let mut p7 = (&raw mut DAT_00664ee8).cast::<u32>();
                                let end = p7.wrapping_add(15);
                                let mut iv9 = i4;
                                loop {
                                    if p7.read() != 0 {
                                        FUN_004b5570(
                                            (DAT_00b69cd8 as *mut u8)
                                                .wrapping_offset(iv9 as isize)
                                                .wrapping_add(0x24)
                                                as *mut i8,
                                            iv3,
                                            iv6,
                                        );
                                    }
                                    iv9 = iv9.wrapping_add(0x20);
                                    p7 = p7.wrapping_add(1);
                                    if !(p7 < end) {
                                        break;
                                    }
                                }
                            }
                            if DAT_00664f28 == 0 {
                                return 1;
                            }
                            let mut p7 = (&raw mut DAT_00664f60).cast::<u32>();
                            let end = p7.wrapping_add(15);
                            loop {
                                if p7.read() != 0 {
                                    FUN_004b5570(
                                        (DAT_00b69cd8 as *mut u8)
                                            .wrapping_offset(i4 as isize)
                                            .wrapping_add(0x224)
                                            as *mut i8,
                                        iv3,
                                        iv6,
                                    );
                                }
                                i4 = i4.wrapping_add(0x20);
                                p7 = p7.wrapping_add(1);
                                if !(p7 < end) {
                                    break;
                                }
                            }
                            return 1;
                        }
                        i4 = i4.wrapping_add(1);
                    }
                }
                p7 = p7.wrapping_add(0x52);
                u10 = u10.wrapping_add(1);
                if !(u10 < DAT_00b6a25c) {
                    break;
                }
            }
        }
        1
    }
}
