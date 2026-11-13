// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00497e00(a: i32, b: i32, c: u32, d: i32, e: u32, f: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00442bc0(
    param_1: i32,
    param_2: i32,
    param_3: u32,
    param_4: i32,
    param_5: u8,
) {
    // SAFETY: the callee is the game's own function; arguments are passed through unchanged.
    unsafe {
        FUN_00497e00(param_1, param_2, param_3, param_4, param_5 as u32, 1);
    }
}
