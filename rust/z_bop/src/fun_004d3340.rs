// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7fe7: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d3340() -> u32 {
    // SAFETY: the global is the game's own memory; written where the original does.
    unsafe {
        DAT_00aa7fe7 = 0;
    }
    1
}
