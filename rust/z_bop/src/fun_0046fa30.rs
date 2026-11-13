// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00999574: i32;
    static mut DAT_009996b4: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0046fa30() -> i32 {
    let mut i_var1: i32 = 0;
    // SAFETY: the globals are the game's own memory; the walk reads the
    // array's entries at a 40-byte stride, as the original does.
    unsafe {
        let mut pi_var2: *mut i32 = &raw mut DAT_00999574;
        let end: *mut i32 = &raw mut DAT_009996b4;
        loop {
            if pi_var2.read_unaligned() == 1 {
                i_var1 = i_var1.wrapping_add(1);
            }
            pi_var2 = pi_var2.wrapping_add(10);
            if !(pi_var2 < end) {
                break;
            }
        }
    }
    i_var1
}
