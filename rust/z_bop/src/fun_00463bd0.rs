// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_00557360: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0046d300(param_1: *mut u32, param_2: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00463bd0() {
    // SAFETY: the global and the callee are the game's own; the pointer
    // passed is the address of that global.
    unsafe {
        _DAT_00557360 = 8;
        FUN_0046d300(&raw mut _DAT_00557360, 4);
    }
}
