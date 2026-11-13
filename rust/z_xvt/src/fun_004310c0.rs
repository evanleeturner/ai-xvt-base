// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_007330ac: i32;
    static mut DAT_00733120: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00431110(param_1: i32, param_2: *mut u32, param_3: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004310c0(param_1: *mut u32, param_2: u32) {
    let mut i_var2: i32 = 0;
    // SAFETY: the globals and callee are the game's own; the table at
    // DAT_00733120 has stride 40 bytes and DAT_007330ac entries.
    unsafe {
        if 0 < DAT_007330ac {
            let mut pi_var1: *mut i32 = &raw mut DAT_00733120;
            loop {
                let a = pi_var1.read_unaligned();
                if a != 0 && pi_var1.add(1).read_unaligned() != 0 {
                    FUN_00431110(a, param_1, param_2);
                }
                pi_var1 = pi_var1.add(10);
                i_var2 = i_var2.wrapping_add(1);
                if !(i_var2 < DAT_007330ac) {
                    break;
                }
            }
        }
    }
}
