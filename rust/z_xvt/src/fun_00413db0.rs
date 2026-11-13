// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00413db0(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
) -> i32 {
    let product = (param_1 as u32).wrapping_mul(param_2 as u32) as i32;
    let shifted = ((product >> 0xf) as u32).wrapping_mul(param_3 as u32);
    let sum = shifted.wrapping_add((param_4 as u32).wrapping_mul(0x8000));
    let mut i_var1 = sum as i32;
    if 0x3fffffff < i_var1 {
        i_var1 = 0x3fffffff;
    }
    if i_var1 < -0x3fffffff {
        i_var1 = -0x3fff0000;
    }
    i_var1 >> 0xf
}
