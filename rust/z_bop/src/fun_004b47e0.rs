// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b47e0(param_1: u32) -> u8 {
    let mut b_var1: u8 = ((param_1 & 0x4000) != 0) as u8;
    if (param_1 & 0x2000) != 0 {
        b_var1 |= 2;
    }
    if (param_1 & 0x1000) != 0 {
        b_var1 |= 4;
    }
    if (param_1 & 0x800) != 0 {
        b_var1 |= 8;
    }
    if (param_1 & 0x400) != 0 {
        b_var1 |= 0x10;
    }
    if (param_1 & 0x200) != 0 {
        b_var1 |= 0x20;
    }
    if (param_1 & 0x100) != 0 {
        b_var1 |= 0x40;
    }
    b_var1
}
