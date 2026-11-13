// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00536714: u8;
    static mut DAT_00536708: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0051c100(param_1: u8, param_2: u8) {
    // SAFETY: the globals are the game's own memory, written where the original does.
    unsafe {
        DAT_00536714 = param_1;
        DAT_00536708 = param_2;
    }
}
