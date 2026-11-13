// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732558: i32;
    static mut DAT_00732578: i32;
    static mut DAT_00732538: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00413cc0(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040ca30(param_1: i32, param_2: i32, param_3: i32) {
    // SAFETY: the globals and the callee are the game's own; values are read
    // in the original's order and passed straight through.
    unsafe {
        FUN_00413cc0(
            DAT_00732558,
            DAT_00732578,
            DAT_00732538,
            param_1,
            param_2,
            param_3,
        );
    }
}
