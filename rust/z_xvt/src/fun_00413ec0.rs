// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00413ec0(param_1: u8, param_2: i32) -> u16 {
    let addr = (param_2 as u32).wrapping_add((param_1 as u32).wrapping_mul(3));
    let p = addr as *const u8;
    // SAFETY: the caller passes a table address; the three bytes at
    // param_2 + param_1 * 3 are read exactly as the original does.
    let (b0, b1, b2) = unsafe { (p.read(), p.byte_add(1).read(), p.byte_add(2).read()) };
    let hi: u8 = (b2 >> 1).wrapping_shl(2);
    let lo: u8 = b1 >> 1;
    (((b0 >> 1) as u16).wrapping_shl(5)) | (((hi as u16).wrapping_shl(8)) | (lo as u16))
}
