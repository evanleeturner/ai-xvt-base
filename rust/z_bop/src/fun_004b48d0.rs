// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b48d0(param_1: u32) -> u8 {
    let mut b_var1: u8 = ((param_1 & 1) != 0) as u8;
    if (param_1 & 4) != 0 {
        b_var1 = 3;
    }
    if (param_1 & 2) != 0 {
        b_var1 |= 2;
    }
    if (param_1 & 8) != 0 {
        b_var1 |= 4;
    }
    if (param_1 & 0x10) != 0 {
        b_var1 |= 5;
    }
    if (param_1 & 0x20) != 0 {
        b_var1 |= 6;
    }
    if (param_1 & 0x40) != 0 {
        b_var1 = 7;
    }
    if (param_1 & 0x80) != 0 {
        b_var1 |= 8;
    }
    b_var1
}
