// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00645ad4: u16;
    static mut DAT_00645a80: u32;
    static mut DAT_00645a8c: u16;
    static mut DAT_00645778: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048f4b0(param_1: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00499040() {
    // SAFETY: the callee and the globals are the game's own memory,
    // read and written in the original's order.
    unsafe {
        FUN_0048f4b0(u32::from(DAT_00645ad4));
        DAT_00645ad4 = 0;
        DAT_00645a80 = 0;
        FUN_0048f4b0(u32::from(DAT_00645a8c));
        DAT_00645a8c = 0;
        DAT_00645778 = 0;
    }
}
