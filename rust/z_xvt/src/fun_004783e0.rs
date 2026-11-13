// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004783e0(param_1: u16, param_2: u16) -> u32 {
    if param_2 == param_1 {
        return 0xFFFF;
    }
    if param_2 == 0 {
        return 0;
    }
    if param_1 < param_2 {
        return 0xFFFF;
    }
    ((param_1 as u32) << 16) / (param_2 as u32)
}
