// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052ba90: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FindWindowA(lpClassName: *const i8, lpWindowName: *const i8) -> *mut core::ffi::c_void;
    fn ShowWindowAsync(hWnd: *mut core::ffi::c_void, nCmdShow: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d5b90() -> u32 {
    // SAFETY: the global string and the Win32 functions are the game's own;
    // the window handle is passed through unchanged.
    unsafe {
        let h_wnd = FindWindowA(
            (&raw mut DAT_0052ba90).cast::<i8>(),
            (&raw mut DAT_0052ba90).cast::<i8>(),
        );
        if !h_wnd.is_null() {
            ShowWindowAsync(h_wnd, 9);
            return 1;
        }
        0
    }
}
