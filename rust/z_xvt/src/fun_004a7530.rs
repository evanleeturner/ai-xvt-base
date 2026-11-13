// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052b050: i32;
    static mut DAT_00645be8: i32;
    static mut DAT_00646868: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004a7530() -> u32 {
    // SAFETY: the globals are the game's own memory; the pointer walks the
    // game's table in 8-int strides, read only, as the original does.
    unsafe {
        if DAT_0052b050 == 0 {
            return 0;
        }
        let end = &raw mut DAT_00646868;
        let mut p = &raw mut DAT_00645be8;
        loop {
            if p.read() != 0 {
                return 1;
            }
            p = p.wrapping_add(8);
            if !(p < end) {
                break;
            }
        }
        0
    }
}
