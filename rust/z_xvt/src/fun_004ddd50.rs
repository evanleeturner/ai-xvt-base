// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn ShowCursor(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004ddd50() -> u32 {
    loop {
        // SAFETY: ShowCursor is the Win32 API, called as the original does.
        let i_var1 = unsafe { ShowCursor(1) };
        if i_var1 >= 0 {
            break;
        }
    }
    1
}
