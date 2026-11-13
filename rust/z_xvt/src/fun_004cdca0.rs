// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00780732: i32;
    static mut DAT_0078077a: i32;
    static mut DAT_0078077e: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cdca0(param_1: i32) -> i32 {
    // SAFETY: the globals are the game's own memory; the table is read at the
    // same strides and indices as the original.
    unsafe {
        let mut i_var1: i32 = 0;
        let mut b_var3: bool = DAT_00780732 == 0;
        if 0 < DAT_00780732 {
            let mut pi_var2: *mut i32 = &raw mut DAT_0078077a;
            loop {
                if pi_var2.read_unaligned() == param_1 {
                    break;
                }
                pi_var2 = pi_var2.wrapping_add(10);
                i_var1 = i_var1.wrapping_add(1);
                if !(i_var1 < DAT_00780732) {
                    break;
                }
            }
            b_var3 = i_var1 == DAT_00780732;
        }
        if b_var3 {
            return 0;
        }
        let base = &raw mut DAT_0078077e;
        base.byte_add((i_var1.wrapping_mul(10) as isize as usize).wrapping_mul(4))
            .cast::<i32>()
            .read_unaligned()
    }
}
