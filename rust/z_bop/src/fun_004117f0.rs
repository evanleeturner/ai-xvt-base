// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005505f0: i32;
    static mut DAT_005505ec: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004117f0() {
    // SAFETY: the globals are the game's own memory, written where the original does.
    unsafe {
        DAT_005505f0 = -1;
        DAT_005505ec = 0;
    }
}
