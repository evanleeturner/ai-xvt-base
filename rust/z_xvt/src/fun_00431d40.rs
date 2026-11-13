// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00431d70(param_1: *mut i32, param_2: *mut u32) -> *mut core::ffi::c_void;
    fn FUN_00430980(param_1: i32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00431d40(param_1: *mut i32, param_2: *mut u32) {
    loop {
        // SAFETY: the callees are the game's own; param_1 points at the
        // caller's int, read where the original reads it, and the returned
        // pointer is non-null when dereferenced.
        unsafe {
            let p_var1 = FUN_00431d70(param_1, param_2) as *mut i32;
            if p_var1.is_null() || param_1.read() == 0 {
                break;
            }
            FUN_00430980(p_var1.read(), p_var1 as usize as i32);
        }
    }
}
