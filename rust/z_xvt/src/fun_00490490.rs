// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052a59c: *mut *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00490490() {
    // SAFETY: the global is the game's own object pointer; its first word is
    // a vtable whose slots are called exactly as the original does, re-reading
    // the global before each call.
    unsafe {
        if !DAT_0052a59c.is_null() {
            let f = (*DAT_0052a59c)
                .byte_add(0x34)
                .cast::<*const core::ffi::c_void>()
                .read_unaligned();
            let f: unsafe extern "stdcall" fn(*mut *mut core::ffi::c_void, i32) =
                core::mem::transmute(f);
            f(DAT_0052a59c, 0);

            let f = (*DAT_0052a59c)
                .byte_add(0x3c)
                .cast::<*const core::ffi::c_void>()
                .read_unaligned();
            let f: unsafe extern "stdcall" fn(*mut *mut core::ffi::c_void, i32) =
                core::mem::transmute(f);
            f(DAT_0052a59c, 0);

            let f = (*DAT_0052a59c)
                .byte_add(0x40)
                .cast::<*const core::ffi::c_void>()
                .read_unaligned();
            let f: unsafe extern "stdcall" fn(*mut *mut core::ffi::c_void, i32) =
                core::mem::transmute(f);
            f(DAT_0052a59c, 0);

            let f = (*DAT_0052a59c)
                .byte_add(0x30)
                .cast::<*const core::ffi::c_void>()
                .read_unaligned();
            let f: unsafe extern "stdcall" fn(*mut *mut core::ffi::c_void, i32, i32, i32) =
                core::mem::transmute(f);
            f(DAT_0052a59c, 0, 0, (!DAT_0052a59c.is_null()) as i32);
        }
    }
}
