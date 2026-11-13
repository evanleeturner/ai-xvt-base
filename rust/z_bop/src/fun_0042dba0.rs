// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a10660: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042dda0(param_1: *mut i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042dba0(param_1: *mut i8) -> u8 {
    // SAFETY: the callee and the global table are the game's own; the index
    // comes from the callee and is used as the original does.
    unsafe {
        let i_var1 = FUN_0042dda0(param_1);
        if i_var1 == -1 {
            return 0;
        }
        (&raw mut DAT_00a10660)
            .byte_offset(i_var1.wrapping_mul(0x145) as isize)
            .read()
    }
}
