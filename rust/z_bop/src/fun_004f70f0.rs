// SPDX-License-Identifier: MIT
use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b6a5aa: i32;
    static mut DAT_00b6a5ae: i32;
    static mut DAT_00aa6114: u32;
    static mut DAT_00aa6134: u32;
    static mut DAT_00aa6138: u32;
    static mut PTR_s_train_0052c1e8: [*const i8; 1];
    static mut s__s__s_0052c300: [i8; 1];
    static mut DAT_0051a854: [i8; 1];
    static mut DAT_00a91cc0: [u32; 1];
    static mut DAT_00aa5ab2: i16;
    static mut DAT_00aa5aac: i16;
    static mut DAT_00aa5aae: i16;
    static mut DAT_00aa471c: [u8; 1];
    static mut DAT_00aa1f20: [u8; 1];
    static mut DAT_00aa3ba0: [u8; 1];
    static mut DAT_00aa47ba: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004e5000(param_1: i32);
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004cc4a0(param_1: *mut i8, param_2: *mut i8) -> *mut c_void;
    fn FUN_004cc660(param_1: *mut c_void, param_2: *mut c_void);
    fn FUN_004cc590(param_1: *mut c_void);
    fn FUN_004cc6c0(param_1: *mut c_void, param_2: *mut c_void, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004f70f0() {
    let mut local_102: i16 = 0;
    let mut local_100: [i8; 256] = [0; 256];
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are used exactly as the original does.
    unsafe {
        let base = (&raw mut DAT_00b6a5aa).cast::<u8>();
        let l102 = (&raw mut local_102).cast::<c_void>();
        let tmp_a = base.cast::<i32>().read_unaligned();
        FUN_004e5000(tmp_a);
        if DAT_00aa6114 != 0 {
            DAT_00aa6134 = 0;
            if DAT_00aa6138 != 0 {
                let mut p = DAT_00aa6114.wrapping_add(0x140) as *const i32;
                loop {
                    let a = base.cast::<i32>().read_unaligned();
                    let b = base
                        .byte_offset(4isize.wrapping_add((a as isize).wrapping_mul(4)))
                        .cast::<i32>()
                        .read_unaligned();
                    if p.read_unaligned() == b {
                        break;
                    }
                    p = p.byte_add(0x148);
                    DAT_00aa6134 = DAT_00aa6134.wrapping_add(1);
                    if !(DAT_00aa6134 < DAT_00aa6138) {
                        break;
                    }
                }
            }
        }
        let a = base.cast::<i32>().read_unaligned();
        let name = (&raw const PTR_s_train_0052c1e8)
            .cast::<*const i8>()
            .offset(a as isize)
            .read_unaligned();
        sprintf(
            local_100.as_mut_ptr(),
            (&raw const s__s__s_0052c300).cast::<i8>(),
            name,
            DAT_00aa6134.wrapping_mul(0x148).wrapping_add(DAT_00aa6114),
        );
        let f = FUN_004cc4a0(local_100.as_mut_ptr(), (&raw mut DAT_0051a854).cast::<i8>());
        if f.is_null() {
            return;
        }
        let a91 = (&raw mut DAT_00a91cc0).cast::<u32>();
        let mut p5 = a91;
        let mut n: i32 = 0x4f7d;
        while n != 0 {
            p5.write_unaligned(0);
            p5 = p5.add(1);
            n -= 1;
        }
        FUN_004cc660(f, (&raw mut DAT_00aa5ab2).cast());
        if DAT_00aa5ab2 != 0xe && DAT_00aa5ab2 != 0xd && DAT_00aa5ab2 != 0xc {
            FUN_004cc590(f);
            return;
        }
        FUN_004cc660(f, (&raw mut DAT_00aa5aac).cast());
        FUN_004cc660(f, (&raw mut DAT_00aa5aae).cast());
        let mut i: i32 = 0;
        FUN_004cc6c0(f, (&raw mut DAT_00aa471c).cast(), 0x9e);
        if 0 < DAT_00aa5aac {
            let mut p = a91.cast::<u8>();
            loop {
                i = i.wrapping_add(1);
                FUN_004cc6c0(f, p.cast(), 0x562);
                p = p.byte_add(0x562);
                if !(i < DAT_00aa5aac as i32) {
                    break;
                }
            }
        }
        i = 0;
        if 0 < DAT_00aa5aae {
            loop {
                i = i.wrapping_add(1);
                FUN_004cc660(f, l102);
                let l = core::ptr::read_volatile(&raw const local_102) as i32;
                let q = (&raw mut DAT_00aa1f20)
                    .cast::<u8>()
                    .offset(l.wrapping_mul(0x72) as isize);
                FUN_004cc6c0(f, q.cast(), 0x72);
                if !(i < DAT_00aa5aae as i32) {
                    break;
                }
            }
        }
        let end1 = (&raw mut DAT_00aa471c).cast::<u8>();
        let mut p4 = (&raw mut DAT_00aa3ba0).cast::<u8>();
        loop {
            i = 0;
            FUN_004cc660(f, l102);
            let mut p6 = p4;
            if 0 < core::ptr::read_volatile(&raw const local_102) {
                loop {
                    i = i.wrapping_add(1);
                    FUN_004cc6c0(f, p6.cast(), 0x2a);
                    p6 = p6.byte_add(0x2a);
                    if !(i < core::ptr::read_volatile(&raw const local_102) as i32) {
                        break;
                    }
                }
            }
            p4 = p4.byte_add(0x126);
            if !((p4 as usize) < (end1 as usize)) {
                break;
            }
        }
        let end2 = (&raw mut DAT_00aa5aac).cast::<u8>();
        let mut p7 = (&raw mut DAT_00aa47ba).cast::<u8>();
        loop {
            FUN_004cc660(f, l102);
            if core::ptr::read_volatile(&raw const local_102) != 0 {
                FUN_004cc6c0(f, p7.cast(), 0x1e5);
            }
            p7 = p7.byte_add(0x1e5);
            if !((p7 as usize) < (end2 as usize)) {
                break;
            }
        }
        FUN_004cc590(f);
    }
}
