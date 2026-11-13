// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00443ee0(param_1: u32) -> i32 {
    let i_var1: i32 = ((param_1 & 0xffff) as i32).wrapping_sub(0x8f);
    let u_var2: u16 = ((i_var1 as u32) >> 0x10) as u16;
    let local_2: u16 = i_var1 as u16;
    (((u_var2 as u32) << 0x10) | (local_2 as u32)) as i32
}
