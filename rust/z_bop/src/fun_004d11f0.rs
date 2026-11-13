// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00acebfd: i32;
    static mut DAT_00acec45: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d11a0(param_1: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d11f0(param_1: i32) {
    // SAFETY: the globals and the callee are the game's own; the pointer
    // walks the game's table of 40-byte entries starting at DAT_00acec45.
    unsafe {
        let mut i_var2: i32 = 0;
        if DAT_00acebfd > 0 {
            let mut p_var3: *mut u32 = &raw mut DAT_00acec45;
            loop {
                let i_var1 = FUN_004d11a0(p_var3.read_unaligned());
                if param_1 == i_var1 {
                    break;
                }
                p_var3 = p_var3.wrapping_add(10);
                i_var2 = i_var2.wrapping_add(1);
                if DAT_00acebfd <= i_var2 {
                    return;
                }
            }
        }
    }
}
