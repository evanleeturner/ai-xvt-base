// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523c2c: i32;
    static mut DAT_00523c34: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00414520() {
    // SAFETY: the globals are the game's own memory, written where the original does.
    unsafe {
        DAT_00523c2c = 0;
        DAT_00523c34 = 0;
    }
}
