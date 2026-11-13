// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00412810(param_1: i32, param_2: i32, param_3: i32) -> i32 {
    let mut param_1 = param_1;
    let mut param_2 = param_2;
    let mut param_3 = param_3;
    if param_1 < 0 {
        param_1 = 0u32.wrapping_sub(param_1 as u32) as i32;
    }
    if param_2 < 0 {
        param_2 = 0u32.wrapping_sub(param_2 as u32) as i32;
    }
    if param_3 < 0 {
        param_3 = 0u32.wrapping_sub(param_3 as u32) as i32;
    }
    if param_2 < param_1 && param_3 < param_1 {
        return (param_1 as u32)
            .wrapping_add((param_3 >> 2) as u32)
            .wrapping_add((param_2 >> 2) as u32) as i32;
    }
    if param_1 < param_2 && param_3 < param_2 {
        return ((param_1 >> 2) as u32)
            .wrapping_add((param_3 >> 2) as u32)
            .wrapping_add(param_2 as u32) as i32;
    }
    ((param_1 >> 2) as u32)
        .wrapping_add((param_2 >> 2) as u32)
        .wrapping_add(param_3 as u32) as i32
}
