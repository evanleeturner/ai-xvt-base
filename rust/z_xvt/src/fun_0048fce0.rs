// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005344c0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _fprintf(stream: *mut core::ffi::c_void, format: *const i8);
    fn _exit(code: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0048fce0(param_1: *mut i8, param_2: i32) {
    // SAFETY: the callees and the global are the game's own; the address of
    // DAT_005344c0 is passed on as the stream, and param_1 is the caller's
    // format string.
    unsafe {
        _fprintf(
            (&raw mut DAT_005344c0).cast::<core::ffi::c_void>(),
            param_1 as *const i8,
        );
        _exit(param_2);
    }
}
