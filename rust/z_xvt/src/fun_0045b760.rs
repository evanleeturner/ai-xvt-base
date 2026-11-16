// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525730: u32;
    static mut DAT_005254f8: u32;
    static mut DAT_00703c88: u32;
    static mut DAT_00732a38: u32;
    static mut DAT_006d4988: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0045b420(param_1: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0045b760(param_1: u32) {
    // SAFETY: the globals and the callee are the game's own memory and code,
    // accessed in the same order as the original.
    unsafe {
        DAT_00525730 = 0;
        if DAT_005254f8 != 0 && param_1 >= DAT_00703c88.wrapping_add(DAT_00732a38) {
            FUN_0045b420(param_1.wrapping_mul(0x23).wrapping_add(DAT_006d4988));
        }
    }
}
