// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005373d0: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00431110(param_1: u32, param_2: *mut u32, param_3: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0042c560() {
    // SAFETY: the global and the callee are the game's own; the global's
    // address is passed to the callee as the original does.
    unsafe {
        DAT_005373d0 = 0x1d;
        FUN_00431110(0, &raw mut DAT_005373d0, 4);
    }
}
