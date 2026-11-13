// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004127d0(param_1: u32, param_2: u32, param_3: u32) -> i32 {
    if param_2 < param_1 && param_3 < param_1 {
        return param_1.wrapping_add((param_3 >> 2).wrapping_add(param_2 >> 2)) as i32;
    }
    if param_1 < param_2 && param_3 < param_2 {
        return (param_3 >> 2)
            .wrapping_add(param_1 >> 2)
            .wrapping_add(param_2) as i32;
    }
    (param_2 >> 2)
        .wrapping_add(param_1 >> 2)
        .wrapping_add(param_3) as i32
}
