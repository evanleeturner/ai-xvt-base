// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00665050: i32;
    static mut DAT_00665054: i32;
    static mut DAT_00665064: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d2290(param_1: i32, param_2: u32) -> u32 {
    let mut i_var2: i32 = 0;
    // SAFETY: the globals are the game's own table; pointers are formed
    // from their addresses and accessed where the original does.
    unsafe {
        let mut pi_var1: *mut i32 = &raw mut DAT_00665050;
        while param_1 != pi_var1.read_unaligned() && pi_var1.read_unaligned() != 0 {
            pi_var1 = pi_var1.wrapping_add(6);
            i_var2 = i_var2.wrapping_add(1);
            if (pi_var1 as usize) >= 0x665410usize {
                return 1;
            }
        }
        let off = (i_var2.wrapping_mul(6)) as isize;
        (&raw mut DAT_00665050)
            .wrapping_offset(off)
            .write_unaligned(param_1);
        (&raw mut DAT_00665054)
            .wrapping_offset(off)
            .write_unaligned(param_2 as i32);
        (&raw mut DAT_00665064)
            .wrapping_offset(off)
            .write_unaligned(1);
    }
    1
}
