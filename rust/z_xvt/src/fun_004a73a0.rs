// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00657c00: *mut *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004a73a0() -> u32 {
    let mut local_6c: [u32; 27] = [0; 27];
    local_6c[0] = 0x6c;
    loop {
        // SAFETY: DAT_00657c00 is the game's own object pointer, whose first
        // word is a vtable; slot at +100 is called with the C's arguments,
        // and local_6c lives for the whole call.
        let i_var2 = unsafe {
            let obj = DAT_00657c00;
            let vtbl = obj.read() as *const u8;
            let f: unsafe extern "stdcall" fn(
                *mut *mut core::ffi::c_void,
                i32,
                *mut u32,
                i32,
                i32,
            ) -> i32 = core::mem::transmute(
                vtbl.byte_add(100)
                    .cast::<*const core::ffi::c_void>()
                    .read_unaligned(),
            );
            f(obj, 0, local_6c.as_mut_ptr(), 0, 0)
        };
        if i_var2 == 0 {
            // uStack_48 is the dword at offset 0x24 of the same stack block.
            return local_6c[9];
        }
        if i_var2 != -0x7789fde4 {
            break;
        }
    }
    0
}
