// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00780732: i32;
    static mut DAT_0078075a: i32;
    static mut DAT_0078077a: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cdce0(param_1: i32) -> *mut i32 {
    // SAFETY: the globals are the game's own memory; the table is walked
    // in 40-byte steps exactly as the original does.
    unsafe {
        let mut i_var2: i32 = 0;
        let mut b_var3: bool = DAT_00780732 == 0;
        if DAT_00780732 > 0 {
            let mut pi_var1: *mut i32 = &raw mut DAT_0078077a;
            loop {
                if pi_var1.read_unaligned() == param_1 {
                    break;
                }
                pi_var1 = pi_var1.byte_add(40);
                i_var2 = i_var2.wrapping_add(1);
                if !(i_var2 < DAT_00780732) {
                    break;
                }
            }
            b_var3 = DAT_00780732 == i_var2;
        }
        if b_var3 {
            return core::ptr::null_mut();
        }
        (&raw mut DAT_0078075a)
            .cast::<u8>()
            .wrapping_offset((i_var2.wrapping_mul(40)) as isize)
            .cast::<i32>()
    }
}
