// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0064ef4c: u32;
    static mut DAT_0064ef48: u32;
    static mut _DAT_0064ef44: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b5b84(param_1: u32, param_2: u32, param_3: u32) {
    // SAFETY: the globals are the game's own memory; written in the original's order.
    unsafe {
        DAT_0064ef4c = param_1;
        DAT_0064ef48 = param_2;
        _DAT_0064ef44 = param_3;
    }
}
