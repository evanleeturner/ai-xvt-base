// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0066925a: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004f7380() -> i16;
    fn FUN_004f7590(param_2: i16) -> i16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004f7420(param_1: i16, param_2: i16) -> i32 {
    // SAFETY: the global and the callees are the game's own; they are read
    // and called in the same order as the original.
    unsafe {
        if (DAT_0066925a as i32).wrapping_sub(param_1 as i32) == 1 {
            return 0;
        }
        let mut u_var1: i16;
        if DAT_0066925a <= param_1 {
            u_var1 = FUN_004f7590(param_2);
        } else {
            u_var1 = FUN_004f7380();
        }
        while DAT_0066925a <= param_1 {
            u_var1 = FUN_004f7590(param_2);
        }
        (u_var1 as i32) | 0x10000
    }
}
