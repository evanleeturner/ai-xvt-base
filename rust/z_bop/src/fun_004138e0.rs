// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0051be2c: i8;
}

unsafe extern "C" {
    fn _fscanf(param_1: i32, param_2: *const i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004138e0(param_1: i32) {
    let mut local_1: i8 = 0;
    loop {
        // SAFETY: the format string and fscanf are the game's own; local_1
        // is a valid destination for the single character the format reads.
        let i_var1 = unsafe {
            _fscanf(
                param_1,
                (&raw mut DAT_0051be2c) as *const i8,
                &raw mut local_1,
            )
        };
        if i_var1 != 1 {
            return;
        }
        if local_1 != b'[' as i8 {
            break;
        }
    }
}
