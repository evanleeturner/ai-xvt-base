// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052c004: i32;
    static mut DAT_00665470: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d9ba0(param_1: i32) -> i32 {
    // SAFETY: the globals are the game's own memory; the array is indexed
    // exactly as the original does.
    unsafe {
        if 0x1f < DAT_0052c004 {
            return 0;
        }
        let mut u_var1: u32 = 0;
        if DAT_0052c004 != 0 {
            let mut pi_var2: *mut i32 = &raw mut DAT_00665470;
            loop {
                if pi_var2.read_unaligned() == param_1 {
                    return 1;
                }
                pi_var2 = pi_var2.wrapping_add(1);
                u_var1 = u_var1.wrapping_add(1);
                if !(u_var1 < DAT_0052c004 as u32) {
                    break;
                }
            }
        }
        (&raw mut DAT_00665470)
            .wrapping_offset(DAT_0052c004 as isize)
            .write_unaligned(param_1);
        DAT_0052c004 = DAT_0052c004.wrapping_add(1);
        1
    }
}
