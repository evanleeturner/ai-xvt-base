// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b15e0(param_1: i32) -> i32 {
    let mut param_1 = param_1;
    let mut i_var1: i32 = 0;
    while 1 < param_1 {
        i_var1 = i_var1.wrapping_add(1);
        param_1 >>= 1;
    }
    i_var1
}
