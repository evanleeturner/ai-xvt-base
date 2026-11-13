// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081b850: i32;
    static mut DAT_00743860: [i32; 8];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004cde00() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004f5c50() -> u8 {
    // SAFETY: the globals and the callee are the game's own memory and code;
    // the array is walked over its eight entries.
    unsafe {
        let mut i_var3: i32 = 0;
        if DAT_0081b850 == 2 {
            return 0;
        }
        let i_var1 = FUN_004cde00();
        let base = &raw mut DAT_00743860 as *mut i32;
        let mut pi_var2 = base;
        let end = base.add(8);
        loop {
            if pi_var2.read() != 0 {
                i_var3 = i_var3.wrapping_add(1);
            }
            pi_var2 = pi_var2.add(1);
            if pi_var2 >= end {
                break;
            }
        }
        (i_var3 == i_var1) as u8
    }
}
