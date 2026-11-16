// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b6a2e0: u8;
    static mut DAT_0051a854: i8;
    static mut DAT_0052b364: i8;
    static mut DAT_00aa6114: *mut c_void;
    static mut DAT_00b69e30: i32;
    static mut DAT_00b6a5ae: u8;
    static mut DAT_00b6bc32: u8;
    static mut DAT_00b6d962: u8;
    static mut DAT_00b6d972: u8;
    static mut DAT_00b7e352: u8;
    static mut DAT_00b8ed32: u8;
    static mut DAT_00b6bc9e: i8;
    static mut DAT_00b6a5c6: i8;
    static mut DAT_00b6a5e6: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cc4a0(a: *mut i8, b: *mut i8) -> *mut c_void;
    fn FUN_004cc6c0(a: *mut c_void, b: *mut c_void, c: u32);
    fn FUN_004cc590(a: *mut c_void);
    fn FUN_004ccf50();
    fn FUN_004ccf70();
    fn FUN_004ddc10(a: i32) -> *mut i8;
    fn FUN_004e5000(a: i32);
    fn FUN_004c9f80(a: *mut c_void);
    fn _free(a: *mut c_void);
    fn _sprintf(a: *mut i8, b: *mut i8, ...) -> i32;
}

/// Copies a NUL-terminated string including the terminator.
unsafe fn copy_str(src: *const i8, dst: *mut i8) {
    let mut i: usize = 0;
    loop {
        // SAFETY: caller guarantees src is NUL-terminated and dst is large enough.
        let c = unsafe { src.add(i).read() };
        unsafe { dst.add(i).write(c) };
        if c == 0 {
            break;
        }
        i += 1;
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cb0c0(param_1: *mut i8) -> u32 {
    let mut local_80: [i8; 128] = [0; 128];
    // SAFETY: all globals and callees are the game's own memory and code;
    // param_1 is a NUL-terminated string from the caller.
    unsafe {
        let buf = (&raw mut DAT_00b6a2e0) as *mut u8;
        let mut i: usize = 0;
        while i < 0x4852c {
            buf.add(i).cast::<u32>().write_unaligned(0);
            i += 4;
        }
        buf.add(0x4852c).cast::<u16>().write_unaligned(0);

        copy_str(param_1, local_80.as_mut_ptr());

        let mut len: u32 = 0;
        while local_80[len as usize] != 0 {
            len += 1;
        }
        let idx = len.wrapping_add(1).wrapping_sub(2) as i32 as isize;
        local_80.as_mut_ptr().offset(idx).write(b'2' as i8);

        let mode = &raw mut DAT_0051a854;
        let mut pf = FUN_004cc4a0(local_80.as_mut_ptr(), mode);
        let b9 = !pf.is_null();
        if b9 {
            FUN_004cc6c0(pf, buf as *mut c_void, 0x4852e);
            FUN_004cc590(pf);
        }
        FUN_004ccf50();
        pf = FUN_004cc4a0(param_1, mode);
        FUN_004ccf70();

        if pf.is_null() {
            if !b9 {
                return 0;
            }
        } else {
            if !b9 {
                (&raw mut DAT_00b6bc32).cast::<u32>().write_unaligned(2);
                let s = FUN_004ddc10(0x7c);
                copy_str(s, &raw mut DAT_00b6bc9e);

                FUN_004e5000(0);
                if !DAT_00aa6114.is_null() {
                    let v = DAT_00aa6114
                        .cast::<u8>()
                        .add(0x140)
                        .cast::<u32>()
                        .read_unaligned();
                    (&raw mut DAT_00b6a5ae).cast::<u32>().write_unaligned(v);
                    let v = DAT_00aa6114
                        .cast::<u8>()
                        .add(0x140)
                        .cast::<u32>()
                        .read_unaligned();
                    (&raw mut DAT_00b6d972).cast::<u32>().write_unaligned(v);
                    _free(DAT_00aa6114);
                    DAT_00aa6114 = core::ptr::null_mut();
                }
                (&raw mut DAT_00b6d962).cast::<u32>().write_unaligned(1);
                FUN_004e5000(0);
                (&raw mut DAT_00b6d962).cast::<u32>().write_unaligned(0);
                if !DAT_00aa6114.is_null() {
                    let v = DAT_00aa6114
                        .cast::<u8>()
                        .add(0x140)
                        .cast::<u32>()
                        .read_unaligned();
                    (&raw mut DAT_00b7e352).cast::<u32>().write_unaligned(v);
                    _free(DAT_00aa6114);
                    DAT_00aa6114 = core::ptr::null_mut();
                }
                DAT_00b69e30 = 4;
                FUN_004e5000(0);
                if !DAT_00aa6114.is_null() {
                    let v = DAT_00aa6114
                        .cast::<u8>()
                        .add(0x140)
                        .cast::<u32>()
                        .read_unaligned();
                    (&raw mut DAT_00b8ed32).cast::<u32>().write_unaligned(v);
                    _free(DAT_00aa6114);
                    DAT_00aa6114 = core::ptr::null_mut();
                }
                DAT_00b69e30 = 0;
            }
            FUN_004c9f80(pf);
            FUN_004cc590(pf);
            if !b9 {
                let s = FUN_004ddc10(0x1d6);
                _sprintf(&raw mut DAT_00b6a5c6, &raw mut DAT_0052b364, buf, s);
                copy_str(&raw mut DAT_00b6a5c6, &raw mut DAT_00b6a5e6);
            }
        }
    }
    1
}
