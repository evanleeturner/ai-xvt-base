// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523574: *mut i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _fscanf(stream: *mut i32, format: *const i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004051f0(param_1: *mut i32) {
    let mut local_1: i8 = 0;
    loop {
        // SAFETY: _fscanf and the format global are the game's own; param_1 is
        // the caller's stream and local_1 is a live local receiving one char.
        let i_var1 = unsafe { _fscanf(param_1, DAT_00523574 as *const i8, &raw mut local_1) };
        if i_var1 != 1 {
            return;
        }
        if local_1 != b'"' as i8 {
            return;
        }
    }
}
