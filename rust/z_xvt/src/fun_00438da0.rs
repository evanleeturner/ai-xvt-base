// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525518: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_00462410();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00438da0() {
    // SAFETY: the global and the callee are the game's own.
    unsafe {
        if DAT_00525518 != 1 {
            FUN_00462410();
        }
    }
}
