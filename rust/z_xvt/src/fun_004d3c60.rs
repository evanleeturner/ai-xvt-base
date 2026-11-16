// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0075966b: *mut *mut core::ffi::c_void;
    static mut DAT_00759667: *mut *mut core::ffi::c_void;
    static mut DAT_0075966f: *mut *mut core::ffi::c_void;
    static mut DAT_0081b6e4: i32;
    static mut DAT_0081b6f0: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d3c60() {
    // SAFETY: the globals are the game's own objects; each is a pointer to an
    // object whose first field is a vtable, and slot 0x6c holds a stdcall method
    // taking the object pointer, called as the original does.
    unsafe {
        let obj = DAT_0075966b;
        let vtbl = obj.read_unaligned();
        let f = vtbl
            .byte_add(0x6c)
            .cast::<*const core::ffi::c_void>()
            .read_unaligned();
        let f: unsafe extern "stdcall" fn(*mut core::ffi::c_void) -> i32 = core::mem::transmute(f);
        let i_var1 = f(obj as *mut core::ffi::c_void);
        if i_var1 == 0 {
            if DAT_0081b6e4 == 0 && DAT_0081b6f0 == 0 {
                let obj = DAT_00759667;
                let vtbl = obj.read_unaligned();
                let f = vtbl
                    .byte_add(0x6c)
                    .cast::<*const core::ffi::c_void>()
                    .read_unaligned();
                let f: unsafe extern "stdcall" fn(*mut core::ffi::c_void) = core::mem::transmute(f);
                f(obj as *mut core::ffi::c_void);
                return;
            }
            let obj = DAT_0075966f;
            let vtbl = obj.read_unaligned();
            let f = vtbl
                .byte_add(0x6c)
                .cast::<*const core::ffi::c_void>()
                .read_unaligned();
            let f: unsafe extern "stdcall" fn(*mut core::ffi::c_void) = core::mem::transmute(f);
            f(obj as *mut core::ffi::c_void);
        }
    }
}
