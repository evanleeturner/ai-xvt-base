// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052ae28: i32;
    static mut DAT_00525534: i32;
    static mut DAT_0052ae50: i32;
    static mut DAT_0070acc8: i32;
    static mut DAT_0052aef8: i32;
    static mut DAT_0052ae2c: i32;
    static mut DAT_0052ae30: i32;
    static mut DAT_0052ae54: i32;
    static mut DAT_00657c0c: i32;
    static mut DAT_006572b8: i32;
    static mut DAT_006572d4: i32;
    static mut DAT_00525520: i32;
    static mut DAT_00525528: i32;
    static mut DAT_00525518: i32;
    static mut DAT_0071f2eb: u8;
    static mut DAT_006572c8: *mut i32;
    static mut DAT_00657c00: *mut i32;
    static mut DAT_006572cc: *mut i32;
    static mut DAT_006572c0: *mut i32;
    static mut DAT_00658020: *mut i32;
    static mut DAT_00657700: [i8; 1];
    static mut s____CleanupAndExit_err____d_0052af38: [i8; 1];
    static mut s_Game_could_not_start_0052af20: [i8; 1];
    static mut s_ERROR_00524f00: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d58c0() -> i32;
    fn FUN_004d5510() -> i32;
    fn FUN_0042fda0();
    fn FUN_0048f560(param_1: i32);
    fn FUN_0048f550(param_1: i32);
    fn FUN_00461f20(param_1: i32, param_2: i32, param_3: i32, param_4: i32);
    fn wsprintfA(buf: *mut i8, fmt: *const i8, ...) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn OutputDebugStringA(s: *const i8);
    fn MessageBoxA(h: *mut c_void, text: *const i8, caption: *const i8, ty: u32) -> i32;
}

type CreateFn = unsafe extern "stdcall" fn(*mut i32, *mut i32, *mut *mut i32, i32) -> i32;
type ReleaseFn = unsafe extern "stdcall" fn(*mut i32) -> i32;
type LockFn = unsafe extern "stdcall" fn(*mut i32, i32, *mut i32, i32, i32) -> i32;
type DescFn = unsafe extern "stdcall" fn(*mut i32, *mut i32) -> i32;

unsafe fn slot(obj: *mut i32, off: usize) -> *const c_void {
    // SAFETY: obj is a COM-style object whose first word is a vtable pointer.
    unsafe {
        let vt = obj.read_unaligned() as u32 as usize;
        ((vt + off) as *const *const c_void).read_unaligned()
    }
}

unsafe fn do_lock(obj: *mut i32, desc: *mut i32) -> i32 {
    // SAFETY: vtable slot 100 is the lock method.
    unsafe {
        let f: LockFn = core::mem::transmute(slot(obj, 100));
        f(obj, 0, desc, 0, 0)
    }
}

unsafe fn do_desc(obj: *mut i32, desc: *mut i32) {
    // SAFETY: vtable slot 0x58 is the surface-desc getter.
    unsafe {
        let f: DescFn = core::mem::transmute(slot(obj, 0x58));
        f(obj, desc);
    }
}

