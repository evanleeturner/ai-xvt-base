// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b0f: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d4c00(param_1: u32) {
    // SAFETY: the global is the game's own memory; its address is odd, so the
    // store is done unaligned.
    unsafe {
        (&raw mut DAT_00aa7b0f).write_unaligned(param_1);
    }
}
