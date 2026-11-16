// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006a8a5c: *mut *mut core::ffi::c_void;
    static mut DAT_00658020: *mut *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0045fcb0() {
    // SAFETY: the globals are the game's own COM-style object pointers; each
    // vtable slot read holds a stdcall function pointer called as the original does.
    unsafe {
        if !DAT_006a8a5c.is_null() {
            let obj1 = DAT_00658020;
            let vtbl1 = obj1.read() as *const u8;
            let f1 = vtbl1
                .byte_add(0x20)
                .cast::<*const core::ffi::c_void>()
                .read_unaligned();
            let f1: unsafe extern "stdcall" fn(
                *mut core::ffi::c_void,
                i32,
                *mut core::ffi::c_void,
            ) = core::mem::transmute(f1);
            f1(
                obj1 as *mut core::ffi::c_void,
                0,
                DAT_006a8a5c as *mut core::ffi::c_void,
            );

            let obj2 = DAT_006a8a5c;
            let vtbl2 = obj2.read() as *const u8;
            let f2 = vtbl2
                .byte_add(0x8)
                .cast::<*const core::ffi::c_void>()
                .read_unaligned();
            let f2: unsafe extern "stdcall" fn(*mut core::ffi::c_void) = core::mem::transmute(f2);
            f2(obj2 as *mut core::ffi::c_void);

            DAT_006a8a5c = core::ptr::null_mut();
        }
    }
}
