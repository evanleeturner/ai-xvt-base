// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa62b0: [u8; 1];
    static mut DAT_00aa62e0: [u8; 1];
    static mut DAT_00aa62e4: [u8; 1];
    static mut DAT_00aa62e8: [u8; 1];
    static mut DAT_00aa62ec: [u8; 1];
    static mut DAT_00aa62ed: [u8; 1];
    static mut DAT_00665440: [u32; 8];
    static mut DAT_00bb72d0: i8;
    static mut DAT_00bb72d1: [i8; 1];
    static mut DAT_00bb7311: [i8; 1];
    static mut DAT_00b69d20: [i8; 1];
    static mut DAT_0052bf50: i32;
    static mut DAT_00aa6ae8: u32;
    static mut DAT_00aa6cf0: u32;
    static mut DAT_00aa6118: *mut u8;
    static mut DAT_00aa6cf8: i32;
    static mut DAT_00aa6af0: u32;
    static mut DAT_00aa6a70: u32;
    static mut DAT_00aa6a74: u8;
    static mut DAT_00aa62a4: u32;
    static mut DAT_00aa6114: i32;
    static mut DAT_00aa6134: i32;
    static mut DAT_00aa6138: i32;
    static mut DAT_0052c208: i32;
    static mut DAT_00b6a5aa: [u8; 1];
    static mut DAT_00b6a5ae: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cd9d0();
    fn FUN_004ccf90() -> i32;
    fn FUN_004d92b0();
    fn FUN_004d11d0() -> i32;
    fn FUN_004cef70(param_1: i32, param_2: *mut u32, param_3: i32);
    fn FUN_004e0490() -> i32;
    fn FUN_004e5000(param_1: i32);
    fn FUN_004e5590(param_1: *mut u8);
}

unsafe extern "stdcall" {
    fn GetTickCount() -> u32;
}

