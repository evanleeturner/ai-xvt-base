// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00449100();
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn DefWindowProcA(hwnd: *mut c_void, msg: u32, wparam: u32, lparam: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004aa7b0(
    param_1: *mut c_void,
    param_2: i32,
    param_3: u32,
    param_4: i32,
) -> i32 {
    // SAFETY: the callee is the game's own function and DefWindowProcA is the
    // system window procedure; arguments are passed through as the original does.
    unsafe {
        if param_2 != 0x311 {
            FUN_00449100();
        }
        if param_2 == 0xf {
            return 0;
        }
        DefWindowProcA(param_1, 0xf, param_3, param_4)
    }
}
