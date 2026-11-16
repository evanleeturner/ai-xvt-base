// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0051a854: [i8; 0];
    static mut DAT_00527004: [i8; 0];
    static mut DAT_0052b364: [i8; 0];
    static mut DAT_00b6a2e0: [u32; 0];
    static mut DAT_00b6bc9e: [i8; 0];
    static mut DAT_00b6a5c6: [i8; 0];
    static mut DAT_00b6a5e6: [i8; 0];
    static mut DAT_00aa6114: *mut c_void;
    static mut DAT_00b69e30: u32;
    static mut DAT_00b6bc32: [u8; 4];
    static mut DAT_00b6a5ae: [u8; 4];
    static mut DAT_00b6d972: [u8; 4];
    static mut DAT_00b6d962: [u8; 4];
    static mut DAT_00b7e352: [u8; 4];
    static mut DAT_00b8ed32: [u8; 4];
    static mut DAT_00b6a5ba: [u8; 4];
    static mut DAT_00b6d97e: [u8; 4];
    static mut DAT_00b7e35e: [u8; 4];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FID_conflict___wfopen(a: *mut i8, b: *mut i8) -> *mut c_void;
    fn _fclose(f: *mut c_void) -> i32;
    fn FUN_004cc4a0(a: *mut i8, b: *mut i8) -> *mut c_void;
    fn FUN_004ddc10(a: i32) -> *mut i8;
    fn FUN_004e5000(a: i32);
    fn _free(p: *mut c_void);
    fn FUN_004cc790(a: *mut c_void, b: *mut c_void, c: i32);
    fn FUN_004cc590(a: *mut c_void);
    fn FUN_004bf650(a: i32) -> u32;
}

unsafe fn copy_str(dst: *mut i8, src: *const i8) {
    let mut i = 0usize;
    // SAFETY: caller guarantees src is NUL-terminated and dst is large enough.
    unsafe {
        loop {
            let c = *src.add(i);
            *dst.add(i) = c;
            i += 1;
            if c == 0 {
                break;
            }
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004bf3a0(param_1: *mut i8) -> u32 {
    let mut local_20: [i8; 32] = [0; 32];
    // SAFETY: all globals and callees are the game's own memory and code;
    // param_1 is a NUL-terminated string supplied by the caller.
    unsafe {
        let mut i_var6: i32 = 0;
        let mut p_fvar2: *mut c_void;
        loop {
            _sprintf(local_20.as_mut_ptr(), c"%s%d.pl2".as_ptr(), param_1, i_var6);
            p_fvar2 =
                FID_conflict___wfopen(local_20.as_mut_ptr(), (&raw mut DAT_0051a854).cast::<i8>());
            if p_fvar2.is_null() {
                break;
            }
            i_var6 = i_var6.wrapping_add(1);
            _fclose(p_fvar2);
        }
        p_fvar2 = FUN_004cc4a0(local_20.as_mut_ptr(), (&raw mut DAT_00527004).cast::<i8>());
        if p_fvar2.is_null() {
            return 0;
        }
        let base = (&raw mut DAT_00b6a2e0).cast::<u32>();
        let mut p = base;
        let mut n = 0x1214bu32;
        while n != 0 {
            p.write_unaligned(0);
            p = p.add(1);
            n -= 1;
        }
        p.cast::<u16>().write_unaligned(0);

        copy_str(base.cast::<i8>(), param_1);

        (&raw mut DAT_00b6bc32).cast::<u32>().write_unaligned(2);
        let s = FUN_004ddc10(0x7c);
        copy_str((&raw mut DAT_00b6bc9e).cast::<i8>(), s);

        let s = FUN_004ddc10(0x1d6);
        _sprintf(
            (&raw mut DAT_00b6a5c6).cast::<i8>(),
            (&raw mut DAT_0052b364).cast::<i8>(),
            param_1,
            s,
        );
        copy_str(
            (&raw mut DAT_00b6a5e6).cast::<i8>(),
            (&raw mut DAT_00b6a5c6).cast::<i8>(),
        );

        FUN_004e5000(0);
        if !DAT_00aa6114.is_null() {
            let v = DAT_00aa6114.byte_add(0x140).cast::<u32>().read_unaligned();
            (&raw mut DAT_00b6a5ae).cast::<u32>().write_unaligned(v);
            let v = DAT_00aa6114.byte_add(0x140).cast::<u32>().read_unaligned();
            (&raw mut DAT_00b6d972).cast::<u32>().write_unaligned(v);
            _free(DAT_00aa6114);
            DAT_00aa6114 = core::ptr::null_mut();
        }
        (&raw mut DAT_00b6d962).cast::<u32>().write_unaligned(1);
        FUN_004e5000(0);
        (&raw mut DAT_00b6d962).cast::<u32>().write_unaligned(0);
        if !DAT_00aa6114.is_null() {
            let v = DAT_00aa6114.byte_add(0x140).cast::<u32>().read_unaligned();
            (&raw mut DAT_00b7e352).cast::<u32>().write_unaligned(v);
            _free(DAT_00aa6114);
            DAT_00aa6114 = core::ptr::null_mut();
        }
        DAT_00b69e30 = 4;
        FUN_004e5000(0);
        if !DAT_00aa6114.is_null() {
            let v = DAT_00aa6114.byte_add(0x140).cast::<u32>().read_unaligned();
            (&raw mut DAT_00b8ed32).cast::<u32>().write_unaligned(v);
            _free(DAT_00aa6114);
            DAT_00aa6114 = core::ptr::null_mut();
        }
        DAT_00b69e30 = 0;
        FUN_004cc790(p_fvar2, base.cast::<c_void>(), 0x4852e);
        FUN_004e5000(3);
        if !DAT_00aa6114.is_null() {
            let v = DAT_00aa6114.byte_add(0x3d0).cast::<u32>().read_unaligned();
            (&raw mut DAT_00b6a5ba).cast::<u32>().write_unaligned(v);
            let v = DAT_00aa6114.byte_add(0x140).cast::<u32>().read_unaligned();
            (&raw mut DAT_00b6d97e).cast::<u32>().write_unaligned(v);
            _free(DAT_00aa6114);
            DAT_00aa6114 = core::ptr::null_mut();
        }
        (&raw mut DAT_00b6d962).cast::<u32>().write_unaligned(1);
        FUN_004e5000(3);
        (&raw mut DAT_00b6d962).cast::<u32>().write_unaligned(0);
        if !DAT_00aa6114.is_null() {
            let v = DAT_00aa6114.byte_add(0x140).cast::<u32>().read_unaligned();
            (&raw mut DAT_00b7e35e).cast::<u32>().write_unaligned(v);
            _free(DAT_00aa6114);
            DAT_00aa6114 = core::ptr::null_mut();
        }
        FUN_004cc590(p_fvar2);
        FUN_004bf650(0);
    }
    1
}
