// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00478350(param_1: u32, param_2: u16) -> u32 {
    if param_2 == 0xffff {
        return param_1;
    }
    let m = u32::from(param_2);
    ((param_1 & 0xffff).wrapping_mul(m) >> 0x10).wrapping_add((param_1 >> 0x10).wrapping_mul(m))
}
