// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a10660: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042dda0(param_1: *mut i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042db50(param_1: *mut i8, param_2: i32) -> i32 {
    // SAFETY: the callee and the global table are the game's own memory; the
    // index returned by the callee addresses an entry in that table.
    unsafe {
        let i_var1 = FUN_0042dda0(param_1);
        if i_var1 == -1 {
            return 0;
        }
        let mut param_2 = param_2;
        if 0xff < param_2 {
            param_2 = 0xff;
        }
        if param_2 < 0 {
            param_2 = 0;
        }
        (&raw mut DAT_00a10660)
            .byte_offset(i_var1.wrapping_mul(0x145) as isize)
            .write(param_2 as i8);
        1
    }
}
