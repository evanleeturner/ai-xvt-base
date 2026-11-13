// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732568: i32;
    static mut DAT_00732530: i32;
    static mut DAT_00732548: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00413cc0(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040ca60(param_1: i32, param_2: i32, param_3: i32) {
    // SAFETY: the globals and the callee are the game's own; values are read
    // in the original's order and passed through unchanged.
    unsafe {
        FUN_00413cc0(
            DAT_00732568,
            DAT_00732530,
            DAT_00732548,
            param_1,
            param_2,
            param_3,
        );
    }
}
