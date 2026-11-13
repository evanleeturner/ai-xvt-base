// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005280f0: i32;
    static mut DAT_005280f4: i32;
    static mut DAT_005280f8: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004ac830() -> i16;
    fn FUN_004acaf0() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004acac0() -> i32 {
    // SAFETY: the globals and callees are the game's own; accessed in the original's order.
    unsafe {
        if DAT_005280f0 == 0 {
            let s_var1 = FUN_004ac830();
            DAT_005280f4 = s_var1 as i32;
            DAT_005280f8 = FUN_004acaf0();
            DAT_005280f0 = 1;
        }
        DAT_005280f8
    }
}
