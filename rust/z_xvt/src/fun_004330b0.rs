// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00733124: i32;
    static mut DAT_00733264: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004330b0() -> i32 {
    let mut i_var1: i32 = 0;
    // SAFETY: the globals are the game's own memory; we only take their
    // addresses and read ints at a stride of 10 ints within the table.
    unsafe {
        let mut pi_var2: *mut i32 = &raw mut DAT_00733124;
        let end: *mut i32 = &raw mut DAT_00733264;
        loop {
            if pi_var2.read_unaligned() == 1 {
                i_var1 = i_var1.wrapping_add(1);
            }
            pi_var2 = pi_var2.wrapping_add(10);
            if pi_var2 >= end {
                break;
            }
        }
    }
    i_var1
}
