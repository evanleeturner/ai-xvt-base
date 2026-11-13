// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00664548: *mut *mut core::ffi::c_void;
    static mut DAT_00528cc0: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004b1600(param_1: i32, param_2: *mut i32, param_3: i32) -> *mut i8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004079e0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004b1c50() {
    // SAFETY: the globals and callees are the game's own; DAT_00664548 points
    // at an object whose vtable slot at offset 0x50 holds a stdcall function.
    unsafe {
        let obj = DAT_00664548;
        let vtable = (*obj) as *const u8;
        let f = vtable
            .byte_add(0x50)
            .cast::<*const core::ffi::c_void>()
            .read_unaligned();
        let f: unsafe extern "stdcall" fn(*mut core::ffi::c_void) -> i32 = core::mem::transmute(f);
        let i_var1 = f(obj as *mut core::ffi::c_void);
        if i_var1 != 0 {
            FUN_004b1600(i_var1, &raw mut DAT_00528cc0, 0x79);
            FUN_004079e0();
        }
    }
}
