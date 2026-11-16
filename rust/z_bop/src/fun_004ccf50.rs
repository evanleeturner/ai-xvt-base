// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b69ba7: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __chdir(param_1: *const i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004ccf50() -> i32 {
    // SAFETY: the global buffer and __chdir belong to the game; the buffer's
    // address is passed as the NUL-terminated path the original uses.
    unsafe {
        __chdir((&raw mut DAT_00b69ba7).cast::<i8>());
    }
    1
}
