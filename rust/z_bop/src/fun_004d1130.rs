// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetTickCount() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d0070(param_1: *mut i32, param_2: *mut u32) -> *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d1130(
    param_1: *mut i32,
    param_2: *mut u32,
    param_3: i32,
) -> *mut i32 {
    let mut local_8: i32 = 0;
    let mut local_4: u32 = 0;
    // SAFETY: the callees are the game's own; param_1 and param_2 point at
    // caller storage written only on success, as in the original.
    unsafe {
        let d_var1 = GetTickCount();
        loop {
            let d_var2 = GetTickCount();
            if (param_3.wrapping_mul(1000) as u32) < d_var2.wrapping_sub(d_var1) {
                return core::ptr::null_mut();
            }
            let p_var3 = FUN_004d0070(&raw mut local_8, &raw mut local_4);
            if !p_var3.is_null() {
                param_1.write_unaligned(local_8);
                param_2.write_unaligned(local_4);
                return p_var3;
            }
        }
    }
}
