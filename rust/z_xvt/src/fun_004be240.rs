// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _ftell(param_1: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004be240(param_1: *mut core::ffi::c_void) {
    // SAFETY: _ftell is the game's own C runtime function; param_1 is passed through unchanged.
    unsafe {
        _ftell(param_1);
    }
}
