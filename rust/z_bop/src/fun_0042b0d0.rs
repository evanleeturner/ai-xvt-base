// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005233a0: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042b0d0(param_1: u32) {
    // SAFETY: the global is the game's own memory, written where the original does.
    unsafe {
        DAT_005233a0 = param_1;
    }
}
