// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7a98: u32;
    static mut DAT_00aa7a9c: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004dcb50() -> u32 {
    // SAFETY: the globals are the game's own memory, read and written
    // where the original does.
    unsafe {
        if DAT_00aa7a98 == DAT_00aa7a9c {
            return 0;
        }
        DAT_00aa7a9c = DAT_00aa7a9c.wrapping_add(1);
        if DAT_00aa7a9c == 0x400 {
            DAT_00aa7a9c = 0;
        }
        1
    }
}
