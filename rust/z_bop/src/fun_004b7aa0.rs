// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _strncmp(s1: *const i8, s2: *const i8, n: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b7aa0(param_1: *mut i8, param_2: i32, param_3: *mut i8) -> i32 {
    let mut param_1 = param_1;
    let mut param_2 = param_2;
    let mut i_var2: i32 = 0;
    loop {
        if param_2 < 0 {
            return -1;
        }
        let i_var3: i32 = param_2 >> 1;
        // SAFETY: the callee is the game's own strncmp; param_1 points at the
        // caller's table of 0x44-byte records and param_3 at the key.
        let i_var1 = unsafe {
            _strncmp(
                param_1.byte_offset(i_var3.wrapping_mul(0x44) as isize) as *const i8,
                param_3 as *const i8,
                0x40,
            )
        };
        if i_var1 == 0 {
            return i_var3.wrapping_add(i_var2);
        }
        if param_2 < 1 {
            return -1;
        }
        if i_var1 < 0 {
            i_var2 = i_var3.wrapping_add(1).wrapping_add(i_var2);
            param_2 = param_2.wrapping_sub(i_var3.wrapping_add(1));
            param_1 =
                param_1.wrapping_offset(i_var3.wrapping_mul(0x44).wrapping_add(0x44) as isize);
        } else {
            param_2 = i_var3.wrapping_add(-1);
        }
    }
}