unsafe fn copy_str(mut src: *const u8, mut dst: *mut u8) {
    // SAFETY: caller passes valid NUL-terminated source and sufficient destination.
    unsafe {
        loop {
            let c = src.read();
            dst.write(c);
            src = src.add(1);
            dst = dst.add(1);
            if c == 0 {
                break;
            }
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d97f0(param_1: i32) -> u32 {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are used at the offsets the original uses.
    unsafe {
        FUN_004cd9d0();
        let iv2 = (param_1 as u32).wrapping_mul(0x3e) as usize;
        copy_str(
            (&raw mut DAT_00aa62b0).cast::<u8>().add(iv2),
            (&raw mut DAT_00665440).cast::<u8>(),
        );
        let mut skip = false;
        if DAT_00bb72d0 == 0 {
            (&raw mut DAT_00b69d20).cast::<i8>().write(0);
        } else {
            let src: *const u8;
            if DAT_00bb72d0 == 1 {
                src = (&raw mut DAT_00bb7311).cast::<u8>();
            } else if DAT_00bb72d0 == 2 {
                src = (&raw mut DAT_00bb72d1).cast::<u8>();
            } else {
                src = core::ptr::null();
                skip = true;
            }
            if !skip {
                copy_str(src, (&raw mut DAT_00b69d20).cast::<u8>());
            }
        }
        let p0 = (&raw mut DAT_00aa62e0).cast::<u8>().add(iv2);
        let p4 = (&raw mut DAT_00aa62e4).cast::<u8>().add(iv2);
        let p8 = (&raw mut DAT_00aa62e8).cast::<u8>().add(iv2);
        let pc = (&raw mut DAT_00aa62ec).cast::<u8>().add(iv2);
        let pd = (&raw mut DAT_00aa62ed).cast::<u8>().add(iv2);

        let mut iv5 = FUN_004ccf90();
        if iv5 == 0 {
            if DAT_0052bf50 != -1 {
                DAT_0052bf50 = -1;
                DAT_00aa6ae8 = 0xffff_ffff;
                DAT_00aa6cf0 = 0;
                let a = (&raw mut DAT_00665440).cast::<u32>();
                for i in 0..8 {
                    a.add(i).write(0);
                }
                let b = DAT_00aa6118;
                for i in 0..0x1000usize {
                    b.add(i).write(0);
                }
                FUN_004d92b0();
            }
            p0.cast::<u32>().write_unaligned(0);
            p4.cast::<u32>().write_unaligned(GetTickCount());
            p8.cast::<u32>().write_unaligned(0);
            pc.write(0);
            pd.write(0);
            return 0;
        }
        if DAT_0052bf50 != -1 && param_1 == DAT_0052bf50 {
            DAT_00aa6cf8 = 0;
            DAT_00aa6ae8 = 0xffff_ffff;
        }
        iv5 = FUN_004d11d0();
        DAT_00aa6af0 = 0x40;
        FUN_004cef70(iv5, &raw mut DAT_00aa6af0, 4);
        if DAT_0052bf50 != -1 && param_1 == DAT_0052bf50 {
            DAT_00aa6af0 = 0x6c;
            FUN_004cef70(iv5, &raw mut DAT_00aa6af0, 4);
        }
        DAT_00aa6a70 = 0;
        DAT_00aa6cf0 = 0;
        let d6 = GetTickCount();
        loop {
            iv5 = FUN_004e0490();
            let d7 = GetTickCount();
            if iv5 == 0x3a {
                break;
            }
            if iv5 == 0x72 {
                break;
            }
            if iv5 == 0x41 {
                if DAT_0052bf50 == -1 || param_1 != DAT_0052bf50 {
                    DAT_00aa6ae8 = 0xffff_ffff;
                } else {
                    let idxp = (&raw mut DAT_00b6a5aa).cast::<u8>();
                    let arr = (&raw mut DAT_00b6a5ae).cast::<u8>();
                    idxp.cast::<i32>().write_unaligned(DAT_00aa6cf8);
                    arr.byte_add((DAT_00aa6cf8 as u32).wrapping_mul(4) as usize)
                        .cast::<u32>()
                        .write_unaligned(DAT_00aa6ae8);
                    iv5 = idxp.cast::<i32>().read_unaligned();
                    FUN_004e5000(iv5);
                    FUN_004e5590(DAT_00aa6118);
                    if DAT_00aa6114 != 0 {
                        DAT_00aa6134 = 0;
                        if DAT_00aa6138 != 0 {
                            let mut pi = (DAT_00aa6114.wrapping_add(0x140)) as *const u8;
                            loop {
                                let t2 = idxp.cast::<u32>().read_unaligned();
                                let t = arr
                                    .byte_add(t2.wrapping_mul(4) as usize)
                                    .cast::<u32>()
                                    .read_unaligned();
                                if pi.cast::<i32>().read_unaligned() == t as i32 {
                                    break;
                                }
                                pi = pi.wrapping_add(0x148);
                                DAT_00aa6134 = DAT_00aa6134.wrapping_add(1);
                                if !(DAT_00aa6134 < DAT_00aa6138) {
                                    break;
                                }
                            }
                        }
                    }
                    DAT_0052c208 = 0;
                    iv5 = 0x41;
                }
            }
            if !(d7.wrapping_sub(d6) < 2000) {
                break;
            }
        }
        if iv5 == 0x3a {
            let d = GetTickCount();
            let u4 = DAT_00aa6a74;
            let u3 = DAT_00aa62a4;
            p4.cast::<u32>().write_unaligned(d);
            p0.cast::<u32>().write_unaligned(DAT_00aa6a70);
            p8.cast::<u32>().write_unaligned(u3);
            pc.write(u4);
            pd.write(1);
        } else if iv5 == 0x72 {
            let d = GetTickCount();
            let u4 = DAT_00aa6a74;
            let u3 = DAT_00aa62a4;
            p4.cast::<u32>().write_unaligned(d);
            p0.cast::<u32>().write_unaligned(DAT_00aa6a70);
            p8.cast::<u32>().write_unaligned(u3);
            pc.write(u4);
            pd.write(0);
        } else {
            let d = GetTickCount();
            p4.cast::<u32>().write_unaligned(d);
            p0.cast::<u32>().write_unaligned(0);
            p8.cast::<u32>().write_unaligned(0);
            pc.write(0);
            pd.write(0);
        }
        1
    }
}
