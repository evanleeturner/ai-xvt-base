// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a004a8: i32;
    static mut DAT_00555c90: i32;
    static mut DAT_00a004a4: i32;
    static mut DAT_00555c94: i32;
    static mut DAT_00a004b0: i32;
    static mut DAT_00555ca0: i32;
    static mut DAT_00a004cc: i32;
    static mut DAT_00555c98: i32;
    static mut DAT_00a004b8: i32;
    static mut DAT_00555c9c: i32;
    static mut DAT_009fe7f4: i32;
    static mut DAT_00555cac: i32;
    static mut DAT_00a00488: i32;
    static mut DAT_00555ca4: i32;
    static mut DAT_00a00480: i32;
    static mut DAT_00555ca8: i32;
    static mut DAT_009d682c: u16;
    static mut DAT_00555cbc: u16;
    static mut DAT_00a00490: i32;
    static mut DAT_00555cb0: i32;
    static mut DAT_009cd272: u16;
    static mut DAT_00a080f4: u16;
    static mut DAT_009a1fe4: u16;
    static mut DAT_00555cb8: u16;
    static mut DAT_009d1260: u16;
    static mut DAT_009a20a2: u16;
    static mut _DAT_009ed234: u32;
    static mut DAT_00555cb4: u16;
    static mut DAT_0052747c: u16;
    static mut DAT_009a7b60: u32;
    static mut DAT_00555c8c: u16;
    static mut DAT_009a8e3c: u32;
    static mut DAT_00555c88: u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00427070() {
    // SAFETY: all globals are the game's own memory, read and written
    // exactly as the original does.
    unsafe {
        DAT_00a004a8 = DAT_00555c90;
        DAT_00a004a4 = DAT_00555c94;
        DAT_00a004b0 = DAT_00555ca0;
        DAT_00a004cc = DAT_00555c98;
        DAT_00a004b8 = DAT_00555c9c;
        DAT_009fe7f4 = DAT_00555cac;
        DAT_00a00488 = DAT_00555ca4;
        DAT_00a00480 = DAT_00555ca8;
        DAT_009d682c = DAT_00555cbc;
        DAT_00a00490 = DAT_00555cb0;
        DAT_009cd272 = DAT_00555cbc.wrapping_sub(1);
        DAT_00a080f4 = DAT_00555cbc >> 1;
        DAT_009a1fe4 = DAT_00555cb8;
        DAT_009d1260 = DAT_00555cb8.wrapping_sub(1);
        DAT_009a20a2 = DAT_00555cb8 >> 1;
        _DAT_009ed234 = DAT_00555cb4 as u32;
        DAT_0052747c = 0xc000;
        DAT_009a7b60 = DAT_00555c8c as u32;
        DAT_009a8e3c = DAT_00555c88 as u32;
    }
}
