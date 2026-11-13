// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa73b0: i8;
    static mut DAT_00aa73b1: i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d5fc0() -> i8 {
    // SAFETY: the globals are the game's own memory, read by value.
    unsafe {
        let mut c_var1: i8 = (DAT_00aa73b0 != 0) as i8;
        if DAT_00aa73b1 != 0 {
            c_var1 = c_var1.wrapping_add(1);
        }
        c_var1
    }
}
