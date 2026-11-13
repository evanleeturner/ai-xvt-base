// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0051be2c: *const i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _fscanf(param_1: *mut core::ffi::c_void, param_2: *const i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004138a0(param_1: *mut core::ffi::c_void) {
    let mut local_1: i8 = 0;
    loop {
        // SAFETY: _fscanf and the format-string global are the game's own;
        // local_1 is a live local the callee writes one char into.
        let i_var1 = unsafe { _fscanf(param_1, DAT_0051be2c, &raw mut local_1) };
        if i_var1 != 1 {
            return;
        }
        if local_1 == b'{' as i8 {
            return;
        }
    }
}
