// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_007330ac: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00431cf0() -> i32 {
    // SAFETY: the global is the game's own memory, read by value.
    unsafe {
        if DAT_007330ac == 0 {
            return 1;
        }
        DAT_007330ac
    }
}
