// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00417900(param_1: *mut u32, param_2: *const u8, param_3: i32) -> u32 {
    let mut param_1 = param_1;
    let mut param_2 = param_2;
    let mut param_3 = param_3;
    let mut i_var1: i32 = param_3 / 0x7c;
    if i_var1 == 0 {
        i_var1 = param_3;
    }
    let mut local_4: i32 = 0x7d;
    loop {
        let mut u_var2: u32 = 0;
        let mut i_var3: i32 = i_var1;
        if 0 < i_var1 {
            loop {
                if param_3 != 0 {
                    // SAFETY: the caller's buffer holds at least param_3 bytes.
                    let b = unsafe { param_2.read() };
                    param_2 = param_2.wrapping_add(1);
                    param_3 = param_3.wrapping_sub(1);
                    u_var2 = u_var2.wrapping_add(b as u32).rotate_left(1);
                }
                i_var3 = i_var3.wrapping_sub(1);
                if i_var3 == 0 {
                    break;
                }
            }
        }
        // SAFETY: the caller's output buffer holds 0x7d words.
        unsafe {
            param_1.write_unaligned(u_var2);
        }
        param_1 = param_1.wrapping_add(1);
        local_4 = local_4.wrapping_sub(1);
        if local_4 == 0 {
            break;
        }
    }
    0x7d
}
