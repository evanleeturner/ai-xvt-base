// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00655114: i16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0051c130(param_1: i16) {
    // SAFETY: the global is the game's own memory, read and written by value.
    unsafe {
        if DAT_00655114 != 5 {
            DAT_00655114 = param_1;
        }
    }
}
