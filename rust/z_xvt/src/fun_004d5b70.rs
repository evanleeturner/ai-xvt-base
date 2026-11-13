// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0075966b: *mut core::ffi::c_void;
    static mut DAT_007596cf: *mut core::ffi::c_void;
    static mut DAT_00759667: *mut core::ffi::c_void;
    static mut DAT_0075966f: *mut core::ffi::c_void;
    static mut DAT_0081b6e4: u32;
    static mut DAT_0081b6f0: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d4d90() -> u32;
    fn FUN_004de0c0() -> u32;
    fn FUN_004ddec0() -> u32;
    fn FUN_004cfd60() -> u32;
}

#[inline(always)]
unsafe fn call_release(obj: *mut core::ffi::c_void) {
    // SAFETY: the caller passes a non-null game object whose first field is a
    // pointer; the C calls that pointer plus 2 bytes with the object as argument.
    unsafe {
        let vt = obj.cast::<*mut core::ffi::c_void>().read_unaligned();
        let f = vt.byte_add(2) as *const core::ffi::c_void;
        let f: unsafe extern "C" fn(*mut core::ffi::c_void) = core::mem::transmute(f);
        f(obj);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d5b70() -> u32 {
    // SAFETY: the callees and globals are the game's own memory; each object
    // pointer is checked non-null before it is called through.
    unsafe {
        FUN_004d4d90();
        FUN_004de0c0();
        FUN_004ddec0();
        FUN_004cfd60();
        if !DAT_0075966b.is_null() {
            call_release(DAT_0075966b);
            DAT_0075966b = core::ptr::null_mut();
        }
        if !DAT_007596cf.is_null() {
            call_release(DAT_007596cf);
            DAT_007596cf = core::ptr::null_mut();
        }
        if !DAT_00759667.is_null() {
            call_release(DAT_00759667);
            DAT_00759667 = core::ptr::null_mut();
        }
        if (DAT_0081b6e4 != 0 || DAT_0081b6f0 != 0) && !DAT_0075966f.is_null() {
            call_release(DAT_0075966f);
            DAT_0075966f = core::ptr::null_mut();
        }
        1
    }
}
