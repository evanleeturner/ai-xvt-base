// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00557560: [u32; 8];
    static mut DAT_00556ed8: [u32; 8];
    static mut DAT_00559788: i32;
    static mut DAT_00557348: i32;
    static mut DAT_005242dc: i32;
    static mut DAT_00556ef8: i32;
    static mut _DAT_0055714c: i32;
    static mut DAT_00557358: i32;
    static mut DAT_005242e0: u32;
    static mut DAT_0052340c: u32;
    static mut DAT_009a8c24: i32;
    static mut DAT_00557354: i32;
    static mut _DAT_00557148: i32;
    static mut DAT_00557580: i32;
    static mut DAT_009fd394: i32;
    static mut DAT_00a07cc8: i32;
    static mut _DAT_009ed23c: u32;
    static mut DAT_009e9658: i32;
    static mut DAT_009a8c2c: u32;
    static mut _DAT_00557360: u32;
    static mut DAT_00a60a28: *mut i8;
    static mut DAT_00a60a2c: *mut i8;
    static mut DAT_00a60a30: *mut i8;
    static mut DAT_00a60a34: *mut i8;
    static mut DAT_00523428: i32;
    static mut DAT_00bb7370: i8;
    static mut DAT_00523418: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac760() -> i32;
    fn FUN_0046fa30() -> i32;
    fn FUN_0046f700() -> i32;
    fn FUN_0046d350(a: i32, b: *mut u32, c: i32);
    fn FUN_0046e040() -> i32;
    fn FUN_0046f5e0(a: *mut i32, b: *mut u32, c: i32) -> *mut i32;
    fn FUN_0046d300(a: *mut u32, b: i32);
    fn FUN_00448da0();
    fn FUN_00448f90(a: i32, b: *mut c_void, c: i32);
    fn FUN_0046f660(a: i32) -> *mut c_void;
    fn FUN_0046f720(a: *mut c_void) -> *mut i8;
    fn FUN_00448ed0();
    fn FUN_00464c60();
    fn FUN_00463d00();
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn timeGetTime() -> u32;
}

/// Copies a NUL-terminated string, terminator included.
unsafe fn copy_cstr(dst: *mut u8, src: *const i8) {
    let mut i = 0usize;
    loop {
        // SAFETY: caller guarantees src is NUL-terminated and dst large enough.
        let b = unsafe { src.add(i).cast::<u8>().read() };
        unsafe { dst.add(i).write(b) };
        if b == 0 {
            break;
        }
        i += 1;
    }
}

