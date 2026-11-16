// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00739290: i32;
    static mut DAT_00733798: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00433680(param_1: i32) {
    // SAFETY: the globals are the game's own memory; the walk over the
    // table of 0x238-byte entries reads exactly where the original does.
    unsafe {
        let mut u_var2: i32 = 0;
        if DAT_00739290 != 0 {
            let mut pi_var1: *mut i32 = &raw mut DAT_00733798;
            while pi_var1.read_unaligned() != param_1 {
                pi_var1 = pi_var1.byte_add(0x8e * 4);
                u_var2 = u_var2.wrapping_add(1);
                if DAT_00739290 <= u_var2 {
                    return;
                }
            }
        }
    }
}
