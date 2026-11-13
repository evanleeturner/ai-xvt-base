// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a606d4: i32;
    static mut DAT_00a1051c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042ddc0(param_1: *mut core::ffi::c_void, param_2: i32, param_3: *mut i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042dda0(param_1: *mut i8) {
    // SAFETY: the callee and the globals are the game's own; the address of
    // DAT_00a1051c is passed through and param_1 is forwarded unchanged.
    unsafe {
        FUN_0042ddc0(
            (&raw mut DAT_00a1051c).cast::<core::ffi::c_void>(),
            DAT_00a606d4.wrapping_sub(1),
            param_1,
        );
    }
}
