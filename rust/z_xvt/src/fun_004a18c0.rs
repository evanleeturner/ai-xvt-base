// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a18c0(param_1: u8, param_2: u8, param_3: u8) -> i32 {
    let a = u32::from(param_1)
        .wrapping_mul(0x3c)
        .wrapping_add(u32::from(param_2));
    let r = u32::from(param_3).wrapping_add(a.wrapping_mul(0x3c));
    r as i32
}
