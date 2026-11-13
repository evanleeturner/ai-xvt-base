// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0051a854: i32;
    static mut DAT_005569d4: *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cc4a0(param_1: *mut i8, param_2: *mut core::ffi::c_void) -> i32;
    fn FUN_004cc5c0(param_1: i32, param_2: i32, param_3: i16);
    fn FUN_004cc5e0(param_1: i32) -> u32;
    fn malloc(size: u32) -> *mut core::ffi::c_void;
    fn FUN_004cc6c0(param_1: i32, param_2: *mut i32, param_3: u32) -> i32;
    fn FUN_0044bb90(
        param_1: *mut i32,
        param_2: *mut i32,
        param_3: *mut i32,
        param_4: *mut u32,
    ) -> i32;
    fn FUN_004cc590(param_1: i32);
    fn FUN_0049eb60(param_1: i16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044b790(
    param_1: i32,
    param_2: *mut i8,
    param_3: *mut i32,
    param_4: *mut i32,
    param_5: *mut u32,
) -> i32 {
    let _ = param_1;
    // SAFETY: all callees and globals are the game's own; pointers are passed
    // through exactly as the original does.
    unsafe {
        let p_file = FUN_004cc4a0(param_2, (&raw mut DAT_0051a854).cast::<core::ffi::c_void>());
        if p_file != 0 {
            FUN_004cc5c0(p_file, 0, 2);
            let size = FUN_004cc5e0(p_file);
            FUN_004cc5c0(p_file, 0, 0);
            let pi_var3 = malloc(size).cast::<i32>();
            DAT_005569d4 = pi_var3;
            if pi_var3.is_null() {
                FUN_0049eb60(0);
            } else {
                let b_var1 = FUN_004cc6c0(p_file, pi_var3, size);
                if b_var1 != 0 {
                    let i_var4 = FUN_0044bb90(pi_var3, param_3, param_4, param_5);
                    if i_var4 != 0 {
                        FUN_004cc590(p_file);
                        return 1;
                    }
                }
            }
            FUN_004cc590(p_file);
        }
        0
    }
}
