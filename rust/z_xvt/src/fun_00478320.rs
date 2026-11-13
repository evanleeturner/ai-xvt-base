// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00478320(param_1: u32, param_2: u16) -> u32 {
    let mut u_var1: u32 = param_1 & 0xffff;
    if param_2 != 0xffff {
        u_var1 = (u32::from(param_2).wrapping_mul(u_var1).wrapping_add(0x8000)) >> 0x10;
    }
    u_var1
}
