// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7209: i32;
    static mut DAT_00aa720d: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn SetCursorPos(x: i32, y: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004de0b0(param_1: i32, param_2: i32) {
    let mut param_1 = param_1;
    let mut param_2 = param_2;
    if param_1 < 0x281 {
        if param_1 < 0 {
            param_1 = 0;
        }
    } else {
        param_1 = 0x280;
    }
    if param_2 < 0x1e1 {
        if param_2 < 0 {
            param_2 = 0;
        }
    } else {
        param_2 = 0x1e0;
    }
    // SAFETY: the globals and SetCursorPos are the game's own / the system's.
    unsafe {
        DAT_00aa7209 = param_1;
        DAT_00aa720d = param_2;
        SetCursorPos(param_1, param_2);
    }
}
