// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004f8950(
        param_1: *mut i8,
        param_2: i32,
        param_3: i16,
        param_4: i16,
        param_5: i16,
        param_6: i16,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004f8910(
    param_1: *mut i8,
    param_2: i16,
    param_3: i16,
    param_4: i16,
    param_5: i16,
    param_6: i16,
) {
    if param_5 >= 0 {
        // SAFETY: the callee is the game's own function; arguments are passed through as the original does.
        unsafe {
            FUN_004f8950(
                param_1,
                param_2 as i32,
                param_3,
                param_4,
                ((param_5 as i32).wrapping_mul(2)) as i16,
                param_6,
            );
        }
    }
}
