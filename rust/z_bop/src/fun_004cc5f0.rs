// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cc5e0(param_1: i32) -> i32;
    fn FUN_004cc5c0(param_1: i32, param_2: i32, param_3: i16) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cc5f0(param_1: i32) -> u32 {
    // SAFETY: the callees are the game's own functions; they are called in the
    // original order with the original arguments.
    unsafe {
        let l_var1 = FUN_004cc5e0(param_1);
        FUN_004cc5c0(param_1, 0, 2);
        let u_var2 = FUN_004cc5e0(param_1) as u32;
        FUN_004cc5c0(param_1, l_var1, 0);
        u_var2
    }
}
