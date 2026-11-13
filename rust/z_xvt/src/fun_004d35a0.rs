// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn strncmp(s1: *const i8, s2: *const i8, n: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d35a0(param_1: *mut i8, param_2: i32, param_3: *mut i8) -> i32 {
    let mut param_1 = param_1;
    let mut param_2 = param_2;
    let mut i_var2: i32 = 0;
    let mut i_var3: i32;
    loop {
        if param_2 < 0 {
            return -1;
        }
        i_var3 = param_2 >> 1;
        // SAFETY: the caller supplies a table of 0x44-byte records and a
        // key string; strncmp is the C library's.
        let i_var1 = unsafe {
            strncmp(
                param_1.wrapping_byte_offset(i_var3.wrapping_mul(0x44) as isize) as *const i8,
                param_3 as *const i8,
                0x40,
            )
        };
        if i_var1 == 0 {
            break;
        }
        if param_2 < 1 {
            return -1;
        }
        if i_var1 < 0 {
            i_var2 = i_var3.wrapping_add(1).wrapping_add(i_var2);
            param_2 = param_2.wrapping_sub(i_var3.wrapping_add(1));
            param_1 =
                param_1.wrapping_byte_offset(i_var3.wrapping_mul(0x44).wrapping_add(0x44) as isize);
        } else {
            param_2 = i_var3.wrapping_sub(1);
        }
    }
    i_var3.wrapping_add(i_var2)
}
