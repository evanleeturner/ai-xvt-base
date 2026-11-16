// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0099f6e0: i32;
    static mut DAT_00999be8: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00470020(param_1: i32) {
    // SAFETY: the globals are the game's own memory; the walk reads the
    // game's table at the same stride and in the same order as the original.
    unsafe {
        let mut u_var2: i32 = 0;
        if DAT_0099f6e0 != 0 {
            let mut pi_var1: *mut i32 = &raw mut DAT_00999be8;
            while pi_var1.read_unaligned() != param_1 {
                pi_var1 = pi_var1.wrapping_add(0x8e);
                u_var2 = u_var2.wrapping_add(1);
                if DAT_0099f6e0 <= u_var2 {
                    return;
                }
            }
        }
    }
}
