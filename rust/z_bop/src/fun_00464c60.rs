// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005242ec: i32;
    static mut DAT_005242e4: i32;
    static mut DAT_005242e8: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00464c60() {
    // SAFETY: the globals are the game's own memory, written where the original does.
    unsafe {
        DAT_005242ec = 1;
        DAT_005242e4 = 0;
        DAT_005242e8 = 0;
    }
}
