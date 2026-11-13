// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005329c4: *const i8;
    static mut DAT_005329ac: *const i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn sprintf(dst: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004d3ca0(param: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn MessageBoxA(
        h_wnd: *mut core::ffi::c_void,
        lp_text: *const i8,
        lp_caption: *const i8,
        u_type: u32,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d41b0(param_1: *mut core::ffi::c_void, param_2: i32) -> i32 {
    let mut local_100: [i8; 256] = [0; 256];
    // SAFETY: the callees and globals are the game's own; local_100 is a
    // 256-byte stack buffer passed where the original passes its buffer.
    unsafe {
        sprintf(local_100.as_mut_ptr(), DAT_005329c4, param_2);
        MessageBoxA(param_1, local_100.as_ptr(), DAT_005329ac, 0);
        FUN_004d3ca0(1);
    }
    0
}
