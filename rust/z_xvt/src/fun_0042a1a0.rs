// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732a24: i32;
    static mut DAT_00732a28: i32;
    static mut DAT_00732a2c: i32;
    static mut DAT_006d499c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00421c70(param_1: u32, param_2: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042a1a0(param_1: u32, param_2: u32) {
    // SAFETY: the callee and the globals are the game's own memory,
    // read and written in the original's order.
    unsafe {
        FUN_00421c70(param_1, 0);
        let mut i_var2 = DAT_00732a2c;
        let mut i_var1 = DAT_00732a28;
        let mut i_var3 = DAT_00732a24;
        FUN_00421c70(param_2, 0);
        i_var2 = i_var2.wrapping_sub(DAT_00732a2c);
        i_var1 = i_var1.wrapping_sub(DAT_00732a28);
        i_var3 = i_var3.wrapping_sub(DAT_00732a24);
        if i_var2 < 0 {
            i_var2 = i_var2.wrapping_neg();
        }
        if i_var1 < 0 {
            i_var1 = i_var1.wrapping_neg();
        }
        if i_var3 < 0 {
            i_var3 = i_var3.wrapping_neg();
        }
        if i_var1 < i_var2 {
            i_var2 = (i_var1 >> 1).wrapping_add(i_var2);
        } else {
            i_var2 = (i_var2 >> 1).wrapping_add(i_var1);
        }
        if i_var3 < i_var2 {
            DAT_006d499c = i_var2.wrapping_add(i_var3 >> 1);
            return;
        }
        DAT_006d499c = (i_var2 >> 1).wrapping_add(i_var3);
    }
}