unsafe fn do_release(obj: *mut i32) {
    // SAFETY: vtable slot 8 is Release.
    unsafe {
        let f: ReleaseFn = core::mem::transmute(slot(obj, 8));
        f(obj);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004a6a80() {
    let mut a: [i32; 27];
    // SAFETY: the globals, callees and COM objects are the game's own.
    unsafe {
        if DAT_0052ae28 == 1 {
            DAT_00525534 = FUN_004d58c0();
            DAT_0052ae50 = DAT_00525534;
            DAT_0070acc8 = FUN_004d5510();
            return;
        }
        if DAT_0052aef8 != 0 {
            DAT_0052aef8 = DAT_0052aef8.wrapping_add(1);
            return;
        }
        DAT_0052aef8 = 1;
        if DAT_0052ae2c == 0 {
            a = [0; 27];
            a[0] = 0x6c;
            a[1] = 1;
            a[26] = 0x200;
            let obj = DAT_006572c8;
            let f: CreateFn = core::mem::transmute(slot(obj, 0x18));
            let r = f(obj, a.as_mut_ptr(), &raw mut DAT_00657c00, 0);
            if r != 0 {
                wsprintfA(
                    (&raw mut DAT_00657700).cast::<i8>(),
                    (&raw const s____CleanupAndExit_err____d_0052af38).cast::<i8>(),
                    4i32,
                );
                OutputDebugStringA((&raw const DAT_00657700).cast::<i8>());
                if !DAT_00657c00.is_null() {
                    do_release(DAT_00657c00);
                    DAT_00657c00 = core::ptr::null_mut();
                }
                if !DAT_006572cc.is_null() {
                    do_release(DAT_006572cc);
                    DAT_006572cc = core::ptr::null_mut();
                }
                if DAT_0052ae30 != 0 && !DAT_006572c0.is_null() {
                    do_release(DAT_006572c0);
                    DAT_006572c0 = core::ptr::null_mut();
                }
                FUN_0042fda0();
                MessageBoxA(
                    core::ptr::null_mut(),
                    (&raw const s_Game_could_not_start_0052af20).cast::<i8>(),
                    (&raw const s_ERROR_00524f00).cast::<i8>(),
                    0,
                );
                return;
            }
            a = [0; 27];
            do_desc(DAT_00657c00, a.as_mut_ptr());
            DAT_00657c0c = a[4];
        }
        if DAT_0052ae30 == 0 {
            a = [0; 27];
            a[0] = 0x6c;
            loop {
                let r = do_lock(DAT_00657c00, a.as_mut_ptr());
                if r == 0 {
                    break;
                }
                if r != -0x7789fde4 {
                    return;
                }
            }
            FUN_0048f560(a[9]);
            FUN_0048f550((DAT_00657c0c as u32).wrapping_mul(0x1e0) as i32);
            DAT_00525534 = a[9];
            DAT_0052ae50 = a[9];
        } else if DAT_0071f2eb == 0 {
            a = [0; 27];
            a[0] = 0x6c;
            loop {
                let r = do_lock(DAT_00657c00, a.as_mut_ptr());
                if r == 0 {
                    FUN_0048f560(a[9]);
                    DAT_00525534 = a[9];
                    DAT_0052ae50 = a[9];
                    a = [0; 27];
                    a[0] = 0x6c;
                    do_desc(DAT_00657c00, a.as_mut_ptr());
                    DAT_00657c0c = a[2];
                    let u3 = (DAT_006572b8 as u32).wrapping_sub(DAT_00525520 as u32) >> 1;
                    let i1 = u3.wrapping_mul(DAT_00525518 as u32);
                    let u3 = (DAT_006572d4 as u32).wrapping_sub(DAT_00525528 as u32) >> 1;
                    let i2 = u3.wrapping_mul(a[2] as u32);
                    DAT_00525534 = (DAT_00525534 as u32).wrapping_add(i1).wrapping_add(i2) as i32;
                    DAT_0052ae50 = (DAT_0052ae50 as u32).wrapping_add(i1).wrapping_add(i2) as i32;
                    FUN_0048f550((a[2] as u32).wrapping_mul(0x1e0) as i32);
                    return;
                }
                if r != -0x7789fde4 {
                    return;
                }
            }
        } else if DAT_0052ae54 == 0 {
            a = [0; 27];
            a[0] = 0x6c;
            loop {
                let r = do_lock(DAT_00658020, a.as_mut_ptr());
                if r == 0 {
                    break;
                }
                if r != -0x7789fde4 {
                    return;
                }
            }
            FUN_0048f560(a[9]);
            DAT_00525534 = a[9];
            DAT_0052ae50 = a[9];
            a = [0; 27];
            a[0] = 0x6c;
            do_desc(DAT_00658020, a.as_mut_ptr());
            DAT_00657c0c = a[2];
            let u3 = (DAT_006572b8 as u32).wrapping_sub(DAT_00525520 as u32) >> 1;
            let i2 = u3.wrapping_mul(DAT_00525518 as u32);
            let u3 = (DAT_006572d4 as u32).wrapping_sub(DAT_00525528 as u32) >> 1;
            let i1 = u3.wrapping_mul(a[2] as u32);
            DAT_00525534 = (DAT_00525534 as u32).wrapping_add(i2).wrapping_add(i1) as i32;
            DAT_0052ae50 = (DAT_0052ae50 as u32).wrapping_add(i2).wrapping_add(i1) as i32;
            FUN_0048f550((a[2] as u32).wrapping_mul(0x1e0) as i32);
            if DAT_0070acc8 != DAT_00657c0c {
                DAT_0070acc8 = DAT_00657c0c;
                FUN_00461f20(DAT_00525534, DAT_00657c0c, 0x1e0, -1);
                return;
            }
        } else {
            a = [0; 27];
            a[0] = 0x6c;
            loop {
                let r = do_lock(DAT_006572c0, a.as_mut_ptr());
                if r == 0 {
                    break;
                }
                if r != -0x7789fde4 {
                    return;
                }
            }
            FUN_0048f560(a[9]);
            DAT_00525534 = a[9];
            DAT_0052ae50 = a[9];
            a = [0; 27];
            a[0] = 0x6c;
            do_desc(DAT_006572c0, a.as_mut_ptr());
            DAT_00657c0c = a[2];
            FUN_0048f550((a[2] as u32).wrapping_mul(0x1e0) as i32);
            if DAT_0070acc8 != DAT_00657c0c {
                DAT_0070acc8 = DAT_00657c0c;
                FUN_00461f20(DAT_00525534, DAT_00657c0c, 0x1e0, -1);
                return;
            }
        }
    }
}
