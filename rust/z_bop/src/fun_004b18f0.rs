// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00528cbc: i32;
    static mut DAT_00528c94: *mut core::ffi::c_void;
    static mut DAT_00528c98: *mut core::ffi::c_void;
    static mut DAT_00664974: *mut core::ffi::c_void;
    static mut DAT_0066497c: *mut core::ffi::c_void;
    static mut DAT_00a91a34: *mut core::ffi::c_void;
    static mut DAT_00664548: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004079e0();
    fn FUN_004b3070();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004b17d0(param_1: *mut core::ffi::c_void);
}

#[inline(always)]
unsafe fn call_release(obj: *mut core::ffi::c_void) {
    // SAFETY: obj is a live game object whose first field is a vtable
    // pointer; slot at offset 8 is a __stdcall function taking the object.
    unsafe {
        let vtable = obj.cast::<*const u8>().read_unaligned();
        let f = vtable
            .byte_add(8)
            .cast::<*const core::ffi::c_void>()
            .read_unaligned();
        let f: unsafe extern "stdcall" fn(*mut core::ffi::c_void) = core::mem::transmute(f);
        f(obj);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004b18f0() {
    // SAFETY: the globals, callees and COM-style objects are the game's own;
    // each object is released through its vtable exactly where the original does.
    unsafe {
        if DAT_00528cbc == 0 {
            FUN_004079e0();
            return;
        }
        if !DAT_00528c94.is_null() {
            FUN_004b17d0(DAT_00528c94);
            DAT_00528c94 = core::ptr::null_mut();
        }
        call_release(DAT_00528c98);
        FUN_004b3070();
        if !DAT_00664974.is_null() {
            call_release(DAT_00664974);
            DAT_00664974 = core::ptr::null_mut();
        }
        if !DAT_0066497c.is_null() {
            call_release(DAT_0066497c);
            DAT_0066497c = core::ptr::null_mut();
        }
        if !DAT_00a91a34.is_null() {
            call_release(DAT_00a91a34);
            DAT_00a91a34 = core::ptr::null_mut();
        }
        if !DAT_00664548.is_null() {
            call_release(DAT_00664548);
            DAT_00664548 = core::ptr::null_mut();
        }
        FUN_004079e0();
        DAT_00528cbc = 0;
    }
}
