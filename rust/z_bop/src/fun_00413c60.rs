// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _fgetc(param_1: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00413c60(param_1: *mut core::ffi::c_void) {
    loop {
        // SAFETY: _fgetc is the game's own C runtime function; param_1 is the
        // caller's stream handle, passed through unchanged.
        let i_var1 = unsafe { _fgetc(param_1) };
        if (i_var1 as i8) == b'\n' as i8 {
            break;
        }
    }
}
