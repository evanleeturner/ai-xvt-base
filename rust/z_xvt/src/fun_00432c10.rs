// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00733120: i32;
    static mut DAT_00733260: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00432c10(param_1: i32) -> i32 {
    let mut i_var1: i32 = 0;
    // SAFETY: the globals are the game's own memory; the table is walked
    // in 10-int strides until the end address, as the original does.
    unsafe {
        let mut pi_var2: *mut i32 = &raw mut DAT_00733120;
        let end: *mut i32 = &raw mut DAT_00733260;
        loop {
            if pi_var2.read() == param_1 {
                return i_var1;
            }
            pi_var2 = pi_var2.wrapping_add(10);
            i_var1 = i_var1.wrapping_add(1);
            if pi_var2 >= end {
                break;
            }
        }
    }
    i_var1
}
