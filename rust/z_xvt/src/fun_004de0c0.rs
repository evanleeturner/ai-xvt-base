// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759af4: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004de0e0(param_1: *mut i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004de0c0() {
    let mut i_var1: i32 = 0xa13b;
    loop {
        // SAFETY: the callee and the global are the game's own; the pointer
        // is passed as the original computes it.
        unsafe {
            FUN_004de0e0(DAT_00759af4.wrapping_add(i_var1) as *mut i8);
        }
        i_var1 = i_var1.wrapping_sub(0x145);
        if i_var1 < 0 {
            break;
        }
    }
}
