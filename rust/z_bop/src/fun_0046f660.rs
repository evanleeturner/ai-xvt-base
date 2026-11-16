// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00999570: i32;
    static mut DAT_009996b0: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0046f660(param_1: i32) -> i32 {
    let mut i_var1: i32 = 0;
    // SAFETY: the globals are the game's own memory; only their addresses
    // are taken and the table is read as the original does.
    unsafe {
        let mut pi_var2: *mut i32 = &raw mut DAT_00999570;
        let end: *mut i32 = &raw mut DAT_009996b0;
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
