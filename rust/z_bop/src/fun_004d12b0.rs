// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00acebfd: i32;
    static mut DAT_00acec45: i32;
    static mut DAT_00acec49: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d12b0(param_1: i32) -> i32 {
    // SAFETY: the globals are the game's own memory; the table entries are
    // read unaligned at the same addresses and strides as the original.
    unsafe {
        let mut i_var1: i32 = 0;
        let cnt = &raw mut DAT_00acebfd;
        let mut b_var3: bool = cnt.read_unaligned() == 0;
        if 0 < cnt.read_unaligned() {
            let mut pi_var2: *mut i32 = &raw mut DAT_00acec45;
            loop {
                if pi_var2.read_unaligned() == param_1 {
                    break;
                }
                pi_var2 = pi_var2.wrapping_add(10);
                i_var1 = i_var1.wrapping_add(1);
                if !(i_var1 < cnt.read_unaligned()) {
                    break;
                }
            }
            b_var3 = cnt.read_unaligned() == i_var1;
        }
        if b_var3 {
            return 0;
        }
        let base: *mut i32 = &raw mut DAT_00acec49;
        base.wrapping_offset(i_var1.wrapping_mul(10) as isize)
            .read_unaligned()
    }
}
