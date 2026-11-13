// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_007806ee: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(a: u32, b: u32, c: u32, d: *const u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d5290(param_1: i32) {
    // SAFETY: RaiseException and the global are the game's own; the divisor
    // is checked nonzero, and as an i64 it cannot overflow.
    unsafe {
        if param_1 == 0 {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        } else {
            DAT_007806ee = (1000i64 / (param_1 as i64)) as i32;
        }
    }
}
