// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081bfba: [u8; 4];
    static mut DAT_0081bfb6: [u8; 4];
    static mut DAT_0081bfbe: [u8; 4];
    static mut DAT_00757cc0: i32;
    static mut DAT_00757cc8: i32;
    static mut DAT_00757ccc: i32;
    static mut DAT_005330e0: [*mut i8; 1];
    static mut DAT_005331e4: [i8; 1];
    static mut DAT_00523570: u8;
    static mut DAT_00743880: [u32; 1];
    static mut DAT_00757672: i16;
    static mut DAT_0075766c: i16;
    static mut DAT_0075766e: i16;
    static mut DAT_007562dc: u8;
    static mut DAT_00753ae0: [u8; 1];
    static mut DAT_00755760: [u8; 1];
    static mut DAT_0075637a: u16;
    static mut DAT_00650478: [u32; 1];
    static mut DAT_00650434: i32;
    static mut DAT_006507a2: [u8; 1];
    static mut DAT_00650822: [u8; 1];
    static mut DAT_006508a2: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004e43d0(param_1: i32);
    fn _sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004be160(param_1: *mut i8, param_2: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn FUN_004be2c0(param_1: *mut core::ffi::c_void, param_2: *mut core::ffi::c_void);
    fn FUN_004be1f0(param_1: *mut core::ffi::c_void);
    fn FUN_004be320(param_1: *mut core::ffi::c_void, param_2: *mut core::ffi::c_void, param_3: i32);
    fn FUN_004be290(param_1: *mut core::ffi::c_void, param_2: *mut i8);
    fn FUN_004be220(param_1: *mut core::ffi::c_void, param_2: i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004f25e0() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are used exactly where the original uses them.
    unsafe {
        let mut s_438: i16 = 0;
        let mut s_436: i16 = 0;
        let mut c_439: i8 = 0;
        let mut ac_42c = [0i8; 256];
        let mut au_32c = [0u32; 203];

        let mut i_train: i32 = (&raw mut DAT_0081bfba).cast::<i32>().read_unaligned();
        FUN_004e43d0(i_train);
        if DAT_00757cc0 != 0 {
            DAT_00757cc8 = 0;
            if DAT_00757ccc != 0 {
                let mut pi3 = (DAT_00757cc0.wrapping_add(0x140)) as usize as *const i32;
                loop {
                    i_train = (&raw mut DAT_0081bfba).cast::<i32>().read_unaligned();
                    let i_tmp: i32 = (&raw mut DAT_0081bfbe)
                        .cast::<u8>()
                        .byte_add((i_train as u32).wrapping_mul(4) as usize)
                        .cast::<i32>()
                        .read_unaligned();
                    if pi3.read_unaligned() == i_tmp {
                        break;
                    }
                    pi3 = pi3.wrapping_add(0x51);
                    DAT_00757cc8 = DAT_00757cc8.wrapping_add(1);
                    if !((DAT_00757cc8 as u32) < (DAT_00757ccc as u32)) {
                        break;
                    }
                }
            }
        }
        i_train = (&raw mut DAT_0081bfba).cast::<i32>().read_unaligned();
        let name = (&raw mut DAT_005330e0)
            .cast::<*mut i8>()
            .offset(i_train as isize)
            .read();
        _sprintf(
            ac_42c.as_mut_ptr(),
            (&raw const DAT_005331e4).cast::<i8>(),
            name,
            ((DAT_00757cc8 as u32)
                .wrapping_mul(0x144)
                .wrapping_add(DAT_00757cc0 as u32)) as i32,
        );
        let f = FUN_004be160(
            ac_42c.as_mut_ptr(),
            (&raw mut DAT_00523570).cast::<core::ffi::c_void>(),
        );
        if f.is_null() {
            return;
        }
        let mut p7 = (&raw mut DAT_00743880).cast::<u32>();
        let mut n: i32 = 0x4f7d;
        while n != 0 {
            p7.write(0);
            p7 = p7.add(1);
            n -= 1;
        }
        FUN_004be2c0(f, (&raw mut DAT_00757672).cast());
        if DAT_00757672 as i32 != 0xc {
            FUN_004be1f0(f);
            return;
        }
        FUN_004be2c0(f, (&raw mut DAT_0075766c).cast());
        FUN_004be2c0(f, (&raw mut DAT_0075766e).cast());
        let mut i2: i32 = 0;
        FUN_004be320(f, (&raw mut DAT_007562dc).cast(), 0x9e);
        if 0 < DAT_0075766c as i32 {
            let mut p = (&raw mut DAT_00743880).cast::<u8>();
            loop {
                i2 += 1;
                FUN_004be320(f, p.cast(), 0x562);
                p = p.wrapping_add(0x562);
                if !(i2 < DAT_0075766c as i32) {
                    break;
                }
            }
        }
        i2 = 0;
        if 0 < DAT_0075766e as i32 {
            loop {
                i2 += 1;
                FUN_004be2c0(f, (&raw mut s_438).cast());
                let off = (s_438 as i32).wrapping_mul(0x72) as isize;
                FUN_004be320(
                    f,
                    (&raw mut DAT_00753ae0)
                        .cast::<u8>()
                        .wrapping_offset(off)
                        .cast(),
                    0x72,
                );
                if !(i2 < DAT_0075766e as i32) {
                    break;
                }
            }
        }
        let mut p8 = (&raw mut DAT_00755760).cast::<u8>();
        loop {
            i2 = 0;
            FUN_004be2c0(f, (&raw mut s_438).cast());
            let mut p5 = p8;
            if 0 < s_438 as i32 {
                loop {
                    i2 += 1;
                    FUN_004be320(f, p5.cast(), 0x2a);
                    p5 = p5.wrapping_add(0x2a);
                    if !(i2 < s_438 as i32) {
                        break;
                    }
                }
            }
            p8 = p8.wrapping_add(0x126);
            if !((p8 as usize) < (&raw mut DAT_007562dc) as usize) {
                break;
            }
        }
        let mut p6 = (&raw mut DAT_0075637a).cast::<u8>();
        loop {
            FUN_004be2c0(f, (&raw mut s_438).cast());
            if s_438 != 0 {
                FUN_004be320(f, p6.cast(), 0x1e5);
            }
            p6 = p6.wrapping_add(0x1e5);
            if !((p6 as usize) < (&raw mut DAT_0075766c) as usize) {
                break;
            }
        }
        let mut i_430: i32 = 0;
        loop {
            let mut u4: u32 = 0;
            let mut i_434: i32 = 0;
            FUN_004be320(f, au_32c.as_mut_ptr().cast(), 0x32a);
            loop {
                FUN_004be290(f, &raw mut c_439);
                let u_tmp: u32 = (&raw mut DAT_0081bfb6).cast::<u32>().read_unaligned();
                if u4 == u_tmp && c_439 != 0 {
                    let src = au_32c.as_ptr();
                    let dst = (&raw mut DAT_00650478).cast::<u32>();
                    let mut k = 0usize;
                    while k < 0xca {
                        dst.add(k).write(src.add(k).read());
                        k += 1;
                    }
                    dst.add(0xca)
                        .cast::<u16>()
                        .write_unaligned(src.add(0xca).cast::<u16>().read_unaligned());
                    i_434 = 1;
                    DAT_00650434 = i_430;
                }
                u4 += 1;
                if !(u4 < 10) {
                    break;
                }
            }
            let mut pb = (&raw mut DAT_006507a2).cast::<u8>();
            loop {
                let p9 = (pb.cast::<u32>().read_unaligned() as usize) as *mut u32;
                if i_434 != 0 {
                    let mut p10 = p9;
                    let mut m = 10;
                    while m != 0 {
                        p10.write_unaligned(0);
                        p10 = p10.wrapping_add(1);
                        m -= 1;
                    }
                }
                FUN_004be2c0(f, (&raw mut s_436).cast());
                if s_436 != 0 {
                    if i_434 == 0 {
                        FUN_004be220(f, s_436 as i32, 1);
                    } else {
                        FUN_004be320(f, p9.cast(), s_436 as i32);
                    }
                }
                if i_434 != 0 {
                    p9.cast::<u8>().wrapping_offset(s_436 as isize).write(0);
                }
                pb = pb.wrapping_add(4);
                if !((pb as usize) < (&raw mut DAT_00650822) as usize) {
                    break;
                }
            }
            pb = (&raw mut DAT_00650822).cast::<u8>();
            loop {
                let p9 = (pb.cast::<u32>().read_unaligned() as usize) as *mut u32;
                if i_434 != 0 {
                    let mut p10 = p9;
                    let mut m = 0x50;
                    while m != 0 {
                        p10.write_unaligned(0);
                        p10 = p10.wrapping_add(1);
                        m -= 1;
                    }
                }
                FUN_004be2c0(f, (&raw mut s_436).cast());
                if s_436 != 0 {
                    if i_434 == 0 {
                        FUN_004be220(f, s_436 as i32, 1);
                    } else {
                        FUN_004be320(f, p9.cast(), s_436 as i32);
                    }
                }
                if i_434 != 0 {
                    p9.cast::<u8>().wrapping_offset(s_436 as isize).write(0);
                }
                pb = pb.wrapping_add(4);
                if !((pb as usize) < (&raw mut DAT_006508a2) as usize) {
                    break;
                }
            }
            i_430 += 1;
            if !(i_430 < 8) {
                break;
            }
        }
        FUN_004be1f0(f);
    }
}
