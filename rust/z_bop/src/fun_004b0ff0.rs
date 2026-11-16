// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00664970: *mut *mut core::ffi::c_void;
    static mut DAT_00528cb8: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004079e0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004b0ff0() {
    // SAFETY: the globals and the callee are the game's own; DAT_00664970
    // points at an object whose first field is a vtable, and slot 2 is
    // called with the object as its argument.
    unsafe {
        let obj = DAT_00664970;
        if !obj.is_null() {
            let vtable = *obj as *const *const core::ffi::c_void;
            let f = vtable.add(2).read_unaligned();
            let func: unsafe extern "stdcall" fn(*mut *mut core::ffi::c_void) =
                core::mem::transmute(f);
            func(obj);
        }
        FUN_004079e0();
        DAT_00528cb8 = 0;
    }
}
