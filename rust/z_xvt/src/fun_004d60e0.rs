// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00532a10: i8;
    static mut DAT_00523570: i8;
    static mut DAT_0064f640: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _fopen(filename: *mut i8, mode: *mut i8) -> i32;
    fn FUN_004be320(stream: i32, buf: *mut core::ffi::c_void, count: u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d60e0() -> u32 {
    // SAFETY: the callees and globals are the game's own; only addresses of
    // the globals are passed on, as the original does.
    unsafe {
        let h_file = _fopen(&raw mut DAT_00532a10, &raw mut DAT_00523570);
        if h_file == 0 {
            return 0;
        }
        let b_ret = FUN_004be320(
            h_file,
            (&raw mut DAT_0064f640).cast::<core::ffi::c_void>(),
            0x10,
        );
        let cond: u32 = if (b_ret as i16) == 0 { 1 } else { 0 };
        cond.wrapping_sub(1) & 0x64f640
    }
}
