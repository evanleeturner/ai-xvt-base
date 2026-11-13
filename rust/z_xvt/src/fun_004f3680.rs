// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0065047a: i16;
    static mut DAT_00650478: i16;
    static mut DAT_00650438: i32;
    static mut DAT_0065043c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004f2e40(param_1: i32);
    fn FUN_004f2c10();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004f3680() {
    // SAFETY: the globals and callees are the game's own memory and code,
    // accessed in the same order as the original.
    unsafe {
        if DAT_0065047a < DAT_00650478 {
            FUN_004f2e40(0);
            return;
        }
        DAT_00650438 = 0;
        DAT_0065043c = 0;
        FUN_004f2c10();
    }
}
