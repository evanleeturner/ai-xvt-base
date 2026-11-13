// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetTickCount() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cc9a0(param_1: *mut i32, param_2: *mut u32) -> *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cdb10(
    param_1: *mut i32,
    param_2: *mut u32,
    param_3: i32,
) -> *mut i32 {
    let mut local_8: i32 = 0;
    let mut local_4: u32 = 0;
    // SAFETY: the callees are the game's own; param_1 and param_2 point at
    // caller-provided storage written only on success, as in the original.
    unsafe {
        let d_var1 = GetTickCount();
        loop {
            let d_var2 = GetTickCount();
            if (param_3.wrapping_mul(1000) as u32) < d_var2.wrapping_sub(d_var1) {
                return core::ptr::null_mut();
            }
            let pi_var3 = FUN_004cc9a0(&raw mut local_8, &raw mut local_4);
            if !pi_var3.is_null() {
                *param_1 = local_8;
                *param_2 = local_4;
                return pi_var3;
            }
        }
    }
}
