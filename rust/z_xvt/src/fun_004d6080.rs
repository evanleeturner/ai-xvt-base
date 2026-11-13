// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759663: *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d6080() {
    // SAFETY: the global is the game's own object pointer; its first field is
    // read as a table pointer and the function pointer is read at a byte
    // offset of 10 into that table, as the original does (void* arithmetic).
    unsafe {
        let obj = DAT_00759663;
        if !obj.is_null() {
            let vtable = obj.cast::<*mut core::ffi::c_void>().read_unaligned();
            let slot = vtable.byte_add(10).cast::<*const core::ffi::c_void>();
            let f: unsafe extern "stdcall" fn(*mut core::ffi::c_void) =
                core::mem::transmute(slot.read_unaligned());
            f(obj);
        }
    }
}
