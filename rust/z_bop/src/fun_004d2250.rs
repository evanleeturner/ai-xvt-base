// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00665050: i32;
    static mut DAT_00665410: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d2250(param_1: i32) -> u32 {
    let mut i_var2: i32 = 0;
    let mut pi_var3: *const i32 = &raw mut DAT_00665050;
    // SAFETY: the globals are the game's own table; entries are read where
    // the original reads them.
    unsafe {
        let end: *const i32 = &raw mut DAT_00665410;
        loop {
            if pi_var3.read_unaligned() == param_1 {
                let u_var1 = (i_var2.wrapping_mul(0x18).wrapping_add(0x665064) as usize
                    as *const u32)
                    .read_unaligned();
                if u_var1 == 0 {
                    return 1;
                }
                let n = (i_var2.wrapping_mul(0x18).wrapping_add(0x665054) as usize as *const u32)
                    .read_unaligned();
                return n / u_var1;
            }
            pi_var3 = pi_var3.wrapping_add(6);
            i_var2 = i_var2.wrapping_add(1);
            if pi_var3 >= end {
                break;
            }
        }
    }
    0
}
