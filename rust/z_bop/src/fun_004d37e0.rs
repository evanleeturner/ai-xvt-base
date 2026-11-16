// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b32: *mut *mut c_void;
    static mut DAT_00aa7b36: *mut *mut c_void;
    static mut DAT_00aa7b3a: *mut *mut c_void;
    static mut DAT_00b69cb0: *mut c_void;
    static mut DAT_00b69cbc: *mut c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d37e0() {
    // SAFETY: the globals are the game's own objects; each points at an
    // object whose first word is a vtable, and slot 0x1b holds a stdcall
    // method taking the object pointer.
    unsafe {
        let obj1 = DAT_00aa7b36;
        let vtbl1 = obj1.read() as *const *mut c_void;
        let f1: unsafe extern "stdcall" fn(*mut c_void) -> i32 =
            core::mem::transmute(vtbl1.add(0x1b).read());
        let r = f1(obj1 as *mut c_void);
        if r == 0 {
            if DAT_00b69cb0.is_null() && DAT_00b69cbc.is_null() {
                let obj2 = DAT_00aa7b32;
                let vtbl2 = obj2.read() as *const *mut c_void;
                let f2: unsafe extern "stdcall" fn(*mut c_void) =
                    core::mem::transmute(vtbl2.add(0x1b).read());
                f2(obj2 as *mut c_void);
                return;
            }
            let obj3 = DAT_00aa7b3a;
            let vtbl3 = obj3.read() as *const *mut c_void;
            let f3: unsafe extern "stdcall" fn(*mut c_void) =
                core::mem::transmute(vtbl3.add(0x1b).read());
            f3(obj3 as *mut c_void);
        }
    }
}
