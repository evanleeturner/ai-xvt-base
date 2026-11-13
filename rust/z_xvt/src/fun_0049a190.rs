// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0049a190(param_1: *mut u32, param_2: *mut u8, param_3: i32) -> u32 {
    let mut param_1 = param_1;
    let mut param_3 = param_3;
    let mut i_var1: i32 = param_3 / 0x7c;
    if i_var1 == 0 {
        i_var1 = param_3;
    }
    let mut local_4: i32 = 0x7d;
    loop {
        let mut u_var3: u32 = 0;
        let mut i_var2: i32 = i_var1;
        let mut pb_var4: *mut u8 = param_2;
        if i_var1 > 0 {
            loop {
                if param_3 != 0 {
                    // SAFETY: param_2 points at the caller's buffer, read where the original reads.
                    unsafe {
                        pb_var4 = pb_var4.byte_add(1);
                        param_3 = param_3.wrapping_sub(1);
                        let sum = u_var3.wrapping_add(u32::from(pb_var4.read()));
                        u_var3 = sum.wrapping_mul(2) | u32::from((sum as i32) < 0);
                    }
                }
                i_var2 = i_var2.wrapping_sub(1);
                if i_var2 == 0 {
                    break;
                }
            }
        }
        // SAFETY: param_1 points at the caller's output array of 0x7d words.
        unsafe {
            param_1.write_unaligned(u_var3);
            param_1 = param_1.wrapping_add(1);
        }
        local_4 = local_4.wrapping_sub(1);
        if local_4 == 0 {
            break;
        }
    }
    0x7d
}
