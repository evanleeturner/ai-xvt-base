// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b69d20: [i8; 1];
    static mut DAT_0052b5c0: [i8; 1];
    static mut DAT_0052b5b0: [i8; 1];
}

unsafe extern "C" {
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004c9bf0(param_1: u32) {
    // SAFETY: the globals are the game's own buffer and format strings;
    // sprintf is the C runtime's, called as the original calls it.
    unsafe {
        if param_1 / 0xe10 == 0 {
            sprintf(
                (&raw mut DAT_00b69d20).cast::<i8>(),
                (&raw const DAT_0052b5c0).cast::<i8>(),
            );
            return;
        }
        sprintf(
            (&raw mut DAT_00b69d20).cast::<i8>(),
            (&raw const DAT_0052b5b0).cast::<i8>(),
            param_1 / 0xe10,
            (((param_1 as u64) / 0x3c) % 0x3c) as i32,
            param_1 % 0x3c,
        );
    }
}
