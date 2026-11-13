// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758d59: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004ddce0() {
    // SAFETY: the global is the game's own memory; the original writes 1 to it.
    unsafe {
        DAT_00758d59 = 1;
    }
}
