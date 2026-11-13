// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006da538: u32;
    static mut DAT_006db594: u32;
    static mut DAT_00732e8c: i16;
    static mut DAT_00732a34: i16;
    static mut DAT_006d9980: i16;
    static mut DAT_0070acc0: i16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0048ed00(param_1: i16, param_2: i16, param_3: i16, param_4: i16) {
    let mut param_1 = param_1;
    let mut param_2 = param_2;
    let mut param_3 = param_3;
    let mut param_4 = param_4;
    if param_2 < 0 {
        param_2 = 0;
    }
    if param_1 < 0 {
        param_1 = 0;
    }
    // SAFETY: the globals are the game's own memory, read and written by value.
    unsafe {
        if DAT_006da538 < (param_3 as i32) as u32 {
            param_3 = DAT_006da538 as i16;
        }
        if DAT_006db594 < (param_4 as i32) as u32 {
            param_4 = DAT_006db594 as i16;
        }
        DAT_00732e8c = param_2;
        DAT_00732a34 = param_4;
        DAT_006d9980 = param_1;
        DAT_0070acc0 = param_3;
    }
}
