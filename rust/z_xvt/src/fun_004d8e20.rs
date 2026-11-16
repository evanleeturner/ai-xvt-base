// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00532adc: i32;
    static mut DAT_0064f688: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d8e20(param_1: i32) -> i32 {
    // SAFETY: the globals are the game's own memory; the table at
    // DAT_0064f688 is indexed exactly as the original does.
    unsafe {
        if 0x1f < DAT_00532adc {
            return 0;
        }
        let mut i_var1: i32 = 0;
        if DAT_00532adc != 0 {
            let mut pi_var2: *mut i32 = &raw mut DAT_0064f688;
            loop {
                if pi_var2.read_unaligned() == param_1 {
                    return 1;
                }
                pi_var2 = pi_var2.wrapping_add(1);
                i_var1 = i_var1.wrapping_add(1);
                if !(i_var1 < DAT_00532adc) {
                    break;
                }
            }
        }
        let base: *mut i32 = &raw mut DAT_0064f688;
        base.offset(DAT_00532adc as isize).write_unaligned(param_1);
        DAT_00532adc = DAT_00532adc.wrapping_add(1);
        1
    }
}
