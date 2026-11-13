// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005293b0: [i8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b1600(param_1: i32, param_2: *mut i32, param_3: i32) -> *mut i8 {
    // SAFETY: param_2 points at the caller's table of (code, string pointer)
    // pairs; the global is the game's own "Unknown Error" string.
    unsafe {
        let mut pc_var1: *mut i8 = (&raw mut DAT_005293b0).cast::<i8>();
        let mut i_var3: i32 = 0;
        let mut pi_var2: *mut i32 = param_2;
        if 0 < param_3 {
            while pi_var2.read_unaligned() != param_1 {
                i_var3 = i_var3.wrapping_add(1);
                pi_var2 = pi_var2.wrapping_add(2);
                if param_3 <= i_var3 {
                    return pc_var1;
                }
            }
            pc_var1 = param_2
                .byte_add((i_var3.wrapping_mul(2).wrapping_add(1) as usize).wrapping_mul(4))
                .read_unaligned() as usize as *mut i8;
        }
        pc_var1
    }
}
