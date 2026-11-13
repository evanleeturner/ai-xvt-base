// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00413d00(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
) -> i32 {
    let mut i_var1: i32 = (param_1 as u32)
        .wrapping_mul(param_4 as u32)
        .wrapping_add((param_2 as u32).wrapping_mul(param_5 as u32))
        .wrapping_add((param_3 as u32).wrapping_mul(param_6 as u32))
        as i32;
    if 0x3fffffff < i_var1 {
        i_var1 = 0x3fffffff;
    }
    if i_var1 < -0x3fffffff {
        i_var1 = -0x3fff0000;
    }
    i_var1 >> 0xf
}
