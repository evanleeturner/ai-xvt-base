// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b2e: *mut *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d5bc0() {
    // SAFETY: the global is the game's own object pointer; when non-null it
    // points at an object whose first word is a vtable, with a stdcall
    // method at byte offset 0x28, called as the original does.
    unsafe {
        let obj = DAT_00aa7b2e;
        if !obj.is_null() {
            let vtable = obj.read_unaligned() as *const u8;
            let f = vtable
                .byte_add(0x28)
                .cast::<unsafe extern "stdcall" fn(*mut core::ffi::c_void)>()
                .read_unaligned();
            f(obj as *mut core::ffi::c_void);
        }
    }
}
