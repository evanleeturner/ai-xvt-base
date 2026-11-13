// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00413cc0(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
) -> i32 {
    let local_a: i64 = (param_3 as i64).wrapping_mul(param_6 as i64);
    let local_b: i64 = (param_1 as i64).wrapping_mul(param_4 as i64);
    let local_c: i64 = (param_2 as i64).wrapping_mul(param_5 as i64);

    let ua: u32 = ((local_a as u32) >> 0xf) | (((local_a as u64 >> 0x20) as u32) << 0x11);
    let ub: u32 = ((local_b as u32) >> 0xf) | (((local_b as u64 >> 0x20) as u32) << 0x11);
    let uc: u32 = ((local_c as u32) >> 0xf) | (((local_c as u64 >> 0x20) as u32) << 0x11);

    ua.wrapping_add(ub.wrapping_add(uc)) as i32
}
