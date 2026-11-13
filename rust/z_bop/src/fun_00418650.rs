// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523428: i32;
    static mut DAT_009e9680: [i8; 0];
    static mut DAT_009ecc34: i32;
    static mut DAT_009ed670: [i32; 0];
    static mut DAT_009a8db0: [i32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004186e0(param_1: i32, param_2: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00418650(param_1: i32) {
    // SAFETY: all globals and the callee belong to the game; pointers are
    // computed and read at the same offsets the original uses.
    unsafe {
        if DAT_00523428 == 0 {
            let flag = (&raw mut DAT_009e9680)
                .cast::<i8>()
                .wrapping_offset((param_1 as isize).wrapping_mul(0x5bd))
                .read();
            if flag != 0 && param_1 != DAT_009ecc34 {
                let mut i_var2: i32 = 0;
                let mut pi_var1: *mut i32 = (&raw mut DAT_009ed670)
                    .cast::<i32>()
                    .wrapping_offset((param_1 as isize).wrapping_mul(0x7e9));
                let count_ptr = (&raw mut DAT_009a8db0)
                    .cast::<i32>()
                    .wrapping_offset(param_1 as isize);
                if 0 < count_ptr.read() {
                    loop {
                        if pi_var1.read_unaligned() == 0
                            && pi_var1.wrapping_add(1).read_unaligned() == 2
                        {
                            FUN_004186e0(param_1, pi_var1);
                        } else {
                            pi_var1 = pi_var1.cast::<u8>().wrapping_add(0x12).cast::<i32>();
                            i_var2 = i_var2.wrapping_add(1);
                        }
                        if !(i_var2 < count_ptr.read()) {
                            break;
                        }
                    }
                }
            }
        }
    }
}
