// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b5251(param_1: i32) -> i32 {
    let mut param_1 = param_1;
    let mut local_8: i32 = 0;
    while 1 < param_1 {
        local_8 = local_8.wrapping_add(1);
        param_1 >>= 1;
    }
    local_8
}
