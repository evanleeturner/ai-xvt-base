// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_007330ac: i32;
    static mut DAT_007330e8: i32;
    static mut DAT_00733120: i32;
    static mut DAT_00733260: i32;
    static mut DAT_00733110: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00432cc0(param_1: i32) -> *mut i32 {
    // SAFETY: the globals are the game's own memory; the pointers walk its
    // table exactly as the original does.
    unsafe {
        if DAT_007330ac == 1 {
            return &raw mut DAT_007330e8;
        }
        let mut i_var3: i32 = 0;
        if DAT_007330ac < 1 {
            return DAT_007330ac as usize as *mut i32;
        }
        let mut pi_var1: *mut i32 = &raw mut DAT_00733120;
        let end: *mut i32 = &raw mut DAT_00733260;
        loop {
            let mut i_var4: i32 = 0;
            let mut pi_var2: *mut i32 = &raw mut DAT_00733120;
            loop {
                if pi_var2.read_unaligned() == pi_var1.read_unaligned() {
                    break;
                }
                pi_var2 = pi_var2.wrapping_add(10);
                i_var4 = i_var4.wrapping_add(1);
                if !(pi_var2 < end) {
                    break;
                }
            }
            if param_1 == i_var4 {
                return (&raw mut DAT_00733110).wrapping_offset(i_var3.wrapping_mul(10) as isize);
            }
            pi_var1 = pi_var1.wrapping_add(10);
            i_var3 = i_var3.wrapping_add(1);
            if DAT_007330ac <= i_var3 {
                return pi_var1;
            }
        }
    }
}
