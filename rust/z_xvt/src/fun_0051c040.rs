// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0051c040(param_1: i32, param_2: i32, param_3: u32) {
    if param_3 != 0 {
        let mut u_var1: u32 = 0;
        loop {
            let u_var2: u32 = (u_var1.wrapping_add(1) as u16) as u32;
            let off = u_var1.wrapping_mul(2);
            let dst = off.wrapping_add(param_1 as u32) as *mut u16;
            let src = off.wrapping_add(param_2 as u32) as *const u16;
            // SAFETY: the caller supplies buffers of at least param_3 u16
            // elements; copied where the original does.
            unsafe {
                dst.write_unaligned(src.read_unaligned());
            }
            u_var1 = u_var2;
            if !(u_var2 < param_3) {
                break;
            }
        }
    }
}
