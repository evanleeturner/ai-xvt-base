// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0064f89c: u32;
    static mut DAT_0075dc46: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004db4e0() -> u32 {
    // SAFETY: the globals are the game's own memory; read and written where the original does.
    unsafe {
        DAT_0064f89c = DAT_0075dc46;
        DAT_0075dc46 = 0;
    }
    1
}
