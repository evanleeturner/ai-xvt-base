// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0073255c: i32;
    static mut DAT_00732584: i32;
    static mut DAT_0073253c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00413cc0(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040ca00(param_1: i32, param_2: i32, param_3: i32) {
    // SAFETY: the globals and the callee are the game's own; values are read
    // in the original's order and passed straight through.
    unsafe {
        FUN_00413cc0(
            DAT_0073255c,
            DAT_00732584,
            DAT_0073253c,
            param_1,
            param_2,
            param_3,
        );
    }
}
