// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d9280(
    param_1: i32,
    param_2: u32,
    param_3: i32,
    param_4: i32,
) -> i32 {
    (param_2.wrapping_mul(0x10000) as i32)
        .wrapping_add((param_2 >> 0x10) as i32)
        .wrapping_add(param_4)
        .wrapping_add(param_3)
        .wrapping_add(param_1)
}
