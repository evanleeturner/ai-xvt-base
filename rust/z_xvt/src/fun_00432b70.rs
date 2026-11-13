// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn timeGetTime() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00431d70(param_1: *mut i32, param_2: *mut u32) -> *mut i32;
    fn FUN_00430980(param_1: i32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00432b70(
    param_1: *mut i32,
    param_2: *mut u32,
    param_3: i32,
) -> *mut i32 {
    let mut local_8: i32 = 0;
    let mut local_4: u32 = 0;
    // SAFETY: the callees are the game's own; param_1 and param_2 point at
    // caller storage written only on success, as the original does.
    unsafe {
        let start_time = timeGetTime();
        loop {
            let current_time = timeGetTime();
            if (param_3.wrapping_mul(1000) as u32) < current_time.wrapping_sub(start_time) {
                return core::ptr::null_mut();
            }
            loop {
                let pi_var3 = FUN_00431d70(&raw mut local_8, &raw mut local_4);
                if pi_var3.is_null() {
                    break;
                }
                if local_8 != 0 {
                    param_1.write_unaligned(local_8);
                    param_2.write_unaligned(local_4);
                    return pi_var3;
                }
                FUN_00430980(pi_var3.read_unaligned(), pi_var3 as usize as i32);
            }
        }
    }
}
