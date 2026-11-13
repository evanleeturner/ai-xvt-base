// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009994c4: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn LAB_0046c750();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0046c730() -> i32 {
    // SAFETY: the global and the label are the game's own. The C adds 0x30
    // bytes to the vtable pointer and calls that address directly, without
    // dereferencing it; that is kept as is.
    unsafe {
        let obj = DAT_009994c4;
        let vtable = obj.cast::<*mut core::ffi::c_void>().read_unaligned();
        let target = vtable.byte_add(0x30) as *const core::ffi::c_void;
        let f: unsafe extern "stdcall" fn(
            *mut core::ffi::c_void,
            i32,
            *mut core::ffi::c_void,
            i32,
            i32,
        ) = core::mem::transmute(target);
        f(
            obj,
            0,
            LAB_0046c750 as usize as *mut core::ffi::c_void,
            0,
            0,
        );
    }
    1
}
