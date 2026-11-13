// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052accc: u32;
    static mut DAT_0052acc8: u32;
    static mut DAT_0052acc4: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0049c1f0() {
    // SAFETY: the globals are the game's own memory, read and written
    // where the original does.
    unsafe {
        DAT_0052accc = 0;
        DAT_0052acc8 = DAT_0052acc4;
    }
}
