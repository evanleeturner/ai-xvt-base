// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa73b0: u8;
    static mut DAT_00aa73f7: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d6210(param_1: i32) -> u8 {
    if param_1 >= 2 {
        return 0;
    }
    // SAFETY: the globals are the game's own arrays; the index is used
    // exactly as the original does.
    unsafe {
        if (&raw mut DAT_00aa73b0).offset(param_1 as isize).read() != 0 {
            return 0;
        }
        (&raw mut DAT_00aa73f7)
            .offset(param_1.wrapping_mul(0x20) as isize)
            .read()
    }
}
