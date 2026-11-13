// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00803f80: *mut core::ffi::c_void;
    static mut DAT_00a68748: *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0040bbc0() {
    // SAFETY: the globals are the game's own memory; the object pointers are
    // read as the original does, and the call targets are the computed
    // addresses (vtable value plus offset) exactly as the C forms them.
    unsafe {
        if !DAT_00a68748.is_null() {
            let vt1 = (DAT_00803f80 as *const *const core::ffi::c_void).read_unaligned();
            let f1 = vt1.byte_add(0x20);
            let f1: unsafe extern "stdcall" fn(
                *mut core::ffi::c_void,
                i32,
                *mut core::ffi::c_void,
            ) = core::mem::transmute(f1);
            f1(DAT_00803f80, 0, DAT_00a68748);

            let vt2 = (DAT_00a68748 as *const *const core::ffi::c_void).read_unaligned();
            let f2 = vt2.byte_add(8);
            let f2: unsafe extern "stdcall" fn(*mut core::ffi::c_void) = core::mem::transmute(f2);
            f2(DAT_00a68748);

            DAT_00a68748 = core::ptr::null_mut();
        }
    }
}
