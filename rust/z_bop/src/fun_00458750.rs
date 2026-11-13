// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00458750(param_1: i32) {
    let mut u_var1: u16 = 0;
    loop {
        let u_var2: u16 = u_var1.wrapping_add(1);
        // SAFETY: param_1 is the address of the caller's structure; the
        // original zeroes sixteen 4-byte slots starting at offset 0x41e.
        unsafe {
            ((param_1
                .wrapping_add(0x41e)
                .wrapping_add((u_var1 as u32).wrapping_mul(4) as i32)) as usize
                as *mut i32)
                .write_unaligned(0);
        }
        u_var1 = u_var2;
        if u_var2 >= 0x10 {
            break;
        }
    }
}
