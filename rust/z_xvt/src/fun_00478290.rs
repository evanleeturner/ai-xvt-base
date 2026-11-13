// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00478290(param_1: u32, param_2: u32, param_3: u32) -> i32 {
    let mut param_1 = param_1;
    let mut param_2 = param_2;
    let mut param_3 = param_3;
    let mut b_var3 = (param_1 as i32) < 0;
    if (param_1 as i32) < 0 {
        param_1 = 0u32.wrapping_sub(param_1);
    }
    if (param_2 as i32) < 0 {
        param_2 = 0u32.wrapping_sub(param_2);
        b_var3 = !b_var3;
    }
    if (param_3 as i32) < 0 {
        param_3 = 0u32.wrapping_sub(param_3);
        b_var3 = !b_var3;
    }
    let prod = (param_1 as u64).wrapping_mul(param_2 as u64);
    let u_var2 = (prod >> 0x20) as u32;
    let i_var1: i32 = if u_var2 < param_3 {
        // param_3 is nonzero here because u_var2 < param_3.
        ((((u_var2 as u64) << 0x20) | (prod & 0xffff_ffff)) / (param_3 as u64)) as i32
    } else {
        0x7fff_ffff
    };
    if b_var3 {
        0u32.wrapping_sub(i_var1 as u32) as i32
    } else {
        i_var1
    }
}
