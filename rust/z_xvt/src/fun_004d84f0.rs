// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d84f0(
    param_1: i32,
    param_2: u32,
    param_3: i32,
    param_4: i32,
) -> i32 {
    let mut result: u32 = param_2.wrapping_mul(0x10000);
    result = result.wrapping_add(param_2 >> 0x10);
    result = result.wrapping_add(param_4 as u32);
    result = result.wrapping_add(param_3 as u32);
    result = result.wrapping_add(param_1 as u32);
    result as i32
}
