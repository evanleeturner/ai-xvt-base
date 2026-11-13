// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00556900: *mut c_void;
    static mut DAT_00556904: *mut c_void;
    static mut DAT_00556908: *mut c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00443820() {
    // SAFETY: the globals are the game's own COM-like object pointers; each
    // vtable slot is read and called exactly where the original does.
    unsafe {
        if !DAT_00556908.is_null() {
            let obj = DAT_00556904;
            let vtbl = obj.cast::<*const c_void>().read_unaligned();
            let slot = vtbl.byte_add(0x20).cast::<*const c_void>().read_unaligned();
            let f: unsafe extern "stdcall" fn(*mut c_void) = core::mem::transmute(slot);
            f(obj);
            DAT_00556908 = core::ptr::null_mut();
        }
        if !DAT_00556904.is_null() {
            let obj = DAT_00556904;
            let vtbl = obj.cast::<*const c_void>().read_unaligned();
            let slot = vtbl.byte_add(8).cast::<*const c_void>().read_unaligned();
            let f: unsafe extern "stdcall" fn(*mut c_void) = core::mem::transmute(slot);
            f(obj);
        }
        if !DAT_00556900.is_null() {
            let obj = DAT_00556900;
            let vtbl = obj.cast::<*const c_void>().read_unaligned();
            let slot = vtbl.byte_add(8).cast::<*const c_void>().read_unaligned();
            let f: unsafe extern "stdcall" fn(*mut c_void) = core::mem::transmute(slot);
            f(obj);
        }
    }
}
