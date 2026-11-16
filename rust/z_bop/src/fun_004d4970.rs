// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b2e: i32;
    static mut DAT_00aa7b3a: *mut core::ffi::c_void;
    static mut DAT_00aa7b0d: i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d4970() {
    // SAFETY: the globals are the game's own memory; DAT_00aa7b3a was checked
    // non-null and points at an object whose first field is a vtable pointer,
    // with a stdcall method at offset 0x80.
    unsafe {
        if DAT_00aa7b2e != 0 && !DAT_00aa7b3a.is_null() {
            let this = DAT_00aa7b3a;
            let vtable = this.cast::<*const core::ffi::c_void>().read_unaligned();
            let func = vtable
                .byte_add(0x80)
                .cast::<*const core::ffi::c_void>()
                .read_unaligned();
            let f: unsafe extern "stdcall" fn(*mut core::ffi::c_void, i32) =
                core::mem::transmute(func);
            f(this, 0);
            DAT_00aa7b0d = 0;
        }
    }
}
