// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005f0e24: u32;
    static mut DAT_005f0e28: u32;
    static mut DAT_005f0e2c: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00453aa0(param_1: u32, param_2: u32, param_3: u32) {
    // SAFETY: the globals are the game's own memory, written where the original does.
    unsafe {
        DAT_005f0e24 = param_1;
        DAT_005f0e28 = param_2;
        DAT_005f0e2c = param_3;
    }
}
