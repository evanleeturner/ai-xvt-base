// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0064f680: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004da050() {
    // SAFETY: the global is the game's own memory; it is written where the original does.
    unsafe {
        DAT_0064f680 = 0;
    }
}
