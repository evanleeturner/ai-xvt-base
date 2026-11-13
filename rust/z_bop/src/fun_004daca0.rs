// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00665574: u32;
    static mut DAT_00b6a264: u32;
    static mut DAT_00b69e24: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d64a0(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_004db640(param_1: i32, param_2: *mut i8, param_3: *mut i32, param_4: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004daca0(param_1: *mut i32, param_2: *mut i8) {
    // SAFETY: the callees and globals are the game's own; the pointers are
    // passed through unchanged as the original does.
    unsafe {
        let u_var1: u32;
        if DAT_00665574 == 0 {
            FUN_004d64a0(param_1, 2, 2);
            FUN_004db640(10, param_2, param_1, DAT_00b6a264);
            FUN_004d64a0(param_1, -2, -2);
            u_var1 = DAT_00b69e24;
        } else {
            FUN_004d64a0(param_1, 2, 2);
            FUN_004db640(10, param_2, param_1, DAT_00b69e24);
            FUN_004d64a0(param_1, -2, -2);
            u_var1 = DAT_00b6a264;
        }
        FUN_004db640(10, param_2, param_1, u_var1);
        DAT_00665574 = 0;
    }
}
