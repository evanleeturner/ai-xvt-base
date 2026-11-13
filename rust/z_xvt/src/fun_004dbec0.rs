// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_007595d0: i32;
    static mut DAT_007595d4: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004dbec0() -> u32 {
    // SAFETY: the globals are the game's own memory, read and written
    // where the original does.
    unsafe {
        if DAT_007595d0 == DAT_007595d4 {
            return 0;
        }
        DAT_007595d4 = (DAT_007595d4 as u32).wrapping_add(1) as i32;
        if DAT_007595d4 == 0x400 {
            DAT_007595d4 = 0;
        }
        1
    }
}
