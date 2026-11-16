// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00739290: u32;
    static mut DAT_00733798: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn timeGetTime() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00433540(param_1: i32) -> u32 {
    // SAFETY: the globals and the table they describe are the game's own
    // memory; the pointer walks the table's 0x8e-int records as the original does.
    unsafe {
        let mut u_var3: u32 = 0;
        let mut pi_var1: *mut i32 = &raw mut DAT_00733798;
        if DAT_00739290 != 0 {
            loop {
                if pi_var1.read_unaligned() == param_1 {
                    return u_var3;
                }
                pi_var1 = pi_var1.wrapping_add(0x8e);
                u_var3 = u_var3.wrapping_add(1);
                if !(u_var3 < DAT_00739290) {
                    break;
                }
            }
        }
        if DAT_00739290 == u_var3 && u_var3 < 0x28 {
            pi_var1.write_unaligned(param_1);
            pi_var1.wrapping_add(1).write_unaligned(0x7f);
            pi_var1.wrapping_add(3).write_unaligned(0x7f);
            pi_var1.wrapping_add(5).write_unaligned(0x7f);
            pi_var1.wrapping_add(2).write_unaligned(0x7f);
            pi_var1.wrapping_add(4).write_unaligned(0x7f);
            pi_var1.wrapping_add(6).write_unaligned(0x7f);
            pi_var1.wrapping_add(7).write_unaligned(0);
            pi_var1.wrapping_add(8).write_unaligned(0x39);
            pi_var1.wrapping_add(0x88).write_unaligned(1);
            let d_var2 = timeGetTime();
            pi_var1.wrapping_add(0x89).write_unaligned(d_var2 as i32);
            pi_var1.wrapping_add(0x8a).write_unaligned(0);
            pi_var1.wrapping_add(0x8b).write_unaligned(0);
            DAT_00739290 = DAT_00739290.wrapping_add(1);
        }
        u_var3
    }
}
