// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_00a90b10: u32;
    static mut _DAT_00a90b14: u32;
    static mut _DAT_00a90b18: u32;
    static mut DAT_00a90b1c: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b15b0(param_1: u32, param_2: u32, param_3: u32, param_4: u32) {
    // SAFETY: the globals are the game's own memory, written where the original does.
    unsafe {
        _DAT_00a90b10 = param_1;
        _DAT_00a90b14 = param_2;
        _DAT_00a90b18 = param_3;
        DAT_00a90b1c = param_4;
    }
}
