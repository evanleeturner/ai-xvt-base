// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00658654: i32;
    static mut DAT_00658658: *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004911f0() -> i32 {
    let mut local_4: [i8; 4] = [0; 4];
    // SAFETY: the globals are the game's own; DAT_00658658 points at a COM-style
    // object whose first dword is a vtable, with the method at offset 0x18,
    // called as the original does.
    unsafe {
        if DAT_00658654 == 0 {
            return 0;
        }
        let pi_var2 = DAT_00658658;
        let vtable = pi_var2.cast::<*const core::ffi::c_void>().read_unaligned();
        let method = vtable
            .byte_add(0x18)
            .cast::<*const core::ffi::c_void>()
            .read_unaligned();
        let f: unsafe extern "stdcall" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> i32 =
            core::mem::transmute(method);
        let i_var1 = f(
            pi_var2.cast::<core::ffi::c_void>(),
            local_4.as_mut_ptr().cast::<core::ffi::c_void>(),
        );
        if i_var1 != 0 {
            return 0;
        }
        ((pi_var2 as usize as i32).wrapping_mul(0x7f) / 2000).wrapping_add(0x7f)
    }
}
