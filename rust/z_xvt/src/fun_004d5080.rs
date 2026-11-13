// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759646: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d5080() {
    // SAFETY: DAT_00759646 is the game's own global byte.
    unsafe {
        DAT_00759646 = 0;
    }
}
