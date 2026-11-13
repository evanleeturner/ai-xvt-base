// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b69a95: *mut core::ffi::c_void;
    static mut DAT_00b69a99: *mut core::ffi::c_void;
    static mut DAT_00b69a9d: *mut core::ffi::c_void;
    static mut DAT_00b69aa1: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _free(param_1: *mut core::ffi::c_void);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004ddbc0() {
    // SAFETY: the globals and _free are the game's own; the pointers are
    // freed and cleared exactly where the original does.
    unsafe {
        if !DAT_00b69a95.is_null() {
            _free(DAT_00b69a95);
            DAT_00b69a95 = core::ptr::null_mut();
        }
        if !DAT_00b69a99.is_null() {
            _free(DAT_00b69a99);
            DAT_00b69a99 = core::ptr::null_mut();
        }
        DAT_00b69a9d = core::ptr::null_mut();
        DAT_00b69aa1 = core::ptr::null_mut();
    }
}
