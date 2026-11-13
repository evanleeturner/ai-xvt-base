// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0099940a: u16;
    static mut DAT_00999404: u16;
    static mut DAT_00999408: u16;
    static mut DAT_00999402: u16;
    static mut DAT_00999400: u16;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004ac740();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0049cba0() {
    // SAFETY: the globals and the callee are the game's own memory and code;
    // they are read and called in the original's order.
    unsafe {
        if DAT_0099940a != 0 {
            FUN_004ac740();
        }
        if DAT_00999404 != 0 {
            FUN_004ac740();
        }
        if DAT_00999408 != 0 {
            FUN_004ac740();
        }
        if DAT_00999402 != 0 {
            FUN_004ac740();
        }
        if DAT_00999400 != 0 {
            FUN_004ac740();
        }
        FUN_004ac740();
        FUN_004ac740();
        FUN_004ac740();
        FUN_004ac740();
        FUN_004ac740();
        FUN_004ac740();
        FUN_004ac740();
        FUN_004ac740();
    }
}
