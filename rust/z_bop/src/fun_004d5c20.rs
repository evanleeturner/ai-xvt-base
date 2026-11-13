// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052baf4: i8;
    static mut DAT_0051a854: i32;
    static mut DAT_00665420: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cc4a0(
        param_1: *mut core::ffi::c_void,
        param_2: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn FUN_004cc6c0(
        param_1: *mut core::ffi::c_void,
        param_2: *mut core::ffi::c_void,
        param_3: u32,
    ) -> i32;
    fn FUN_004cc590(param_1: *mut core::ffi::c_void) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d5c20() -> u32 {
    // SAFETY: the callees and globals are the game's own; the addresses of
    // the globals are passed through as the original does.
    unsafe {
        let p_var2 = FUN_004cc4a0(
            (&raw mut DAT_0052baf4).cast::<core::ffi::c_void>(),
            (&raw mut DAT_0051a854).cast::<core::ffi::c_void>(),
        );
        if p_var2.is_null() {
            return 0;
        }
        let b_var1 = FUN_004cc6c0(
            p_var2,
            (&raw mut DAT_00665420).cast::<core::ffi::c_void>(),
            0x10,
        );
        FUN_004cc590(p_var2);
        let eq = ((b_var1 as i16) == 0) as u32;
        eq.wrapping_sub(1) & 0x665420
    }
}
