// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7201: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004b4bf0(param_1: *mut i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004b4c70() {
    // SAFETY: the global and the callee are the game's own; the pointer is
    // the game's base address plus an offset, passed through as the original does.
    unsafe {
        if DAT_00aa7201 != 0 {
            let mut i_var1: i32 = 0x87bc;
            loop {
                FUN_004b4bf0(DAT_00aa7201.wrapping_add(i_var1) as *mut i8);
                i_var1 = i_var1.wrapping_sub(0x44);
                if i_var1 < 0 {
                    break;
                }
            }
        }
    }
}
