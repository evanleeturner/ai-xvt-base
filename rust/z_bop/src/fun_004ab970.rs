// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0066e200: [i8; 0];
    static mut DAT_0066e700: *mut *mut core::ffi::c_void;
    static mut DAT_0066ddd8: *mut *mut core::ffi::c_void;
    static mut DAT_00527eac: i32;
    static mut DAT_0066ddcc: *mut *mut core::ffi::c_void;
    static mut DAT_005280bc: [i8; 0];
    static mut DAT_005280a4: [i8; 0];
    static mut DAT_00521bb8: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0046c6a0() -> i32;
    fn wsprintfA(param_1: *mut i8, param_2: *const i8, ...) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn OutputDebugStringA(param_1: *const i8);
    fn MessageBoxA(
        param_1: *mut core::ffi::c_void,
        param_2: *const i8,
        param_3: *const i8,
        param_4: u32,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ab970(param_1: i32) -> i32 {
    // SAFETY: the globals, callees and COM-style objects are the game's own;
    // each object's vtable slot at +8 is called with the object, as the original does.
    unsafe {
        wsprintfA(
            (&raw mut DAT_0066e200).cast::<i8>(),
            (&raw const DAT_005280bc).cast::<i8>(),
            param_1,
        );
        OutputDebugStringA((&raw const DAT_0066e200).cast::<i8>());
        if !DAT_0066e700.is_null() {
            let obj = DAT_0066e700;
            let vtbl = (*obj) as *const u8;
            let f = vtbl
                .byte_add(8)
                .cast::<*const core::ffi::c_void>()
                .read_unaligned();
            let f: unsafe extern "C" fn(*mut core::ffi::c_void) = core::mem::transmute(f);
            f(obj.cast());
            DAT_0066e700 = core::ptr::null_mut();
        }
        if !DAT_0066ddd8.is_null() {
            let obj = DAT_0066ddd8;
            let vtbl = (*obj) as *const u8;
            let f = vtbl
                .byte_add(8)
                .cast::<*const core::ffi::c_void>()
                .read_unaligned();
            let f: unsafe extern "C" fn(*mut core::ffi::c_void) = core::mem::transmute(f);
            f(obj.cast());
            DAT_0066ddd8 = core::ptr::null_mut();
        }
        if DAT_00527eac != 0 && !DAT_0066ddcc.is_null() {
            let obj = DAT_0066ddcc;
            let vtbl = (*obj) as *const u8;
            let f = vtbl
                .byte_add(8)
                .cast::<*const core::ffi::c_void>()
                .read_unaligned();
            let f: unsafe extern "C" fn(*mut core::ffi::c_void) = core::mem::transmute(f);
            f(obj.cast());
            DAT_0066ddcc = core::ptr::null_mut();
        }
        FUN_0046c6a0();
        MessageBoxA(
            core::ptr::null_mut(),
            (&raw const DAT_005280a4).cast::<i8>(),
            (&raw const DAT_00521bb8).cast::<i8>(),
            0,
        );
    }
    0
}
