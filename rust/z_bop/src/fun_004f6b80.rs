// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa6114: i32;
    static mut DAT_00aa6134: i32;
    static mut DAT_00aa6138: i32;
    static mut PTR_s_train_0052c1e8: [*mut i8; 0];
    static mut s__s__s_0052c300: [i8; 0];
    static mut DAT_0051a854: [i8; 0];
    static mut DAT_00b6a5aa: [u8; 0];
    static mut DAT_00b6a5ae: [u8; 0];
    static mut DAT_00b6a5a6: [u8; 0];
    static mut DAT_00a91cc0: [u32; 0];
    static mut DAT_00aa5ab2: i16;
    static mut DAT_00aa5aac: i16;
    static mut DAT_00aa5aae: i16;
    static mut DAT_00aa471c: [u8; 0];
    static mut DAT_00aa1f20: [u8; 0];
    static mut DAT_00aa3ba0: [u8; 0];
    static mut DAT_00aa47ba: [u8; 0];
    static mut DAT_00669258: [u32; 0];
    static mut DAT_00669214: i32;
    static mut DAT_00669582: [u8; 0];
    static mut DAT_00669602: [u8; 0];
    static mut DAT_00669682: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004e5000(param_1: i32);
    fn _sprintf(param_1: *mut i8, param_2: *const i8, ...) -> i32;
    fn FUN_004cc4a0(param_1: *mut i8, param_2: *const i8) -> *mut c_void;
    fn FUN_004cc660(param_1: *mut c_void, param_2: *mut c_void);
    fn FUN_004cc590(param_1: *mut c_void);
    fn FUN_004cc6c0(param_1: *mut c_void, param_2: *mut c_void, param_3: i32);
    fn FUN_004cc630(param_1: *mut c_void, param_2: *mut c_void);
    fn FUN_004cc5c0(param_1: *mut c_void, param_2: i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004f6b80() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are read and written exactly where the original does.
    unsafe {
        let mut local_439: i8 = 0;
        let mut local_438: i16 = 0;
        let mut local_436: i16 = 0;
        let mut local_434: i32;
        let mut local_430: i32;
        let mut local_42c: [i8; 256] = [0; 256];
        let mut local_32c: [u32; 203] = [0; 203];

        let mut i_train: i32 = (&raw mut DAT_00b6a5aa).cast::<i32>().read_unaligned();
        FUN_004e5000(i_train);
        if DAT_00aa6114 != 0 {
            DAT_00aa6134 = 0;
            if DAT_00aa6138 != 0 {
                let mut piVar3 = DAT_00aa6114.wrapping_add(0x140) as *const i32;
                loop {
                    i_train = (&raw mut DAT_00b6a5aa).cast::<i32>().read_unaligned();
                    let i_tmp: i32 = (&raw mut DAT_00b6a5ae)
                        .cast::<u8>()
                        .byte_add((i_train as u32).wrapping_mul(4) as usize)
                        .cast::<i32>()
                        .read_unaligned();
                    if piVar3.read_unaligned() == i_tmp {
                        break;
                    }
                    piVar3 = piVar3.byte_add(0x148);
                    DAT_00aa6134 = DAT_00aa6134.wrapping_add(1);
                    if !((DAT_00aa6134 as u32) < (DAT_00aa6138 as u32)) {
                        break;
                    }
                }
            }
        }
        i_train = (&raw mut DAT_00b6a5aa).cast::<i32>().read_unaligned();
        let name = (&raw mut PTR_s_train_0052c1e8)
            .cast::<*mut i8>()
            .offset(i_train as isize)
            .read();
        let rec = (DAT_00aa6134 as u32)
            .wrapping_mul(0x148)
            .wrapping_add(DAT_00aa6114 as u32) as *const i8;
        _sprintf(
            local_42c.as_mut_ptr(),
            (&raw mut s__s__s_0052c300).cast::<i8>(),
            name,
            rec,
        );
        let f = FUN_004cc4a0(local_42c.as_mut_ptr(), (&raw mut DAT_0051a854).cast::<i8>());
        if f.is_null() {
            return;
        }
        let base = (&raw mut DAT_00a91cc0).cast::<u32>();
        let mut p = base;
        let mut n = 0x4f7du32;
        while n != 0 {
            p.write(0);
            p = p.add(1);
            n -= 1;
        }
        FUN_004cc660(f, (&raw mut DAT_00aa5ab2).cast());
        if DAT_00aa5ab2 != 0xe && DAT_00aa5ab2 != 0xd && DAT_00aa5ab2 != 0xc {
            FUN_004cc590(f);
            return;
        }
        FUN_004cc660(f, (&raw mut DAT_00aa5aac).cast());
        FUN_004cc660(f, (&raw mut DAT_00aa5aae).cast());
        let mut i_var2: i32 = 0;
        FUN_004cc6c0(f, (&raw mut DAT_00aa471c).cast(), 0x9e);
        if 0 < DAT_00aa5aac {
            let mut p7 = base.cast::<u8>();
            loop {
                i_var2 = i_var2.wrapping_add(1);
                FUN_004cc6c0(f, p7.cast(), 0x562);
                p7 = p7.byte_add(0x562);
                if !(i_var2 < DAT_00aa5aac as i32) {
                    break;
                }
            }
        }
        i_var2 = 0;
        if 0 < DAT_00aa5aae {
            loop {
                i_var2 = i_var2.wrapping_add(1);
                FUN_004cc660(f, (&raw mut local_438).cast());
                FUN_004cc6c0(
                    f,
                    (&raw mut DAT_00aa1f20)
                        .cast::<u8>()
                        .offset((local_438 as i32).wrapping_mul(0x72) as isize)
                        .cast(),
                    0x72,
                );
                if !(i_var2 < DAT_00aa5aae as i32) {
                    break;
                }
            }
        }
        let mut p8 = (&raw mut DAT_00aa3ba0).cast::<u8>();
        loop {
            i_var2 = 0;
            FUN_004cc660(f, (&raw mut local_438).cast());
            let mut p5 = p8;
            if 0 < local_438 {
                loop {
                    i_var2 = i_var2.wrapping_add(1);
                    FUN_004cc6c0(f, p5.cast(), 0x2a);
                    p5 = p5.byte_add(0x2a);
                    if !(i_var2 < local_438 as i32) {
                        break;
                    }
                }
            }
            p8 = p8.byte_add(0x126);
            if !((p8 as usize) < ((&raw mut DAT_00aa471c) as usize)) {
                break;
            }
        }
        let mut p6 = (&raw mut DAT_00aa47ba).cast::<u8>();
        loop {
            FUN_004cc660(f, (&raw mut local_438).cast());
            if local_438 != 0 {
                FUN_004cc6c0(f, p6.cast(), 0x1e5);
            }
            p6 = p6.byte_add(0x1e5);
            if !((p6 as usize) < ((&raw mut DAT_00aa5aac) as usize)) {
                break;
            }
        }
        local_430 = 0;
        loop {
            let mut u_var4: u32 = 0;
            local_434 = 0;
            FUN_004cc6c0(f, local_32c.as_mut_ptr().cast(), 0x32a);
            loop {
                FUN_004cc630(f, (&raw mut local_439).cast());
                let u_tmp: u32 = (&raw mut DAT_00b6a5a6).cast::<u32>().read_unaligned();
                if u_tmp == u_var4 && local_439 != 0 {
                    let mut src = local_32c.as_ptr();
                    let mut dst = (&raw mut DAT_00669258).cast::<u32>();
                    let mut k = 0xcau32;
                    while k != 0 {
                        dst.write_unaligned(src.read());
                        src = src.add(1);
                        dst = dst.add(1);
                        k -= 1;
                    }
                    dst.cast::<u16>()
                        .write_unaligned(src.cast::<u16>().read_unaligned());
                    local_434 = 1;
                    DAT_00669214 = local_430;
                }
                u_var4 = u_var4.wrapping_add(1);
                if !(u_var4 < 10) {
                    break;
                }
            }
            let mut pb = (&raw mut DAT_00669582).cast::<u8>();
            loop {
                let p9 = pb.cast::<*mut u32>().read_unaligned();
                if local_434 != 0 {
                    let mut q = p9;
                    let mut k = 10;
                    while k != 0 {
                        q.write_unaligned(0);
                        q = q.add(1);
                        k -= 1;
                    }
                }
                FUN_004cc660(f, (&raw mut local_436).cast());
                if local_436 != 0 {
                    if local_434 == 0 {
                        FUN_004cc5c0(f, local_436 as i32, 1);
                    } else {
                        FUN_004cc6c0(f, p9.cast(), local_436 as i32);
                    }
                }
                if local_434 != 0 {
                    p9.cast::<u8>().offset(local_436 as isize).write(0);
                }
                pb = pb.byte_add(4);
                if !((pb as usize) < ((&raw mut DAT_00669602) as usize)) {
                    break;
                }
            }
            pb = (&raw mut DAT_00669602).cast::<u8>();
            loop {
                let p9 = pb.cast::<*mut u32>().read_unaligned();
                if local_434 != 0 {
                    let mut q = p9;
                    let mut k = 0x50;
                    while k != 0 {
                        q.write_unaligned(0);
                        q = q.add(1);
                        k -= 1;
                    }
                }
                FUN_004cc660(f, (&raw mut local_436).cast());
                if local_436 != 0 {
                    if local_434 == 0 {
                        FUN_004cc5c0(f, local_436 as i32, 1);
                    } else {
                        FUN_004cc6c0(f, p9.cast(), local_436 as i32);
                    }
                }
                if local_434 != 0 {
                    p9.cast::<u8>().offset(local_436 as isize).write(0);
                }
                pb = pb.byte_add(4);
                if !((pb as usize) < ((&raw mut DAT_00669682) as usize)) {
                    break;
                }
            }
            local_430 = local_430.wrapping_add(1);
            if !(local_430 < 8) {
                break;
            }
        }
        FUN_004cc590(f);
    }
}
