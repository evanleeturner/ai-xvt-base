// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00acebfd: i32;
    static mut DAT_00acec45: i32;
    static mut DAT_00acec25: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d12f0(param_1: i32) -> *mut i32 {
    // SAFETY: the globals are the game's own memory; the table is walked
    // by address exactly as the original does, with unaligned reads.
    unsafe {
        let mut i_var2: i32 = 0;
        let count = DAT_00acebfd;
        let mut b_var3 = count == 0;
        if count > 0 {
            let mut p_var1: *mut i32 = &raw mut DAT_00acec45;
            loop {
                if p_var1.read_unaligned() == param_1 {
                    break;
                }
                p_var1 = p_var1.wrapping_add(10);
                i_var2 = i_var2.wrapping_add(1);
                if !(i_var2 < DAT_00acebfd) {
                    break;
                }
            }
            b_var3 = DAT_00acebfd == i_var2;
        }
        if b_var3 {
            return core::ptr::null_mut();
        }
        (&raw mut DAT_00acec25).wrapping_offset(i_var2.wrapping_mul(10) as isize)
    }
}