unsafe fn len_cstr(s: *const i8) -> usize {
    let mut i = 0usize;
    // SAFETY: s is NUL-terminated.
    while unsafe { s.add(i).read() } != 0 {
        i += 1;
    }
    i
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00463790() -> u32 {
    let mut local_110: u32 = 0;
    let mut b_var3: i32 = 1;
    let mut local_10c: i32 = 0;
    let mut local_104: u32 = 0;
    let mut local_100 = [0u8; 256];
    // SAFETY: all globals and callees are the game's own; pointers passed
    // refer to locals that live for the whole call.
    unsafe {
        let p1 = (&raw mut DAT_00557560).cast::<u32>();
        let p2 = (&raw mut DAT_00556ed8).cast::<u32>();
        for i in 0..8usize {
            p1.add(i).write(0);
        }
        for i in 0..8usize {
            p2.add(i).write(0);
        }
        DAT_00559788 = 0;
        DAT_00557348 = 0;
        DAT_005242dc = 0;
        DAT_00556ef8 = 0;
        _DAT_0055714c = 0;
        DAT_00557358 = 0;
        DAT_005242e0 = 0;
        DAT_0052340c = 0;
        DAT_009a8c24 = 0;
        DAT_00557354 = 5;
        _DAT_00557148 = 0x16;
        DAT_00557580 = 0x17;
        if DAT_009fd394 == 1 {
            DAT_00a07cc8 = 0;
            _DAT_009ed23c = 0x1e;
            DAT_009e9658 = 0;
            DAT_009a8c2c = 0x1e;
            FUN_004ac760();
            return 1;
        }
        let local_108 = FUN_0046fa30();
        let mut i_var8 = FUN_0046f700();
        _DAT_00557360 = 0x14;
        FUN_0046d350(i_var8, &raw mut _DAT_00557360, 4);
        i_var8 = FUN_0046e040();
        if i_var8 != 0 {
            i_var8 = 0;
            while i_var8 < local_108 {
                loop {
                    let p = FUN_0046f5e0(&raw mut local_10c, &raw mut local_104, 0x3c);
                    if p.is_null() {
                        return 0;
                    }
                    if p.read() == 0x14 {
                        break;
                    }
                }
                i_var8 = i_var8.wrapping_add(1);
            }
            _DAT_00557360 = 0x13;
            FUN_0046d300(&raw mut _DAT_00557360, 4);
        }
        FUN_00448da0();
        FUN_00448f90(1, DAT_00a60a28.cast::<c_void>(), 0x30);
        loop {
            let pi4 = FUN_0046f5e0(&raw mut local_10c, &raw mut local_104, 0x3c);
            if pi4.is_null() {
                return 0;
            }
            if pi4.read() == 0x1d {
                let d_var5 = timeGetTime();
                if 200 < (d_var5.wrapping_sub(local_110) as i32) {
                    let pv = FUN_0046f660(local_10c);
                    let pc7 = FUN_0046f720(pv);
                    if pc7.is_null() {
                        copy_cstr(local_100.as_mut_ptr(), DAT_00a60a2c);
                    } else {
                        copy_cstr(local_100.as_mut_ptr(), pc7);
                        b_var3 = (b_var3 == 0) as i32;
                        let src = if b_var3 != 0 {
                            DAT_00a60a30
                        } else {
                            DAT_00a60a34
                        };
                        let n = len_cstr(local_100.as_ptr().cast::<i8>());
                        copy_cstr(local_100.as_mut_ptr().add(n), src);
                    }
                    FUN_00448f90(3, local_100.as_mut_ptr().cast::<c_void>(), 0x30);
                    local_110 = d_var5;
                }
            }
            if pi4.read() == 0x13 {
                FUN_00448ed0();
                let a = FUN_0046f700();
                _DAT_00557360 = 0x1b;
                FUN_0046d350(a, &raw mut _DAT_00557360, 4);
                FUN_004ac760();
                DAT_00a07cc8 = 0;
                DAT_009e9658 = 0;
                DAT_009a8c2c = 0;
                if DAT_00523428 == 0 {
                    _DAT_009ed23c = 0x1e;
                } else {
                    _DAT_009ed23c = 0x82;
                }
                if DAT_00bb7370 == 4 {
                    DAT_00523418 = 0x3b;
                } else if DAT_00bb7370 == 6 {
                    DAT_00523418 = 0x27;
                } else {
                    DAT_00523418 = 0x1d;
                }
                let mut i_var8 = FUN_0046e040();
                if i_var8 != 0 {
                    DAT_005242e0 = 1;
                    if local_108 != 1 {
                        DAT_005242e0 = 2;
                    }
                    FUN_00464c60();
                    while DAT_005242e0 != 0 && DAT_009a8c2c < 100 {
                        FUN_00463d00();
                        i_var8 = FUN_004ac760();
                        DAT_009a8c2c = DAT_009a8c2c.wrapping_add(i_var8 as u32);
                    }
                    DAT_005242e0 = 0;
                    i_var8 = FUN_004ac760();
                    DAT_009a8c2c = DAT_009a8c2c.wrapping_add(i_var8 as u32);
                    _DAT_009ed23c = DAT_009a8c2c;
                    if (DAT_009a8c2c as i32) < 0x23 {
                        i_var8 = 0x23u32.wrapping_sub(DAT_009a8c2c) as i32;
                        _DAT_009ed23c = DAT_009a8c2c.wrapping_add(i_var8 as u32);
                        DAT_009a8c2c = DAT_009a8c2c.wrapping_add(i_var8 as u32);
                        DAT_0052340c = DAT_0052340c.wrapping_sub(i_var8 as u32);
                    }
                }
                return 1;
            }
        }
    }
}
