// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0064ee24: u32;
    static mut DAT_0064ee20: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b5ba7(param_1: u32, param_2: u32) {
    // SAFETY: the globals are the game's own memory, written where the original does.
    unsafe {
        DAT_0064ee24 = param_1;
        DAT_0064ee20 = param_2;
    }
}
