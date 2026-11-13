// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn ShowCursor(bShow: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004de150() -> i32 {
    loop {
        // SAFETY: ShowCursor is the Win32 API call, taking a plain integer.
        let i_var1 = unsafe { ShowCursor(0) };
        if i_var1 >= 0 {
            break;
        }
    }
    1
}
