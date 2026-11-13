// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732e52: u16;
    static mut DAT_00732e58: u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0048ece0(param_1: u16, param_2: u16) {
    // SAFETY: the globals are the game's own memory, written where the original does.
    unsafe {
        DAT_00732e52 = param_2;
        DAT_00732e58 = param_1;
    }
}
