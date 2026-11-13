// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b69aa7: [i8; 1];
}

unsafe extern "C" {
    fn __chdir(path: *const i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004ccf70() -> u32 {
    // SAFETY: the global and __chdir are the game's own; the address of the
    // path buffer is passed through unchanged.
    unsafe {
        __chdir((&raw const DAT_00b69aa7).cast::<i8>());
    }
    1
}
