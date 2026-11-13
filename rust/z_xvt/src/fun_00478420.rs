// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00478420(param_1: u32, param_2: u32) -> u32 {
    let mut param_1 = param_1;
    let mut param_2 = param_2;
    if param_2 == param_1 {
        return 0xffff;
    }
    if param_2 == 0 {
        return 0xffff;
    }
    if param_2 <= param_1 {
        return 0xffff;
    }
    while param_1 > 0xffff || param_2 > 0xffff {
        param_1 >>= 1;
        param_2 >>= 1;
    }
    param_1.wrapping_shl(0x10) / param_2
}
