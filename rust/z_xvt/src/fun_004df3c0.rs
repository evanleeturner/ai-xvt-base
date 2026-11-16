// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0064fca8: i32;
    static mut DAT_0064fcac: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004df420();
    fn FUN_004df760(a: i32, b: i32, c: i32) -> i32;
    fn FUN_004db6c0(a: *mut core::ffi::c_void, b: *mut core::ffi::c_void);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004df3c0() {
    let mut local_buf: [i32; 4] = [0; 4];
    local_buf[0] = 0x78;
    local_buf[2] = 0x208;
    local_buf[1] = 0xbe;
    local_buf[3] = 0x122;
    // SAFETY: the callees and globals are the game's own; local_buf lives
    // on this frame for the duration of the call.
    unsafe {
        let i_var1 = FUN_004df760(0, 0, 0xff);
        DAT_0064fca8 = 0;
        DAT_0064fcac = i_var1 as u8;
        FUN_004db6c0(
            FUN_004df420 as *const () as *mut core::ffi::c_void,
            local_buf.as_mut_ptr().cast::<core::ffi::c_void>(),
        );
    }
}
