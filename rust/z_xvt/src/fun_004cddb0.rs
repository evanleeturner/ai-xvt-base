// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_007806fa: i32;
    static mut DAT_00759645: i8;
    static mut DAT_00758830: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d4d90();
    fn FUN_004cdc60(param_1: i32);
    fn FUN_004d4d00() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cddb0(param_1: i32) {
    // SAFETY: the globals and callees are the game's own; they are read,
    // written and called in the original's order.
    unsafe {
        if DAT_007806fa != 0 {
            FUN_004d4d90();
            FUN_004cdc60(param_1);
            if DAT_00759645 != 0 {
                DAT_00758830 = FUN_004d4d00();
            }
        }
    }
}
