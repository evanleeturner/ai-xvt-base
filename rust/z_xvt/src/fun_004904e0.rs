// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052a59c: *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004904e0() -> *mut i32 {
    // SAFETY: the global is the game's own object pointer; its first word is
    // a vtable whose slot at offset 0x10 is a stdcall method taking the
    // object and two out buffers, kept on this stack frame.
    unsafe {
        let obj = DAT_0052a59c;
        if !obj.is_null() {
            let mut local_8 = [0i8; 4];
            let mut local_4 = [0i8; 4];
            let vtable = obj.read_unaligned() as usize as *const u8;
            let f = vtable.byte_add(0x10).cast::<u32>().read_unaligned();
            let func: unsafe extern "stdcall" fn(
                *mut i32,
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
            ) = core::mem::transmute(f as usize as *const core::ffi::c_void);
            func(
                obj,
                local_8.as_mut_ptr().cast(),
                local_4.as_mut_ptr().cast(),
            );
            return obj;
        }
        core::ptr::null_mut()
    }
}
