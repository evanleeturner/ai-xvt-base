// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758d41: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d0430(param_1: *mut i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d04b0() {
    // SAFETY: the global and the callee are the game's own; the pointer
    // passed is the game's table entry at the offset the original computes.
    unsafe {
        if DAT_00758d41 != 0 {
            let mut i_var1: i32 = 0x87bc;
            loop {
                FUN_004d0430(DAT_00758d41.wrapping_add(i_var1) as usize as *mut i8);
                i_var1 = i_var1.wrapping_sub(0x44);
                if i_var1 < 0 {
                    break;
                }
            }
        }
    }
}
