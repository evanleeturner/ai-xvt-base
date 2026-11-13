// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00413e80(param_1: u8, param_2: i32) -> u16 {
    let addr = (param_2 as u32).wrapping_add((param_1 as u32).wrapping_mul(3));
    let p = addr as *const u8;
    // SAFETY: the caller passes a table address; three bytes are read at
    // the computed entry exactly as the original does.
    unsafe {
        let b0 = p.read();
        let b1 = p.byte_add(1).read();
        let b2 = p.byte_add(2).read();
        let hi: u8 = ((b0 >> 1) as u32).wrapping_shl(3) as u8;
        let lo: u8 = b2 >> 1;
        ((b1 as u16).wrapping_shl(5)) | (((hi as u16).wrapping_shl(8)) | (lo as u16))
    }
}
