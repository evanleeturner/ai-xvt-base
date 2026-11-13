// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732a24: i32;
    static mut DAT_00732a28: i32;
    static mut DAT_00732a2c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00421c70(param_1: u32, param_2: u32);
    fn FUN_00418cd0(param_1: i32, param_2: i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042a140(param_1: u32, param_2: u32) {
    // SAFETY: the callees and globals are the game's own; globals are read
    // in the same order as the original, around the two calls.
    unsafe {
        FUN_00421c70(param_2, 0);
        let i_var3 = DAT_00732a2c;
        let i_var2 = DAT_00732a28;
        let i_var1 = DAT_00732a24;
        FUN_00421c70(param_1, 0);
        FUN_00418cd0(
            i_var3.wrapping_sub(DAT_00732a2c),
            i_var2.wrapping_sub(DAT_00732a28),
            i_var1.wrapping_sub(DAT_00732a24),
        );
    }
}
