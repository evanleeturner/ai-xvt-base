// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00645bcc: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn acmStreamPrepareHeader(
        has: *mut core::ffi::c_void,
        pash: *mut core::ffi::c_void,
        fdw_prepare: u32,
    ) -> u32;
    fn acmStreamConvert(
        has: *mut core::ffi::c_void,
        pash: *mut core::ffi::c_void,
        fdw_convert: u32,
    ) -> u32;
    fn acmStreamUnprepareHeader(
        has: *mut core::ffi::c_void,
        pash: *mut core::ffi::c_void,
        fdw_unprepare: u32,
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a7980(
    param_1: u32,
    param_2: *mut u32,
    param_3: u32,
    param_4: *mut i32,
) {
    let mut local_54: [u32; 0x15] = [0; 0x15];
    // SAFETY: param_2 and param_4 point at the caller's values; the global and
    // the ACM functions are the game's own, and the header lives on this frame.
    unsafe {
        local_54[3] = param_3;
        local_54[4] = param_4.read() as u32;
        local_54[7] = param_1;
        local_54[8] = param_2.read();
        local_54[0] = 0x54;
        let puStack_64 = local_54.as_mut_ptr();
        let hs = DAT_00645bcc as usize as *mut core::ffi::c_void;
        let hdr = puStack_64.cast::<core::ffi::c_void>();
        acmStreamPrepareHeader(hs, hdr, 0);
        acmStreamConvert(hs, hdr, 0);
        acmStreamUnprepareHeader(hs, hdr, 0);
        param_4.write(puStack_64 as usize as u32 as i32);
        param_2.write(core::ptr::read_volatile(&local_54[0]));
    }
}
