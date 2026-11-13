// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00acec45: i32;
    static mut DAT_00acf145: i32;
    static mut DAT_00acec49: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d1280(param_1: i32) {
    // SAFETY: the globals are the game's own table of 32 entries of 40 bytes;
    // reads and writes are unaligned at the original's byte offsets.
    unsafe {
        let mut i_var1: i32 = 0;
        let mut pi_var2: *mut i32 = &raw mut DAT_00acec45;
        let end: *mut i32 = &raw mut DAT_00acf145;
        loop {
            if pi_var2.read_unaligned() == param_1 {
                break;
            }
            pi_var2 = pi_var2.wrapping_add(10);
            i_var1 = i_var1.wrapping_add(1);
            if !(pi_var2 < end) {
                break;
            }
        }
        if i_var1 != 0x20 {
            let base: *mut i32 = &raw mut DAT_00acec49;
            base.wrapping_offset(i_var1.wrapping_mul(10) as isize)
                .write_unaligned(0);
        }
    }
}
