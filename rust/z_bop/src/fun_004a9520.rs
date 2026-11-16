// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a7bac: i32;
    static mut DAT_009a8c14: i32;
    static mut DAT_00a0813c: i16;
    static mut DAT_00a07ce4: i16;
    static mut DAT_009a6fe0: i16;
    static mut DAT_009d8c00: i16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a9520(param_1: i16, param_2: i16, param_3: i16, param_4: i16) {
    let mut param_1 = param_1;
    let mut param_2 = param_2;
    let mut param_3 = param_3;
    let mut param_4 = param_4;
    // SAFETY: the globals are the game's own memory, read and written
    // where the original does.
    unsafe {
        if param_2 < 0 {
            param_2 = 0;
        }
        if param_1 < 0 {
            param_1 = 0;
        }
        if (DAT_009a7bac as u32) < (param_3 as i32 as u32) {
            param_3 = DAT_009a7bac as i16;
        }
        if (DAT_009a8c14 as u32) < (param_4 as i32 as u32) {
            param_4 = DAT_009a8c14 as i16;
        }
        DAT_00a0813c = param_2;
        DAT_00a07ce4 = param_4;
        DAT_009a6fe0 = param_1;
        DAT_009d8c00 = param_3;
    }
}
