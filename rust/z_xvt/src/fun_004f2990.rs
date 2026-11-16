// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081bfba: [u8; 4];
    static mut DAT_0081bfbe: [u8; 0];
    static mut DAT_00757cc0: i32;
    static mut DAT_00757cc8: i32;
    static mut DAT_00757ccc: i32;
    static mut PTR_s_train_005330e0: [*mut i8; 0];
    static mut s__s__s_005331e4: [i8; 0];
    static mut DAT_00523570: [i8; 0];
    static mut DAT_00757672: i16;
    static mut DAT_0075766c: i16;
    static mut DAT_0075766e: i16;
    static mut DAT_007562dc: [u8; 0];
    static mut DAT_00743880: [u8; 0];
    static mut DAT_00753ae0: [u8; 0];
    static mut DAT_00755760: [u8; 0];
    static mut DAT_0075637a: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004e43d0(param_1: i32);
    #[link_name = "sprintf"]
    fn _sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004be160(param_1: *mut i8, param_2: *mut i8) -> *mut c_void;
    fn FUN_004be2c0(param_1: *mut c_void, param_2: *mut c_void);
    fn FUN_004be1f0(param_1: *mut c_void);
    fn FUN_004be320(param_1: *mut c_void, param_2: *mut c_void, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004f2990() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are used exactly as the original does.
    unsafe {
        let mut s_stack: i16 = 0;
        let mut buf = [0i8; 256];
        let idx_ptr = (&raw mut DAT_0081bfba).cast::<i32>();
        let tbl = (&raw mut DAT_0081bfbe).cast::<u8>();

        FUN_004e43d0(idx_ptr.read_unaligned());
        if DAT_00757cc0 != 0 {
            DAT_00757cc8 = 0;
            if DAT_00757ccc != 0 {
                let mut p3 = (DAT_00757cc0.wrapping_add(0x140)) as *const i32;
                loop {
                    let i_idx = idx_ptr.read_unaligned();
                    let i_tbl = tbl
                        .byte_add((i_idx as u32).wrapping_mul(4) as usize)
                        .cast::<i32>()
                        .read_unaligned();
                    if p3.read_unaligned() == i_tbl {
                        break;
                    }
                    p3 = p3.byte_add(0x144);
                    DAT_00757cc8 = DAT_00757cc8.wrapping_add(1);
                    if !((DAT_00757cc8 as u32) < (DAT_00757ccc as u32)) {
                        break;
                    }
                }
            }
        }
        let i_idx = idx_ptr.read_unaligned();
        let name = (&raw mut PTR_s_train_005330e0)
            .cast::<*mut i8>()
            .offset(i_idx as isize)
            .read();
        let arg = (DAT_00757cc8 as u32)
            .wrapping_mul(0x144)
            .wrapping_add(DAT_00757cc0 as u32) as *mut i8;
        _sprintf(
            buf.as_mut_ptr(),
            (&raw mut s__s__s_005331e4).cast::<i8>(),
            name,
            arg,
        );
        let f = FUN_004be160(buf.as_mut_ptr(), (&raw mut DAT_00523570).cast::<i8>());
        if f.is_null() {
            return;
        }
        let base = (&raw mut DAT_00743880).cast::<u8>();
        let mut p5 = base.cast::<u32>();
        let mut n = 0x4f7du32;
        while n != 0 {
            p5.write_unaligned(0);
            p5 = p5.add(1);
            n -= 1;
        }
        let ps = &raw mut s_stack;
        FUN_004be2c0(f, (&raw mut DAT_00757672).cast());
        if DAT_00757672 != 0xc {
            FUN_004be1f0(f);
            return;
        }
        FUN_004be2c0(f, (&raw mut DAT_0075766c).cast());
        FUN_004be2c0(f, (&raw mut DAT_0075766e).cast());
        let mut i = 0i32;
        FUN_004be320(f, (&raw mut DAT_007562dc).cast(), 0x9e);
        if 0 < DAT_0075766c as i32 {
            let mut p6 = base;
            loop {
                i = i.wrapping_add(1);
                FUN_004be320(f, p6.cast(), 0x562);
                p6 = p6.byte_add(0x562);
                if !(i < DAT_0075766c as i32) {
                    break;
                }
            }
        }
        i = 0;
        if 0 < DAT_0075766e as i32 {
            loop {
                i = i.wrapping_add(1);
                FUN_004be2c0(f, ps.cast());
                let off = (s_stack as i32).wrapping_mul(0x72);
                FUN_004be320(
                    f,
                    (&raw mut DAT_00753ae0)
                        .cast::<u8>()
                        .byte_offset(off as isize)
                        .cast(),
                    0x72,
                );
                if !(i < DAT_0075766e as i32) {
                    break;
                }
            }
        }
        let end1 = (&raw mut DAT_007562dc).cast::<u8>() as usize;
        let mut p4 = (&raw mut DAT_00755760).cast::<u8>();
        loop {
            i = 0;
            FUN_004be2c0(f, ps.cast());
            let mut p6 = p4;
            if 0 < s_stack as i32 {
                loop {
                    i = i.wrapping_add(1);
                    FUN_004be320(f, p6.cast(), 0x2a);
                    p6 = p6.byte_add(0x2a);
                    if !(i < s_stack as i32) {
                        break;
                    }
                }
            }
            p4 = p4.byte_add(0x126);
            if !((p4 as usize) < end1) {
                break;
            }
        }
        let end2 = (&raw mut DAT_0075766c) as usize;
        let mut p7 = (&raw mut DAT_0075637a).cast::<u8>();
        loop {
            FUN_004be2c0(f, ps.cast());
            if s_stack != 0 {
                FUN_004be320(f, p7.cast(), 0x1e5);
            }
            p7 = p7.byte_add(0x1e5);
            if !((p7 as usize) < end2) {
                break;
            }
        }
        FUN_004be1f0(f);
    }
}
